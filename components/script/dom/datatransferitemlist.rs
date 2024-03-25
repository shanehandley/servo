/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::convert::TryFrom;

use dom_struct::dom_struct;

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::DataTransferItemListBinding::DataTransferItemListMethods;
use crate::dom::bindings::reflector::{DomObject, reflect_dom_object, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::datatransferitem::{DataTransferItem, DataTransferItemValue};
use crate::dom::file::File;
use crate::dom::window::Window;

// https://html.spec.whatwg.org/multipage/dnd.html#datatransferitemlist
#[dom_struct]
pub struct DataTransferItemList {
    reflector_: Reflector,
    list: DomRefCell<Vec<DomRoot<DataTransferItem>>>,
    types: Vec<DOMString>
}

impl DataTransferItemList {
    fn new_inherited(list: &[&DataTransferItem]) -> DataTransferItemList {
        DataTransferItemList {
            reflector_: Reflector::new(),
            list: DomRefCell::new(list.iter().map(|item|
                DomRoot::from_ref(&**item)
            ).collect()),
            types: vec![]
        }
    }

    #[allow(crown::unrooted_must_root)]
    pub fn new(
        window: &Window,
        list: &[&DataTransferItem]
    ) -> DomRoot<DataTransferItemList> {
        reflect_dom_object(
            Box::new(DataTransferItemList::new_inherited(list)),
            window,
        )
    }

    pub fn add_string(&self, data: DOMString, type_: DOMString) -> Option<DomRoot<DataTransferItem>> {
        let item = DataTransferItem::new(
            &self.global().as_window(),
            DOMString::from_string("string".to_owned()),
            type_, 
            DataTransferItemValue::String(data)
        );

        self.add(item)
    }

    fn add(&self, item: DomRoot<DataTransferItem>) -> Option<DomRoot<DataTransferItem>> {
        if self.list.borrow().iter().find(|x| *x == &item).is_none() {
            self.list.borrow_mut().push(item.clone());

            return Some(item)
        }

        None
    }

    // Remove each item in the item list whose kind is Plain Unicode string
    pub fn remove_all_strings(&self) {
        self.list.borrow_mut().retain(|item| {
            item.kind() != DOMString::from_string("string".to_owned())
        });
    }

    pub fn remove_by_format(&self, format: &DOMString) {
        warn!("Removing item by format: {:?}", format.clone());

        self.list.borrow_mut().retain(|item| {
            &item.type_() != format
        });
    }

    pub fn get_files(&self) -> Vec<DomRoot<File>> {
        let files = self.list.borrow().iter().filter_map(
            |item| item.get_as_file()
        ).collect();

        files
    }

    // <https://html.spec.whatwg.org/multipage/dnd.html#concept-datatransfer-types>
    pub fn types(&self) -> Vec<DOMString> {
        let mut types: Vec<DOMString> = self.list.borrow()
            .iter()
            .filter(|item| item.kind().to_ascii_lowercase() == "string")
            .map(|item| item.type_())
            .collect();

        if self.list.borrow().iter().any(|item| match item.value() {
            DataTransferItemValue::File(_) => true,
                _ => false
        }) {
            types.push(DOMString::from_string("Files".to_owned()));
        };

        types
    }
}

#[allow(non_snake_case)]
impl DataTransferItemListMethods for DataTransferItemList {
    // https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransferitemlist-add
    fn Add(&self, data: DOMString, type_: DOMString) -> Option<DomRoot<DataTransferItem>> {
        warn!("ADDING A STRING ===== {:?} | {:?}", data, type_);

        self.add_string(data, type_)
    }

    // https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransferitemlist-add
    fn Add_(&self, data: &File) -> Option<DomRoot<DataTransferItem>> {
        warn!("ADDING A FILE ===== {:?} | {:?}", data.name(), data.type_string());

        let item = DataTransferItem::new(
            &self.global().as_window(),
            DOMString::from_string("file".to_owned()),
            DOMString::from_string(data.type_string()),
            DataTransferItemValue::File(DomRoot::from_ref(data))
        );

        self.add(item)
    }

    // https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransferitemlist-remove
    fn Remove(&self, index: u32) {
        if let Ok(i) = usize::try_from(index) {
            self.list.borrow_mut().remove(i);
        }
    }

    // https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransferitemlist-clear
    fn Clear(&self) {
        self.list.borrow_mut().clear();
    }

    // https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransferitemlist-length
    fn Length(&self) -> u32 {
        u32::try_from(self.list.borrow().len()).unwrap_or(0)
    }

    // https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransferitemlist-length
    fn IndexedGetter(&self, index: u32) -> Option<DomRoot<DataTransferItem>> {
        if let Ok(i) = usize::try_from(index) {
            return self.list
                .borrow()
                .get(i)
                .map(|item| DomRoot::from_ref(&**item))
        }

        None
    }
}
