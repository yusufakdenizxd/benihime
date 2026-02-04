use std::{
    borrow::Borrow,
    future::Future,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering::Relaxed},
    },
};

use tokio::sync::Notify;

pub async fn cancelable_future<T>(
    future: impl Future<Output = T>,
    cancel: impl Borrow<TaskHandle>,
) -> Option<T> {
    tokio::select! {
        biased;
        _ = cancel.borrow().canceled() => {
            None
        }
        res = future => {
            Some(res)
        }
    }
}

#[derive(Default, Debug)]
struct Shared {
    state: AtomicU64,
    notify: Notify,
}

impl Shared {
    fn generation(&self) -> u32 {
        self.state.load(Relaxed) as u32
    }

    fn num_running(&self) -> u32 {
        (self.state.load(Relaxed) >> 32) as u32
    }
    fn inc_generation(&self, num_running: u32) -> (u32, u32) {
        let state = self.state.load(Relaxed);
        let generation = state as u32;
        let prev_running = (state >> 32) as u32;
        if prev_running == 0 && num_running == 0 {
            return (generation, 0);
        }
        let new_generation = generation.saturating_add(1);
        self.state.store(
            new_generation as u64 | ((num_running as u64) << 32),
            Relaxed,
        );
        self.notify.notify_waiters();
        (new_generation, prev_running)
    }

    fn inc_running(&self, generation: u32) {
        let mut state = self.state.load(Relaxed);
        loop {
            let current_generation = state as u32;
            if current_generation != generation {
                break;
            }
            let off = 1 << 32;
            let res = self.state.compare_exchange_weak(
                state,
                state.saturating_add(off),
                Relaxed,
                Relaxed,
            );
            match res {
                Ok(_) => break,
                Err(new_state) => state = new_state,
            }
        }
    }

    fn dec_running(&self, generation: u32) {
        let mut state = self.state.load(Relaxed);
        loop {
            let current_generation = state as u32;
            if current_generation != generation {
                break;
            }
            let num_running = (state >> 32) as u32;
            assert_ne!(num_running, 0);
            let off = 1 << 32;
            let res = self
                .state
                .compare_exchange_weak(state, state - off, Relaxed, Relaxed);
            match res {
                Ok(_) => break,
                Err(new_state) => state = new_state,
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct TaskController {
    shared: Arc<Shared>,
}

impl TaskController {
    pub fn new() -> Self {
        TaskController::default()
    }
    pub fn cancel(&mut self) -> bool {
        self.shared.inc_generation(0).1 != 0
    }

    pub fn is_running(&self) -> bool {
        self.shared.num_running() != 0
    }

    pub fn restart(&mut self) -> TaskHandle {
        TaskHandle {
            generation: self.shared.inc_generation(1).0,
            shared: self.shared.clone(),
        }
    }
}

impl Drop for TaskController {
    fn drop(&mut self) {
        self.cancel();
    }
}

pub struct TaskHandle {
    shared: Arc<Shared>,
    generation: u32,
}

impl Clone for TaskHandle {
    fn clone(&self) -> Self {
        self.shared.inc_running(self.generation);
        TaskHandle {
            shared: self.shared.clone(),
            generation: self.generation,
        }
    }
}

impl Drop for TaskHandle {
    fn drop(&mut self) {
        self.shared.dec_running(self.generation);
    }
}

impl TaskHandle {
    pub async fn canceled(&self) {
        let notified = self.shared.notify.notified();
        if !self.is_canceled() {
            notified.await
        }
    }

    pub fn is_canceled(&self) -> bool {
        self.generation != self.shared.generation()
    }
}
