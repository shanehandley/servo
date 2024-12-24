/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::rc::Rc;

use dom_struct::dom_struct;
use js::rust::HandleObject;

use crate::dom::bindings::codegen::Bindings::NavigationBinding::NavigationType;
use crate::dom::bindings::codegen::Bindings::NavigationTransitionBinding::NavigationTransitionMethods;
use crate::dom::bindings::reflector::{reflect_dom_object_with_proto, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::navigationhistoryentry::NavigationHistoryEntry;
use crate::dom::promise::Promise;
use crate::dom::window::Window;
use crate::script_runtime::CanGc;

#[dom_struct]
/// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#navigationtransition>
pub struct NavigationTransition {
    reflector_: Reflector,
    navigation_type: NavigationType,
    from_entry: DomRoot<NavigationHistoryEntry>,
    #[ignore_malloc_size_of = "promises are hard"]
    finished_promise: Option<Rc<Promise>>,
}

impl NavigationTransition {
    pub fn new_with_proto(
        window: &Window,
        proto: Option<HandleObject>,
        navigation_type: NavigationType,
        from_entry: DomRoot<NavigationHistoryEntry>,
    ) -> DomRoot<NavigationTransition> {
        reflect_dom_object_with_proto(
            Box::new(NavigationTransition::new_inherited(
                navigation_type,
                from_entry,
            )),
            window,
            proto,
            CanGc::note(),
        )
    }

    fn new_inherited(
        navigation_type: NavigationType,
        from_entry: DomRoot<NavigationHistoryEntry>,
    ) -> NavigationTransition {
        NavigationTransition {
            reflector_: Reflector::new(),
            navigation_type,
            from_entry,
            finished_promise: None,
        }
    }
}

#[allow(non_snake_case)]
impl NavigationTransitionMethods<crate::DomTypeHolder> for NavigationTransition {
    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigationtransition-NavigationTimingType>
    fn NavigationType(&self) -> NavigationType {
        self.navigation_type
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigationtransition-from>
    fn From(&self) -> DomRoot<NavigationHistoryEntry> {
        todo!()
    }

    fn Finished(&self) -> Rc<Promise> {
        todo!()
    }
}
