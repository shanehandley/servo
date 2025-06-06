/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! The [Response](https://fetch.spec.whatwg.org/#responses) object
//! resulting from a [fetch operation](https://fetch.spec.whatwg.org/#concept-fetch)
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;

use http::HeaderMap;
use hyper_serde::Serde;
use malloc_size_of_derive::MallocSizeOf;
use serde::{Deserialize, Serialize};
use servo_arc::Arc;
use servo_url::ServoUrl;

use crate::fetch::headers::extract_mime_type_as_mime;
use crate::http_status::HttpStatus;
use crate::{
    FetchMetadata, FilteredMetadata, Metadata, NetworkError, ReferrerPolicy, ResourceFetchTiming,
    ResourceTimingType,
};

/// [Response type](https://fetch.spec.whatwg.org/#concept-response-type)
#[derive(Clone, Debug, Deserialize, MallocSizeOf, PartialEq, Serialize)]
pub enum ResponseType {
    Basic,
    Cors,
    Default,
    Error(NetworkError),
    Opaque,
    OpaqueRedirect,
}

/// [Response termination reason](https://fetch.spec.whatwg.org/#concept-response-termination-reason)
#[derive(Clone, Copy, Debug, Deserialize, MallocSizeOf, Serialize)]
pub enum TerminationReason {
    EndUserAbort,
    Fatal,
    Timeout,
}

/// The response body can still be pushed to after fetch
/// This provides a way to store unfinished response bodies
#[derive(Clone, Debug, MallocSizeOf, PartialEq)]
pub enum ResponseBody {
    Empty, // XXXManishearth is this necessary, or is Done(vec![]) enough?
    Receiving(Vec<u8>),
    Done(Vec<u8>),
}

impl ResponseBody {
    pub fn is_done(&self) -> bool {
        match *self {
            ResponseBody::Done(..) => true,
            ResponseBody::Empty | ResponseBody::Receiving(..) => false,
        }
    }
}

/// [Cache state](https://fetch.spec.whatwg.org/#concept-response-cache-state)
#[derive(Clone, Debug, Deserialize, MallocSizeOf, Serialize)]
pub enum CacheState {
    None,
    Local,
    Validated,
    Partial,
}

/// [Https state](https://fetch.spec.whatwg.org/#concept-response-https-state)
#[derive(Clone, Copy, Debug, Deserialize, MallocSizeOf, PartialEq, Serialize)]
pub enum HttpsState {
    None,
    Deprecated,
    Modern,
}

#[derive(Clone, Debug, Deserialize, MallocSizeOf, Serialize)]
pub struct ResponseInit {
    pub url: ServoUrl,
    #[serde(
        deserialize_with = "::hyper_serde::deserialize",
        serialize_with = "::hyper_serde::serialize"
    )]
    #[ignore_malloc_size_of = "Defined in hyper"]
    pub headers: HeaderMap,
    pub status_code: u16,
    pub referrer: Option<ServoUrl>,
    pub location_url: Option<Result<ServoUrl, String>>,
}

/// A [Response](https://fetch.spec.whatwg.org/#concept-response) as defined by the Fetch spec
#[derive(Clone, Debug, MallocSizeOf)]
pub struct Response {
    pub response_type: ResponseType,
    pub termination_reason: Option<TerminationReason>,
    url: Option<ServoUrl>,
    pub url_list: Vec<ServoUrl>,
    pub status: HttpStatus,
    #[ignore_malloc_size_of = "Defined in hyper"]
    pub headers: HeaderMap,
    #[ignore_malloc_size_of = "Mutex heap size undefined"]
    pub body: Arc<Mutex<ResponseBody>>,
    pub cache_state: CacheState,
    pub https_state: HttpsState,
    pub referrer: Option<ServoUrl>,
    pub referrer_policy: ReferrerPolicy,
    /// [CORS-exposed header-name list](https://fetch.spec.whatwg.org/#concept-response-cors-exposed-header-name-list)
    pub cors_exposed_header_name_list: Vec<String>,
    /// [Location URL](https://fetch.spec.whatwg.org/#concept-response-location-url)
    pub location_url: Option<Result<ServoUrl, String>>,
    /// [Internal response](https://fetch.spec.whatwg.org/#concept-internal-response), only used if the Response
    /// is a filtered response
    pub internal_response: Option<Box<Response>>,
    /// whether or not to try to return the internal_response when asked for actual_response
    pub return_internal: bool,
    /// <https://fetch.spec.whatwg.org/#concept-response-aborted>
    #[ignore_malloc_size_of = "AtomicBool heap size undefined"]
    pub aborted: Arc<AtomicBool>,
    /// track network metrics
    #[ignore_malloc_size_of = "Mutex heap size undefined"]
    pub resource_timing: Arc<Mutex<ResourceFetchTiming>>,

    /// <https://fetch.spec.whatwg.org/#concept-response-range-requested-flag>
    pub range_requested: bool,
}

impl Response {
    pub fn new(url: ServoUrl, resource_timing: ResourceFetchTiming) -> Response {
        Response {
            response_type: ResponseType::Default,
            termination_reason: None,
            url: Some(url),
            url_list: vec![],
            status: HttpStatus::default(),
            headers: HeaderMap::new(),
            body: Arc::new(Mutex::new(ResponseBody::Empty)),
            cache_state: CacheState::None,
            https_state: HttpsState::None,
            referrer: None,
            referrer_policy: ReferrerPolicy::EmptyString,
            cors_exposed_header_name_list: vec![],
            location_url: None,
            internal_response: None,
            return_internal: true,
            aborted: Arc::new(AtomicBool::new(false)),
            resource_timing: Arc::new(Mutex::new(resource_timing)),
            range_requested: false,
        }
    }

    pub fn from_init(init: ResponseInit, resource_timing_type: ResourceTimingType) -> Response {
        let mut res = Response::new(init.url, ResourceFetchTiming::new(resource_timing_type));
        res.location_url = init.location_url;
        res.headers = init.headers;
        res.referrer = init.referrer;
        res.status = HttpStatus::new_raw(init.status_code, vec![]);
        res
    }

    pub fn network_error(e: NetworkError) -> Response {
        Response {
            response_type: ResponseType::Error(e),
            termination_reason: None,
            url: None,
            url_list: vec![],
            status: HttpStatus::new_error(),
            headers: HeaderMap::new(),
            body: Arc::new(Mutex::new(ResponseBody::Empty)),
            cache_state: CacheState::None,
            https_state: HttpsState::None,
            referrer: None,
            referrer_policy: ReferrerPolicy::EmptyString,
            cors_exposed_header_name_list: vec![],
            location_url: None,
            internal_response: None,
            return_internal: true,
            aborted: Arc::new(AtomicBool::new(false)),
            resource_timing: Arc::new(Mutex::new(ResourceFetchTiming::new(
                ResourceTimingType::Error,
            ))),
            range_requested: false,
        }
    }

    pub fn network_internal_error<T: Into<String>>(msg: T) -> Response {
        Self::network_error(NetworkError::Internal(msg.into()))
    }

    pub fn url(&self) -> Option<&ServoUrl> {
        self.url.as_ref()
    }

    pub fn is_network_error(&self) -> bool {
        matches!(self.response_type, ResponseType::Error(..))
    }

    pub fn get_network_error(&self) -> Option<&NetworkError> {
        match self.response_type {
            ResponseType::Error(ref e) => Some(e),
            _ => None,
        }
    }

    pub fn actual_response(&self) -> &Response {
        if self.return_internal && self.internal_response.is_some() {
            self.internal_response.as_ref().unwrap()
        } else {
            self
        }
    }

    pub fn actual_response_mut(&mut self) -> &mut Response {
        if self.return_internal && self.internal_response.is_some() {
            self.internal_response.as_mut().unwrap()
        } else {
            self
        }
    }

    pub fn to_actual(self) -> Response {
        if self.return_internal && self.internal_response.is_some() {
            *self.internal_response.unwrap()
        } else {
            self
        }
    }

    pub fn get_resource_timing(&self) -> Arc<Mutex<ResourceFetchTiming>> {
        Arc::clone(&self.resource_timing)
    }

    /// Convert to a filtered response, of type `filter_type`.
    /// Do not use with type Error or Default
    #[rustfmt::skip]
    pub fn to_filtered(self, filter_type: ResponseType) -> Response {
        match filter_type {
            ResponseType::Default |
            ResponseType::Error(..) => panic!(),
            _ => (),
        }

        let old_response = self.to_actual();

        if let ResponseType::Error(e) = old_response.response_type {
            return Response::network_error(e);
        }

        let old_headers = old_response.headers.clone();
        let exposed_headers = old_response.cors_exposed_header_name_list.clone();
        let mut response = old_response.clone();
        response.internal_response = Some(Box::new(old_response));
        response.response_type = filter_type;

        match response.response_type {
            ResponseType::Default |
            ResponseType::Error(..) => unreachable!(),

            ResponseType::Basic => {
                let headers = old_headers.iter().filter(|(name, _)| {
                    !matches!(&*name.as_str().to_ascii_lowercase(), "set-cookie" | "set-cookie2")
                }).map(|(n, v)| (n.clone(), v.clone())).collect();
                response.headers = headers;
            },

            ResponseType::Cors => {
                let headers = old_headers.iter().filter(|(name, _)| {
                    match &*name.as_str().to_ascii_lowercase() {
                        "cache-control" | "content-language" | "content-length" | "content-type" |
                        "expires" | "last-modified" | "pragma" => true,
                        "set-cookie" | "set-cookie2" => false,
                        header => {
                            exposed_headers.iter().any(|h| *header == h.as_str().to_ascii_lowercase())
                        }
                    }
                }).map(|(n, v)| (n.clone(), v.clone())).collect();
                response.headers = headers;
            },

            ResponseType::Opaque => {
                response.url_list = vec![];
                response.url = None;
                response.headers = HeaderMap::new();
                response.status = HttpStatus::new_error();
                response.body = Arc::new(Mutex::new(ResponseBody::Empty));
                response.cache_state = CacheState::None;
            },

            ResponseType::OpaqueRedirect => {
                response.headers = HeaderMap::new();
                response.status = HttpStatus::new_error();
                response.body = Arc::new(Mutex::new(ResponseBody::Empty));
                response.cache_state = CacheState::None;
            },
        }

        response
    }

    pub fn metadata(&self) -> Result<FetchMetadata, NetworkError> {
        fn init_metadata(response: &Response, url: &ServoUrl) -> Metadata {
            let mut metadata = Metadata::default(url.clone());
            metadata.set_content_type(extract_mime_type_as_mime(&response.headers).as_ref());
            metadata.location_url.clone_from(&response.location_url);
            metadata.headers = Some(Serde(response.headers.clone()));
            metadata.status.clone_from(&response.status);
            metadata.https_state = response.https_state;
            metadata.referrer.clone_from(&response.referrer);
            metadata.referrer_policy = response.referrer_policy;
            metadata.redirected = response.actual_response().url_list.len() > 1;
            metadata
        }

        if let Some(error) = self.get_network_error() {
            return Err(error.clone());
        }

        let metadata = self.url.as_ref().map(|url| init_metadata(self, url));

        if let Some(ref response) = self.internal_response {
            match response.url {
                Some(ref url) => {
                    let unsafe_metadata = init_metadata(response, url);

                    match self.response_type {
                        ResponseType::Basic => Ok(FetchMetadata::Filtered {
                            filtered: FilteredMetadata::Basic(metadata.unwrap()),
                            unsafe_: unsafe_metadata,
                        }),
                        ResponseType::Cors => Ok(FetchMetadata::Filtered {
                            filtered: FilteredMetadata::Cors(metadata.unwrap()),
                            unsafe_: unsafe_metadata,
                        }),
                        ResponseType::Default => unreachable!(),
                        ResponseType::Error(ref network_err) => Err(network_err.clone()),
                        ResponseType::Opaque => Ok(FetchMetadata::Filtered {
                            filtered: FilteredMetadata::Opaque,
                            unsafe_: unsafe_metadata,
                        }),
                        ResponseType::OpaqueRedirect => Ok(FetchMetadata::Filtered {
                            filtered: FilteredMetadata::OpaqueRedirect(url.clone()),
                            unsafe_: unsafe_metadata,
                        }),
                    }
                },
                None => Err(NetworkError::Internal(
                    "No url found in unsafe response".to_owned(),
                )),
            }
        } else {
            assert_eq!(self.response_type, ResponseType::Default);
            Ok(FetchMetadata::Unfiltered(metadata.unwrap()))
        }
    }
}
