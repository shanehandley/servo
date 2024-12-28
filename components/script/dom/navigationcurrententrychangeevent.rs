/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::rust::HandleObject;
use servo_atoms::Atom;

use crate::dom::bindings::codegen::Bindings::EventBinding::EventMethods;
use crate::dom::bindings::codegen::Bindings::NavigationBinding::NavigationType;
use crate::dom::bindings::codegen::Bindings::NavigationCurrentEntryChangeEventBinding::{
    NavigationCurrentEntryChangeEventInit, NavigationCurrentEntryChangeEventMethods,
};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::reflector::reflect_dom_object_with_proto;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::event::Event;
use crate::dom::navigationhistoryentry::NavigationHistoryEntry;
use crate::dom::window::Window;
use crate::script_runtime::CanGc;

#[dom_struct]
/// <https://html.spec.whatwg.org/multipage/#navigationcurrententrychangeevent>
pub struct NavigationCurrentEntryChangeEvent {
    event: Event,
    navigation_type: Option<NavigationType>,
    from: DomRoot<NavigationHistoryEntry>,
}

impl NavigationCurrentEntryChangeEvent {
    fn new_inherited(
        init: &NavigationCurrentEntryChangeEventInit,
    ) -> NavigationCurrentEntryChangeEvent {
        NavigationCurrentEntryChangeEvent {
            event: Event::new_inherited(),
            navigation_type: init.navigationType.clone(),
            from: init.from.clone(),
        }
    }

    fn new_with_proto(
        window: &Window,
        proto: Option<HandleObject>,
        type_: Atom,
        init: &NavigationCurrentEntryChangeEventInit,
        can_gc: CanGc,
    ) -> DomRoot<NavigationCurrentEntryChangeEvent> {
        let ev = reflect_dom_object_with_proto(
            Box::new(NavigationCurrentEntryChangeEvent::new_inherited(init)),
            window,
            proto,
            can_gc,
        );

        {
            let event = ev.upcast::<Event>();
            event.init_event(type_, init.parent.bubbles, init.parent.cancelable);
        }

        ev
    }

    pub fn new(
        window: &Window,
        proto: Option<HandleObject>,
        type_: Atom,
        init: &NavigationCurrentEntryChangeEventInit,
        can_gc: CanGc,
    ) -> DomRoot<NavigationCurrentEntryChangeEvent> {
        NavigationCurrentEntryChangeEvent::new_with_proto(window, proto, type_, init, can_gc)
    }
}

impl NavigationCurrentEntryChangeEventMethods<crate::DomTypeHolder>
    for NavigationCurrentEntryChangeEvent
{
    /// <https://html.spec.whatwg.org/multipage/#dom-navigationcurrententrychangeevent-from>
    fn From(&self) -> super::bindings::root::DomRoot<NavigationHistoryEntry> {
        self.from.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigationcurrententrychangeevent-navigationtype>
    fn GetNavigationType(&self) -> Option<NavigationType> {
        self.navigation_type.clone()
    }

    /// <https://dom.spec.whatwg.org/#dom-event-istrusted>
    fn IsTrusted(&self) -> bool {
        self.upcast::<Event>().IsTrusted()
    }

    fn Constructor(
        window: &Window,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        type_: DOMString,
        init: &NavigationCurrentEntryChangeEventInit,
    ) -> DomRoot<NavigationCurrentEntryChangeEvent> {
        NavigationCurrentEntryChangeEvent::new_with_proto(
            window,
            proto,
            Atom::from(type_),
            init,
            can_gc,
        )
    }
}
