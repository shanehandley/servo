/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use net_traits::session_history::SessionHistoryEntry;

use crate::dom::bindings::codegen::Bindings::NavigationHistoryEntryBinding::NavigationHistoryEntryMethods;
use crate::dom::bindings::codegen::Bindings::WindowBinding::Window_Binding::WindowMethods;
use crate::dom::bindings::reflector::{reflect_dom_object_with_proto, DomObject, Reflector};
use crate::dom::bindings::root::{Dom, DomRoot};
use crate::dom::bindings::str::{DOMString, USVString};
use crate::dom::document::Document;
use crate::dom::eventtarget::EventTarget;

/// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#navigationhistoryentry>
#[dom_struct]
pub struct NavigationHistoryEntry {
    event_target: EventTarget,
    url: Option<USVString>,
    key: DOMString,
    id: DOMString,
    index: i64,
    #[no_trace]
    #[ignore_malloc_size_of = "todo"]
    session_history_entry: SessionHistoryEntry,
}

impl NavigationHistoryEntry {
    fn document(&self) -> DomRoot<Document> {
        let global = self.global();
        let window = global.as_window();

        window.Document()
    }
}

impl NavigationHistoryEntryMethods<crate::DomTypeHolder> for NavigationHistoryEntry {
    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigationhistoryentry-url>
    fn GetUrl(&self) -> Option<USVString> {
        // Step 1. Let document be this's relevant global object's associated Document.
        let document = self.document();

        // Step 2. If document is not fully active, then return the empty string.
        if !document.is_fully_active() {
            return None;
        }

        // Step 3. Let she be this's session history entry.

        // Step 4. If she's document does not equal document, and she's document state's request
        // referrer policy is "no-referrer" or "origin", then return null.

        // Return she's URL, serialized.
        self.url.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigationdestination-key>
    fn Key(&self) -> DOMString {
        self.key.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigationhistoryentry-id>
    fn Id(&self) -> DOMString {
        self.id.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigationhistoryentry-index>
    fn Index(&self) -> i64 {
        self.index
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigationhistoryentry-samedocument>
    fn SameDocument(&self) -> bool {
        // Step 1. Let document be this's relevant global object's associated Document.
        let document = self.document();

        // Step 2. If document is not fully active, then return false.
        if !document.is_fully_active() {
            return false;
        }

        // Step 3. Return true if this's session history entry's document equals document, and false
        // otherwise.

        todo!()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigationhistoryentry-getstate>
    fn GetState(&self, cx: crate::script_runtime::JSContext, rval: js::gc::MutableHandleValue) {
        todo!()
    }

    /// <https://html.spec.whatwg.org/multipage/#handler-navigationhistoryentry-ondispose>
    fn GetOndispose(
        &self,
    ) -> Option<
        std::rc::Rc<
            crate::dom::bindings::codegen::Bindings::EventHandlerBinding::EventHandlerNonNull,
        >,
    > {
        todo!()
    }

    /// <https://html.spec.whatwg.org/multipage/#handler-navigationhistoryentry-ondispose>
    fn SetOndispose(
        &self,
        value: Option<
            std::rc::Rc<
                super::bindings::codegen::Bindings::EventHandlerBinding::EventHandlerNonNull,
            >,
        >,
    ) {
        todo!()
    }
}
