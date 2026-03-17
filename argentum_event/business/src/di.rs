use std::sync::Arc;

use crate::EventDispatcher;

pub struct EventBusinessDi {
    pub event_dispatcher: Arc<EventDispatcher>,
}

#[derive(Default)]
pub struct EventBusinessDiBuilder {
    pub event_dispatcher: Arc<EventDispatcher>,
}

impl EventBusinessDiBuilder {
    pub fn new() -> Self {
        EventBusinessDiBuilder {
            event_dispatcher: Arc::new(EventDispatcher::new()),
        }
    }

    pub fn build(&self) -> Result<Arc<EventBusinessDi>, String> {
        Ok(Arc::new(EventBusinessDi {
            event_dispatcher: self.event_dispatcher.clone(),
        }))
    }
}
