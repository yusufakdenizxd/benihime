use std::{cell::RefCell, rc::Rc};

use benihime_event::{runtime_local, send_blocking, status::StatusMessage};
use futures_util::{
    future::{BoxFuture, Future, FutureExt, LocalBoxFuture},
    stream::{FuturesUnordered, StreamExt},
};
use once_cell::sync::OnceCell;
use tokio::sync::mpsc::{Receiver, Sender, channel};

use crate::{editor_state::EditorState, ui::composer::Composer};

pub type EditorCompositorCallback = Box<dyn FnOnce(&mut EditorState, &mut Composer) + Send>;
pub type EditorCallback = Box<dyn FnOnce(&mut EditorState) + Send>;

pub type LocalEditorCompositorCallback = Box<dyn FnOnce(&mut EditorState, &mut Composer)>;
pub type LocalEditorCallback = Box<dyn FnOnce(&mut EditorState)>;

runtime_local! {
    static JOB_QUEUE: OnceCell<Sender<Callback>> = OnceCell::new();
}

pub async fn dispatch_callback(job: Callback) {
    let _ = JOB_QUEUE.wait().send(job).await;
}

pub async fn dispatch(job: impl FnOnce(&mut EditorState, &mut Composer) + Send + 'static) {
    let _ = JOB_QUEUE
        .wait()
        .send(Callback::EditorCompositor(Box::new(job)))
        .await;
}

pub fn dispatch_blocking(job: impl FnOnce(&mut EditorState, &mut Composer) + Send + 'static) {
    let jobs = JOB_QUEUE.wait();
    send_blocking(jobs, Callback::EditorCompositor(Box::new(job)))
}

pub enum Callback {
    EditorCompositor(EditorCompositorCallback),
    Editor(EditorCallback),
}

pub enum LocalCallback {
    EditorCompositor(LocalEditorCompositorCallback),
    Editor(LocalEditorCallback),
}

pub type JobFuture = BoxFuture<'static, anyhow::Result<Option<Callback>>>;
pub type LocalJobFuture = LocalBoxFuture<'static, anyhow::Result<Option<LocalCallback>>>;

pub struct Job {
    pub future: BoxFuture<'static, anyhow::Result<Option<Callback>>>,
    pub wait: bool,
}

pub struct LocalJob {
    pub future: LocalJobFuture,
    pub wait: bool,
}

pub struct Jobs {
    pub wait_futures: FuturesUnordered<JobFuture>,
    pub callbacks: Receiver<Callback>,
    pub local_callbacks: Rc<RefCell<Vec<LocalCallback>>>,
    pub status_messages: Receiver<StatusMessage>,
}

impl Job {
    pub fn new<F: Future<Output = anyhow::Result<()>> + Send + 'static>(f: F) -> Self {
        Self {
            future: f.map(|r| r.map(|()| None)).boxed(),
            wait: false,
        }
    }

    pub fn with_callback<F: Future<Output = anyhow::Result<Callback>> + Send + 'static>(
        f: F,
    ) -> Self {
        Self {
            future: f.map(|r| r.map(Some)).boxed(),
            wait: false,
        }
    }

    pub fn wait_before_exiting(mut self) -> Self {
        self.wait = true;
        self
    }
}

impl LocalJob {
    pub fn new<F: Future<Output = anyhow::Result<()>> + 'static>(f: F) -> Self {
        Self {
            future: f.map(|r| r.map(|()| None)).boxed_local(),
            wait: false,
        }
    }

    pub fn with_callback<F: Future<Output = anyhow::Result<Option<LocalCallback>>> + 'static>(
        f: F,
    ) -> Self {
        Self {
            future: f.boxed_local(),
            wait: false,
        }
    }

    pub fn wait_before_exiting(mut self) -> Self {
        self.wait = true;
        self
    }
}

impl Jobs {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (tx, rx) = channel(1024);
        let _ = JOB_QUEUE.set(tx);
        let status_messages = benihime_event::status::setup();
        Self {
            wait_futures: FuturesUnordered::new(),
            callbacks: rx,
            local_callbacks: Rc::new(RefCell::new(Vec::new())),
            status_messages,
        }
    }

    pub fn spawn<F: Future<Output = anyhow::Result<()>> + Send + 'static>(&mut self, f: F) {
        self.add(Job::new(f));
    }

    pub fn callback<F: Future<Output = anyhow::Result<Callback>> + Send + 'static>(
        &mut self,
        f: F,
    ) {
        self.add(Job::with_callback(f));
    }

    pub fn spawn_local<F: Future<Output = anyhow::Result<()>> + 'static>(&mut self, f: F) {
        self.add_local(LocalJob::new(f));
    }

    pub fn callback_local<F: Future<Output = anyhow::Result<Option<LocalCallback>>> + 'static>(
        &mut self,
        f: F,
    ) {
        self.add_local(LocalJob::with_callback(f));
    }

    pub fn handle_callback(
        &self,
        editor: &mut EditorState,
        compositor: &mut Composer,
        call: anyhow::Result<Option<Callback>>,
    ) {
        match call {
            Ok(None) => {}
            Ok(Some(call)) => match call {
                Callback::EditorCompositor(call) => call(editor, compositor),
                Callback::Editor(call) => call(editor),
            },
            Err(e) => {
                editor.set_error(format!("Async job failed: {}", e));
            }
        }
    }

    pub fn handle_local_callback(
        &self,
        editor: &mut EditorState,
        compositor: &mut Composer,
        call: anyhow::Result<Option<LocalCallback>>,
    ) {
        match call {
            Ok(None) => {}
            Ok(Some(call)) => match call {
                LocalCallback::EditorCompositor(call) => call(editor, compositor),
                LocalCallback::Editor(call) => call(editor),
            },
            Err(e) => {
                editor.set_error(format!("Async job failed: {}", e));
            }
        }
    }

    pub fn add(&self, j: Job) {
        if j.wait {
            self.wait_futures.push(j.future);
        } else {
            tokio::spawn(async move {
                match j.future.await {
                    Ok(Some(cb)) => dispatch_callback(cb).await,
                    Ok(None) => (),
                    Err(err) => benihime_event::status::report(err).await,
                }
            });
        }
    }

    pub fn add_local(&self, j: LocalJob) {
        let local_callbacks = self.local_callbacks.clone();

        tokio::task::spawn_local(async move {
            match j.future.await {
                Ok(Some(cb)) => {
                    local_callbacks.borrow_mut().push(cb);
                }
                Ok(None) => (),
                Err(err) => benihime_event::status::report(err).await,
            }
        });
    }

    pub async fn finish(
        &mut self,
        editor: &mut EditorState,
        mut compositor: Option<&mut Composer>,
    ) -> anyhow::Result<()> {
        log::debug!("waiting on jobs...");
        let mut wait_futures = std::mem::take(&mut self.wait_futures);

        while let (Some(job), tail) = wait_futures.into_future().await {
            match job {
                Ok(callback) => {
                    wait_futures = tail;

                    if let Some(callback) = callback {
                        #[allow(clippy::needless_option_as_deref)]
                        match callback {
                            Callback::EditorCompositor(call) if compositor.is_some() => {
                                call(editor, compositor.as_deref_mut().unwrap())
                            }
                            Callback::Editor(call) => call(editor),

                            _ => (),
                        }
                    }
                }
                Err(e) => {
                    self.wait_futures = tail;
                    return Err(e);
                }
            }
        }

        Ok(())
    }
}
