/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::rc::Rc;

use dom_struct::dom_struct;
use js::jsval::JSVal;
use js::rust::HandleObject;
use servo_atoms::Atom;

use crate::dom::bindings::codegen::Bindings::ClipboardItemBinding::{
    ClipboardItemMethods, ClipboardItemOptions, PresentationStyle,
};
use crate::dom::bindings::error::Fallible;
use crate::dom::bindings::import::module::SafeJSContext;
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::record::Record;
use crate::dom::bindings::reflector::{
    reflect_dom_object, reflect_dom_object_with_proto, DomObject, Reflector,
};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::bindings::utils::to_frozen_array;
use crate::dom::promise::Promise;
use crate::dom::window::Window;

/// <https://w3c.github.io/clipboard-apis/#representation>
pub struct ClipboardItemRepresentation {
    mime_type: String,
    is_custom: bool,
    data: String,
}

/// <https://w3c.github.io/clipboard-apis/#clipboarditem>
#[dom_struct]
pub struct ClipboardItem {
    reflector: Reflector,
    presentation_style: PresentationStyle,
    #[ignore_malloc_size_of = "promises are hard"]
    items: Record<DOMString, DOMString>,
    //  items: Record<DOMString, Rc<Promise>>,
    // representations: Vec<ClipboardItemRepresentation>
}

impl ClipboardItem {
    #[allow(non_snake_case)]
    pub fn Constructor(
        global: &Window,
        proto: Option<HandleObject>,
        // items: Record<DOMString, Rc<Promise>>,
        items: Record<DOMString, DOMString>,
        options: &ClipboardItemOptions,
    ) -> DomRoot<ClipboardItem> {
        reflect_dom_object_with_proto(
            Box::new(ClipboardItem {
                reflector: Reflector::new(),
                presentation_style: PresentationStyle::Unspecified,
                items,
                // representations: Vec::new()
            }),
            global,
            proto,
        )
    }

    #[allow(non_snake_case)]
    pub fn Supports(global: &Window, type_: DOMString) -> bool {
        false
    }
}

#[allow(non_snake_case)]
impl ClipboardItemMethods for ClipboardItem {
    /// <https://w3c.github.io/clipboard-apis/#dom-clipboarditem-presentationstyle>
    fn PresentationStyle(&self) -> PresentationStyle {
        self.presentation_style.clone()
    }

    /// <https://w3c.github.io/clipboard-apis/#dom-clipboarditem-types>
    fn Types(&self, cx: SafeJSContext) -> JSVal {
        let items: Vec<String> = vec![];

        to_frozen_array(&items.as_slice(), cx)
    }

    /// <https://w3c.github.io/clipboard-apis/#dom-clipboarditem-gettype>
    fn GetType(&self, type_: DOMString) -> Rc<Promise> {
        Promise::new(&self.global())
    }
}
