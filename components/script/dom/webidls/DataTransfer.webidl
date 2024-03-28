/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// https://html.spec.whatwg.org/multipage/dnd.html#the-datatransfer-interface
enum DropEffect {
  "none",
  "copy",
  "link",
  "move"
};

enum EffectAllowed {
  "none",
  "copy",
  "copyLink",
  "copyMove",
  "link",
  "linkMove",
  "move",
  "all",
  "uninitialized"
};

[Exposed=Window]
interface DataTransfer {
  constructor();

  attribute DropEffect dropEffect;
  attribute EffectAllowed effectAllowed;

  [SameObject] readonly attribute DataTransferItemList items;

  undefined setDragImage(Element image, long x, long y);

  /* old interface */
  readonly attribute /* FrozenArray<DOMString> */ any types;
  DOMString getData(DOMString format);
  undefined setData(DOMString format, DOMString data);
  undefined clearData(optional DOMString format);
  [SameObject] readonly attribute FileList files;
};
