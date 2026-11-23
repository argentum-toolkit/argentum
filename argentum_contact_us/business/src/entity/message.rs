use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_business::data_type::id::Id;
use jiff::Timestamp;

const MAX_SUBJECT_LENGTH: usize = 255;

#[derive(thiserror::Error, Debug, Clone)]
pub enum MessageError {
    #[error("Subject should not be empty")]
    EmptySubject,

    #[error("Subject is too long")]
    TooLongSubject,

    #[error("Message should not be empty")]
    EmptyMessage,
}

#[derive(Clone)]
pub struct Message<I>
where
    I: Id,
{
    pub id: I,
    pub user_id: Option<I>,
    pub name: Option<String>,
    pub email: Option<EmailAddress>,
    pub subject: String,
    pub message: String,
    pub created_at: Timestamp,
}

impl<I> Message<I>
where
    I: Id,
{
    pub fn new_guest(
        id: I,
        name: String,
        email: EmailAddress,
        subject: &str,
        message: &str,
    ) -> Result<Self, MessageError> {
        Self::new(id, None, Some(name), Some(email), subject, message)
    }

    pub fn new_member(
        id: I,
        user_id: I,
        subject: &str,
        message: &str,
    ) -> Result<Self, MessageError> {
        Self::new(id, Some(user_id), None, None, subject, message)
    }

    fn new(
        id: I,
        user_id: Option<I>,
        name: Option<String>,
        email: Option<EmailAddress>,
        subject: &str,
        message: &str,
    ) -> Result<Self, MessageError> {
        if subject.is_empty() {
            return Err(MessageError::EmptySubject);
        }

        if subject.chars().count() > MAX_SUBJECT_LENGTH {
            return Err(MessageError::TooLongSubject);
        }

        if message.is_empty() {
            return Err(MessageError::EmptyMessage);
        }

        Ok(Message {
            id,
            user_id,
            name,
            email,
            subject: subject.to_owned(),
            message: message.to_owned(),
            created_at: Timestamp::now(),
        })
    }
}

#[cfg(test)]
mod test {
    use argentum_standard_business::data_type::EmailAddress;
    use argentum_standard_business::data_type::id::IdFactory;
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;

    use crate::entity::message::MessageError;

    use super::Message;

    #[test]
    fn creates_guest_request() {
        let id = IdFactoryMock::new().create();
        let email = EmailAddress::try_new("demo@test.com").expect("Email should be valid");

        let request = Message::new_guest(id, "John Cooper".to_string(), email, "Hi", "Text")
            .expect("Request should be valid");

        assert_eq!(request.id, id);
        assert!(request.user_id.is_none());
        assert_eq!(request.name.as_deref(), Some("John Cooper"));
        assert!(request.email.is_some());
        assert_eq!(request.subject, "Hi");
        assert_eq!(request.message, "Text");
    }

    #[test]
    fn creates_member_request() {
        let id_factory = IdFactoryMock::new();
        let id = id_factory.create();
        let user_id = id_factory.create();

        let request =
            Message::new_member(id, user_id, "Hi", "Text").expect("Request should be valid");

        assert_eq!(request.user_id, Some(user_id));
        assert!(request.name.is_none());
        assert!(request.email.is_none());
    }

    #[test]
    fn rejects_empty_subject() {
        let id = IdFactoryMock::new().create();
        let email = EmailAddress::try_new("demo@test.com").expect("Email should be valid");

        let result = Message::new_guest(id, "John".to_string(), email, "", "Text");

        assert!(matches!(result, Err(MessageError::EmptySubject)));
    }

    #[test]
    fn rejects_too_long_subject() {
        let id = IdFactoryMock::new().create();
        let email = EmailAddress::try_new("demo@test.com").expect("Email should be valid");
        let subject = "a".repeat(256);

        let result = Message::new_guest(id, "John".to_string(), email, &subject, "Text");

        assert!(matches!(result, Err(MessageError::TooLongSubject)));
    }

    #[test]
    fn rejects_empty_message() {
        let id = IdFactoryMock::new().create();
        let email = EmailAddress::try_new("demo@test.com").expect("Email should be valid");

        let result = Message::new_guest(id, "John".to_string(), email, "Hi", "");

        assert!(matches!(result, Err(MessageError::EmptyMessage)));
    }
}
