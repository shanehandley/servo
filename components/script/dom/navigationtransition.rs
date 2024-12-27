/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;

use crate::dom::bindings::codegen::Bindings::NavigationBinding::NavigationType;
use crate::dom::bindings::codegen::Bindings::NavigationTransitionBinding::NavigationTransitionMethods;
use crate::dom::bindings::reflector::{reflect_dom_object_with_proto, DomObject, Reflector};
use crate::dom::bindings::root::{Dom, DomRoot};
use crate::dom::bindings::str::{DOMString, USVString};
use crate::dom::navigationhistoryentry::NavigationHistoryEntry;

/// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#navigationtransition>
#[dom_struct]
pub struct NavigationTransition {
    reflector_: Reflector,
    from: DomRoot<NavigationHistoryEntry>,
    navigation_type: NavigationType,
}

impl NavigationTransition {}

impl NavigationTransitionMethods<crate::DomTypeHolder> for NavigationTransition {
    fn From(&self) -> DomRoot<NavigationHistoryEntry> {
        self.from.clone()
    }

    fn Finished(&self) -> std::rc::Rc<super::types::Promise> {
        todo!()
    }

    fn NavigationType(&self) -> NavigationType {
        self.navigation_type.clone()
    }
}
