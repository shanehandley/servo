/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::cell::Cell;

use dom_struct::dom_struct;
use js::jsapi::Heap;
use js::jsval::JSVal;
use js::rust::HandleObject;

use crate::dom::bindings::codegen::Bindings::DataTransferBinding::{
    DataTransferMethods, DropEffect, EffectAllowed,
};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::reflector::{reflect_dom_object_with_proto, DomObject, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::bindings::utils::to_frozen_array;
use crate::dom::datatransferitem::DataTransferItemValue;
use crate::dom::datatransferitemlist::{DataTransferItemList, DataTransferMode};
use crate::dom::element::Element;
use crate::dom::filelist::FileList;
use crate::dom::htmlimageelement::HTMLImageElement;
use crate::dom::window::Window;
use crate::script_runtime::JSContext as SafeJSContext;
use crate::test::DomRefCell;

// Optional UI information when a DataTransfer object is associated with drag & drop event
// <https://html.spec.whatwg.org/multipage/#drag-data-store-bitmap>
#[derive(JSTraceable, MallocSizeOf, PartialEq)]
struct DataTransferBitmap {
    image: DomRoot<HTMLImageElement>,
    image_x: i32,
    image_y: i32,
}

// https://html.spec.whatwg.org/multipage/#datatransfer
#[dom_struct]
pub struct DataTransfer {
    reflector_: Reflector,
    drop_effect: DomRefCell<DropEffect>,
    effect_allowed: DomRefCell<EffectAllowed>,
    item_list: DomRoot<DataTransferItemList>,
    // <https://html.spec.whatwg.org/multipage/#dom-datatransfer-files>
    files: DomRoot<FileList>,
    // <https://html.spec.whatwg.org/multipage/#drag-data-store-bitmap>
    bitmap_image: DomRefCell<Option<DataTransferBitmap>>,
    // <https://html.spec.whatwg.org/multipage/#dom-datatransfer-types>
    #[ignore_malloc_size_of = "mozjs"]
    frozen_types: DomRefCell<Option<Heap<JSVal>>>,
    // Used to co-ordinate the cached value of frozen_types with self.item_list
    cache_key: Cell<u32>,
}

impl DataTransfer {
    #[allow(crown::unrooted_must_root)]
    pub fn new_inherited(
        files: DomRoot<FileList>,
        item_list: DomRoot<DataTransferItemList>,
    ) -> DataTransfer {
        DataTransfer {
            reflector_: Reflector::new(),
            drop_effect: DomRefCell::new(DropEffect::None),
            effect_allowed: DomRefCell::new(EffectAllowed::None),
            item_list,
            files,
            bitmap_image: DomRefCell::new(None),
            frozen_types: DomRefCell::new(None),
            cache_key: Cell::new(0),
        }
    }

    #[allow(crown::unrooted_must_root)]
    fn new_with_proto(global: &Window, proto: Option<HandleObject>) -> DomRoot<DataTransfer> {
        let files = FileList::new(global, Vec::new());
        let items = DataTransferItemList::new(global, &[]);

        let data_transfer = DataTransfer::new_inherited(files, items);

        reflect_dom_object_with_proto(Box::new(data_transfer), global, proto)
    }

    #[allow(non_snake_case)]
    pub fn Constructor(global: &Window, proto: Option<HandleObject>) -> DomRoot<DataTransfer> {
        DataTransfer::new_with_proto(global, proto)
    }
}

#[allow(non_snake_case)]
impl DataTransferMethods for DataTransfer {
    /// <https://html.spec.whatwg.org/multipage/#dom-datatransfer-dropeffect>
    fn DropEffect(&self) -> DropEffect {
        *self.drop_effect.borrow()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-datatransfer-dropeffect>
    fn SetDropEffect(&self, value: DropEffect) {
        if self.item_list.get_mode() == DataTransferMode::ReadWrite {
            *self.drop_effect.borrow_mut() = value;
        }
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-datatransfer-effectallowed>
    fn EffectAllowed(&self) -> EffectAllowed {
        *self.effect_allowed.borrow()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-datatransfer-effectallowed>
    fn SetEffectAllowed(&self, value: EffectAllowed) {
        if self.item_list.get_mode() == DataTransferMode::ReadWrite {
            *self.effect_allowed.borrow_mut() = value;
        }
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-datatransfer-items>
    fn Items(&self) -> DomRoot<DataTransferItemList> {
        self.item_list.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-datatransfer-setdragimage>
    fn SetDragImage(&self, image: &Element, x: i32, y: i32) {
        // TODO Step 1

        // Step 2
        if self.item_list.get_mode() != DataTransferMode::ReadWrite {
            return;
        }

        // Step 3
        if image.is::<HTMLImageElement>() {
            if let Some(image_element) = image.downcast::<HTMLImageElement>() {
                *self.bitmap_image.borrow_mut() = Some(DataTransferBitmap {
                    image: DomRoot::from_ref(image_element),
                    image_x: x,
                    image_y: y,
                })
            }
        }
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-datatransfer-getdata>
    fn GetData(&self, format: DOMString) -> DOMString {
        // TODO Step 1

        // Step 2
        if self.item_list.get_mode() == DataTransferMode::Protected {
            return DOMString::new();
        }

        let string_format = format.to_ascii_lowercase();

        // Step 3, 4, 5 & 6
        let parsed_format = match string_format.as_str() {
            "text" => "text/plain",
            "url" => "text/uri-list",
            f => f,
        };

        let value: Option<DataTransferItemValue> = self
            .item_list
            .get_string_value_by_format(DOMString::from(parsed_format));

        // Step 7
        if value.is_none() {
            return DOMString::new();
        }

        // Step 8, 9, & 10
        match (value, parsed_format) {
            (Some(DataTransferItemValue::String(val)), "text/uri-list") => val,
            (Some(DataTransferItemValue::String(val)), _) => val,
            _ => DOMString::new(),
        }
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-datatransfer-setdata>
    fn SetData(&self, format: DOMString, data: DOMString) {
        // TODO Step 1

        // Step 2
        if self.item_list.get_mode() != DataTransferMode::ReadWrite {
            return;
        }

        // Step 3 & 4
        let parsed_format = DOMString::from_string(
            match format.to_ascii_lowercase().as_str() {
                "text" => "text/plain",
                "url" => "text/uri-list",
                f => f,
            }
            .to_owned(),
        );

        // Step 5
        self.item_list
            .remove_string_entries_by_format(&parsed_format);

        // Step 6
        let _ = self.item_list.add_string(data, parsed_format);
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-datatransfer-cleardata>
    fn ClearData(&self, format: Option<DOMString>) {
        // TODO Step 1

        // Step 2
        if self.item_list.get_mode() != DataTransferMode::ReadWrite {
            return;
        }

        match format {
            // Step 3
            None => {
                self.item_list.remove_string_entries();
            },
            // Step 4 & 5
            Some(s) => {
                let parsed_format = DOMString::from_string(
                    match s.to_ascii_lowercase().as_str() {
                        "text" => "text/plain",
                        "url" => "text/uri-list",
                        f => f,
                    }
                    .to_owned(),
                );

                self.item_list
                    .remove_string_entries_by_format(&parsed_format);
            },
        }
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-datatransfer-files>
    fn Files(&self) -> DomRoot<FileList> {
        FileList::new(&self.global().as_window(), self.item_list.get_files())
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-datatransfer-types>
    fn Types(&self, cx: SafeJSContext) -> JSVal {
        // If our cache key matches the item_lists's cache key, we're safe to return the cached value
        if self.item_list.cache_key() == self.cache_key.get() {
            if let Some(types) = &*self.frozen_types.borrow() {
                return types.get();
            }
        }

        let frozen_types = to_frozen_array(self.item_list.types().as_slice(), cx);

        // Safety: need to create the Heap value in its final memory location before setting it.
        *self.frozen_types.borrow_mut() = Some(Heap::default());

        self.frozen_types
            .borrow()
            .as_ref()
            .unwrap()
            .set(frozen_types);

        self.cache_key.set(self.item_list.cache_key());

        frozen_types
    }
}
