use std::{
    any::{Any, TypeId, type_name},
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::di_container::DiError;

type FactoryResult = Result<Box<dyn Any + Send + Sync>, DiError>;

type Factory = Box<dyn Fn(&Container) -> FactoryResult + Send + Sync>;

pub struct Container {
    instances: Mutex<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
    factories: HashMap<TypeId, Factory>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            instances: Mutex::new(HashMap::new()),
            factories: HashMap::new(),
        }
    }

    pub fn provide_factory<T, F>(&mut self, factory: F)
    where
        T: ?Sized + Send + Sync + 'static,
        F: Fn(&Container) -> Result<Arc<T>, DiError> + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();

        self.factories.insert(
            type_id,
            Box::new(move |container| {
                let service = factory(container)?;

                Ok(Box::new(service))
            }),
        );
    }

    pub fn resolve<T>(&self) -> Result<Arc<T>, DiError>
    where
        T: ?Sized + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();

        //
        // 1. Looking for explicit instance.
        //
        {
            let instances = self.instances.lock().unwrap();

            if let Some(service) = instances.get(&type_id) {
                return service.downcast_ref::<Arc<T>>().cloned().ok_or_else(|| {
                    DiError::TypeMismatch {
                        type_name: type_name::<T>(),
                    }
                });
            }
        }

        //
        // 2. looking for a factory.
        //

        let factory = self
            .factories
            .get(&type_id)
            .ok_or_else(|| DiError::NotRegistered {
                type_name: type_name::<T>(),
            })?;

        let service = factory(self)?;

        let service = service
            .downcast::<Arc<T>>()
            .map_err(|_| DiError::TypeMismatch {
                type_name: type_name::<T>(),
            })?;

        let service = *service;

        //
        // 3. Save instance
        //
        let mut instances = self.instances.lock().unwrap();

        if let Some(existing) = instances.get(&type_id) {
            return existing.downcast_ref::<Arc<T>>().cloned().ok_or_else(|| {
                DiError::TypeMismatch {
                    type_name: type_name::<T>(),
                }
            });
        }

        instances.insert(type_id, Box::new(service.clone()));

        Ok(service)
    }
}
