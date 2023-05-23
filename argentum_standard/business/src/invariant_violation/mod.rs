use std::collections::HashMap;

pub type ViolationObject = HashMap<String, Violations>;
pub type ViolationArray = Vec<Violations>;

#[derive(Debug, Clone)]
pub enum ViolationItems {
    Object(ViolationObject),
    Array(ViolationArray),
}

impl ViolationItems {
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

    pub items: Option<ViolationItems>,
}

impl Violations {
    pub fn new(errors: Vec<String>, items: Option<ViolationItems>) -> Self {
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

impl Into<Violations> for &str {
    fn into(self) -> Violations {
        Violations::new(vec![self.to_string()], None)
    }
}

#[cfg(test)]
mod tests {
    use crate::invariant_violation::{ViolationItems, Violations};
    use std::collections::HashMap;

    const TEST_ERROR: &str = "Test error";
    const ANOTHER_ERROR: &str = "Another error";

    #[test]
    fn test_violations_are_empty() {
        {
            let v = Violations::new(vec![], None);
            assert_eq!(true, v.is_empty());
        }

        {
            let v = Violations::new(vec![], Some(ViolationItems::Array(vec![])));
            assert_eq!(true, v.is_empty());
        }
    }

    #[test]
    fn test_violations_are_not_empty() {
        {
            let v = Violations::new(vec![TEST_ERROR.to_string()], None);
            assert_eq!(false, v.is_empty());
        }

        {
            let v = Violations::new(
                vec![TEST_ERROR.to_string()],
                Some(ViolationItems::Array(vec![Violations::new(
                    vec![TEST_ERROR.to_string(), ANOTHER_ERROR.to_string()],
                    None,
                )])),
            );
            assert_eq!(false, v.is_empty());
        }
    }

    #[test]
    fn test_violation_item_is_empty() {
        {
            let b = ViolationItems::Array(vec![]);
            assert_eq!(true, b.is_empty())
        }
        {
            let b = ViolationItems::Object(HashMap::from([]));
            assert_eq!(true, b.is_empty())
        }
    }

    #[test]
    fn test_violation_item_is_not_empty() {
        {
            let b = ViolationItems::Array(vec![Violations::new(vec![], None)]);

            assert_eq!(false, b.is_empty())
        }

        {
            let b = ViolationItems::Object(HashMap::from([(
                "some-field-name".to_string(),
                Violations::new(vec![], None),
            )]));
            assert_eq!(false, b.is_empty())
        }
    }
}
