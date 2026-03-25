use crate::{Event, EventListener};
use async_trait::async_trait;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct EventMock {
    pub data: String,
}

impl Event for EventMock {
    fn name(&self) -> &'static str {
        "TestEvent"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Default)]
pub struct ListenerMock {
    pub events_received: std::sync::Mutex<Vec<String>>,
}

#[async_trait]
impl EventListener for ListenerMock {
    async fn listen(&self, event: &dyn Event) {
        if let Some(test_event) = event.as_any().downcast_ref::<EventMock>() {
            let mut events = self.events_received.lock().unwrap();
            events.push(test_event.data.clone());
        }
    }
}
