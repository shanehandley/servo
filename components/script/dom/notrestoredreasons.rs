/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::jsval::JSVal;

use crate::dom::bindings::codegen::Bindings::NotRestoredReasonsBinding::NotRestoredReasonsMethods;
use crate::dom::bindings::import::module::SafeJSContext;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::str::DOMString;
use crate::dom::notrestoredreasondetails::NotRestoredReasonDetails;

#[dom_struct]
/// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#the-notrestoredreasons-interface>
pub struct NotRestoredReasons {
    reflector_: Reflector,
    reasons: Option<Vec<NotRestoredReasonDetails>>,
    children: Option<Vec<NotRestoredReasons>>,
    src: Option<DOMString>,
    id: Option<DOMString>,
    name: Option<DOMString>,
    url: Option<DOMString>,
}

impl NotRestoredReasons {}

impl NotRestoredReasonsMethods for NotRestoredReasons {
    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-not-restored-reasons-src>
    fn GetSrc(&self) -> Option<DOMString> {
        self.src.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-not-restored-reasons-id>
    fn GetId(&self) -> Option<DOMString> {
        self.id.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-not-restored-reasons-name>
    fn GetName(&self) -> Option<DOMString> {
        self.name.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-not-restored-reasons-url>
    fn GetUrl(&self) -> Option<DOMString> {
        self.url.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-not-restored-reasons-reasons>
    fn Reasons(&self, cx: SafeJSContext) -> JSVal {
        todo!()
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-not-restored-reasons-children>
    fn Children(&self, cx: SafeJSContext) -> JSVal {
        todo!()
    }
}
