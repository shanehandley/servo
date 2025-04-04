/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::collections::{HashMap, VecDeque};
use std::fmt;

use base::Epoch;
use base::id::{
    BroadcastChannelRouterId, BrowsingContextId, HistoryStateId, MessagePortId,
    MessagePortRouterId, PipelineId, ServiceWorkerId, ServiceWorkerRegistrationId, WebViewId,
};
use canvas_traits::canvas::{CanvasId, CanvasMsg};
use constellation_traits::{LogEntry, TraversalDirection};
use devtools_traits::{ScriptToDevtoolsControlMsg, WorkerId};
use embedder_traits::{
    EmbedderMsg, MediaSessionEvent, TouchEventType, TouchSequenceId, ViewportDetails,
};
use euclid::default::Size2D as UntypedSize2D;
use ipc_channel::ipc::{IpcReceiver, IpcSender};
use net_traits::CoreResourceMsg;
use net_traits::storage_thread::StorageType;
use serde::{Deserialize, Serialize};
use servo_url::{ImmutableOrigin, ServoUrl};
use strum_macros::IntoStaticStr;
#[cfg(feature = "webgpu")]
use webgpu_traits::{WebGPU, WebGPUAdapterResponse};
use webrender_api::ImageKey;

use crate::mem::MemoryReportResult;
use crate::{
    AnimationState, AuxiliaryWebViewCreationRequest, BroadcastMsg, DocumentState,
    IFrameLoadInfoWithData, LoadData, MessagePortMsg, NavigationHistoryBehavior, PortMessageTask,
    StructuredSerializedData, WindowSizeType, WorkerGlobalScopeInit, WorkerScriptLoadOrigin,
};

/// An iframe sizing operation.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct IFrameSizeMsg {
    /// The child browsing context for this iframe.
    pub browsing_context_id: BrowsingContextId,
    /// The size and scale factor of the iframe.
    pub size: ViewportDetails,
    /// The kind of sizing operation.
    pub type_: WindowSizeType,
}

/// Whether the default action for a touch event was prevented by web content
#[derive(Debug, Deserialize, Serialize)]
pub enum TouchEventResult {
    /// Allowed by web content
    DefaultAllowed(TouchSequenceId, TouchEventType),
    /// Prevented by web content
    DefaultPrevented(TouchSequenceId, TouchEventType),
}

/// Messages sent from the `ScriptThread` to the `Constellation`.
#[derive(Deserialize, IntoStaticStr, Serialize)]
pub enum ScriptToConstellationMessage {
    /// Request to complete the transfer of a set of ports to a router.
    CompleteMessagePortTransfer(MessagePortRouterId, Vec<MessagePortId>),
    /// The results of attempting to complete the transfer of a batch of ports.
    MessagePortTransferResult(
        /* The router whose transfer of ports succeeded, if any */
        Option<MessagePortRouterId>,
        /* The ids of ports transferred successfully */
        Vec<MessagePortId>,
        /* The ids, and buffers, of ports whose transfer failed */
        HashMap<MessagePortId, VecDeque<PortMessageTask>>,
    ),
    /// A new message-port was created or transferred, with corresponding control-sender.
    NewMessagePort(MessagePortRouterId, MessagePortId),
    /// A global has started managing message-ports
    NewMessagePortRouter(MessagePortRouterId, IpcSender<MessagePortMsg>),
    /// A global has stopped managing message-ports
    RemoveMessagePortRouter(MessagePortRouterId),
    /// A task requires re-routing to an already shipped message-port.
    RerouteMessagePort(MessagePortId, PortMessageTask),
    /// A message-port was shipped, let the entangled port know.
    MessagePortShipped(MessagePortId),
    /// A message-port has been discarded by script.
    RemoveMessagePort(MessagePortId),
    /// Entangle two message-ports.
    EntanglePorts(MessagePortId, MessagePortId),
    /// A global has started managing broadcast-channels.
    NewBroadcastChannelRouter(
        BroadcastChannelRouterId,
        IpcSender<BroadcastMsg>,
        ImmutableOrigin,
    ),
    /// A global has stopped managing broadcast-channels.
    RemoveBroadcastChannelRouter(BroadcastChannelRouterId, ImmutableOrigin),
    /// A global started managing broadcast channels for a given channel-name.
    NewBroadcastChannelNameInRouter(BroadcastChannelRouterId, String, ImmutableOrigin),
    /// A global stopped managing broadcast channels for a given channel-name.
    RemoveBroadcastChannelNameInRouter(BroadcastChannelRouterId, String, ImmutableOrigin),
    /// Broadcast a message to all same-origin broadcast channels,
    /// excluding the source of the broadcast.
    ScheduleBroadcast(BroadcastChannelRouterId, BroadcastMsg),
    /// Forward a message to the embedder.
    ForwardToEmbedder(EmbedderMsg),
    /// Broadcast a storage event to every same-origin pipeline.
    /// The strings are key, old value and new value.
    BroadcastStorageEvent(
        StorageType,
        ServoUrl,
        Option<String>,
        Option<String>,
        Option<String>,
    ),
    /// Indicates whether this pipeline is currently running animations.
    ChangeRunningAnimationsState(AnimationState),
    /// Requests that a new 2D canvas thread be created. (This is done in the constellation because
    /// 2D canvases may use the GPU and we don't want to give untrusted content access to the GPU.)
    CreateCanvasPaintThread(
        UntypedSize2D<u64>,
        IpcSender<(IpcSender<CanvasMsg>, CanvasId, ImageKey)>,
    ),
    /// Notifies the constellation that this frame has received focus.
    Focus,
    /// Get the top-level browsing context info for a given browsing context.
    GetTopForBrowsingContext(BrowsingContextId, IpcSender<Option<WebViewId>>),
    /// Get the browsing context id of the browsing context in which pipeline is
    /// embedded and the parent pipeline id of that browsing context.
    GetBrowsingContextInfo(
        PipelineId,
        IpcSender<Option<(BrowsingContextId, Option<PipelineId>)>>,
    ),
    /// Get the nth child browsing context ID for a given browsing context, sorted in tree order.
    GetChildBrowsingContextId(
        BrowsingContextId,
        usize,
        IpcSender<Option<BrowsingContextId>>,
    ),
    /// All pending loads are complete, and the `load` event for this pipeline
    /// has been dispatched.
    LoadComplete,
    /// A new load has been requested, with an option to replace the current entry once loaded
    /// instead of adding a new entry.
    LoadUrl(LoadData, NavigationHistoryBehavior),
    /// Abort loading after sending a LoadUrl message.
    AbortLoadUrl,
    /// Post a message to the currently active window of a given browsing context.
    PostMessage {
        /// The target of the posted message.
        target: BrowsingContextId,
        /// The source of the posted message.
        source: PipelineId,
        /// The expected origin of the target.
        target_origin: Option<ImmutableOrigin>,
        /// The source origin of the message.
        /// <https://html.spec.whatwg.org/multipage/#dom-messageevent-origin>
        source_origin: ImmutableOrigin,
        /// The data to be posted.
        data: StructuredSerializedData,
    },
    /// Inform the constellation that a fragment was navigated to and whether or not it was a replacement navigation.
    NavigatedToFragment(ServoUrl, NavigationHistoryBehavior),
    /// HTMLIFrameElement Forward or Back traversal.
    TraverseHistory(TraversalDirection),
    /// Inform the constellation of a pushed history state.
    PushHistoryState(HistoryStateId, ServoUrl),
    /// Inform the constellation of a replaced history state.
    ReplaceHistoryState(HistoryStateId, ServoUrl),
    /// Gets the length of the joint session history from the constellation.
    JointSessionHistoryLength(IpcSender<u32>),
    /// Notification that this iframe should be removed.
    /// Returns a list of pipelines which were closed.
    RemoveIFrame(BrowsingContextId, IpcSender<Vec<PipelineId>>),
    /// Successful response to [crate::ConstellationControlMsg::SetThrottled].
    SetThrottledComplete(bool),
    /// A load has been requested in an IFrame.
    ScriptLoadedURLInIFrame(IFrameLoadInfoWithData),
    /// A load of the initial `about:blank` has been completed in an IFrame.
    ScriptNewIFrame(IFrameLoadInfoWithData),
    /// Script has opened a new auxiliary browsing context.
    CreateAuxiliaryWebView(AuxiliaryWebViewCreationRequest),
    /// Mark a new document as active
    ActivateDocument,
    /// Set the document state for a pipeline (used by screenshot / reftests)
    SetDocumentState(DocumentState),
    /// Update the layout epoch in the constellation (used by screenshot / reftests).
    SetLayoutEpoch(Epoch, IpcSender<bool>),
    /// Update the pipeline Url, which can change after redirections.
    SetFinalUrl(ServoUrl),
    /// Script has handled a touch event, and either prevented or allowed default actions.
    TouchEventProcessed(TouchEventResult),
    /// A log entry, with the top-level browsing context id and thread name
    LogEntry(Option<String>, LogEntry),
    /// Discard the document.
    DiscardDocument,
    /// Discard the browsing context.
    DiscardTopLevelBrowsingContext,
    /// Notifies the constellation that this pipeline has exited.
    PipelineExited,
    /// Send messages from postMessage calls from serviceworker
    /// to constellation for storing in service worker manager
    ForwardDOMMessage(DOMMessage, ServoUrl),
    /// <https://w3c.github.io/ServiceWorker/#schedule-job-algorithm>
    ScheduleJob(Job),
    /// Notifies the constellation about media session events
    /// (i.e. when there is metadata for the active media session, playback state changes...).
    MediaSessionEvent(PipelineId, MediaSessionEvent),
    #[cfg(feature = "webgpu")]
    /// Create a WebGPU Adapter instance
    RequestAdapter(
        IpcSender<WebGPUAdapterResponse>,
        wgpu_core::instance::RequestAdapterOptions,
        wgpu_core::id::AdapterId,
    ),
    #[cfg(feature = "webgpu")]
    /// Get WebGPU channel
    GetWebGPUChan(IpcSender<Option<WebGPU>>),
    /// Notify the constellation of a pipeline's document's title.
    TitleChanged(PipelineId, String),
    /// Notify the constellation that the size of some `<iframe>`s has changed.
    IFrameSizes(Vec<IFrameSizeMsg>),
    /// Request results from the memory reporter.
    ReportMemory(IpcSender<MemoryReportResult>),
}

impl fmt::Debug for ScriptToConstellationMessage {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let variant_string: &'static str = self.into();
        write!(formatter, "ScriptMsg::{variant_string}")
    }
}

/// Entities required to spawn service workers
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScopeThings {
    /// script resource url
    pub script_url: ServoUrl,
    /// network load origin of the resource
    pub worker_load_origin: WorkerScriptLoadOrigin,
    /// base resources required to create worker global scopes
    pub init: WorkerGlobalScopeInit,
    /// the port to receive devtools message from
    pub devtools_chan: Option<IpcSender<ScriptToDevtoolsControlMsg>>,
    /// service worker id
    pub worker_id: WorkerId,
}

/// Message that gets passed to service worker scope on postMessage
#[derive(Debug, Deserialize, Serialize)]
pub struct DOMMessage {
    /// The origin of the message
    pub origin: ImmutableOrigin,
    /// The payload of the message
    pub data: StructuredSerializedData,
}

/// Channels to allow service worker manager to communicate with constellation and resource thread
#[derive(Deserialize, Serialize)]
pub struct SWManagerSenders {
    /// Sender of messages to the constellation.
    pub swmanager_sender: IpcSender<SWManagerMsg>,
    /// Sender for communicating with resource thread.
    pub resource_sender: IpcSender<CoreResourceMsg>,
    /// Sender of messages to the manager.
    pub own_sender: IpcSender<ServiceWorkerMsg>,
    /// Receiver of messages from the constellation.
    pub receiver: IpcReceiver<ServiceWorkerMsg>,
}

/// Messages sent to Service Worker Manager thread
#[derive(Debug, Deserialize, Serialize)]
pub enum ServiceWorkerMsg {
    /// Timeout message sent by active service workers
    Timeout(ServoUrl),
    /// Message sent by constellation to forward to a running service worker
    ForwardDOMMessage(DOMMessage, ServoUrl),
    /// <https://w3c.github.io/ServiceWorker/#schedule-job-algorithm>
    ScheduleJob(Job),
    /// Exit the service worker manager
    Exit,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
/// <https://w3c.github.io/ServiceWorker/#dfn-job-type>
pub enum JobType {
    /// <https://w3c.github.io/ServiceWorker/#register>
    Register,
    /// <https://w3c.github.io/ServiceWorker/#unregister-algorithm>
    Unregister,
    /// <https://w3c.github.io/ServiceWorker/#update-algorithm>
    Update,
}

#[derive(Debug, Deserialize, Serialize)]
/// The kind of error the job promise should be rejected with.
pub enum JobError {
    /// <https://w3c.github.io/ServiceWorker/#reject-job-promise>
    TypeError,
    /// <https://w3c.github.io/ServiceWorker/#reject-job-promise>
    SecurityError,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(clippy::large_enum_variant)]
/// Messages sent from Job algorithms steps running in the SW manager,
/// in order to resolve or reject the job promise.
pub enum JobResult {
    /// <https://w3c.github.io/ServiceWorker/#reject-job-promise>
    RejectPromise(JobError),
    /// <https://w3c.github.io/ServiceWorker/#resolve-job-promise>
    ResolvePromise(Job, JobResultValue),
}

#[derive(Debug, Deserialize, Serialize)]
/// Jobs are resolved with the help of various values.
pub enum JobResultValue {
    /// Data representing a serviceworker registration.
    Registration {
        /// The Id of the registration.
        id: ServiceWorkerRegistrationId,
        /// The installing worker, if any.
        installing_worker: Option<ServiceWorkerId>,
        /// The waiting worker, if any.
        waiting_worker: Option<ServiceWorkerId>,
        /// The active worker, if any.
        active_worker: Option<ServiceWorkerId>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
/// <https://w3c.github.io/ServiceWorker/#dfn-job>
pub struct Job {
    /// <https://w3c.github.io/ServiceWorker/#dfn-job-type>
    pub job_type: JobType,
    /// <https://w3c.github.io/ServiceWorker/#dfn-job-scope-url>
    pub scope_url: ServoUrl,
    /// <https://w3c.github.io/ServiceWorker/#dfn-job-script-url>
    pub script_url: ServoUrl,
    /// <https://w3c.github.io/ServiceWorker/#dfn-job-client>
    pub client: IpcSender<JobResult>,
    /// <https://w3c.github.io/ServiceWorker/#job-referrer>
    pub referrer: ServoUrl,
    /// Various data needed to process job.
    pub scope_things: Option<ScopeThings>,
}

impl Job {
    /// <https://w3c.github.io/ServiceWorker/#create-job-algorithm>
    pub fn create_job(
        job_type: JobType,
        scope_url: ServoUrl,
        script_url: ServoUrl,
        client: IpcSender<JobResult>,
        referrer: ServoUrl,
        scope_things: Option<ScopeThings>,
    ) -> Job {
        Job {
            job_type,
            scope_url,
            script_url,
            client,
            referrer,
            scope_things,
        }
    }
}

impl PartialEq for Job {
    /// Equality criteria as described in <https://w3c.github.io/ServiceWorker/#dfn-job-equivalent>
    fn eq(&self, other: &Self) -> bool {
        // TODO: match on job type, take worker type and `update_via_cache_mode` into account.
        let same_job = self.job_type == other.job_type;
        if same_job {
            match self.job_type {
                JobType::Register | JobType::Update => {
                    self.scope_url == other.scope_url && self.script_url == other.script_url
                },
                JobType::Unregister => self.scope_url == other.scope_url,
            }
        } else {
            false
        }
    }
}

/// Messages outgoing from the Service Worker Manager thread to constellation
#[derive(Debug, Deserialize, Serialize)]
pub enum SWManagerMsg {
    /// Placeholder to keep the enum,
    /// as it will be needed when implementing
    /// <https://github.com/servo/servo/issues/24660>
    PostMessageToClient,
}
