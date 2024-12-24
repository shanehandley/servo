/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::rc::Rc;

use dom_struct::dom_struct;
use js::rust::HandleObject;
use uuid::Uuid;

use crate::dom::bindings::codegen::Bindings::EventHandlerBinding::EventHandlerNonNull;
use crate::dom::bindings::codegen::Bindings::NavigationHistoryEntryBinding::NavigationHistoryEntryMethods;
use crate::dom::bindings::import::module::SafeJSContext;
use crate::dom::bindings::reflector::reflect_dom_object_with_proto;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::{DOMString, USVString};
use crate::dom::eventtarget::EventTarget;
use crate::dom::window::Window;
use crate::script_runtime::CanGc;

#[dom_struct]
/// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#navigationhistoryentry>
pub struct NavigationHistoryEntry {
    event_target: EventTarget,
    /// <https://html.spec.whatwg.org/multipage/browsing-the-web.html#session-history-entry>
    ///
    /// The following properties are taken directly from SessionHistoryEntry
    url: String,
    key: String,
    id: String,
}

impl NavigationHistoryEntry {
    pub fn new_with_proto(
        window: &Window,
        proto: Option<HandleObject>,
    ) -> DomRoot<NavigationHistoryEntry> {
        reflect_dom_object_with_proto(
            Box::new(NavigationHistoryEntry::new_inherited()),
            window,
            proto,
            CanGc::note(),
        )
    }

    fn new_inherited() -> NavigationHistoryEntry {
        NavigationHistoryEntry {
            event_target: EventTarget::new_inherited(),
            url: String::new(),
            key: Uuid::new_v4().simple().to_string(),
            id: Uuid::new_v4().simple().to_string(),
        }
    }
}

#[allow(non_snake_case)]
impl NavigationHistoryEntryMethods<crate::DomTypeHolder> for NavigationHistoryEntry {
    fn GetUrl(&self) -> Option<USVString> {
        None
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigationhistoryentry-key>
    fn Key(&self) -> DOMString {
        DOMString::from(self.key.clone())
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigationhistoryentry-id>
    fn Id(&self) -> DOMString {
        DOMString::from(self.id.clone())
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigationhistoryentry-index>
    fn Index(&self) -> i64 {
        0
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigationhistoryentry-samedocument>
    fn SameDocument(&self) -> bool {
        false
    }

    fn GetOndispose(&self) -> Option<Rc<EventHandlerNonNull>> {
        todo!()
    }

    fn SetOndispose(&self, value: Option<Rc<EventHandlerNonNull>>) {
        todo!()
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigationhistoryentry-getstate>
    fn GetState(&self, cx: SafeJSContext, rval: js::gc::MutableHandleValue) {
        todo!()
    }
}
