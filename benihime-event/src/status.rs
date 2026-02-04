use std::{borrow::Cow, time::Duration};

use once_cell::sync::OnceCell;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{runtime_local, send_blocking};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Severity {
    Hint,
    Info,
    Warning,
    Error,
}

pub struct StatusMessage {
    pub severity: Severity,
    pub message: Cow<'static, str>,
}

impl From<anyhow::Error> for StatusMessage {
    fn from(err: anyhow::Error) -> Self {
        StatusMessage {
            severity: Severity::Error,
            message: err.to_string().into(),
        }
    }
}

impl From<&'static str> for StatusMessage {
    fn from(msg: &'static str) -> Self {
        StatusMessage {
            severity: Severity::Info,
            message: msg.into(),
        }
    }
}

runtime_local! {
    static MESSAGES: OnceCell<Sender<StatusMessage>> = OnceCell::new();
}

pub async fn report(msg: impl Into<StatusMessage>) {
    let _ = MESSAGES
        .wait()
        .send_timeout(msg.into(), Duration::from_millis(10))
        .await;
}

pub fn report_blocking(msg: impl Into<StatusMessage>) {
    let messages = MESSAGES.wait();
    send_blocking(messages, msg.into())
}

pub fn setup() -> Receiver<StatusMessage> {
    let (tx, rx) = tokio::sync::mpsc::channel(128);
    let _ = MESSAGES.set(tx);
    rx
}
