/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::rust::HandleObject;

use super::bindings::codegen::Bindings::NavigationHistoryEntryBinding::NavigationHistoryEntryMethods;
use crate::dom::bindings::codegen::Bindings::NavigationDestinationBinding::NavigationDestinationMethods;
use crate::dom::bindings::import::module::SafeJSContext;
use crate::dom::bindings::reflector::{reflect_dom_object_with_proto, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::{DOMString, USVString};
use crate::dom::navigationhistoryentry::NavigationHistoryEntry;
use crate::dom::window::Window;
use crate::script_runtime::CanGc;

#[dom_struct]
/// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#the-navigationdestination-interface>
pub struct NavigationDestination {
    reflector_: Reflector,
    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#concept-navigationdestination-url>
    url: USVString,
    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#concept-navigationdestination-entry>
    entry: Option<NavigationHistoryEntry>,
    // TODO: state
    key: DOMString,
    id: DOMString,
}

impl NavigationDestination {
    pub fn new_with_proto(
        window: &Window,
        proto: Option<HandleObject>,
    ) -> DomRoot<NavigationDestination> {
        reflect_dom_object_with_proto(
            Box::new(NavigationDestination::new_inherited()),
            window,
            proto,
            CanGc::note(),
        )
    }

    fn new_inherited() -> NavigationDestination {
        NavigationDestination {
            reflector_: Reflector::new(),
            url: USVString::from(String::new()),
            entry: None,
            key: DOMString::new(),
            id: DOMString::new(),
        }
    }
}

#[allow(non_snake_case)]
impl NavigationDestinationMethods<crate::DomTypeHolder> for NavigationDestination {
    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigationdestination-url>
    fn Url(&self) -> USVString {
        USVString(String::from("TODO"))
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigationdestination-key>
    fn Key(&self) -> DOMString {
        match &self.entry {
            // Step 1: If this's entry is null, then return the empty string.
            None => DOMString::new(),
            // Step 2: Return this's entry's key.
            Some(entry) => entry.Key(),
        }
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigationdestination-id>
    fn Id(&self) -> DOMString {
        match &self.entry {
            // Step 1: If this's entry is null, then return the empty string.
            None => DOMString::new(),
            // Step 2: Return this's entry's id.
            Some(entry) => entry.Id(),
        }
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigationdestination-index>
    fn Index(&self) -> i64 {
        match &self.entry {
            // Step 1: If this's entry is null, then return −1.
            None => -1,
            // Step 2: Return this's entry's index.
            Some(entry) => entry.Index(),
        }
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigationdestination-samedocument>
    fn SameDocument(&self) -> bool {
        true
    }

    fn GetState(&self, cx: SafeJSContext, rval: js::gc::MutableHandleValue) {
        todo!()
    }
}
