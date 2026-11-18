use std::sync::Arc;

use argentum_event_business::{Event, EventListener};
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::Id;
use argentum_user_account_business::event::BeforeMemberDeletedAccount;
use async_trait::async_trait;

use crate::repository::CredentialRepositoryTrait;

pub struct MemberDeletedAccountListener<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    credential_repository: Arc<dyn CredentialRepositoryTrait<UserId = I>>,
    logger: Arc<L>,
}

impl<I, L> MemberDeletedAccountListener<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub fn new(
        credential_repository: Arc<dyn CredentialRepositoryTrait<UserId = I>>,
        logger: Arc<L>,
    ) -> Self {
        Self {
            credential_repository,
            logger,
        }
    }
}

#[async_trait]
impl<I, L> EventListener for MemberDeletedAccountListener<I, L>
where
    I: Id + 'static,
    L: LoggerTrait,
{
    async fn listen(&self, event: &dyn Event) {
        let Some(event) = event
            .as_any()
            .downcast_ref::<BeforeMemberDeletedAccount<I>>()
        else {
            return;
        };

        // EventListener cannot return an error, so a failure is logged and the member is still deleted.
        if let Err(e) = self
            .credential_repository
            .delete_by_user_id(event.member_id)
            .await
        {
            self.logger
                .error(format!("OIDC credentials are not deleted. {e:?}"));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::entity::Credential;
    use crate::repository::CredentialRepositoryError;
    use argentum_event_business::EventDispatcher;
    use argentum_log_business::{Level, LoggerTrait};
    use argentum_standard_business::data_type::id::UIntId;
    use argentum_user_account_business::event::BeforeMemberDeletedAccount;
    use jiff::Timestamp;
    use std::sync::Mutex;

    #[derive(Default)]
    struct CredentialRepositoryStub {
        credentials: Mutex<Vec<Credential<UIntId>>>,
        fail_delete: bool,
    }

    #[async_trait]
    impl CredentialRepositoryTrait for CredentialRepositoryStub {
        type UserId = UIntId;

        async fn insert(
            &self,
            credential: &Credential<UIntId>,
        ) -> Result<(), CredentialRepositoryError> {
            self.credentials.lock().unwrap().push(Credential::new(
                &credential.external_id,
                credential.user_id,
                &credential.provider,
                credential.created_at,
            ));
            Ok(())
        }

        async fn find_by_external_id(
            &self,
            _external_id: &str,
            _provider: &str,
        ) -> Result<Option<Credential<UIntId>>, CredentialRepositoryError> {
            Ok(None)
        }

        async fn delete_by_user_id(
            &self,
            user_id: UIntId,
        ) -> Result<(), CredentialRepositoryError> {
            if self.fail_delete {
                return Err(CredentialRepositoryError::Delete(None));
            }

            self.credentials
                .lock()
                .unwrap()
                .retain(|c| c.user_id != user_id);
            Ok(())
        }
    }

    #[derive(Default)]
    struct LoggerStub {
        errors: Mutex<Vec<String>>,
    }

    impl LoggerTrait for LoggerStub {
        fn log<S: AsRef<str>>(&self, level: Level, msg: S) {
            if let Level::Error = level {
                self.errors.lock().unwrap().push(msg.as_ref().to_string());
            }
        }
    }

    fn credential(user_id: UIntId, external_id: &str) -> Credential<UIntId> {
        Credential::new(external_id, user_id, "GitHub", Timestamp::now())
    }

    #[tokio::test]
    async fn deletes_only_target_member_credentials() {
        let repository = Arc::new(CredentialRepositoryStub::default());
        repository.insert(&credential(1, "ext-1")).await.unwrap();
        repository.insert(&credential(1, "ext-2")).await.unwrap();
        repository.insert(&credential(2, "ext-3")).await.unwrap();

        let logger = Arc::new(LoggerStub::default());
        let listener = MemberDeletedAccountListener::new(repository.clone(), logger.clone());

        listener.listen(&BeforeMemberDeletedAccount::new(1)).await;

        let remaining = repository.credentials.lock().unwrap();
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].user_id, 2);
        assert!(logger.errors.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn ignores_unrelated_events() {
        let repository = Arc::new(CredentialRepositoryStub::default());
        repository.insert(&credential(1, "ext-1")).await.unwrap();

        let logger = Arc::new(LoggerStub::default());
        let listener = MemberDeletedAccountListener::new(repository.clone(), logger);

        listener
            .listen(&argentum_event_business::mock::EventMock {
                data: "other".into(),
            })
            .await;

        assert_eq!(repository.credentials.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn logs_error_and_does_not_panic_when_delete_fails() {
        let repository = Arc::new(CredentialRepositoryStub {
            fail_delete: true,
            ..Default::default()
        });
        repository.insert(&credential(1, "ext-1")).await.unwrap();

        let logger = Arc::new(LoggerStub::default());
        let listener = MemberDeletedAccountListener::new(repository.clone(), logger.clone());

        listener.listen(&BeforeMemberDeletedAccount::new(1)).await;

        assert_eq!(logger.errors.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn dispatcher_calls_listener_on_member_deleted_event() {
        let repository = Arc::new(CredentialRepositoryStub::default());
        repository.insert(&credential(1, "ext-1")).await.unwrap();

        let dispatcher = EventDispatcher::new();
        let logger = Arc::new(LoggerStub::default());
        dispatcher
            .register_listener(Arc::new(MemberDeletedAccountListener::new(
                repository.clone(),
                logger,
            )))
            .unwrap();

        dispatcher
            .dispatch(BeforeMemberDeletedAccount::new(1u64))
            .await
            .unwrap();

        assert!(repository.credentials.lock().unwrap().is_empty());
    }
}
