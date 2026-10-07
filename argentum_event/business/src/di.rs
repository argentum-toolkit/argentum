use argentum_standard_business::di_container::Container;
use std::sync::Arc;

use crate::EventDispatcher;

pub fn event_business_provide(container: &mut Container) {
    container.provide_factory::<EventDispatcher, _>(|_| Ok(Arc::new(EventDispatcher::new())));
}
