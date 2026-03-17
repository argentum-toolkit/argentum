use crate::{Event, EventListener};
use async_trait::async_trait;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct TestEvent {
    pub data: String,
}

impl Event for TestEvent {
    fn name(&self) -> &'static str {
        "TestEvent"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Default)]
pub struct MockListener {
    pub events_received: std::sync::Mutex<Vec<String>>,
}

#[async_trait]
impl EventListener for MockListener {
    async fn handle(&self, event: &(dyn Event + 'static)) {
        if let Some(test_event) = event.as_any().downcast_ref::<TestEvent>() {
            let mut events = self.events_received.lock().unwrap();
            events.push(test_event.data.clone());
        }
    }
}
