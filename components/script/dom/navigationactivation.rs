/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;

use crate::dom::bindings::codegen::Bindings::NavigationActivationBinding::NavigationActivationMethods;
use crate::dom::bindings::codegen::Bindings::NavigationBinding::NavigationType;
use crate::dom::bindings::reflector::Reflector;
use crate::dom::bindings::root::DomRoot;
use crate::dom::navigationhistoryentry::NavigationHistoryEntry;

#[dom_struct]
/// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#navigationactivation>
pub struct NavigationActivation {
    reflector_: Reflector,
    old_entry: Option<DomRoot<NavigationHistoryEntry>>,
    new_entry: Option<DomRoot<NavigationHistoryEntry>>,
    navigation_type: NavigationType,
}

impl NavigationActivation {
    pub fn new(navigation_type: NavigationType) -> NavigationActivation {
        NavigationActivation {
            reflector_: Reflector::new(),
            old_entry: None,
            new_entry: None,
            navigation_type,
        }
    }
}

#[allow(non_snake_case)]
impl NavigationActivationMethods<crate::DomTypeHolder> for NavigationActivation {
    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigationactivation-from>
    fn GetFrom(&self) -> Option<DomRoot<NavigationHistoryEntry>> {
        self.old_entry.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigationactivation-entry>
    fn Entry(&self) -> DomRoot<NavigationHistoryEntry> {
        todo!()
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigationactivation-NavigationTimingType>
    fn NavigationType(&self) -> NavigationType {
        self.navigation_type
    }
}
