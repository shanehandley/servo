/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::rc::Rc;

use dom_struct::dom_struct;

use crate::dom::bindings::codegen::Bindings::DataTransferItemBinding::{
    DataTransferItemMethods, FunctionStringCallback,
};
use crate::dom::bindings::callback::ExceptionHandling;
use crate::dom::bindings::reflector::{Reflector, reflect_dom_object};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::window::Window;

use super::file::File;

#[derive(Clone, JSTraceable, MallocSizeOf)]
pub enum DataTransferItemValue {
    File(DomRoot<File>),
    String(DOMString),
}

#[dom_struct]
pub struct DataTransferItem {
    reflector_: Reflector,
    kind: DOMString, // 'string' or 'file'
    type_: DOMString,
    value: DataTransferItemValue,
}

impl DataTransferItem {
    fn new_inherited(kind: DOMString, type_: DOMString, value: DataTransferItemValue) -> DataTransferItem {
        DataTransferItem {
            reflector_: Reflector::new(),
            kind,
            type_,
            value
        }
    }

    pub fn new(
        window: &Window,
        kind: DOMString,
        type_: DOMString,
        value: DataTransferItemValue
    ) -> DomRoot<DataTransferItem> {
        reflect_dom_object(
            Box::new(DataTransferItem::new_inherited(
                kind, type_, value
            )),
            window,
        )
    }

    pub fn kind(&self) -> DOMString {
        self.kind.clone()
    }

    pub fn type_(&self) -> DOMString {
        self.type_.clone()
    }

    pub fn value(&self) -> DataTransferItemValue {
        self.value.clone()
    }

    pub fn get_as_file(&self) -> Option<DomRoot<File>> {
        match &self.value {
            DataTransferItemValue::File(f) => Some(f.clone()),
            _ => None
        }
    }

}

#[allow(non_snake_case)]
impl DataTransferItemMethods for DataTransferItem {
    // https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransferitem-getasstring
    fn GetAsString(&self, callback: Option<Rc<FunctionStringCallback>>) {
        if let (Some(callback), &DataTransferItemValue::String(ref text)) = (callback, &self.value) {
            let _ = callback.Call__(text.clone(), ExceptionHandling::Report);
        }
    }

    // https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransferitem-getasfile
    fn GetAsFile(&self) -> Option<DomRoot<File>> {
        self.get_as_file()
    }

    // https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransferitem-kind
    fn Kind(&self) -> DOMString {
        self.kind()
    }

    // https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransferitem-type
    fn Type(&self) -> DOMString {
        self.type_.clone()
    }
}
