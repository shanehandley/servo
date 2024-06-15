/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;

use crate::dom::bindings::codegen::Bindings::CSPViolationReportBodyBinding::CSPViolationReportBodyMethods;
use crate::dom::bindings::codegen::Bindings::SecurityPolicyViolationEventBinding::SecurityPolicyViolationEventDisposition;
use crate::dom::bindings::str::{DOMString, USVString};
use crate::dom::reportbody::ReportBody;

/// <https://w3c.github.io/webappsec-csp/#cspviolationreportbody>
#[dom_struct]
pub struct CSPViolationReportBody {
    report_body: ReportBody,
}

impl CSPViolationReportBodyMethods for CSPViolationReportBody {
    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-cspviolationreportbody-documenturl>
    fn DocumentURL(&self) -> USVString {
        USVString::from("".to_owned())
    }

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-cspviolationreportbody-referrer>
    fn GetReferrer(&self) -> Option<USVString> {
        None
    }

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-cspviolationreportbody-blockedurl>
    fn GetBlockedURL(&self) -> Option<USVString> {
        None
    }

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-cspviolationreportbody-effectivedirective>
    fn EffectiveDirective(&self) -> DOMString {
        DOMString::new()
    }

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-cspviolationreportbody-originalpolicy>
    fn OriginalPolicy(&self) -> DOMString {
        DOMString::new()
    }

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-cspviolationreportbody-sourcefile>
    fn GetSourceFile(&self) -> Option<USVString> {
        None
    }

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-cspviolationreportbody-sample>
    fn GetSample(&self) -> Option<DOMString> {
        None
    }

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-cspviolationreportbody-disposition>
    fn Disposition(&self) -> SecurityPolicyViolationEventDisposition {
        SecurityPolicyViolationEventDisposition::Report
    }

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-cspviolationreportbody-statuscode>
    fn StatusCode(&self) -> u16 {
        0
    }

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-cspviolationreportbody-linenumber>
    fn GetLineNumber(&self) -> Option<u32> {
        None
    }

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-cspviolationreportbody-columnnumber>
    fn GetColumnNumber(&self) -> Option<u32> {
        None
    }
}
