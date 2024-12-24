/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::cell::Cell;

use dom_struct::dom_struct;
use js::rust::HandleObject;
use malloc_size_of_derive::MallocSizeOf;
use servo_atoms::Atom;

use super::types::NavigationDestination;
use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::EventBinding::EventMethods;
use crate::dom::bindings::codegen::Bindings::NavigateEventBinding::{
    NavigateEventInit, NavigateEventMethods, NavigationFocusReset, NavigationInterceptOptions,
    NavigationScrollBehavior,
};
use crate::dom::bindings::codegen::Bindings::NavigationBinding::NavigationType;
use crate::dom::bindings::error::{Error, Fallible};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::reflector::reflect_dom_object_with_proto;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::bindings::trace::RootedTraceableBox;
use crate::dom::event::{Event, EventStatus};
use crate::dom::formdata::FormData;
//  use crate::dom::navigationdestination::NavigationDestination;
use crate::dom::window::Window;
use crate::script_runtime::{CanGc, JSContext as SafeJSContext};

/// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#concept-navigateevent-interception-state>
#[derive(Copy, Clone, MallocSizeOf)]
pub enum InterceptionState {
    None,
    Intercepted,
    Committed,
    Scrolled,
    Finished,
}

#[dom_struct]
pub struct NavigateEvent {
    event: Event,
    navigation_type: DomRefCell<NavigationType>,
    destination: DomRoot<NavigationDestination>,
    form_data: Option<DomRoot<FormData>>,
    can_intercept: Cell<bool>,
    hash_change: Cell<bool>,
    user_initiated: Cell<bool>,
    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#concept-navigateevent-focusreset>
    focus_reset: Cell<Option<NavigationFocusReset>>,
    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#concept-navigateevent-scroll>
    scroll_behavior: Cell<Option<NavigationScrollBehavior>>,
    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#concept-navigateevent-interception-state>
    #[no_trace]
    interception_state: Cell<InterceptionState>,
}

impl NavigateEvent {
    fn new_inherited(
        navigation_type: NavigationType,
        destination: DomRoot<NavigationDestination>,
        can_intercept: bool,
        hash_change: bool,
        user_initiated: bool,
    ) -> NavigateEvent {
        NavigateEvent {
            event: Event::new_inherited(),
            navigation_type: DomRefCell::new(navigation_type),
            destination,
            form_data: None,
            can_intercept: Cell::new(can_intercept),
            hash_change: Cell::new(hash_change),
            user_initiated: Cell::new(user_initiated),
            focus_reset: Cell::new(None),
            scroll_behavior: Cell::new(None),
            interception_state: Cell::new(InterceptionState::None),
        }
    }

    // fn new_with_proto(
    //     window: &Window,
    //     proto: Option<HandleObject>,
    //     type_: Atom,
    //     event_init: RootedTraceableBox<NavigateEventInit>,
    // ) -> DomRoot<NavigateEvent> {
    //     reflect_dom_object_with_proto(Box::new(NavigateEvent::new_inherited(
    //         event_init.navigationType,
    //         event_init.destination.clone(),
    //         event_init.canIntercept,
    //         event_init.hashChange,
    //         event_init.userInitiated,

    //     )), window, proto, CanGc::note())
    // }

    #[allow(non_snake_case)]
    //  pub fn Constructor(
    //      window: &Window,
    //      proto: Option<HandleObject>,
    //      can_gc: CanGc,
    //      type_: DOMString,
    //      init: RootedTraceableBox<NavigateEventInit>,
    //  ) -> Fallible<DomRoot<NavigateEvent>> {
    //      let ev = reflect_dom_object_with_proto(
    //          Box::new(NavigateEvent::new_inherited(
    //              init.navigationType,
    //              init.destination.clone(),
    //              init.canIntercept,
    //              init.hashChange,
    //              init.userInitiated,
    //          )),
    //          window,
    //          proto,
    //          can_gc,
    //      );

    //      {
    //          let event = ev.upcast::<Event>();
    //          event.init_event(Atom::from(type_), false, false);
    //      }

    //      Ok(ev)
    //  }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#navigateevent-perform-shared-checks>
    fn perform_shared_checks(&self) -> Fallible<()> {
        // 1. If event's relevant global object's associated Document is not fully active, then
        // throw an "InvalidStateError" DOMException.
        // let document = document_from_node(&self);

        // if !document.is_fully_active() {

        // }

        let event = self.upcast::<Event>();

        // 2. If event's isTrusted attribute was initialized to false, then throw a "SecurityError"
        // DOMException.
        if !event.IsTrusted() {
            return Err(Error::Security);
        }

        // 3. If event's canceled flag is set, then throw an "InvalidStateError" DOMException.
        if event.status() == EventStatus::Canceled {
            return Err(Error::InvalidState);
        }

        return Ok(());
    }
}

impl NavigateEventMethods<crate::DomTypeHolder> for NavigateEvent {
    fn Constructor(
        global: &Window,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        type_: DOMString,
        event_init: RootedTraceableBox<NavigateEventInit>,
    ) -> Fallible<DomRoot<NavigateEvent>> {
        let ev = reflect_dom_object_with_proto(
            Box::new(NavigateEvent::new_inherited(
                event_init.navigationType,
                event_init.destination.clone(),
                event_init.canIntercept,
                event_init.hashChange,
                event_init.userInitiated,
            )),
            global,
            proto,
            can_gc,
        );

        {
            let event = ev.upcast::<Event>();
            event.init_event(Atom::from(type_), false, false);
        }

        Ok(ev)
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigateevent-navigationtype>
    fn NavigationType(&self) -> NavigationType {
        self.navigation_type.borrow().clone()
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigateevent-destination>
    fn Destination(&self) -> DomRoot<NavigationDestination> {
        self.destination.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigateevent-canintercept>
    fn CanIntercept(&self) -> bool {
        self.can_intercept.get()
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigateevent-userinitiated>
    fn UserInitiated(&self) -> bool {
        self.user_initiated.get()
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigateevent-hashchange>
    fn HashChange(&self) -> bool {
        self.hash_change.get()
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigateevent-formdata>
    fn GetFormData(&self) -> Option<DomRoot<FormData>> {
        self.form_data.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigateevent-downloadrequest>
    fn GetDownloadRequest(&self) -> Option<DOMString> {
        None
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigateevent-hasuavisualtransition>
    fn HasUAVisualTransition(&self) -> bool {
        false
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigateevent-intercept>
    fn Intercept(&self, options: &NavigationInterceptOptions) -> Fallible<()> {
        // Step 1: Perform shared checks given this.
        self.perform_shared_checks()?;

        // Step 2. If this's canIntercept attribute was initialized to false, then throw a
        // "SecurityError" DOMException.
        if !self.CanIntercept() {
            return Err(Error::Security);
        }

        // Step 3: If this's dispatch flag is unset, then throw an "InvalidStateError" DOMException.
        let event = self.upcast::<Event>();

        if event.dispatching() {
            return Err(Error::InvalidState);
        }

        // Step 4: Assert: this's interception state is either "none" or "intercepted".

        // Step 5: Set this's interception state to "intercepted".
        self.interception_state.set(InterceptionState::Intercepted);

        // Step 6: If options["handler"] exists, then append it to this's navigation handler list.

        // Step 7: If options["focusReset"] exists, then:
        if let Some(focus_reset) = options.focusReset {
            // Step 7.1: If this's focus reset behavior is not null, and it is not equal to
            // options["focusReset"], then the user agent may report a warning to the console
            // indicating that the focusReset option for a previous call to intercept() was
            // overridden by this new value, and the previous value will be ignored.
            if let Some(current_focus_reset) = self.focus_reset.get() {
                if current_focus_reset != focus_reset {
                    warn!("todo")
                }
            }

            // Step 7.2: Set this's focus reset behavior to options["focusReset"].
            self.focus_reset.set(Some(focus_reset));
        }

        // Step 8: If options["scroll"] exists, then:
        if let Some(scroll_behavior) = options.scroll {
            // Step 8.1: If this's scroll behavior is not null, and it is not equal to
            // options["scroll"], then the user agent may report a warning to the console indicating
            // that the scroll option for a previous call to intercept() was overridden by this new
            // value, and the previous value will be ignored.
            if let Some(current_scroll_behavior) = self.scroll_behavior.get() {
                if current_scroll_behavior != scroll_behavior {
                    warn!("todo")
                }
            }

            // Step 8,2: Set this's scroll behavior to options["scroll"].
            self.scroll_behavior.set(Some(scroll_behavior))
        }

        return Ok(());
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigateevent-scroll>
    fn Scroll(&self) -> Fallible<()> {
        // 1. Perform shared checks given this.
        self.perform_shared_checks()?;

        // 2. If this's interception state is not "committed", then throw an "InvalidStateError" DOMException.
        // if let Some(InterceptionState) =

        // 3. Process scroll behavior given this.

        return Ok(());
    }

    fn IsTrusted(&self) -> bool {
        self.event.IsTrusted()
    }

    fn Info(&self, cx: SafeJSContext, retval: js::gc::MutableHandleValue) {
        todo!()
    }
}
