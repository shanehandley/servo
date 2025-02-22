use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
/// An id to differeniate one network request from another.
pub struct NavigableId(usize);

impl Default for NavigableId {
    fn default() -> Self {
        Self(0)
    }
}

struct DocumentState {
    // pub document_id: DocumentId
}

struct SessionHistoryEntry {

}


/// <https://html.spec.whatwg.org/multipage/document-sequences.html#navigable>
pub struct Navigable {
    id: NavigableId,
    parent: Option<NavigableId>,
    is_closing: bool,
}

impl Navigable {
    pub fn new() -> Self {
        Self {
            id: Default::default(),
            parent: None,
            is_closing: false
        }
    }
}
