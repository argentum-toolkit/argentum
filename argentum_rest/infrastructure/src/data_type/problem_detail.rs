use crate::data_type::SerializableBody;
use hyper::StatusCode;
use serde::Serialize;

pub trait ProblemDetailExtension: erased_serde::Serialize {}
erased_serde::serialize_trait_object!(ProblemDetailExtension);

const PROBLEM_TYPE_BLANK: &str = "about:blank";

#[derive(Serialize)]
pub struct ProblemDetail {
    #[serde(rename = "type")]
    pub problem_type: String,

    pub title: String,

    pub status: u16,

    pub detail: Option<String>,

    //TODO: add trace_id
    // pub trace_id: String
    #[serde(flatten)]
    pub extension: Option<Box<dyn ProblemDetailExtension>>,
}

impl ProblemDetail {
    pub fn new(
        problem_type: Option<String>,
        title: String,
        status: StatusCode,
        detail: Option<String>,
        extension: Option<Box<dyn ProblemDetailExtension>>,
    ) -> Self {
        let problem_type = match problem_type {
            None => PROBLEM_TYPE_BLANK.to_string(),
            Some(s) => s,
        };

        Self {
            problem_type,
            title,
            status: status.as_u16(),
            detail,
            extension,
        }
    }
}

impl SerializableBody for ProblemDetail {}

#[cfg(test)]
mod tests {
    use crate::data_type::error::BadRequestError;
    use crate::data_type::problem_detail::PROBLEM_TYPE_BLANK;
    use crate::data_type::ProblemDetail;
    use argentum_standard_business::invariant_violation::Violations;
    use hyper::StatusCode;

    #[test]
    fn test_constructor() {
        {
            let title = "undefined error".to_string();
            let problem =
                ProblemDetail::new(None, title.clone(), StatusCode::IM_A_TEAPOT, None, None);

            assert_eq!(PROBLEM_TYPE_BLANK.to_string(), problem.problem_type);
            assert_eq!(title, problem.title);
            assert_eq!(StatusCode::IM_A_TEAPOT, problem.status);
            assert_eq!(None, problem.detail);
            assert_eq!(true, problem.extension.is_none());
        }

        {
            let title = "some error".to_string();
            let problem_type = "https://example.com/test".to_string();
            let detail = "Some description".to_string();
            let ext = Box::new(BadRequestError::new(
                Violations::new(vec![], None),
                Violations::new(vec![], None),
                Violations::new(vec![], None),
                Violations::new(vec![], None),
            ));

            let problem = ProblemDetail::new(
                Some(problem_type.clone()),
                title.clone(),
                StatusCode::OK,
                Some(detail.clone()),
                Some(ext),
            );

            assert_eq!(problem_type, problem.problem_type);
            assert_eq!(title, problem.title);
            assert_eq!(StatusCode::OK, problem.status);
            assert_eq!(Some(detail), problem.detail);
            assert_eq!(true, problem.extension.is_some());
        }
    }
}
