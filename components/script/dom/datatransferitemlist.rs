/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::cell::Cell;
use std::convert::TryFrom;

use dom_struct::dom_struct;

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::DataTransferItemListBinding::DataTransferItemListMethods;
use crate::dom::bindings::error::{Error, Fallible};
use crate::dom::bindings::reflector::{DomObject, reflect_dom_object, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::datatransferitem::{DataTransferItem, DataTransferItemValue};
use crate::dom::file::File;
use crate::dom::window::Window;

use servo_rand::random;

#[derive(JSTraceable, MallocSizeOf, PartialEq)]
pub enum DataTransferMode {
    ReadOnly,
    ReadWrite,
    Protected
}

// https://html.spec.whatwg.org/multipage/dnd.html#datatransferitemlist
#[dom_struct]
pub struct DataTransferItemList {
    reflector_: Reflector,
    list: DomRefCell<Vec<DomRoot<DataTransferItem>>>,
    types: DomRefCell<Vec<DOMString>>,
    mode: DataTransferMode,
    cache_key: Cell<u32>
}

impl DataTransferItemList {
    fn new_inherited(list: &[&DataTransferItem]) -> DataTransferItemList {
        DataTransferItemList {
            reflector_: Reflector::new(),
            list: DomRefCell::new(list.iter().map(|item|
                DomRoot::from_ref(&**item)
            ).collect()),
            types: DomRefCell::new(vec![]),
            mode: DataTransferMode::ReadWrite,
            cache_key: Cell::new(0)
        }
    }

    #[allow(crown::unrooted_must_root)]
    pub fn new(
        window: &Window,
        list: &[&DataTransferItem],
    ) -> DomRoot<DataTransferItemList> {
        reflect_dom_object(
            Box::new(DataTransferItemList::new_inherited(list)),
            window,
        )
    }

    pub fn add_string(&self, data: DOMString, format: DOMString) -> Fallible<Option<DomRoot<DataTransferItem>>> {
        if self.list.borrow().iter().find(
            |x| *x.type_() == format.clone().to_ascii_lowercase().as_str().to_owned()
        ).is_some() {
            return Err(Error::NotSupported);
        }

        Ok(Some(self.add(DataTransferItem::new(
            &self.global().as_window(),
            DOMString::from("string"),
            format, 
            DataTransferItemValue::String(data)
        ))))
    }

    fn add(&self, item: DomRoot<DataTransferItem>) -> DomRoot<DataTransferItem> {
        self.list.borrow_mut().push(item.clone());

        self.regenerate_types();

        return item
    }

    // Remove each item in the item list whose kind is Plain Unicode string
    pub fn remove_string_entries(&self) {
        if self.list.borrow().is_empty() {
            return;
        }

        self.list.borrow_mut().retain(|item| {
            item.kind() != DOMString::from_string("string".to_owned())
        });

        self.regenerate_types();
    }

    pub fn remove_string_entries_by_format(&self, format: &DOMString) {
        if self.list.borrow().is_empty() {
            return;
        }

        if !self.list.borrow().iter().any(
            |item| &item.type_() == format
        ) {
            return;
        }

        self.list.borrow_mut().retain(|item| {
            &item.kind() == "file" || &item.type_() != format
        });

        self.regenerate_types();
    }

    pub fn get_files(&self) -> Vec<DomRoot<File>> {
        let files = self.list.borrow().iter().filter_map(
            |item| item.get_as_file()
        ).collect();

        files
    }

    // <https://html.spec.whatwg.org/multipage/dnd.html#concept-datatransfer-types>
    fn regenerate_types(&self) {
        // Step 1 & 2.1
        let mut types: Vec<DOMString> = self.list.borrow()
            .iter()
            .filter(|item| item.kind().to_ascii_lowercase() == "string")
            .map(|item| item.type_())
            .collect();

        // Step 2.2
        if self.list.borrow().iter().any(|item| match item.value() {
            DataTransferItemValue::File(_) => true,
                _ => false
        }) {
            types.push(DOMString::from_string("Files".to_owned()));
        };

        *self.types.borrow_mut() = types;
        self.cache_key.set(random::<u32>());
    }

    pub fn types(&self) -> Vec<DOMString> {
        self.types.borrow().to_vec()
    }

    pub fn cache_key(&self) -> u32 {
        self.cache_key.get()
    }

    pub fn get_mode(&self) -> &DataTransferMode {
        &self.mode
    }
}

#[allow(non_snake_case)]
impl DataTransferItemListMethods for DataTransferItemList {
    // https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransferitemlist-add
    fn Add(&self, data: DOMString, type_: DOMString) -> Fallible<Option<DomRoot<DataTransferItem>>> {
        warn!("ADDING A STRING ===== {:?} | {:?}", data, type_);

        // Step 1
        if self.mode != DataTransferMode::ReadWrite {
            return Ok(None);
        }

        // Step 2.1
        if self.list.borrow().iter().find(
            |x| *x.type_() == type_.clone().to_ascii_lowercase().as_str().to_owned()
        ).is_some() {
            return Err(Error::NotSupported);
        }

        // Step 2.2
        let item = DataTransferItem::new(
            &self.global().as_window(),
            DOMString::from("string"),
            type_, 
            DataTransferItemValue::String(data)
        );

        return Ok(Some(self.add(item)))
    }

    // https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransferitemlist-add
    fn Add_(&self, data: &File) -> Fallible<Option<DomRoot<DataTransferItem>>> {
        warn!("ADDING A FILE ===== {:?} | {:?}", data.name(), data.type_string());

        // Step 1
        if self.mode != DataTransferMode::ReadWrite {
            return Ok(None);
        }

        let item = DataTransferItem::new(
            &self.global().as_window(),
            DOMString::from_string("file".to_owned()),
            DOMString::from_string(data.type_string()),
            DataTransferItemValue::File(DomRoot::from_ref(data))
        );

        Ok(Some(self.add(item)))
    }

    // https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransferitemlist-remove
    fn Remove(&self, index: u32) -> Fallible<()> {
        // Step 1
        if self.mode != DataTransferMode::ReadWrite {
            return Err(Error::InvalidState);
        }

        if let Ok(i) = usize::try_from(index) {
            // Step 2
            if i < self.list.borrow().len() {
                // Step 3
                self.list.borrow_mut().remove(i);
                self.regenerate_types();
            }
        }

        Ok(())
    }

    // https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransferitemlist-clear
    fn Clear(&self) {
        if self.mode == DataTransferMode::ReadWrite {
            // Avoid regenerating the internal types cache key when the item list is already empty
            if !self.list.borrow().is_empty() {
                self.list.borrow_mut().clear();
                self.regenerate_types();
            }
        }
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
