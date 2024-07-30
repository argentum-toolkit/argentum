use argentum_notification_business::{Notification, NotificationError, NotificatorTrait};
use argentum_user_business::repository::user_repository::AuthenticatedUserRepositoryTrait;

use lettre::{Message, SmtpTransport, Transport};

///
/// How to init notificator via `lettre`
///
///```rust
/// use argentum_notification_infrastructure::email_notificator::EmailNotificator;
/// use lettre::SmtpTransport;
/// use lettre::transport::smtp::authentication::Credentials;use argentum_user_business::mock::repository::authenticated_user_repository_mock::AuthenticatedUserRepositoryMock;
///
/// let smtp_creds = Credentials::new(
///     "smtp_username".to_string(),
///     "smtp_password".to_string()
/// );
///
/// let mailer = SmtpTransport::relay("smtp.gmail.com")
///     .unwrap()
///     .credentials(smtp_creds)
///     .build();
///
/// let authenticated_user_repository = AuthenticatedUserRepositoryMock::new();
///
/// let notificator = EmailNotificator::new(
///     "Demo app <demo@examples.com>".to_string(),
///     &authenticated_user_repository,
///     &mailer,
/// );
/// ```
pub struct EmailNotificator<'s> {
    from: String,
    user_repository: &'s dyn AuthenticatedUserRepositoryTrait,
    smtp_transport: &'s SmtpTransport,
}

impl<'s> EmailNotificator<'s> {
    pub fn new(
        from: String,
        user_repository: &'s dyn AuthenticatedUserRepositoryTrait,
        smtp_transport: &'s SmtpTransport,
    ) -> EmailNotificator<'s> {
        EmailNotificator {
            from,
            user_repository,
            smtp_transport,
        }
    }
}

impl<'s> NotificatorTrait for EmailNotificator<'s> {
    fn send(&self, notification: Notification) -> Result<(), NotificationError> {
        let user = match self.user_repository.find(&notification.user_id) {
            Ok(o) => match o {
                Some(u) => u,
                None => return Err(NotificationError::UserNotFoundError),
            },
            Err(e) => return Err(NotificationError::UserRepositoryError(e)),
        };

        let last = match user.name.last {
            Some(l) => l.to_string(),
            None => "".to_string(),
        };

        let to_mbox = format!("{} {} <{}>", user.name.first, last, user.email.as_string())
            .parse()
            .unwrap();

        let building_result = Message::builder()
            .from(self.from.parse().unwrap())
            .to(to_mbox)
            .subject(notification.subject)
            .body(notification.body);

        let email = match building_result {
            Ok(email) => email,
            Err(_) => {
                return Err(NotificationError::MessageSendingExternalError);
            }
        };

        match self.smtp_transport.send(&email) {
            Ok(_) => Ok(()),
            Err(_) => Err(NotificationError::MessageSendingExternalError),
        }
    }
}
