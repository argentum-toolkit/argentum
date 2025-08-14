use crate::data_type::NamePart;

#[derive(Clone, Debug)]
pub struct Name {
    pub first: NamePart,
    pub last: Option<NamePart>,
    pub additional: Option<NamePart>,
}

impl Name {
    pub fn new(first: NamePart, last: Option<NamePart>, additional: Option<NamePart>) -> Self {
        Self {
            first,
            last,
            additional,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::{Name, NamePart};

    #[test]
    fn test_new_full() {
        let first = NamePart::try_new("Lucian".into()).expect("Should be valid");
        let last = NamePart::try_new("Fisher".into()).expect("Should be valid");
        let additional = NamePart::try_new("Bushra".into()).expect("Should be valid");

        let name = Name::new(first, Some(last), Some(additional));

        assert_eq!(name.first.to_string(), "Lucian".to_string());
        assert_eq!(name.last.map(|n| n.to_string()), Some("Fisher".to_string()));
        assert_eq!(
            name.additional.map(|n| n.to_string()),
            Some("Bushra".to_string())
        );
    }

    #[test]
    fn test_new_minimal() {
        let first = NamePart::try_new("Lucian".into()).expect("Should be valid");

        let name = Name::new(first, None, None);

        assert_eq!(name.first.to_string(), "Lucian".to_string());
        assert!(name.last.is_none());
        assert!(name.additional.is_none());
    }
}
