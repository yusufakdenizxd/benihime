use tokio::{
    sync::mpsc::{self, Sender, error::TrySendError},
    time::Instant,
};

pub trait AsyncHook: Sync + Send + 'static + Sized {
    type Event: Sync + Send + 'static;
    fn handle_event(&mut self, event: Self::Event, timeout: Option<Instant>) -> Option<Instant>;

    fn finish_debounce(&mut self);

    fn spawn(self) -> mpsc::Sender<Self::Event> {
        let (tx, rx) = mpsc::channel(256);
        if tokio::runtime::Handle::try_current().is_ok() {
            tokio::spawn(run(self, rx));
        }
        tx
    }
}

async fn run<Hook: AsyncHook>(mut hook: Hook, mut rx: mpsc::Receiver<Hook::Event>) {
    let mut deadline = None;
    loop {
        let event = match deadline {
            Some(deadline_) => {
                let res = tokio::time::timeout_at(deadline_, rx.recv()).await;
                match res {
                    Ok(event) => event,
                    Err(_) => {
                        hook.finish_debounce();
                        deadline = None;
                        continue;
                    }
                }
            }
            None => rx.recv().await,
        };
        let Some(event) = event else {
            break;
        };
        deadline = hook.handle_event(event, deadline);
    }
}

pub fn send_blocking<T>(tx: &Sender<T>, data: T) {
    match tx.try_send(data) {
        Ok(()) => {}
        Err(TrySendError::Full(_)) => {}
        Err(TrySendError::Closed(_)) => {}
    }
}

pub fn try_send<T>(tx: &Sender<T>, data: T) -> bool {
    tx.try_send(data).is_ok()
}
