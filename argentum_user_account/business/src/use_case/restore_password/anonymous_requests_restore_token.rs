use crate::entity::restore_password_token::RestorePasswordToken;
use crate::repository::restore_password_token_repository::RestorePasswordTokenRepositoryTrait;
use crate::use_case::restore_password::error::RestorePasswordError;

use argentum_log_business::LoggerTrait;
use argentum_notification_business::{Notification, NotificatorTrait};
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::IdFactory;
use argentum_user_business::repository::user_repository::AuthenticatedUserRepositoryTrait;
use argentum_user_business::token::GeneratorTrait;
use std::sync::Arc;

pub struct AnonymousRequestsRestoreToken {
    //configurable param
    product_name: String,
    /// First part of url
    restore_password_front_url: String,
    id_factory: Arc<dyn IdFactory>,
    user_repository: Arc<dyn AuthenticatedUserRepositoryTrait>,
    restore_password_token_repository: Arc<dyn RestorePasswordTokenRepositoryTrait>,
    token_generator: Arc<dyn GeneratorTrait>,
    notificator: Arc<dyn NotificatorTrait>,
    logger: Arc<dyn LoggerTrait>,
}

impl AnonymousRequestsRestoreToken {
    pub fn new(
        product_name: String,
        restore_password_front_url: String,
        id_factory: Arc<dyn IdFactory>,
        user_repository: Arc<dyn AuthenticatedUserRepositoryTrait>,
        restore_password_token_repository: Arc<dyn RestorePasswordTokenRepositoryTrait>,
        token_generator: Arc<dyn GeneratorTrait>,
        notificator: Arc<dyn NotificatorTrait>,
        logger: Arc<dyn LoggerTrait>,
    ) -> AnonymousRequestsRestoreToken {
        AnonymousRequestsRestoreToken {
            product_name,
            restore_password_front_url,
            id_factory,
            user_repository,
            restore_password_token_repository,
            token_generator,
            notificator,
            logger,
        }
    }

    pub fn execute(
        &self,
        email: EmailAddress,
    ) -> Result<RestorePasswordToken, RestorePasswordError> {
        let result = self.user_repository.find_by_email(&email);
        let user = match result {
            Err(err) => return Err(RestorePasswordError::GetUserError(err)),
            Ok(o) => match o {
                Some(user) => user,
                None => return Err(RestorePasswordError::UserNotFoundError),
            },
        };

        let token = self.token_generator.generate(&user.id);
        let id = self.id_factory.create();

        let restore_token = RestorePasswordToken::new(id, user.id.clone(), token.clone());

        if let Err(e) = self
            .restore_password_token_repository
            .delete_users_tokens(&user.id)
        {
            return Err(RestorePasswordError::TokenRepositoryError(e));
        }

        if let Err(e) = self.restore_password_token_repository.save(&restore_token) {
            return Err(RestorePasswordError::TokenRepositoryError(e));
        }

        let last = match user.name.last {
            Some(l) => l,
            None => "".to_string(),
        };

        let body = format!(
            "<p>Hello, dear <b>{} {}</b>!</p>

            <p>
            We're received a request to reset the password for the {} account.
            You can change your password by following the link bellow
            </p>
            <a href=\"{}{}\">Reset my password</a>
            <p>
            <br/><br/>
            <i>The {} team</i>
            </p>

        ",
            user.name.first,
            last,
            self.product_name,
            self.restore_password_front_url,
            token,
            self.product_name,
        );

        let subject = format!("Reset password {}", self.product_name);
        let notification = Notification::new(user.id, body, subject);
        if let Err(e) = self.notificator.send(notification) {
            self.logger
                .error(format!("Restore token is not sent. {:?}", e));
        }

        Ok(restore_token)
    }
}

#[cfg(test)]
mod tests {
    use crate::mock::repository::restore_password_token_repository_mock::RestorePasswordTokenRepositoryMock;
    use crate::mock::token::TokenGeneratorMock;
    use crate::repository::restore_password_token_repository::RestorePasswordTokenRepositoryTrait;
    use crate::use_case::restore_password::anonymous_requests_restore_token::{
        AnonymousRequestsRestoreToken, RestorePasswordError,
    };

    use argentum_log_business::{DefaultLogger, Level, StdoutWriter};
    use argentum_notification_business::mock::StdoutNotificator;
    use argentum_standard_business::data_type::email::EmailAddress;
    use argentum_standard_business::data_type::id::IdFactory;
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use argentum_user_business::data_type::Name;
    use argentum_user_business::entity::user::AuthenticatedUser;
    use argentum_user_business::mock::repository::authenticated_user_repository_mock::AuthenticatedUserRepositoryMock;
    use argentum_user_business::repository::user_repository::AuthenticatedUserRepositoryTrait;
    use std::sync::Arc;

    #[test]
    fn anonymous_requests_restore_token() -> Result<(), &'static str> {
        let id_factory = Arc::new(IdFactoryMock::new());
        let token_repository = Arc::new(RestorePasswordTokenRepositoryMock::new());
        let user_repository = Arc::new(AuthenticatedUserRepositoryMock::new());
        let token_generator = Arc::new(TokenGeneratorMock::new());

        let notificator = Arc::new(StdoutNotificator::new());
        let writer = Arc::new(StdoutWriter::new());
        let logger = Arc::new(DefaultLogger::new(Level::Info, writer.clone()));

        let uc = AnonymousRequestsRestoreToken::new(
            "Test company".to_string(),
            "http://localhost/reset-password".to_string(),
            id_factory.clone(),
            user_repository.clone(),
            token_repository.clone(),
            token_generator.clone(),
            notificator.clone(),
            logger.clone(),
        );

        let user_id = id_factory.create();
        let user_name = Name::try_new("Dionne".into(), Some("Morrison".into()), None).unwrap();

        let email = EmailAddress::try_new("test@mail.com".into()).unwrap();

        let user = AuthenticatedUser::new(&user_id, user_name, email.clone());

        user_repository.save(&user).unwrap();

        let result = uc.execute(email);

        if let Err(_) = result {
            return Err("User is not registered");
        }

        let token = result.unwrap();

        assert!(user_id.eq(&token.user_id), "Wrong user id in token");

        match token_repository.find_by_token(token.token).unwrap() {
            Some(stored_token) => {
                assert!(stored_token.id.eq(&token.id), "Wrong token id");
                assert!(user_id.eq(&stored_token.user_id), "Wrong user id in token");

                Ok(())
            }
            None => Err("Token is not saved"),
        }
    }

    #[test]
    fn anonymous_requests_restore_token_for_not_registered_email() -> Result<(), &'static str> {
        let id_factory = Arc::new(IdFactoryMock::new());
        let token_repository = Arc::new(RestorePasswordTokenRepositoryMock::new());
        let user_repository = Arc::new(AuthenticatedUserRepositoryMock::new());
        let token_generator = Arc::new(TokenGeneratorMock::new());

        let notificator = Arc::new(StdoutNotificator::new());
        let writer = Arc::new(StdoutWriter::new());
        let logger = Arc::new(DefaultLogger::new(Level::Info, writer.clone()));

        let uc = AnonymousRequestsRestoreToken::new(
            "Test company".to_string(),
            "http://localhost/reset-password".to_string(),
            id_factory.clone(),
            user_repository.clone(),
            token_repository.clone(),
            token_generator.clone(),
            notificator.clone(),
            logger.clone(),
        );

        let email = EmailAddress::try_new("test@mail.com".into()).unwrap();

        let result = uc.execute(email);

        match result {
            Err(e) => match e {
                RestorePasswordError::UserNotFoundError => Ok(()),
                _ => Err("Wrong error type"),
            },
            Ok(_) => Err("Should return not found error"),
        }
    }
}
