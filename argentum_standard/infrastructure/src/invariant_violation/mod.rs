use argentum_standard_business::invariant_violation::{ViolationItem, Violations};
use serde::{Serialize, Serializer};
use std::collections::HashMap;

pub type ViolationObjectDto = HashMap<String, ViolationsDto>;
pub type ViolationArrayDto = Vec<ViolationsDto>;

#[derive(Debug, Clone)]
pub enum ViolationItemDto {
    Object(ViolationObjectDto),
    Array(ViolationArrayDto),
}

impl ViolationItemDto {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Object(o) => o.is_empty(),
            Self::Array(a) => a.is_empty(),
        }
    }
}

impl From<&ViolationItem> for ViolationItemDto {
    fn from(item: &ViolationItem) -> Self {
        match item {
            ViolationItem::Object(o) => {
                let map = o
                    .into_iter()
                    .map(|(k, v)| (k.clone(), ViolationsDto::from(v)))
                    .collect();
                ViolationItemDto::Object(map)
            }
            ViolationItem::Array(a) => {
                let arr = a.iter().map(ViolationsDto::from).collect();

                ViolationItemDto::Array(arr)
            }
        }
    }
}

impl From<&Violations> for ViolationsDto {
    fn from(violations: &Violations) -> Self {
        let items = match &violations.items {
            None => None,
            Some(i) => Some(ViolationItemDto::from(i)),
        };
        ViolationsDto::new(violations.errors.clone(), items)
    }
}

impl Serialize for ViolationItemDto {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Array(a) => serde::Serialize::serialize(a, serializer),
            Self::Object(o) => serde::Serialize::serialize(o, serializer),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ViolationsDto {
    pub errors: Vec<String>,

    pub items: Option<ViolationItemDto>,
}

impl ViolationsDto {
    pub fn new(errors: Vec<String>, items: Option<ViolationItemDto>) -> Self {
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

impl Into<ViolationsDto> for &str {
    fn into(self) -> ViolationsDto {
        ViolationsDto::new(vec![self.to_string()], None)
    }
}

#[cfg(test)]
mod tests {
    use crate::invariant_violation::{ViolationItemDto, ViolationsDto};
    use std::collections::HashMap;

    const TEST_ERROR: &str = "Test error";
    const ANOTHER_ERROR: &str = "Another error";

    #[test]
    fn test_violations_are_empty() {
        {
            let v = ViolationsDto::new(vec![], None);
            assert!(v.is_empty());
        }

        {
            let v = ViolationsDto::new(vec![], Some(ViolationItemDto::Array(vec![])));
            assert!(v.is_empty());
        }
    }

    #[test]
    fn test_violations_are_not_empty() {
        {
            let v = ViolationsDto::new(vec![TEST_ERROR.to_string()], None);
            assert!(!v.is_empty());
        }

        {
            let v = ViolationsDto::new(
                vec![TEST_ERROR.to_string()],
                Some(ViolationItemDto::Array(vec![ViolationsDto::new(
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
            let b = ViolationItemDto::Array(vec![]);
            assert!(b.is_empty())
        }
        {
            let b = ViolationItemDto::Object(HashMap::from([]));
            assert!(b.is_empty())
        }
    }

    #[test]
    fn test_violation_item_is_not_empty() {
        {
            let b = ViolationItemDto::Array(vec![ViolationsDto::new(vec![], None)]);

            assert!(!b.is_empty())
        }

        {
            let b = ViolationItemDto::Object(HashMap::from([(
                "some-field-name".to_string(),
                ViolationsDto::new(vec![], None),
            )]));
            assert!(!b.is_empty())
        }
    }
}
