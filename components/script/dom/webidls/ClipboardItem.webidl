// typedef Promise<(DOMString or Blob)> ClipboardItemData;

[SecureContext, Exposed=Window]
interface ClipboardItem {
  // constructor(record<DOMString, ClipboardItemData> items,
  //             optional ClipboardItemOptions options = {});
  constructor(record<DOMString, DOMString> items,
              optional ClipboardItemOptions options = {});

  readonly attribute PresentationStyle presentationStyle;
  readonly attribute /* FrozenArray<DOMString> */ any types;

  Promise<Blob> getType(DOMString type);

  static boolean supports(DOMString type);
};

enum PresentationStyle { "unspecified", "inline", "attachment" };

dictionary ClipboardItemOptions {
  PresentationStyle presentationStyle = "unspecified";
};
