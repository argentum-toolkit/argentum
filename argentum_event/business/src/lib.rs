//! Event component
//!
//! The easiest way to work with events, listeners, and dispatchers
//! Provides a macro to create and manage your own events

///
/// The way to create event and boilerplate to listening and dispatching
///
/// ```rust
/// use argentum_event_business::event_boilerplate;
///
/// // Event type
/// pub struct MyEvent {}
///
/// // Create a boilerplate
/// event_boilerplate!(MyEvent, MyEventListenerTrait, MyEventDispatcher);
///
/// ```
///
/// It is equivalent of a code below.
///
/// ```rust
/// use argentum_event_business::event_boilerplate;
///
/// pub struct MyEvent {}
///
/// pub trait MyEventListenerTrait {
///     fn listen(&self, _e: &MyEvent) {}
/// }
///
/// pub struct MyEventDispatcher<Listener: MyEventListenerTrait> {
///     listeners: Vec<Listener>,
/// }
///
/// impl<Listener: MyEventListenerTrait> MyEventDispatcher<Listener> {
///     pub fn new(listeners: Vec<Listener>) -> MyEventDispatcher<Listener> {
///         MyEventDispatcher { listeners }
///     }
///
///     pub fn dispatch(&self, e: &MyEvent) {
///         self.listeners.iter().for_each(move |l| {
///             l.listen(e);
///         });
///     }
/// }
/// ```
///
/// Of course, you can do it manually but macro provides the shorter and easier way.
///
///
/// Next step: create a listener
///
/// ```ignore
/// pub struct MyListener {}
///
/// impl MyEventListenerTrait for MyListener {
///     fn listen(&self, _e: &MyEvent) {
///         println!("listened MyEvent")
///     }
/// }
///```
///
/// Instantiate a listener, and a dispatcher. We mean that you will inject a dispatcher in your services
///
/// ```ignore
/// let my_listener = MyListener {};
/// let my_dispatcher = MyEventDispatcher::new(vec![my_listener]);
/// ```
///
/// Now we can emit an event
///
/// ```ignore
/// let e = MyEvent {};
/// my_dispatcher.dispatch(&e);
/// ```
///
#[macro_export]
macro_rules! event_boilerplate {
    ($event: ident, $listener_trait:ident, $dispatcher:ident) => {
        pub trait $listener_trait {
            fn listen(&self, _e: &$event) {}
        }

        pub struct $dispatcher<Listener: $listener_trait> {
            listeners: Vec<Listener>,
        }

        impl<Listener: $listener_trait> $dispatcher<Listener> {
            pub fn new(listeners: Vec<Listener>) -> $dispatcher<Listener> {
                $dispatcher { listeners }
            }

            pub fn dispatch(&self, e: &$event) {
                self.listeners.iter().for_each(move |l| {
                    l.listen(e);
                });
            }
        }
    };
}

#[cfg(test)]
mod test {
    pub struct TestEvent {
        user_id: u8,
    }

    impl TestEvent {
        pub fn new(user_id: u8) -> TestEvent {
            TestEvent { user_id }
        }
    }

    event_boilerplate!(TestEvent, TestListenerTrait, TestEventDispatcher);

    pub struct TestListener {}

    impl TestListenerTrait for TestListener {
        fn listen(&self, e: &TestEvent) {
            println!("listened TestEvent for user. user_id {}", e.user_id)
        }
    }

    #[test]
    fn test() -> Result<(), &'static str> {
        let listener = TestListener {};
        let dispatcher = TestEventDispatcher::new(vec![listener]);

        let event = TestEvent::new(1);
        dispatcher.dispatch(&event);

        assert_eq!(1, 1);

        Ok(())
    }
}
