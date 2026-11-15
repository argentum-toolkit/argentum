use crate::entity::user::Member;
use crate::repository::user_repository::{ExternalUserError, MemberRepositoryTrait};
use argentum_standard_business::data_type::id::Id;
use std::sync::Arc;

pub struct GetMemberBySlugUc<I>
where
    I: Id,
{
    member_repository: Arc<dyn MemberRepositoryTrait<Id = I>>,
}

impl<I> GetMemberBySlugUc<I>
where
    I: Id,
{
    pub fn new(member_repository: Arc<dyn MemberRepositoryTrait<Id = I>>) -> Self {
        Self { member_repository }
    }

    pub async fn execute(&self, slug: &str) -> Result<Member<I>, GetMemberBySlugError> {
        self.member_repository
            .find_by_slug(slug)
            .await?
            .ok_or(GetMemberBySlugError::MemberNotFound)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetMemberBySlugError {
    #[error("Member not found")]
    MemberNotFound,

    #[error("Member repository error")]
    MemberRepositoryError(
        #[source]
        #[from]
        ExternalUserError,
    ),
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use argentum_standard_business::data_type::EmailAddress;

    use crate::{
        data_type::{Name, NamePart},
        entity::user::Member,
        mock::repository::member_repository_mock::MemberRepositoryMock,
        repository::user_repository::MemberRepositoryTrait,
    };

    use super::{GetMemberBySlugError, GetMemberBySlugUc};

    fn member_name() -> Result<Name, String> {
        let first = NamePart::try_new("Avery").map_err(|error| format!("{error:?}"))?;
        let last = NamePart::try_new("Example").map_err(|error| format!("{error:?}"))?;

        Ok(Name::new(first, Some(last), None))
    }

    fn member_email() -> Result<EmailAddress, String> {
        EmailAddress::try_new("avery@example.com").map_err(|error| format!("{error:?}"))
    }

    #[tokio::test]
    async fn finds_member_by_exact_slug() -> Result<(), String> {
        let repository = Arc::new(MemberRepositoryMock::<u64>::new());
        let member = Member::new(17, member_name()?, member_email()?, true);
        let slug = member.slug.clone();
        repository
            .create(&member)
            .await
            .map_err(|error| format!("{error:?}"))?;

        let found = GetMemberBySlugUc::new(repository)
            .execute(&slug)
            .await
            .map_err(|error| format!("{error:?}"))?;

        assert_eq!(found.id, member.id);
        assert_eq!(found.slug, slug);
        Ok(())
    }

    #[tokio::test]
    async fn returns_not_found_for_an_unknown_slug() {
        let repository = Arc::new(MemberRepositoryMock::<u64>::new());

        let result = GetMemberBySlugUc::new(repository)
            .execute("unknown-member-slug")
            .await;

        assert!(matches!(result, Err(GetMemberBySlugError::MemberNotFound)));
    }
}
