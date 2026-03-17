use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use crate::{Event, EventListener};

#[derive(Default)]
pub struct EventDispatcher {
    listeners: Mutex<Vec<Arc<dyn EventListener>>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        EventDispatcher {
            listeners: Mutex::new(Vec::new()),
        }
    }

    pub fn register_listener(
        &self,
        listener: Arc<dyn EventListener>,
    ) -> Result<(), EventDispatcherError> {
        self.listeners.lock()?.push(listener);

        Ok(())
    }

    pub async fn dispatch(&self, event: impl Event + 'static) -> Result<(), EventDispatcherError> {
        let event = Arc::new(event);

        let listeners = self.listeners.lock()?.clone();
        for listener in listeners {
            listener.handle(&*event).await;
        }

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum EventDispatcherError {
    #[error("Mutex was poisoned")]
    MutexPoisoned,
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for EventDispatcherError {
    fn from(_: PoisonError<MutexGuard<'_, T>>) -> Self {
        EventDispatcherError::MutexPoisoned
    }
}

#[cfg(test)]
mod tests {
    use crate::mock::{MockListener, TestEvent};

    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_register_listener() {
        let dispatcher = EventDispatcher::new();
        let listener = Arc::new(MockListener::default());

        let result = dispatcher.register_listener(listener.clone());
        assert!(result.is_ok());

        let listeners = dispatcher.listeners.lock();
        match listeners {
            Ok(guard) => assert_eq!(guard.len(), 1),
            Err(_) => panic!("Mutex was poisoned"),
        }
    }

    #[tokio::test]
    async fn test_dispatch_event() {
        let dispatcher = EventDispatcher::new();
        let listener = Arc::new(MockListener::default());

        dispatcher.register_listener(listener.clone()).unwrap();

        let event = TestEvent {
            data: "test_data".to_string(),
        };
        let result = dispatcher.dispatch(event).await;
        assert!(result.is_ok());

        let events_received = listener.events_received.lock();
        match events_received {
            Ok(guard) => {
                assert_eq!(guard.len(), 1);
                assert_eq!(guard[0], "test_data");
            }
            Err(_) => panic!("Mutex was poisoned"),
        }
    }

    #[tokio::test]
    async fn test_dispatch_multiple_listeners() {
        let dispatcher = EventDispatcher::new();
        let listener1 = Arc::new(MockListener::default());
        let listener2 = Arc::new(MockListener::default());

        dispatcher.register_listener(listener1.clone()).unwrap();
        dispatcher.register_listener(listener2.clone()).unwrap();

        let event = TestEvent {
            data: "test_data".to_string(),
        };
        let result = dispatcher.dispatch(event).await;
        assert!(result.is_ok());

        // Verify both listeners received the event
        let events_received1 = listener1.events_received.lock();
        match events_received1 {
            Ok(guard) => {
                assert_eq!(guard.len(), 1);
                assert_eq!(guard[0], "test_data");
            }
            Err(_) => panic!("Mutex was poisoned"),
        }

        let events_received2 = listener2.events_received.lock();
        match events_received2 {
            Ok(guard) => {
                assert_eq!(guard.len(), 1);
                assert_eq!(guard[0], "test_data");
            }
            Err(_) => panic!("Mutex was poisoned"),
        }
    }
}
