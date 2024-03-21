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
use crate::dom::bindings::inheritance::Castable;
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
    r#type: DOMString,
    value: DataTransferItemValue,
}

impl DataTransferItem {
    fn new_inherited(kind: DOMString, r#type: DOMString, value: DataTransferItemValue) -> DataTransferItem {
        DataTransferItem {
            reflector_: Reflector::new(),
            kind,
            r#type,
            value
        }
    }

    pub fn new(
        window: &Window,
        kind: DOMString,
        r#type: DOMString,
        value: DataTransferItemValue
    ) -> DomRoot<DataTransferItem> {
        reflect_dom_object(
            Box::new(DataTransferItem::new_inherited(
                kind, r#type, value
            )),
            window,
        )
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
        match &self.value {
            DataTransferItemValue::File(f) => Some(f.clone()),
            _ => None
        }
    }

    // https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransferitem-kind
    fn Kind(&self) -> DOMString {
        self.kind.clone()
    }

    // https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransferitem-type
    fn Type(&self) -> DOMString {
        self.r#type.clone()
    }
}
