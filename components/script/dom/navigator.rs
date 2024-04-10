/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::cell::Cell;
use std::convert::TryInto;
use std::sync::LazyLock;

use dom_struct::dom_struct;
use http::header::CONTENT_TYPE;
use http::{HeaderMap, Method};
use ipc_channel::ipc;
use js::jsval::JSVal;
use lazy_static::lazy_static;
use net_traits::request::{
    is_cors_safelisted_request_header, CredentialsMode, RequestBuilder, RequestMode,
};
use servo_url::ServoUrl;

use crate::body::{Extractable, ExtractedBody};
use crate::document_loader::LoadType;
use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::NavigatorBinding::NavigatorMethods;
use crate::dom::bindings::codegen::Bindings::WindowBinding::Window_Binding::WindowMethods;
use crate::dom::bindings::codegen::Bindings::XMLHttpRequestBinding::BodyInit;
use crate::dom::bindings::codegen::UnionTypes::ReadableStreamOrXMLHttpRequestBodyInit;
use crate::dom::bindings::error::{Error, Fallible};
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::{DomRoot, MutNullableDom};
use crate::dom::bindings::str::{DOMString, USVString};
use crate::dom::bindings::utils::to_frozen_array;
use crate::dom::bluetooth::Bluetooth;
use crate::dom::gamepad::Gamepad;
use crate::dom::gamepadevent::GamepadEventType;
use crate::dom::gpu::GPU;
use crate::dom::mediadevices::MediaDevices;
use crate::dom::mediasession::MediaSession;
use crate::dom::mimetypearray::MimeTypeArray;
use crate::dom::navigatorinfo;
use crate::dom::permissions::Permissions;
use crate::dom::pluginarray::PluginArray;
use crate::dom::serviceworkercontainer::ServiceWorkerContainer;
use crate::dom::window::Window;
use crate::dom::xrsystem::XRSystem;
use crate::script_runtime::JSContext;

pub(super) fn hardware_concurrency() -> u64 {
    static CPUS: LazyLock<u64> = LazyLock::new(|| num_cpus::get().try_into().unwrap_or(1));

    *CPUS
}

#[dom_struct]
pub struct Navigator {
    reflector_: Reflector,
    bluetooth: MutNullableDom<Bluetooth>,
    plugins: MutNullableDom<PluginArray>,
    mime_types: MutNullableDom<MimeTypeArray>,
    service_worker: MutNullableDom<ServiceWorkerContainer>,
    xr: MutNullableDom<XRSystem>,
    mediadevices: MutNullableDom<MediaDevices>,
    /// <https://www.w3.org/TR/gamepad/#dfn-gamepads>
    gamepads: DomRefCell<Vec<MutNullableDom<Gamepad>>>,
    permissions: MutNullableDom<Permissions>,
    mediasession: MutNullableDom<MediaSession>,
    gpu: MutNullableDom<GPU>,
    /// <https://www.w3.org/TR/gamepad/#dfn-hasgamepadgesture>
    has_gamepad_gesture: Cell<bool>,
}

impl Navigator {
    fn new_inherited() -> Navigator {
        Navigator {
            reflector_: Reflector::new(),
            bluetooth: Default::default(),
            plugins: Default::default(),
            mime_types: Default::default(),
            service_worker: Default::default(),
            xr: Default::default(),
            mediadevices: Default::default(),
            gamepads: Default::default(),
            permissions: Default::default(),
            mediasession: Default::default(),
            gpu: Default::default(),
            has_gamepad_gesture: Cell::new(false),
        }
    }

    pub fn new(window: &Window) -> DomRoot<Navigator> {
        reflect_dom_object(Box::new(Navigator::new_inherited()), window)
    }

    pub fn xr(&self) -> Option<DomRoot<XRSystem>> {
        self.xr.get()
    }

    pub fn get_gamepad(&self, index: usize) -> Option<DomRoot<Gamepad>> {
        self.gamepads.borrow().get(index).and_then(|g| g.get())
    }

    pub fn set_gamepad(&self, index: usize, gamepad: &Gamepad) {
        if let Some(gamepad_to_set) = self.gamepads.borrow().get(index) {
            gamepad_to_set.set(Some(gamepad));
        }
        if self.has_gamepad_gesture.get() {
            gamepad.set_exposed(true);
            if self.global().as_window().Document().is_fully_active() {
                gamepad.notify_event(GamepadEventType::Connected);
            }
        }
    }

    pub fn remove_gamepad(&self, index: usize) {
        if let Some(gamepad_to_remove) = self.gamepads.borrow_mut().get(index) {
            gamepad_to_remove.set(None);
        }
        self.shrink_gamepads_list();
    }

    /// <https://www.w3.org/TR/gamepad/#dfn-selecting-an-unused-gamepad-index>
    pub fn select_gamepad_index(&self) -> u32 {
        let mut gamepad_list = self.gamepads.borrow_mut();
        if let Some(index) = gamepad_list.iter().position(|g| g.get().is_none()) {
            index as u32
        } else {
            let len = gamepad_list.len();
            gamepad_list.resize_with(len + 1, Default::default);
            len as u32
        }
    }

    fn shrink_gamepads_list(&self) {
        let mut gamepad_list = self.gamepads.borrow_mut();
        for i in (0..gamepad_list.len()).rev() {
            if gamepad_list.get(i).is_none() {
                gamepad_list.remove(i);
            } else {
                break;
            }
        }
    }

    pub fn has_gamepad_gesture(&self) -> bool {
        self.has_gamepad_gesture.get()
    }

    pub fn set_has_gamepad_gesture(&self, has_gamepad_gesture: bool) {
        self.has_gamepad_gesture.set(has_gamepad_gesture);
    }
}

#[allow(non_snake_case)]
impl NavigatorMethods for Navigator {
    // https://html.spec.whatwg.org/multipage/#dom-navigator-product
    fn Product(&self) -> DOMString {
        navigatorinfo::Product()
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-productsub
    fn ProductSub(&self) -> DOMString {
        navigatorinfo::ProductSub()
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-vendor
    fn Vendor(&self) -> DOMString {
        navigatorinfo::Vendor()
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-vendorsub
    fn VendorSub(&self) -> DOMString {
        navigatorinfo::VendorSub()
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-taintenabled
    fn TaintEnabled(&self) -> bool {
        navigatorinfo::TaintEnabled()
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-appname
    fn AppName(&self) -> DOMString {
        navigatorinfo::AppName()
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-appcodename
    fn AppCodeName(&self) -> DOMString {
        navigatorinfo::AppCodeName()
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-platform
    fn Platform(&self) -> DOMString {
        navigatorinfo::Platform()
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-useragent
    fn UserAgent(&self) -> DOMString {
        navigatorinfo::UserAgent(self.global().get_user_agent())
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-appversion
    fn AppVersion(&self) -> DOMString {
        navigatorinfo::AppVersion()
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-navigator-bluetooth
    fn Bluetooth(&self) -> DomRoot<Bluetooth> {
        self.bluetooth.or_init(|| Bluetooth::new(&self.global()))
    }

    // https://html.spec.whatwg.org/multipage/#navigatorlanguage
    fn Language(&self) -> DOMString {
        navigatorinfo::Language()
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-languages
    #[allow(unsafe_code)]
    fn Languages(&self, cx: JSContext) -> JSVal {
        to_frozen_array(&[self.Language()], cx)
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-plugins
    fn Plugins(&self) -> DomRoot<PluginArray> {
        self.plugins.or_init(|| PluginArray::new(&self.global()))
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-mimetypes
    fn MimeTypes(&self) -> DomRoot<MimeTypeArray> {
        self.mime_types
            .or_init(|| MimeTypeArray::new(&self.global()))
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-javaenabled
    fn JavaEnabled(&self) -> bool {
        false
    }

    // https://w3c.github.io/ServiceWorker/#navigator-service-worker-attribute
    fn ServiceWorker(&self) -> DomRoot<ServiceWorkerContainer> {
        self.service_worker
            .or_init(|| ServiceWorkerContainer::new(&self.global()))
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-cookieenabled
    fn CookieEnabled(&self) -> bool {
        true
    }

    /// <https://www.w3.org/TR/gamepad/#dom-navigator-getgamepads>
    fn GetGamepads(&self) -> Vec<Option<DomRoot<Gamepad>>> {
        let global = self.global();
        let window = global.as_window();
        let doc = window.Document();

        // TODO: Handle permissions policy once implemented
        if !doc.is_fully_active() || !self.has_gamepad_gesture.get() {
            return Vec::new();
        }

        self.gamepads.borrow().iter().map(|g| g.get()).collect()
    }
    // https://w3c.github.io/permissions/#navigator-and-workernavigator-extension
    fn Permissions(&self) -> DomRoot<Permissions> {
        self.permissions
            .or_init(|| Permissions::new(&self.global()))
    }

    /// <https://immersive-web.github.io/webxr/#dom-navigator-xr>
    fn Xr(&self) -> DomRoot<XRSystem> {
        self.xr.or_init(|| XRSystem::new(self.global().as_window()))
    }

    /// <https://w3c.github.io/mediacapture-main/#dom-navigator-mediadevices>
    fn MediaDevices(&self) -> DomRoot<MediaDevices> {
        self.mediadevices
            .or_init(|| MediaDevices::new(&self.global()))
    }

    /// <https://w3c.github.io/mediasession/#dom-navigator-mediasession>
    fn MediaSession(&self) -> DomRoot<MediaSession> {
        self.mediasession.or_init(|| {
            // There is a single MediaSession instance per Pipeline
            // and only one active MediaSession globally.
            //
            // MediaSession creation can happen in two cases:
            //
            // - If content gets `navigator.mediaSession`
            // - If a media instance (HTMLMediaElement so far) starts playing media.
            let global = self.global();
            let window = global.as_window();
            MediaSession::new(window)
        })
    }

    // https://gpuweb.github.io/gpuweb/#dom-navigator-gpu
    fn Gpu(&self) -> DomRoot<GPU> {
        self.gpu.or_init(|| GPU::new(&self.global()))
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigator-hardwareconcurrency>
    fn HardwareConcurrency(&self) -> u64 {
        hardware_concurrency()
    }

    /// <https://w3c.github.io/beacon/#dom-navigator-sendbeacon>
    fn SendBeacon(
        &self,
        url: USVString,
        data: Option<ReadableStreamOrXMLHttpRequestBodyInit>,
    ) -> Fallible<bool> {
        let global = self.global();
        let window = global.as_window();
        let document = window.Document();

        // Step 1: Set base to this's relevant settings object's API base URL.
        let base_url = global.api_base_url();

        // Step 2: Set origin to this's relevant settings object's origin.
        let origin = global.origin().immutable().clone();

        // Step 3: Set parsedUrl to the result of the URL parser steps with url and base. If the
        // algorithm returns an error, or if parsedUrl's scheme is not "http" or "https", throw a
        // "TypeError" exception and terminate these steps.
        let parsed_url = ServoUrl::parse_with_base(Some(&base_url), url.as_ref())
            .map_err(|_| Error::Type("Invalid URL: URL could not be parsed.".to_string()))?;

        if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
            return Err(Error::Type(
                "Invalid URL: URL scheme must be http or https.".to_string(),
            ));
        }

        // Step 4: Let headerList be an empty list.
        let mut header_list = HeaderMap::new();

        // Step 5: Let corsMode be "no-cors".
        let mut request_mode = RequestMode::NoCors;

        let mut request_body: Option<ExtractedBody> = None;

        // Step 6 If data is not null:
        if let Some(request_data) = data {
            // Step 6.1: Set transmittedData and contentType to the result of extracting data's byte
            // stream with the keepalive flag set.

            // If the request data is a ReadableStream, it is incompatible with keepalive requests,
            // and a TypeError should be returned. This is defined in the spec as a step in the
            // BodyInit extraction algorithm, but Servo's implementation does not support keepalive
            // so it is omitted. It's necessary to perform this check before extraction instead.
            // See: https://fetch.spec.whatwg.org/#concept-bodyinit-extract
            if let BodyInit::ReadableStream(_) = request_data {
                return Err(Error::Type(
                    "Cannot extract a ReadableStream if keepalive is true".to_string(),
                ));
            }

            let transmitted_data = request_data
                .extract(&global)
                .map_err(|_| Error::Type("Invalid Data".to_string()))?;

            // Step 6.2: If the amount of data that can be queued to be sent by keepalive enabled
            // requests is exceeded by the size of transmittedData (as defined in
            // HTTP-network-or-cache fetch), set the return value to false and terminate these
            // steps.

            // Servo does not currently implement keepalive, each request is closed on completion.
            // The expectation here is that as additional keepalive requests are made, we must
            // ensure that the total bytes do not exceed a maximum size; correctly determining this
            // is dependent on an implementation of fetch groups. As a compromise until keepalive is
            // implemented, prevent individual request from exceeding the limit of 64 kibibytes as
            // defined in the fetch spec:
            // https://fetch.spec.whatwg.org/#concept-http-network-or-cache-fetch
            // https://fetch.spec.whatwg.org/#fetch-groups
            if let Some(length) = transmitted_data.total_bytes {
                if length > 65_536 {
                    return Ok(false);
                }
            }

            // Step 6.3: If contentType is not null:
            if let Some(content_type) = &transmitted_data.content_type {
                // Set corsMode to "cors".
                request_mode = RequestMode::CorsMode;

                // If contentType value is a CORS-safelisted request-header value for the
                // Content-Type header, set corsMode to "no-cors".
                //
                // https://fetch.spec.whatwg.org/#cors-safelisted-request-header
                if is_cors_safelisted_request_header(&CONTENT_TYPE, &content_type.to_string()) {
                    request_mode = RequestMode::NoCors;
                }

                // Append a Content-Type header with value contentType to headerList.
                header_list.insert(CONTENT_TYPE, content_type.to_string().parse().unwrap());
            }

            request_body = Some(transmitted_data);
        }

        // Step 7: Set the return value to true, return the sendBeacon() call, and continue to run
        // the following steps in parallel...
        // let referrer = global.get_referrer();

        // Step 7: A new request, initialized according to the spec
        // TODO: Mark as a keepalive request once supported
        // TODO: Include initiator_type once supported
        let request = RequestBuilder::new(parsed_url, global.get_referrer())
            .method(Method::POST)
            .mode(request_mode)
            .body(request_body.map(|e| e.into_net_request_body().0))
            .credentials_mode(CredentialsMode::Include)
            .headers(header_list)
            .origin(origin);

        // This is a send and forget request, so a response listener is omitted
        let (action_sender, _) = ipc::channel().unwrap();

        document.fetch_async(LoadType::Beacon, request, action_sender);

        Ok(true)
    }
}
