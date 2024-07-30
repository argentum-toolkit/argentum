use std::collections::BTreeMap;

pub type ViolationObject = BTreeMap<String, Violations>;
pub type ViolationArray = Vec<Violations>;
pub type InvariantResult<T> = Result<T, Violations>;

#[derive(Debug, Clone)]
pub enum ViolationItem {
    Object(ViolationObject),
    Array(ViolationArray),
}

impl ViolationItem {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Object(o) => o.is_empty(),
            Self::Array(a) => a.is_empty(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Violations {
    pub errors: Vec<String>,

    pub items: Option<ViolationItem>,
}

impl Violations {
    pub fn new(errors: Vec<String>, items: Option<ViolationItem>) -> Self {
        Self { errors, items }
    }

    pub fn is_empty(&self) -> bool {
        let items_empty = match &self.items {
            None => true,
            Some(e) => e.is_empty(),
        };

        self.errors.is_empty() && items_empty
    }
}

impl From<&str> for Violations {
    fn from(val: &str) -> Self {
        Violations::new(vec![val.to_string()], None)
    }
}

#[cfg(test)]
mod tests {
    use crate::invariant_violation::{ViolationItem, Violations};
    use std::collections::BTreeMap;

    const TEST_ERROR: &str = "Test error";
    const ANOTHER_ERROR: &str = "Another error";

    #[test]
    fn test_violations_are_empty() {
        {
            let v = Violations::new(vec![], None);
            assert!(v.is_empty());
        }

        {
            let v = Violations::new(vec![], Some(ViolationItem::Array(vec![])));
            assert!(v.is_empty());
        }
    }

    #[test]
    fn test_violations_are_not_empty() {
        {
            let v = Violations::new(vec![TEST_ERROR.to_string()], None);
            assert!(!v.is_empty());
        }

        {
            let v = Violations::new(
                vec![TEST_ERROR.to_string()],
                Some(ViolationItem::Array(vec![Violations::new(
                    vec![TEST_ERROR.to_string(), ANOTHER_ERROR.to_string()],
                    None,
                )])),
            );
            assert!(!v.is_empty());
        }
    }

    #[test]
    fn test_violation_item_is_empty() {
        {
            let b = ViolationItem::Array(vec![]);
            assert!(b.is_empty())
        }
        {
            let b = ViolationItem::Object(BTreeMap::from([]));
            assert!(b.is_empty())
        }
    }

    #[test]
    fn test_violation_item_is_not_empty() {
        {
            let b = ViolationItem::Array(vec![Violations::new(vec![], None)]);

            assert!(!b.is_empty())
        }

        {
            let b = ViolationItem::Object(BTreeMap::from([(
                "some-field-name".to_string(),
                Violations::new(vec![], None),
            )]));
            assert!(!b.is_empty())
        }
    }
}
