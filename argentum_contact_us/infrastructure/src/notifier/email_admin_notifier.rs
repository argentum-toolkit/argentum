use crate::notifier::body::{build_body, build_subject};

use argentum_contact_us_business::entity::Message;
use argentum_contact_us_business::notifier::admin_notifier::{
    AdminNotifierError, AdminNotifierTrait,
};
use argentum_standard_business::data_type::id::Id;
use argentum_user_business::entity::user::Member;
use argentum_user_business::repository::user_repository::MemberRepositoryTrait;
use async_trait::async_trait;
use lettre::message::header::ContentType;
use lettre::{Message as EmailMessage, SmtpTransport, Transport, address::AddressError};
use std::sync::Arc;

///
/// Sends admin notifications via SMTP using `lettre`.
///
/// The sender's name and email are resolved from the member's account
/// when the message was created by a member.
///
///```rust,no_run
/// use argentum_contact_us_infrastructure::notifier::EmailAdminNotifier;
/// use argentum_user_business::mock::repository::member_repository_mock::MemberRepositoryMock;
/// use argentum_standard_business::data_type::id::UIntId;
/// use lettre::SmtpTransport;
/// use lettre::transport::smtp::authentication::Credentials;
///
/// use std::sync::Arc;
///
/// let smtp_creds = Credentials::new(
///     "smtp_username".to_string(),
///     "smtp_password".to_string(),
/// );
///
/// let mailer = SmtpTransport::relay("smtp.gmail.com")
///     .unwrap()
///     .credentials(smtp_creds)
///     .build();
///
/// let notifier = EmailAdminNotifier::<UIntId>::new(
///     "Demo app <demo@examples.com>".to_string(),
///     "admin@examples.com".to_string(),
///     "Demo app".to_string(),
///     Arc::new(MemberRepositoryMock::new()),
///     Arc::new(mailer),
/// );
/// ```
pub struct EmailAdminNotifier<I>
where
    I: Id,
{
    from: String,
    admin_email: String,
    product_name: String,
    member_repository: Arc<dyn MemberRepositoryTrait<Id = I>>,
    smtp_transport: Arc<SmtpTransport>,
}

impl<I> EmailAdminNotifier<I>
where
    I: Id,
{
    pub fn new(
        from: String,
        admin_email: String,
        product_name: String,
        member_repository: Arc<dyn MemberRepositoryTrait<Id = I>>,
        smtp_transport: Arc<SmtpTransport>,
    ) -> Self {
        Self {
            from,
            admin_email,
            product_name,
            member_repository,
            smtp_transport,
        }
    }
}

#[async_trait]
impl<I> AdminNotifierTrait for EmailAdminNotifier<I>
where
    I: Id,
{
    type Id = I;

    async fn send(&self, message: &Message<Self::Id>) -> Result<(), AdminNotifierError> {
        let sender: Option<Member<I>> = match &message.user_id {
            Some(user_id) => Some(
                self.member_repository
                    .find(*user_id)
                    .await
                    .map_err(|e| AdminNotifierError::External(Arc::new(e)))?
                    .ok_or_else(|| {
                        AdminNotifierError::External(Arc::new(std::io::Error::other(format!(
                            "Member `{user_id}` is not found"
                        ))))
                    })?,
            ),
            None => None,
        };

        let subject = build_subject(message, sender.as_ref(), &self.product_name);
        let body = build_body(message, sender.as_ref(), &self.product_name);

        let to_mbox = self
            .admin_email
            .clone()
            .parse()
            .map_err(|e: AddressError| AdminNotifierError::External(Arc::new(e)))?;

        let from_mbox = self
            .from
            .clone()
            .parse()
            .map_err(|e: AddressError| AdminNotifierError::External(Arc::new(e)))?;

        let email = EmailMessage::builder()
            .from(from_mbox)
            .to(to_mbox)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body)
            .map_err(|e| AdminNotifierError::External(Arc::new(e)))?;

        match self.smtp_transport.send(&email) {
            Ok(_) => Ok(()),
            Err(e) => Err(AdminNotifierError::External(Arc::new(e))),
        }
    }
}
