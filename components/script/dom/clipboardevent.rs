/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::rust::HandleObject;
use servo_atoms::Atom;

use crate::dom::bindings::codegen::Bindings::ClipboardEventBinding::{
    ClipboardEventInit, ClipboardEventMethods,
};
use crate::dom::bindings::codegen::Bindings::EventBinding::Event_Binding::EventMethods;
use crate::dom::bindings::error::Fallible;
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::reflector::reflect_dom_object_with_proto;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::datatransfer::DataTransfer;
use crate::dom::event::Event;
use crate::dom::window::Window;

// <https://w3c.github.io/clipboard-apis/#clipboard-event-interfaces>
#[dom_struct]
pub struct ClipboardEvent {
    event: Event,
    clipboard_data: Option<DomRoot<DataTransfer>>,
}

impl ClipboardEvent {
    /// <https://w3c.github.io/clipboard-apis/#clipboardevent>
    fn new_inherited(init: Option<&ClipboardEventInit>) -> ClipboardEvent {
        let clipboard_data = init
            .unwrap_or(&ClipboardEventInit::empty())
            .clipboardData
            .clone();

        ClipboardEvent {
            event: Event::new_inherited(),
            clipboard_data,
        }
    }

    fn new_with_proto(
        global: &Window,
        proto: Option<HandleObject>,
        type_: DOMString,
        init: &ClipboardEventInit,
    ) -> DomRoot<ClipboardEvent> {
        let ev = reflect_dom_object_with_proto(
            Box::new(ClipboardEvent::new_inherited(Some(init))),
            global,
            proto,
        );

        {
            let event = ev.upcast::<Event>();
            event.init_event(Atom::from(type_), true, false);
        }

        ev
    }

    #[allow(non_snake_case)]
    pub fn Constructor(
        global: &Window,
        proto: Option<HandleObject>,
        type_: DOMString,
        init: &ClipboardEventInit,
    ) -> Fallible<DomRoot<ClipboardEvent>> {
        let event = ClipboardEvent::new_with_proto(global, proto, type_, init);

        Ok(event)
    }
}

#[allow(non_snake_case)]
impl ClipboardEventMethods for ClipboardEvent {
    /// <https://w3c.github.io/clipboard-apis/#clipboardevent-clipboarddata>
    fn GetClipboardData(&self) -> Option<DomRoot<DataTransfer>> {
        self.clipboard_data.clone()
    }

    /// <https://dom.spec.whatwg.org/#dom-event-istrusted>
    fn IsTrusted(&self) -> bool {
        self.event.IsTrusted()
    }
}
