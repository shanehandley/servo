/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::rust::HandleObject;
use servo_atoms::Atom;

use crate::dom::bindings::codegen::Bindings::DragEventBinding::{DragEventInit, DragEventMethods};
use crate::dom::bindings::codegen::Bindings::MouseEventBinding::MouseEventMethods;
use crate::dom::bindings::error::Fallible;
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::reflector::{reflect_dom_object, reflect_dom_object_with_proto};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::datatransfer::DataTransfer;
use crate::dom::event::Event;
use crate::dom::types::MouseEvent;
use crate::dom::window::Window;

#[dom_struct]
/// <https://html.spec.whatwg.org/multipage/#dragevent>
pub struct DragEvent {
    mouseevent: MouseEvent,
    data_transfer: Option<DomRoot<DataTransfer>>,
}

impl DragEvent {
    pub fn new_uninitialized(global: &Window) -> DomRoot<DragEvent> {
        reflect_dom_object(Box::new(DragEvent::new_inherited(None)), global)
    }

    /// <https://html.spec.whatwg.org/multipage/#drageventinit>
    fn new_inherited(init: Option<&DragEventInit>) -> DragEvent {
        let data_transfer = init.unwrap_or(&DragEventInit::empty()).dataTransfer.clone();

        DragEvent {
            mouseevent: MouseEvent::new_inherited(),
            data_transfer,
        }
    }

    fn new_with_proto(
        global: &Window,
        proto: Option<HandleObject>,
        type_: DOMString,
        init: &DragEventInit,
    ) -> DomRoot<DragEvent> {
        let ev = reflect_dom_object_with_proto(
            Box::new(DragEvent::new_inherited(Some(init))),
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
        init: &DragEventInit,
    ) -> Fallible<DomRoot<DragEvent>> {
        let event = DragEvent::new_with_proto(global, proto, type_, init);

        Ok(event)
    }
}

#[allow(non_snake_case)]
impl DragEventMethods for DragEvent {
    /// <https://html.spec.whatwg.org/multipage/#dom-dragevent-datatransfer>
    fn GetDataTransfer(&self) -> Option<DomRoot<DataTransfer>> {
        self.data_transfer.clone()
    }

    /// <https://dom.spec.whatwg.org/#dom-event-istrusted>
    fn IsTrusted(&self) -> bool {
        self.mouseevent.IsTrusted()
    }
}
