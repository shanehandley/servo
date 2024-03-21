/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::borrow::Borrow;

use dom_struct::dom_struct;

use js::rust::HandleObject;

use crate::dom::bindings::codegen::Bindings::DataTransferBinding::{DataTransferMethods, DropEffect, EffectAllowed};
use crate::dom::bindings::reflector::Reflector;
use crate::dom::bindings::reflector::reflect_dom_object;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::datatransferitemlist::DataTransferItemList;
use crate::dom::element::Element;
use crate::dom::filelist::FileList;
use crate::dom::window::Window;

// https://html.spec.whatwg.org/multipage/dnd.html#datatransfer
#[dom_struct]
pub struct DataTransfer {
    reflector_: Reflector,
    drop_effect: DropEffect,
    effect_allowed: EffectAllowed,
    items: DomRoot<DataTransferItemList>,
    files: DomRoot<FileList>
}

impl DataTransfer {
    // https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransfer
    fn new_inherited(files: DomRoot<FileList>, items: DomRoot<DataTransferItemList>) -> DataTransfer {
        DataTransfer {
            reflector_: Reflector::new(),
            drop_effect: DropEffect::None,
            effect_allowed: EffectAllowed::None,
            items,
            files
        }
    }

    pub fn new(
        window: &Window,
    ) -> DomRoot<DataTransfer> {
        let files = FileList::new(window, Vec::new());
        let items = DataTransferItemList::new(window, &[]);

        reflect_dom_object(
            Box::new(DataTransfer::new_inherited(files, items)),
            window,
        )
    }

    #[allow(non_snake_case)]
    pub fn Constructor(
        global: &Window,
        proto: Option<HandleObject>,
    ) -> DomRoot<DataTransfer> {
        DataTransfer::new(global)
    }

    fn files(&self) {
        let items = self.items.borrow();
    }
}

#[allow(non_snake_case)]
impl DataTransferMethods for DataTransfer {
    fn DropEffect(&self) -> DropEffect {
        self.drop_effect
    }
    
    fn SetDropEffect(&self, value: DropEffect) {

    }

    fn EffectAllowed(&self) -> EffectAllowed {
        self.effect_allowed
    }
    
    fn SetEffectAllowed(&self, value: EffectAllowed) {
        
    }
    
    fn Items(&self) -> DomRoot<DataTransferItemList> {
        self.items.clone()
    }
    
    fn SetDragImage(&self, image: &Element, x: i32, y: i32) {
        
    }

    fn GetData(&self, format: DOMString) -> DOMString {
        DOMString::new()
    }
    
    fn SetData(&self, format: DOMString, data: DOMString) {
        
    }
    
    // https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransfer-cleardata
    fn ClearData(&self, format: Option<DOMString>) {
        // TODO Step 1
        // TODO Step 2

        match format {
            // TODO Step 3
            None => {
                // If the method was called with no arguments, remove each item in the drag data store
                // item list whose kind is Plain Unicode string, and return.
            },
            // Step 4 & 5
            Some(s) => {
                let _fmt = match s.to_ascii_lowercase().as_str() {
                    "text" => "text/plain",
                    "url" => "text/uri-list",
                    f => f
                };
            }
        }
    }
    
    fn Files(&self) -> DomRoot<FileList> {
        self.files.clone()
    }
}
