/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::rc::Rc;
use std::str::FromStr;

use dom_struct::dom_struct;
use http::header::HeaderMap as HyperHeaders;
use hyper_serde::Serde;
use js::rust::{HandleObject, HandleValue};
use net_traits::http_status::HttpStatus;
use servo_url::ServoUrl;
use url::Position;

use crate::body::{BodyMixin, BodyType, Extractable, ExtractedBody, consume_body};
use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::HeadersBinding::HeadersMethods;
use crate::dom::bindings::codegen::Bindings::ResponseBinding;
use crate::dom::bindings::codegen::Bindings::ResponseBinding::{
    ResponseMethods, ResponseType as DOMResponseType,
};
use crate::dom::bindings::codegen::Bindings::XMLHttpRequestBinding::BodyInit;
use crate::dom::bindings::error::{Error, Fallible};
use crate::dom::bindings::reflector::{DomGlobal, Reflector, reflect_dom_object_with_proto};
use crate::dom::bindings::root::{DomRoot, MutNullableDom};
use crate::dom::bindings::str::{ByteString, USVString, serialize_jsval_to_json_utf8};
use crate::dom::globalscope::GlobalScope;
use crate::dom::headers::{Guard, Headers, is_obs_text, is_vchar};
use crate::dom::promise::Promise;
use crate::dom::readablestream::ReadableStream;
use crate::dom::underlyingsourcecontainer::UnderlyingSourceType;
use crate::script_runtime::{CanGc, JSContext, StreamConsumer};

#[dom_struct]
pub(crate) struct Response {
    reflector_: Reflector,
    headers_reflector: MutNullableDom<Headers>,
    #[no_trace]
    status: DomRefCell<HttpStatus>,
    response_type: DomRefCell<DOMResponseType>,
    #[no_trace]
    url: DomRefCell<Option<ServoUrl>>,
    #[no_trace]
    url_list: DomRefCell<Vec<ServoUrl>>,
    /// The stream of <https://fetch.spec.whatwg.org/#body>.
    body_stream: MutNullableDom<ReadableStream>,
    #[ignore_malloc_size_of = "StreamConsumer"]
    stream_consumer: DomRefCell<Option<StreamConsumer>>,
    redirected: DomRefCell<bool>,
}

#[allow(non_snake_case)]
impl Response {
    pub(crate) fn new_inherited(global: &GlobalScope, can_gc: CanGc) -> Response {
        let stream = ReadableStream::new_with_external_underlying_source(
            global,
            UnderlyingSourceType::FetchResponse,
            can_gc,
        )
        .expect("Failed to create ReadableStream with external underlying source");
        Response {
            reflector_: Reflector::new(),
            headers_reflector: Default::default(),
            status: DomRefCell::new(HttpStatus::default()),
            response_type: DomRefCell::new(DOMResponseType::Default),
            url: DomRefCell::new(None),
            url_list: DomRefCell::new(vec![]),
            body_stream: MutNullableDom::new(Some(&*stream)),
            stream_consumer: DomRefCell::new(None),
            redirected: DomRefCell::new(false),
        }
    }

    /// <https://fetch.spec.whatwg.org/#dom-response>
    pub(crate) fn new(global: &GlobalScope, can_gc: CanGc) -> DomRoot<Response> {
        Self::new_with_proto(global, None, can_gc)
    }

    fn new_with_proto(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        can_gc: CanGc,
    ) -> DomRoot<Response> {
        reflect_dom_object_with_proto(
            Box::new(Response::new_inherited(global, can_gc)),
            global,
            proto,
            can_gc,
        )
    }

    pub(crate) fn error_stream(&self, error: Error, can_gc: CanGc) {
        if let Some(body) = self.body_stream.get() {
            body.error_native(error, can_gc);
        }
    }
}

impl BodyMixin for Response {
    fn is_disturbed(&self) -> bool {
        self.body_stream
            .get()
            .is_some_and(|stream| stream.is_disturbed())
    }

    fn is_locked(&self) -> bool {
        self.body_stream
            .get()
            .is_some_and(|stream| stream.is_locked())
    }

    fn body(&self) -> Option<DomRoot<ReadableStream>> {
        self.body_stream.get()
    }

    fn get_mime_type(&self, can_gc: CanGc) -> Vec<u8> {
        let headers = self.Headers(can_gc);
        headers.extract_mime_type()
    }
}

// https://fetch.spec.whatwg.org/#redirect-status
fn is_redirect_status(status: u16) -> bool {
    status == 301 || status == 302 || status == 303 || status == 307 || status == 308
}

// https://tools.ietf.org/html/rfc7230#section-3.1.2
fn is_valid_status_text(status_text: &ByteString) -> bool {
    // reason-phrase  = *( HTAB / SP / VCHAR / obs-text )
    for byte in status_text.iter() {
        if !(*byte == b'\t' || *byte == b' ' || is_vchar(*byte) || is_obs_text(*byte)) {
            return false;
        }
    }
    true
}

// https://fetch.spec.whatwg.org/#null-body-status
fn is_null_body_status(status: u16) -> bool {
    status == 101 || status == 204 || status == 205 || status == 304
}

impl ResponseMethods<crate::DomTypeHolder> for Response {
    /// <https://fetch.spec.whatwg.org/#dom-response>
    fn Constructor(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        body_init: Option<BodyInit>,
        init: &ResponseBinding::ResponseInit,
    ) -> Fallible<DomRoot<Response>> {
        // 1. Set this’s response to a new response.
        // Our Response/Body types don't actually hold onto an internal fetch Response.
        let response = Response::new_with_proto(global, proto, can_gc);

        // 2. Set this’s headers to a new Headers object with this’s relevant realm,
        // whose header list is this’s response’s header list and guard is "response".
        response.Headers(can_gc).set_guard(Guard::Response);

        // 3. Let bodyWithType be null.
        // 4. If body is non-null, then set bodyWithType to the result of extracting body.
        let body_with_type = match body_init {
            Some(body) => Some(body.extract(global, can_gc)?),
            None => None,
        };

        // 5. Perform *initialize a response* given this, init, and bodyWithType.
        initialize_response(global, can_gc, body_with_type, init, response)
    }

    /// <https://fetch.spec.whatwg.org/#dom-response-error>
    fn Error(global: &GlobalScope, can_gc: CanGc) -> DomRoot<Response> {
        let response = Response::new(global, can_gc);
        *response.response_type.borrow_mut() = DOMResponseType::Error;
        response.Headers(can_gc).set_guard(Guard::Immutable);
        *response.status.borrow_mut() = HttpStatus::new_error();
        response
    }

    /// <https://fetch.spec.whatwg.org/#dom-response-redirect>
    fn Redirect(
        global: &GlobalScope,
        url: USVString,
        status: u16,
        can_gc: CanGc,
    ) -> Fallible<DomRoot<Response>> {
        // Step 1
        let base_url = global.api_base_url();
        let parsed_url = base_url.join(&url.0);

        // Step 2
        let url = match parsed_url {
            Ok(url) => url,
            Err(_) => return Err(Error::Type("ServoUrl could not be parsed".to_string())),
        };

        // Step 3
        if !is_redirect_status(status) {
            return Err(Error::Range("status is not a redirect status".to_string()));
        }

        // Step 4
        // see Step 4 continued
        let response = Response::new(global, can_gc);

        // Step 5
        *response.status.borrow_mut() = HttpStatus::new_raw(status, vec![]);

        // Step 6
        let url_bytestring =
            ByteString::from_str(url.as_str()).unwrap_or(ByteString::new(b"".to_vec()));
        response
            .Headers(can_gc)
            .Set(ByteString::new(b"Location".to_vec()), url_bytestring)?;

        // Step 4 continued
        // Headers Guard is set to Immutable here to prevent error in Step 6
        response.Headers(can_gc).set_guard(Guard::Immutable);

        // Step 7
        Ok(response)
    }

    /// <https://fetch.spec.whatwg.org/#dom-response-json>
    #[allow(unsafe_code)]
    fn CreateFromJson(
        cx: JSContext,
        global: &GlobalScope,
        data: HandleValue,
        init: &ResponseBinding::ResponseInit,
        can_gc: CanGc,
    ) -> Fallible<DomRoot<Response>> {
        // 1. Let bytes the result of running serialize a JavaScript value to JSON bytes on data.
        let json_str = serialize_jsval_to_json_utf8(cx, data)?;

        // 2. Let body be the result of extracting bytes
        // The spec's definition of JSON bytes is a UTF-8 encoding so using a DOMString here handles
        // the encoding part.
        let body_init = BodyInit::String(json_str);
        let mut body = body_init.extract(global, can_gc)?;

        // 3. Let responseObject be the result of creating a Response object, given a new response,
        // "response", and the current realm.
        let response = Response::new(global, can_gc);
        response.Headers(can_gc).set_guard(Guard::Response);

        // 4. Perform initialize a response given responseObject, init, and (body, "application/json").
        body.content_type = Some("application/json".into());
        initialize_response(global, can_gc, Some(body), init, response)
    }

    /// <https://fetch.spec.whatwg.org/#dom-response-type>
    fn Type(&self) -> DOMResponseType {
        *self.response_type.borrow() //into()
    }

    /// <https://fetch.spec.whatwg.org/#dom-response-url>
    fn Url(&self) -> USVString {
        USVString(String::from(
            (*self.url.borrow())
                .as_ref()
                .map(serialize_without_fragment)
                .unwrap_or(""),
        ))
    }

    /// <https://fetch.spec.whatwg.org/#dom-response-redirected>
    fn Redirected(&self) -> bool {
        return *self.redirected.borrow();
    }

    /// <https://fetch.spec.whatwg.org/#dom-response-status>
    fn Status(&self) -> u16 {
        self.status.borrow().raw_code()
    }

    /// <https://fetch.spec.whatwg.org/#dom-response-ok>
    fn Ok(&self) -> bool {
        self.status.borrow().is_success()
    }

    /// <https://fetch.spec.whatwg.org/#dom-response-statustext>
    fn StatusText(&self) -> ByteString {
        ByteString::new(self.status.borrow().message().to_vec())
    }

    /// <https://fetch.spec.whatwg.org/#dom-response-headers>
    fn Headers(&self, can_gc: CanGc) -> DomRoot<Headers> {
        self.headers_reflector
            .or_init(|| Headers::for_response(&self.global(), can_gc))
    }

    /// <https://fetch.spec.whatwg.org/#dom-response-clone>
    fn Clone(&self, can_gc: CanGc) -> Fallible<DomRoot<Response>> {
        // Step 1
        if self.is_locked() || self.is_disturbed() {
            return Err(Error::Type("cannot clone a disturbed response".to_string()));
        }

        // Step 2
        let new_response = Response::new(&self.global(), can_gc);
        new_response
            .Headers(can_gc)
            .copy_from_headers(self.Headers(can_gc))?;
        new_response
            .Headers(can_gc)
            .set_guard(self.Headers(can_gc).get_guard());

        // https://fetch.spec.whatwg.org/#concept-response-clone
        // Instead of storing a net_traits::Response internally, we
        // only store the relevant fields, and only clone them here
        *new_response.response_type.borrow_mut() = *self.response_type.borrow();
        new_response
            .status
            .borrow_mut()
            .clone_from(&self.status.borrow());
        new_response.url.borrow_mut().clone_from(&self.url.borrow());
        new_response
            .url_list
            .borrow_mut()
            .clone_from(&self.url_list.borrow());

        if let Some(stream) = self.body_stream.get().clone() {
            new_response.body_stream.set(Some(&*stream));
        }

        // Step 3
        // TODO: This step relies on promises, which are still unimplemented.

        // Step 4
        Ok(new_response)
    }

    /// <https://fetch.spec.whatwg.org/#dom-body-bodyused>
    fn BodyUsed(&self) -> bool {
        self.is_disturbed()
    }

    /// <https://fetch.spec.whatwg.org/#dom-body-body>
    fn GetBody(&self) -> Option<DomRoot<ReadableStream>> {
        self.body()
    }

    /// <https://fetch.spec.whatwg.org/#dom-body-text>
    fn Text(&self, can_gc: CanGc) -> Rc<Promise> {
        consume_body(self, BodyType::Text, can_gc)
    }

    /// <https://fetch.spec.whatwg.org/#dom-body-blob>
    fn Blob(&self, can_gc: CanGc) -> Rc<Promise> {
        consume_body(self, BodyType::Blob, can_gc)
    }

    /// <https://fetch.spec.whatwg.org/#dom-body-formdata>
    fn FormData(&self, can_gc: CanGc) -> Rc<Promise> {
        consume_body(self, BodyType::FormData, can_gc)
    }

    /// <https://fetch.spec.whatwg.org/#dom-body-json>
    fn Json(&self, can_gc: CanGc) -> Rc<Promise> {
        consume_body(self, BodyType::Json, can_gc)
    }

    /// <https://fetch.spec.whatwg.org/#dom-body-arraybuffer>
    fn ArrayBuffer(&self, can_gc: CanGc) -> Rc<Promise> {
        consume_body(self, BodyType::ArrayBuffer, can_gc)
    }

    /// <https://fetch.spec.whatwg.org/#dom-body-bytes>
    fn Bytes(&self, can_gc: CanGc) -> std::rc::Rc<Promise> {
        consume_body(self, BodyType::Bytes, can_gc)
    }
}

/// <https://fetch.spec.whatwg.org/#initialize-a-response>
fn initialize_response(
    global: &GlobalScope,
    can_gc: CanGc,
    body: Option<ExtractedBody>,
    init: &ResponseBinding::ResponseInit,
    response: DomRoot<Response>,
) -> Result<DomRoot<Response>, Error> {
    // 1. If init["status"] is not in the range 200 to 599, inclusive, then throw a RangeError.
    if init.status < 200 || init.status > 599 {
        return Err(Error::Range(format!(
            "init's status member should be in the range 200 to 599, inclusive, but is {}",
            init.status
        )));
    }

    // 2. If init["statusText"] is not the empty string and does not match the reason-phrase token production,
    // then throw a TypeError.
    if !is_valid_status_text(&init.statusText) {
        return Err(Error::Type(
            "init's statusText member does not match the reason-phrase token production"
                .to_string(),
        ));
    }

    // 3. Set response’s response’s status to init["status"].
    // 4. Set response’s response’s status message to init["statusText"].
    *response.status.borrow_mut() =
        HttpStatus::new_raw(init.status, init.statusText.clone().into());

    // 5. If init["headers"] exists, then fill response’s headers with init["headers"].
    if let Some(ref headers_member) = init.headers {
        response
            .Headers(can_gc)
            .fill(Some(headers_member.clone()))?;
    }

    // 6. If body is non-null, then:
    if let Some(ref body) = body {
        // 6.1 If response’s status is a null body status, then throw a TypeError.
        if is_null_body_status(init.status) {
            return Err(Error::Type(
                "Body is non-null but init's status member is a null body status".to_string(),
            ));
        };

        // 6.2 Set response’s body to body’s body.
        response.body_stream.set(Some(&*body.stream));

        // 6.3 If body’s type is non-null and response’s header list does not contain `Content-Type`,
        // then append (`Content-Type`, body’s type) to response’s header list.
        if let Some(content_type_contents) = &body.content_type {
            if !response
                .Headers(can_gc)
                .Has(ByteString::new(b"Content-Type".to_vec()))
                .unwrap()
            {
                response.Headers(can_gc).Append(
                    ByteString::new(b"Content-Type".to_vec()),
                    ByteString::new(content_type_contents.as_bytes().to_vec()),
                )?;
            }
        };
    } else {
        // Reset FetchResponse to an in-memory stream with empty byte sequence here for
        // no-init-body case. This is because the Response/Body types here do not hold onto a
        // fetch Response object.
        let stream = ReadableStream::new_from_bytes(global, Vec::with_capacity(0), can_gc)?;
        response.body_stream.set(Some(&*stream));
    }

    Ok(response)
}

fn serialize_without_fragment(url: &ServoUrl) -> &str {
    &url[..Position::AfterQuery]
}

impl Response {
    pub(crate) fn set_type(&self, new_response_type: DOMResponseType, can_gc: CanGc) {
        *self.response_type.borrow_mut() = new_response_type;
        self.set_response_members_by_type(new_response_type, can_gc);
    }

    pub(crate) fn set_headers(
        &self,
        option_hyper_headers: Option<Serde<HyperHeaders>>,
        can_gc: CanGc,
    ) {
        self.Headers(can_gc)
            .set_headers(match option_hyper_headers {
                Some(hyper_headers) => hyper_headers.into_inner(),
                None => HyperHeaders::new(),
            });
    }

    pub(crate) fn set_status(&self, status: &HttpStatus) {
        self.status.borrow_mut().clone_from(status);
    }

    pub(crate) fn set_final_url(&self, final_url: ServoUrl) {
        *self.url.borrow_mut() = Some(final_url);
    }

    pub(crate) fn set_redirected(&self, is_redirected: bool) {
        *self.redirected.borrow_mut() = is_redirected;
    }

    fn set_response_members_by_type(&self, response_type: DOMResponseType, can_gc: CanGc) {
        match response_type {
            DOMResponseType::Error => {
                *self.status.borrow_mut() = HttpStatus::new_error();
                self.set_headers(None, can_gc);
            },
            DOMResponseType::Opaque => {
                *self.url_list.borrow_mut() = vec![];
                *self.status.borrow_mut() = HttpStatus::new_error();
                self.set_headers(None, can_gc);
                self.body_stream.set(None);
            },
            DOMResponseType::Opaqueredirect => {
                *self.status.borrow_mut() = HttpStatus::new_error();
                self.set_headers(None, can_gc);
                self.body_stream.set(None);
            },
            DOMResponseType::Default => {},
            DOMResponseType::Basic => {},
            DOMResponseType::Cors => {},
        }
    }

    pub(crate) fn set_stream_consumer(&self, sc: Option<StreamConsumer>) {
        *self.stream_consumer.borrow_mut() = sc;
    }

    pub(crate) fn stream_chunk(&self, chunk: Vec<u8>, can_gc: CanGc) {
        // Note, are these two actually mutually exclusive?
        if let Some(stream_consumer) = self.stream_consumer.borrow().as_ref() {
            stream_consumer.consume_chunk(chunk.as_slice());
        } else if let Some(body) = self.body_stream.get() {
            body.enqueue_native(chunk, can_gc);
        }
    }

    #[cfg_attr(crown, allow(crown::unrooted_must_root))]
    pub(crate) fn finish(&self, can_gc: CanGc) {
        if let Some(body) = self.body_stream.get() {
            body.controller_close_native(can_gc);
        }
        let stream_consumer = self.stream_consumer.borrow_mut().take();
        if let Some(stream_consumer) = stream_consumer {
            stream_consumer.stream_end();
        }
    }
}
