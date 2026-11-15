use crate::data_type::Name;
use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_business::data_type::id::Id;
use chrono::{DateTime, Utc};

pub trait UserTrait {
    type Id: Id;

    fn id(&self) -> Self::Id;
}

//============== MEMBER USER
#[derive(Clone)]
pub struct Member<I>
where
    I: Id,
{
    pub id: I,
    pub created_at: DateTime<Utc>,
    pub name: Name,
    pub email: EmailAddress,
    pub email_confirmed: bool,
    pub slug: String,
}

impl<I> Member<I>
where
    I: Id,
{
    pub fn new(id: I, name: Name, email: EmailAddress, email_confirmed: bool) -> Self {
        Member {
            slug: Self::build_slug(id, &name),
            id,
            created_at: Utc::now(),
            name,
            email,
            email_confirmed,
        }
    }

    pub fn confirm_email(&mut self) {
        self.email_confirmed = true;
    }

    fn build_slug(id: I, name: &Name) -> String {
        let mut parts = vec![name.first.to_string()];
        if let Some(last) = &name.last {
            parts.push(last.to_string());
        }
        parts.push(id.to_string());
        parts.join("-")
    }
}

impl<I> UserTrait for Member<I>
where
    I: Id,
{
    type Id = I;
    fn id(&self) -> Self::Id {
        self.id
    }
}

//============== GUEST USER

#[derive(Clone)]
pub struct Guest<I>
where
    I: Id,
{
    pub id: I,
    pub created_at: DateTime<Utc>,
}

impl<I> Guest<I>
where
    I: Id,
{
    pub fn new(id: I) -> Self {
        Guest {
            id,
            created_at: Utc::now(),
        }
    }
}

impl<I> UserTrait for Guest<I>
where
    I: Id,
{
    type Id = I;
    fn id(&self) -> Self::Id {
        self.id
    }
}

//============== ENUM USER allows operating with guest as with mem
#[derive(Clone)]
pub enum User<I>
where
    I: Id,
{
    Guest(Guest<I>),
    Member(Member<I>),
}

#[cfg(test)]
mod tests {
    use super::Member;
    use crate::data_type::{Name, NamePart};
    use argentum_standard_business::data_type::EmailAddress;

    fn part(value: &str) -> NamePart {
        NamePart::try_new(value).expect("Name part should be valid")
    }

    fn email() -> EmailAddress {
        EmailAddress::try_new("lucian@example.com").expect("Email should be valid")
    }

    #[test]
    fn test_new_slug_full_name() {
        let name = Name::new(part("Lucian"), Some(part("Fisher")), Some(part("Bushra")));

        let member = Member::new(7u64, name, email(), true);

        assert_eq!(member.slug, "Lucian-Fisher-7");
    }

    #[test]
    fn test_new_slug_without_last_name() {
        let name = Name::new(part("Lucian"), None, None);

        let member = Member::new(7u64, name, email(), true);

        assert_eq!(member.slug, "Lucian-7");
    }

    #[test]
    fn test_new_slug_differs_for_identical_names_with_different_ids() {
        let first = Member::new(
            1u64,
            Name::new(part("Lucian"), Some(part("Fisher")), None),
            email(),
            true,
        );
        let second = Member::new(
            2u64,
            Name::new(part("Lucian"), Some(part("Fisher")), None),
            email(),
            true,
        );

        assert_ne!(first.slug, second.slug);
    }

    #[test]
    fn test_slug_unchanged_when_name_changes() {
        let mut member = Member::new(
            7u64,
            Name::new(part("Lucian"), Some(part("Fisher")), None),
            email(),
            true,
        );

        member.name = Name::new(part("Bushra"), None, None);

        assert_eq!(member.slug, "Lucian-Fisher-7");
    }
}
