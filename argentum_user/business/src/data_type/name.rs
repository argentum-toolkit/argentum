use crate::data_type::NamePart;

#[derive(Clone, Debug)]
pub struct Name {
    pub first: NamePart,
    pub last: Option<NamePart>,
    pub patronymic: Option<NamePart>,
}

impl Name {
    pub fn new(first: NamePart, last: Option<NamePart>, patronymic: Option<NamePart>) -> Self {
        Self {
            first,
            last,
            patronymic,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::{Name, NamePart};

    #[test]
    fn test_new_full() {
        let first = NamePart::try_new("Lucian".into()).unwrap();
        let last = NamePart::try_new("Fisher".into()).unwrap();
        let patronymic = NamePart::try_new("Bushra".into()).unwrap();

        let name = Name::new(first, Some(last), Some(patronymic));

        assert_eq!(name.first.to_string(), "Lucian".to_string());
        assert_eq!(name.last.unwrap().to_string(), "Fisher".to_string());
        assert_eq!(name.patronymic.unwrap().to_string(), "Bushra".to_string());
    }

    #[test]
    fn test_new_minimal() {
        let first = NamePart::try_new("Lucian".into()).unwrap();

        let name = Name::new(first, None, None);

        assert_eq!(name.first.to_string(), "Lucian".to_string());
        assert!(name.last.is_none());
        assert!(name.patronymic.is_none());
    }
}
