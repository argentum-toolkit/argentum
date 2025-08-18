use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TestViolationItemsTest {
    pub tst_1: Option<String>,

}

impl TestViolationItemsTest {
    pub fn new(
        tst_1: Option<String>,
    ) -> Self {
        Self {
            tst_1,
        }
    }
}

impl SerializableBody for TestViolationItemsTest {}

impl DeserializableSchemaRaw<'_> for TestViolationItemsTest {
    type Raw = TestViolationItemsTestRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

                let tst_1 = raw.tst_1;

        if argentum_violations.is_empty() {
            Ok(Self::new(
                tst_1,
            ))
        } else {
            Err(Violations::new(
                vec!["wrong data for TestViolationItemsTest".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }

}

#[derive(serde::Deserialize)]
pub struct TestViolationItemsTestRaw {
    #[serde(rename = "tst1")]
    pub tst_1: Option<String>,
}
