/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::sync::Weak;

use dom_struct::dom_struct;

use crate::dom::bindings::codegen::Bindings::NavigationActivationBinding::NavigationActivationMethods;
use crate::dom::bindings::codegen::Bindings::NavigationBinding::NavigationType;
use crate::dom::bindings::reflector::{reflect_dom_object_with_proto, DomObject, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::navigationhistoryentry::NavigationHistoryEntry;

/// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#navigationactivation>
#[dom_struct]
pub struct NavigationActivation {
    reflector_: Reflector,
    from: Option<DomRoot<NavigationHistoryEntry>>,
    entry: DomRoot<NavigationHistoryEntry>,
    old_entry: DomRoot<NavigationHistoryEntry>,
    navigation_type: NavigationType,
    #[no_trace]
    #[ignore_malloc_size_of = "todo"]
    activation: Option<Weak<NavigationActivation>>,
}

impl NavigationActivation {}

impl NavigationActivationMethods<crate::DomTypeHolder> for NavigationActivation {
    /// <https://html.spec.whatwg.org/multipage/#dom-navigationactivation-from>
    fn GetFrom(&self) -> Option<DomRoot<NavigationHistoryEntry>> {
        self.from.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigationactivation-entry>
    fn Entry(&self) -> DomRoot<NavigationHistoryEntry> {
        self.entry.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigationactivation-navigationtype>
    fn NavigationType(&self) -> NavigationType {
        self.navigation_type.clone()
    }
}
