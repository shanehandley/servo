/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;

use js::rust::HandleObject;
use js::jsval::JSVal;

use crate::dom::bindings::codegen::Bindings::FileListBinding::FileList_Binding::FileListMethods;
use crate::dom::bindings::codegen::Bindings::DataTransferBinding::{DataTransferMethods, DropEffect, EffectAllowed};
use crate::dom::bindings::reflector::{DomObject, Reflector, reflect_dom_object};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::bindings::utils::to_frozen_array;
use crate::dom::datatransferitemlist::DataTransferItemList;
use crate::dom::element::Element;
use crate::dom::filelist::FileList;
use crate::dom::window::Window;
use crate::script_runtime::JSContext as SafeJSContext;

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
    
    // <https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransfer-setdragimage>
    fn SetDragImage(&self, image: &Element, x: i32, y: i32) {
        
    }

    // <https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransfer-getdata>
    fn GetData(&self, format: DOMString) -> DOMString {
        // TODO Step 1
        // TODO Step 2

        // Step 3, 4, 5 & 6
        let _fmt = match format.to_ascii_lowercase().as_str() {
            "text" => "text/plain",
            "url" => "text/uri-list",
            f => f
        };

        // Step 7

        DOMString::new()
    }

    // <https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransfer-setdata>
    fn SetData(&self, format: DOMString, data: DOMString) {
        // TODO Step 1
        // TODO Step 2

        // Step 3 & 4
        let parsed_format = DOMString::from_string(
            match format.to_ascii_lowercase().as_str() {
                "text" => "text/plain",
                "url" => "text/uri-list",
                lowercase_format => lowercase_format
            }.to_owned()
        );

        // Step 5
        // Remove the item in the drag data store item list whose kind is text and whose type string
        // is equal to format, if there is one.
        self.items.remove_by_format(&parsed_format);

        // Step 6
        // Add an item to the drag data store item list whose kind is text, whose type string is
        // equal to format, and whose data is the string given by the method's second argument.
        self.items.add_string(parsed_format, data);
    }
    
    // <https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransfer-cleardata>
    fn ClearData(&self, format: Option<DOMString>) {
        warn!("CLEARING DATA ==== {:?}", format.clone().unwrap_or(DOMString::from_string("none_provided".to_owned())));

        // TODO Step 1
        // TODO Step 2

        match format {
            None => {
                // Step 3
                self.items.remove_all_strings();
            },
            // Step 4 & 5
            Some(s) => {
                let parsed_format = DOMString::from_string(match s.to_ascii_lowercase().as_str() {
                    "text" => "text/plain",
                    "url" => "text/uri-list",
                    f => f
                }.to_owned());

                self.items.remove_by_format(&parsed_format);
            }
        }
    }
    
    // <https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransfer-files>
    fn Files(&self) -> DomRoot<FileList> {
        let file_list = FileList::new(
            &self.global().as_window(),
            self.items.get_files()
        );

        // This is causing a crash
//         /**
//          * Traceback (most recent call last):
//     File "/Users/shane/code/servo/tests/wpt/tests/tools/wptserve/wptserve/handlers.py", line 373, in __call__
//     rv = self.func(request, response)

//     File "/Users/shane/code/servo/tests/wpt/tests/html/semantics/forms/form-submission-0/resources/file-submission.py", line 7, in main
//     testinput = request.POST.first(b"testinput")

//     File "/Users/shane/code/servo/tests/wpt/tests/tools/wptserve/wptserve/request.py", line 576, in first
//     raise KeyError(key)
//   KeyError: b'testinput'
//          */
        warn!("FILE LIST LENGTH ==== {:?}", file_list.Length());
        
        for file in file_list.iter_files() {
            warn!("FILE === {:?}", file.name());
        }

        let files = self.files.clone();

        file_list
    }

    // <https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransfer-types>
    fn Types(&self, cx: SafeJSContext) -> JSVal {
        to_frozen_array(self.items.types().as_slice(), cx)
    }
}
