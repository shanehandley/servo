/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use servo_url::ServoUrl;

use crate::dom::bindings::codegen::Bindings::NavigationHistoryEntryBinding::NavigationHistoryEntry_Binding::NavigationHistoryEntryMethods;
use crate::dom::bindings::codegen::Bindings::NavigationDestinationBinding::NavigationDestinationMethods;
use crate::dom::bindings::reflector::{reflect_dom_object_with_proto, DomObject, Reflector};
use crate::dom::bindings::str::{DOMString, USVString};
use crate::script_runtime::JSContext;

use super::navigationhistoryentry::NavigationHistoryEntry;

/// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#the-navigationdestination-interface>
#[dom_struct]
pub struct NavigationDestination {
    reflector_: Reflector,
    #[no_trace]
    url: ServoUrl,
    key: DOMString,
    id: DOMString,
    same_document: bool,
    entry: Option<NavigationHistoryEntry>,
}

impl NavigationDestination {}

impl NavigationDestinationMethods<crate::DomTypeHolder> for NavigationDestination {
    /// <https://html.spec.whatwg.org/multipage/#dom-navigationdestination-url>
    fn Url(&self) -> USVString {
        USVString::from(self.url.to_string())
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigationdestination-key>
    fn Key(&self) -> DOMString {
        match &self.entry {
            // Step 1. If this's entry is null, then return the empty string.
            None => DOMString::new(),
            // Step 2. Return this's entry's key.
            Some(entry) => entry.Key(),
        }
    }

    fn Id(&self) -> DOMString {
        self.id.clone()
    }

    fn Index(&self) -> i64 {
        todo!()
    }

    fn SameDocument(&self) -> bool {
        false
    }

    fn GetState(&self, cx: JSContext, rval: js::gc::MutableHandleValue) {
        todo!()
    }
}
