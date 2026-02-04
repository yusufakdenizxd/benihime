use std::future::Future;

use parking_lot::{RwLock, RwLockReadGuard};
use tokio::sync::Notify;

use crate::runtime_local;

runtime_local! {
    static REDRAW_NOTIFY: Notify = Notify::const_new();

    static RENDER_LOCK: RwLock<()> = RwLock::new(());
}

pub type RenderLockGuard = RwLockReadGuard<'static, ()>;

pub fn request_redraw() {
    REDRAW_NOTIFY.notify_one();
}

pub fn redraw_requested() -> impl Future<Output = ()> {
    REDRAW_NOTIFY.notified()
}

pub fn start_frame() {
    drop(RENDER_LOCK.write());
    let notify = REDRAW_NOTIFY.notified();
    tokio::pin!(notify);
    notify.enable();
}

pub fn lock_frame() -> RenderLockGuard {
    RENDER_LOCK.read()
}

pub struct RequestRedrawOnDrop;

impl Drop for RequestRedrawOnDrop {
    fn drop(&mut self) {
        request_redraw();
    }
}
