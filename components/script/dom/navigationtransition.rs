/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::rc::Rc;

use dom_struct::dom_struct;

use crate::dom::bindings::codegen::Bindings::NavigationBinding::NavigationType;
use crate::dom::bindings::codegen::Bindings::NavigationTransitionBinding::NavigationTransitionMethods;
use crate::dom::bindings::reflector::Reflector;
use crate::dom::bindings::root::DomRoot;
use crate::dom::navigationhistoryentry::NavigationHistoryEntry;
use crate::dom::promise::Promise;

/// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#navigationtransition>
#[dom_struct]
pub struct NavigationTransition {
    reflector_: Reflector,
    old_entry: DomRoot<NavigationHistoryEntry>,
    new_entry: DomRoot<NavigationHistoryEntry>,
    navigation_type: NavigationType,
    #[ignore_malloc_size_of = "promises are hard"]
    finished_promise: Rc<Promise>,
}

impl NavigationTransitionMethods<crate::DomTypeHolder> for NavigationTransition {
    /// <https://html.spec.whatwg.org/multipage/#dom-navigationactivation-from>
    fn From(&self) -> DomRoot<NavigationHistoryEntry> {
        self.old_entry.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigationtransition-finished>
    fn Finished(&self) -> Rc<Promise> {
        self.finished_promise.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigationtransition-navigationtype>
    fn NavigationType(&self) -> NavigationType {
        self.navigation_type.clone()
    }
}
