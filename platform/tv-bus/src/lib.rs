//! In-process event bus with subscriber filters.
use tv_kernel::events::{EventEnvelope, EventKind};
use std::sync::{Arc, Mutex};

pub type Handler = Arc<dyn Fn(&EventEnvelope) + Send + Sync>;

#[derive(Clone, Default)]
pub struct EventBus {
    inner: Arc<Mutex<Vec<(Option<EventKind>, Handler)>>>,
    log: Arc<Mutex<Vec<EventEnvelope>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe(&self, kind: Option<EventKind>, handler: Handler) {
        self.inner.lock().expect("bus").push((kind, handler));
    }

    pub fn publish(&self, envelope: EventEnvelope) {
        {
            self.log.lock().expect("log").push(envelope.clone());
        }
        let subs = self.inner.lock().expect("bus").clone();
        for (filter, handler) in subs {
            if filter.is_none() || filter == Some(envelope.event.kind) {
                handler(&envelope);
            }
        }
    }

    pub fn history(&self) -> Vec<EventEnvelope> {
        self.log.lock().expect("log").clone()
    }

    pub fn clear(&self) {
        self.log.lock().expect("log").clear();
    }
}
