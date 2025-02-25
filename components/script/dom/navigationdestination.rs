/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use servo_url::ServoUrl;
use js::rust::MutableHandleValue;
use script_traits::StructuredSerializedData;

use crate::dom::bindings::codegen::Bindings::NavigationHistoryEntryBinding::NavigationHistoryEntry_Binding::NavigationHistoryEntryMethods;
use crate::dom::bindings::codegen::Bindings::NavigationDestinationBinding::NavigationDestinationMethods;
use crate::dom::bindings::reflector::{DomObject, Reflector};
use crate::dom::bindings::str::{DOMString, USVString};
use crate::dom::navigationhistoryentry::NavigationHistoryEntry;
use crate::script_runtime::JSContext;

/// <https://html.spec.whatwg.org/multipage/#the-navigationdestination-interface>
#[dom_struct]
pub struct NavigationDestination {
    reflector_: Reflector,
    #[no_trace]
    url: ServoUrl,
    key: DOMString,
    id: DOMString,
    same_document: bool,
    entry: Option<NavigationHistoryEntry>,
    #[no_trace]
    state: StructuredSerializedData,
}

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

    /// <https://html.spec.whatwg.org/multipage/#dom-navigationdestination-id>
    fn Id(&self) -> DOMString {
        match &self.entry {
            // Step 1. If this's entry is null, then return the empty string.
            None => DOMString::new(),
            // Step 2. Return this's entry's ID.
            Some(history_entry) => history_entry.Id(),
        }
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigationdestination-index>
    fn Index(&self) -> i64 {
        match &self.entry {
            // Step 1. If this's entry is null, then return -1
            None => -1,
            // Step 2. Return this's entry's index.
            Some(history_entry) => history_entry.Index(),
        }
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigationdestination-samedocument>
    fn SameDocument(&self) -> bool {
        self.same_document
    }

    /// The getState() method steps are to return StructuredDeserialize(this's state).
    ///
    /// <https://html.spec.whatwg.org/multipage/#dom-navigationdestination-getstate>
    fn GetState(&self, _cx: JSContext, _rval: MutableHandleValue) {
        todo!()

        // let data = StructuredSerializedData {
        //     serialized: self.state.serialized.clone(),
        //     ports: None,
        //     blobs: None,
        // };

        // if let Ok(data) = structuredclone::read(&self.global(), data, _rval) {
        // } else {
        // }
    }
}
