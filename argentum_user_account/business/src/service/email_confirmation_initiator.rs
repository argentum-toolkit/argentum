use crate::entity::EmailConfirmationToken;
use crate::repository::email_confirmation_token_repository::{
    EmailConfirmationTokenRepositoryError, EmailConfirmationTokenRepositoryTrait,
};
use argentum_notification_business::{Notification, NotifierTrait};
use argentum_user_business::entity::user::Member;
use std::sync::Arc;

use crate::token::GeneratorTrait;
use argentum_standard_business::data_type::id::Id;

pub struct EmailConfirmationInitiator<I>
where
    I: Id,
{
    token_generator: Arc<dyn GeneratorTrait<Id = I>>,
    email_confirmation_token_repository: Arc<dyn EmailConfirmationTokenRepositoryTrait<Id = I>>,
    notifier: Arc<dyn NotifierTrait<Id = I>>,
    product_name: String,
    email_confirmation_front_url: String,
}

impl<I> EmailConfirmationInitiator<I>
where
    I: Id,
{
    pub fn new(
        token_generator: Arc<dyn GeneratorTrait<Id = I>>,
        email_confirmation_token_repository: Arc<dyn EmailConfirmationTokenRepositoryTrait<Id = I>>,
        notifier: Arc<dyn NotifierTrait<Id = I>>,
        product_name: &str,
        email_confirmation_front_url: &str,
    ) -> Self {
        Self {
            token_generator,
            email_confirmation_token_repository,
            notifier,
            product_name: product_name.into(),
            email_confirmation_front_url: email_confirmation_front_url.into(),
        }
    }

    pub async fn initiate_confirmation(
        &self,
        user: &Member<I>,
    ) -> Result<(), EmailConfirmationInitiatorError> {
        // Generate email confirmation token
        let token = self.token_generator.generate(user.id);
        let email_confirmation_token = EmailConfirmationToken::new(user.id, user.id, &token);

        // Save email confirmation token
        self.email_confirmation_token_repository
            .save(&email_confirmation_token)
            .await?;

        // Send email confirmation notification
        self.send_email_confirmation_notification(user, &token)
            .await?;

        Ok(())
    }

    async fn send_email_confirmation_notification(
        &self,
        user: &Member<I>,
        token: &str,
    ) -> Result<(), EmailConfirmationInitiatorError> {
        let body = format!(
            "<p>Hello, <b>{}</b>!</p>
            <p>Thank you for registering with {}. To complete your registration, please confirm your email address by clicking the link below:</p>
            <p><a href=\"{}{}\" style=\"background-color: #007bff; color: white; padding: 10px 20px; text-decoration: none; border-radius: 5px;\">Confirm Email Address</a></p>
            <p>If you did not request this registration, please ignore this email.</p>
            <p><br/><br/><i>Best regards,<br>The {} Team</i></p>",
            user.name.first, self.product_name, self.email_confirmation_front_url, token, self.product_name,
        );

        let subject = format!("Confirm Your Email for {}", self.product_name);
        let notification = Notification::new(user.id, body, subject);

        self.notifier
            .send(notification)
            .await
            .map_err(|_| EmailConfirmationInitiatorError::NotificationError)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum EmailConfirmationInitiatorError {
    #[error("Failed to save email confirmation token")]
    SavingEmailConfirmationTokenError(
        #[source]
        #[from]
        EmailConfirmationTokenRepositoryError,
    ),

    #[error("Failed to send email confirmation notification")]
    NotificationError,
}

#[cfg(test)]
mod tests {
    use crate::mock::token::TokenGeneratorMock;
    use crate::service::email_confirmation_initiator::EmailConfirmationInitiator;
    use argentum_notification_business::mock::StdoutNotifier;
    use argentum_standard_business::data_type::EmailAddress;
    use argentum_standard_business::data_type::id::IdFactory;
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use argentum_user_business::data_type::{Name, NamePart};
    use argentum_user_business::entity::user::Member;
    use std::sync::Arc;

    use crate::mock::repository::email_confirmation_token_repository_mock::EmailConfirmationTokenRepositoryMock;

    #[tokio::test]
    async fn initiate_confirmation_success() -> Result<(), &'static str> {
        let token_generator = Arc::new(TokenGeneratorMock::new());
        let email_confirmation_token_repository =
            Arc::new(EmailConfirmationTokenRepositoryMock::new());
        let notifier = Arc::new(StdoutNotifier::new());
        let id_factory = Arc::new(IdFactoryMock::new());

        let initiator = EmailConfirmationInitiator::new(
            token_generator,
            email_confirmation_token_repository,
            notifier,
            "Test Product",
            "http://localhost:3000/confirm/",
        );

        let user_id = id_factory.create();
        let name = Name::new(
            NamePart::try_new("John".into()).expect("Should be valid"),
            Some(NamePart::try_new("Doe".into()).expect("Should be valid")),
            None,
        );
        let email = EmailAddress::try_new("john.doe@example.com".into()).expect("Should be valid");
        let user = Member::new(user_id.clone(), name, email, false);

        let result = initiator.initiate_confirmation(&user).await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Confirmation initiation failed"),
        }
    }
}
