use std::borrow::Borrow;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

use servo_url::ServoUrl;

use crate::conversions::Convert;
use crate::dom::bindings::root::DomRoot;
use crate::dom::document::Document;

/// <https://html.spec.whatwg.org/multipage/#document-state-2>
#[derive(Default)]
pub struct DocumentState {
    pub document: Option<DomRoot<Document>>,
    pub reload_pending: bool,
}

impl Convert<SessionHistoryEntry> for DocumentState {
    fn convert(self) -> SessionHistoryEntry {
        let document = self.document.expect("Document must be set");

        SessionHistoryEntry {
            step: Default::default(),
            url: document.borrow().url(),
            document_state: DocumentState {
                document: Some(document),
                reload_pending: false
            },
            scroll_restoration_mode: Default::default()
        }
    }
}

#[derive(Default)]
pub enum SessionHistoryEntryStep {
    #[default]
    Pending,
    Integer(usize),
}

/// <https://html.spec.whatwg.org/multipage/#scroll-restoration-mode>
#[derive(Default)]
pub enum ScrollRestorationMode {
    /// The user agent is responsible for restoring the scroll position upon navigation.
    #[default]
    Auto,
    /// The page is responsible for restoring the scroll position and the user agent does not
    /// attempt to do so automatically
    Manual,
}

/// <https://html.spec.whatwg.org/multipage/#session-history-entry>
pub struct SessionHistoryEntry {
    step: SessionHistoryEntryStep,
    url: ServoUrl,
    document_state: DocumentState,
    scroll_restoration_mode: ScrollRestorationMode,
}

impl Default for SessionHistoryEntry {
    fn default() -> Self {
        Self {
            step: Default::default(),
            url: ServoUrl::parse("about:blank").unwrap(),
            document_state: Default::default(),
            scroll_restoration_mode: ScrollRestorationMode::Auto,
        }
    }
}

pub struct Navigable {
    id: u64,
    parent: Option<Weak<Navigable>>,
    current_session_history_entry: Rc<SessionHistoryEntry>,
    active_session_history_entry: Rc<SessionHistoryEntry>,
    pub is_closing: bool,
    pub is_delaying_load_events: bool,
}

impl Navigable {
    /// <https://html.spec.whatwg.org/multipage/#initialize-the-navigable>
    pub fn initialize(&mut self, document_state: DocumentState, parent: &Option<Weak<Navigable>>) {
        // 1. Assert: documentState's document is non-null.
        debug_assert!(document_state.document.is_some());

        // 2. Let entry be a new session history entry, with:
        let entry = Rc::new(document_state.convert());

        // 3. Set navigable's current session history entry to entry.
        self.current_session_history_entry.clone_from(&entry);

        // 4.Set navigable's active session history entry to entry.
        self.active_session_history_entry.clone_from(&entry);

        // 5. Set navigable's parent to parent.
        self.parent.clone_from(parent);
    }
}

pub struct TraversableNavigable {
    current_session_history_step: usize,
    session_history_entries: Vec<Rc<SessionHistoryEntry>>
}

/// Trait for a parallel queue.
/// <https://html.spec.whatwg.org/multipage/infrastructure.html#parallel-queue>
pub trait ParallelQueue {
    type Step: FnOnce() + Send + 'static;

    /// Enqueue a step into the parallel queue.
    fn enqueue(&self, step: Self::Step);

    /// Start processing the parallel queue.
    fn start() -> Self
    where
        Self: Sized;

    /// Stop processing the parallel queue.
    fn stop(&mut self);
}

/// A concrete implementation of the ParallelQueue trait.
pub struct ParallelQueueImpl<T>
where
    T: FnOnce() + Send + 'static,
{
    queue: Arc<Mutex<Vec<T>>>,        // Shared queue of tasks.
    condvar: Arc<Condvar>,            // Condition variable for signaling.
    is_running: Arc<Mutex<bool>>,     // Indicates if the worker thread should run.
    worker_handle: Option<thread::JoinHandle<()>>, // Worker thread handle.
}

impl<T> ParallelQueueImpl<T>
where
    T: FnOnce() + Send + 'static,
{
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(Vec::new())),
            condvar: Arc::new(Condvar::new()),
            is_running: Arc::new(Mutex::new(false)),
            worker_handle: None,
        }
    }

    /// Internal function to process tasks.
    fn process_tasks(queue: Arc<Mutex<Vec<T>>>, condvar: Arc<Condvar>, is_running: Arc<Mutex<bool>>) {
        loop {
            let mut guard = queue.lock().unwrap();

            // Wait for tasks or a signal to stop.
            while guard.is_empty() && *is_running.lock().unwrap() {
                guard = condvar.wait(guard).unwrap();
            }

            // Exit the loop if the queue is stopped.
            if !*is_running.lock().unwrap() {
                break;
            }

            // Process tasks while the queue is non-empty.
            while let Some(task) = guard.pop() {
                drop(guard); // Unlock the queue while running the task.
                task();      // Run the task.
                guard = queue.lock().unwrap(); // Reacquire the lock.
            }
        }
    }
}

impl<T> ParallelQueue for ParallelQueueImpl<T>
where
    T: FnOnce() + Send + 'static,
{
    type Step = T;

    fn enqueue(&self, step: Self::Step) {
        let mut guard = self.queue.lock().unwrap();
        guard.push(step);
        drop(guard); // Unlock before notifying.
        self.condvar.notify_one(); // Signal the worker thread.
    }

    fn start() -> Self {
        let parallel_queue = Self::new();
        let queue_clone = parallel_queue.queue.clone();
        let condvar_clone = parallel_queue.condvar.clone();
        let is_running_clone = parallel_queue.is_running.clone();

        // Set the running flag to true and start the worker thread.
        {
            let mut is_running = is_running_clone.lock().unwrap();
            *is_running = true;
        }

        let handle = thread::spawn(move || {
            Self::process_tasks(queue_clone, condvar_clone, is_running_clone);
        });

        ParallelQueueImpl {
            worker_handle: Some(handle),
            ..parallel_queue
        }
    }

    fn stop(&mut self) {
        // Set the running flag to false and notify all waiting threads.
        {
            let mut is_running = self.is_running.lock().unwrap();
            *is_running = false;
        }
        self.condvar.notify_all();

        // Join the worker thread if it's running.
        if let Some(handle) = self.worker_handle.take() {
            handle.join().expect("Failed to join worker thread");
        }
    }
}

fn queue_test() {
    // Start the parallel queue.
    let mut queue = ParallelQueueImpl::start();

    // Enqueue some tasks.
    queue.enqueue(|| println!("Task 1 is running"));
    queue.enqueue(|| println!("Task 2 is running"));
    queue.enqueue(|| println!("Task 3 is running"));

    // Allow time for tasks to execute.
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Stop the queue.
    queue.stop();
    println!("Parallel queue has been stopped.");
}