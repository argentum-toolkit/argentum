#![allow(unused_qualifications)]

#[cfg(any(feature = "client", feature = "server"))]
use crate::header;
use crate::models;

//argentum-customization
use argentum_rest_infrastructure::data_type::SerializableBody;
//argentum-customization end

//argentum-customization next line
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AnonymousRegistrationResult {
    #[serde(rename = "aonymous_id")]
    pub aonymous_id: uuid::Uuid,

    #[serde(rename = "token")]
    pub token: String,
}

//argentum-customization next line
impl SerializableBody for AnonymousRegistrationResult {}

impl AnonymousRegistrationResult {
    #[allow(clippy::new_without_default)]
    pub fn new(aonymous_id: uuid::Uuid, token: String) -> AnonymousRegistrationResult {
        AnonymousRegistrationResult { aonymous_id, token }
    }
}

/// Converts the AnonymousRegistrationResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AnonymousRegistrationResult {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping aonymous_id in query parameter serialization
            Some("token".to_string()),
            Some(self.token.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AnonymousRegistrationResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AnonymousRegistrationResult {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub aonymous_id: Vec<uuid::Uuid>,
            pub token: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing AnonymousRegistrationResult".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "aonymous_id" => intermediate_rep.aonymous_id.push(
                        <uuid::Uuid as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "token" => intermediate_rep.token.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing AnonymousRegistrationResult".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AnonymousRegistrationResult {
            aonymous_id: intermediate_rep
                .aonymous_id
                .into_iter()
                .next()
                .ok_or_else(|| "aonymous_id missing in AnonymousRegistrationResult".to_string())?,
            token: intermediate_rep
                .token
                .into_iter()
                .next()
                .ok_or_else(|| "token missing in AnonymousRegistrationResult".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AnonymousRegistrationResult> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AnonymousRegistrationResult>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<AnonymousRegistrationResult>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for AnonymousRegistrationResult - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<AnonymousRegistrationResult>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <AnonymousRegistrationResult as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into AnonymousRegistrationResult - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

//argentum-customization next line
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ChangePasswordSchema {
    #[serde(rename = "token")]
    pub token: String,

    #[serde(rename = "password")]
    pub password: String,
}

//argentum-customization next line
impl SerializableBody for ChangePasswordSchema {}

impl ChangePasswordSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(token: String, password: String) -> ChangePasswordSchema {
        ChangePasswordSchema { token, password }
    }
}

/// Converts the ChangePasswordSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ChangePasswordSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("token".to_string()),
            Some(self.token.to_string()),
            Some("password".to_string()),
            Some(self.password.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ChangePasswordSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ChangePasswordSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub token: Vec<String>,
            pub password: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ChangePasswordSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "token" => intermediate_rep.token.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "password" => intermediate_rep.password.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ChangePasswordSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ChangePasswordSchema {
            token: intermediate_rep
                .token
                .into_iter()
                .next()
                .ok_or_else(|| "token missing in ChangePasswordSchema".to_string())?,
            password: intermediate_rep
                .password
                .into_iter()
                .next()
                .ok_or_else(|| "password missing in ChangePasswordSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ChangePasswordSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ChangePasswordSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ChangePasswordSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for ChangePasswordSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<ChangePasswordSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ChangePasswordSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into ChangePasswordSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

//argentum-customization next line
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LoginResult {
    #[serde(rename = "user_id")]
    pub user_id: uuid::Uuid,

    #[serde(rename = "token")]
    pub token: String,
}

//argentum-customization next line
impl SerializableBody for LoginResult {}

impl LoginResult {
    #[allow(clippy::new_without_default)]
    pub fn new(user_id: uuid::Uuid, token: String) -> LoginResult {
        LoginResult { user_id, token }
    }
}

/// Converts the LoginResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LoginResult {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping user_id in query parameter serialization
            Some("token".to_string()),
            Some(self.token.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LoginResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LoginResult {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub user_id: Vec<uuid::Uuid>,
            pub token: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing LoginResult".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "user_id" => intermediate_rep.user_id.push(
                        <uuid::Uuid as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "token" => intermediate_rep.token.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing LoginResult".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LoginResult {
            user_id: intermediate_rep
                .user_id
                .into_iter()
                .next()
                .ok_or_else(|| "user_id missing in LoginResult".to_string())?,
            token: intermediate_rep
                .token
                .into_iter()
                .next()
                .ok_or_else(|| "token missing in LoginResult".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LoginResult> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LoginResult>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<LoginResult>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for LoginResult - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LoginResult> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <LoginResult as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into LoginResult - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

//argentum-customization next line
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LoginWithPasswordSchema {
    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "password")]
    pub password: String,
}

//argentum-customization next line
impl SerializableBody for LoginWithPasswordSchema {}

impl LoginWithPasswordSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(email: String, password: String) -> LoginWithPasswordSchema {
        LoginWithPasswordSchema { email, password }
    }
}

/// Converts the LoginWithPasswordSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LoginWithPasswordSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("email".to_string()),
            Some(self.email.to_string()),
            Some("password".to_string()),
            Some(self.password.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LoginWithPasswordSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LoginWithPasswordSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub email: Vec<String>,
            pub password: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing LoginWithPasswordSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "email" => intermediate_rep.email.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "password" => intermediate_rep.password.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing LoginWithPasswordSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LoginWithPasswordSchema {
            email: intermediate_rep
                .email
                .into_iter()
                .next()
                .ok_or_else(|| "email missing in LoginWithPasswordSchema".to_string())?,
            password: intermediate_rep
                .password
                .into_iter()
                .next()
                .ok_or_else(|| "password missing in LoginWithPasswordSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LoginWithPasswordSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LoginWithPasswordSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<LoginWithPasswordSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for LoginWithPasswordSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<LoginWithPasswordSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <LoginWithPasswordSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into LoginWithPasswordSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// RFC 7807 Problem Details for HTTP APIs
//argentum-customization next line
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ProblemDetail {
    #[serde(rename = "code")]
    pub code: u32,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

//argentum-customization next line
impl SerializableBody for ProblemDetail {}

impl ProblemDetail {
    #[allow(clippy::new_without_default)]
    pub fn new(code: u32) -> ProblemDetail {
        ProblemDetail {
            code,
            message: None,
        }
    }
}

/// Converts the ProblemDetail value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ProblemDetail {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("code".to_string()),
            Some(self.code.to_string()),
            self.message
                .as_ref()
                .map(|message| vec!["message".to_string(), message.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ProblemDetail value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ProblemDetail {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<u32>,
            pub message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ProblemDetail".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ProblemDetail".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ProblemDetail {
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in ProblemDetail".to_string())?,
            message: intermediate_rep.message.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ProblemDetail> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ProblemDetail>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ProblemDetail>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for ProblemDetail - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ProblemDetail> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ProblemDetail as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into ProblemDetail - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

//argentum-customization next line
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RegistrationWithPasswordResult {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
}

//argentum-customization next line
impl SerializableBody for RegistrationWithPasswordResult {}

impl RegistrationWithPasswordResult {
    #[allow(clippy::new_without_default)]
    pub fn new(id: uuid::Uuid) -> RegistrationWithPasswordResult {
        RegistrationWithPasswordResult { id }
    }
}

/// Converts the RegistrationWithPasswordResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RegistrationWithPasswordResult {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping id in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RegistrationWithPasswordResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RegistrationWithPasswordResult {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<uuid::Uuid>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing RegistrationWithPasswordResult".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(
                        <uuid::Uuid as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing RegistrationWithPasswordResult"
                                .to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RegistrationWithPasswordResult {
            id: intermediate_rep
                .id
                .into_iter()
                .next()
                .ok_or_else(|| "id missing in RegistrationWithPasswordResult".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RegistrationWithPasswordResult> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RegistrationWithPasswordResult>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<RegistrationWithPasswordResult>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for RegistrationWithPasswordResult - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<RegistrationWithPasswordResult>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RegistrationWithPasswordResult as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RegistrationWithPasswordResult - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

//argentum-customization next line
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RegistrationWithPasswordSchema {
    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "name")]
    pub name: models::UserName,

    #[serde(rename = "password")]
    pub password: String,
}

//argentum-customization next line
impl SerializableBody for RegistrationWithPasswordSchema {}

impl RegistrationWithPasswordSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        email: String,
        name: models::UserName,
        password: String,
    ) -> RegistrationWithPasswordSchema {
        RegistrationWithPasswordSchema {
            email,
            name,
            password,
        }
    }
}

/// Converts the RegistrationWithPasswordSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RegistrationWithPasswordSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("email".to_string()),
            Some(self.email.to_string()),
            // Skipping name in query parameter serialization
            Some("password".to_string()),
            Some(self.password.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RegistrationWithPasswordSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RegistrationWithPasswordSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub email: Vec<String>,
            pub name: Vec<models::UserName>,
            pub password: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing RegistrationWithPasswordSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "email" => intermediate_rep.email.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <models::UserName as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "password" => intermediate_rep.password.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing RegistrationWithPasswordSchema"
                                .to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RegistrationWithPasswordSchema {
            email: intermediate_rep
                .email
                .into_iter()
                .next()
                .ok_or_else(|| "email missing in RegistrationWithPasswordSchema".to_string())?,
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in RegistrationWithPasswordSchema".to_string())?,
            password: intermediate_rep
                .password
                .into_iter()
                .next()
                .ok_or_else(|| "password missing in RegistrationWithPasswordSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RegistrationWithPasswordSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RegistrationWithPasswordSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<RegistrationWithPasswordSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for RegistrationWithPasswordSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<RegistrationWithPasswordSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RegistrationWithPasswordSchema as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RegistrationWithPasswordSchema - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

//argentum-customization next line
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RequestRestoreTokenSchema {
    #[serde(rename = "email")]
    pub email: String,
}

//argentum-customization next line
impl SerializableBody for RequestRestoreTokenSchema {}

impl RequestRestoreTokenSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(email: String) -> RequestRestoreTokenSchema {
        RequestRestoreTokenSchema { email }
    }
}

/// Converts the RequestRestoreTokenSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RequestRestoreTokenSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> =
            vec![Some("email".to_string()), Some(self.email.to_string())];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RequestRestoreTokenSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RequestRestoreTokenSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub email: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing RequestRestoreTokenSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "email" => intermediate_rep.email.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing RequestRestoreTokenSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RequestRestoreTokenSchema {
            email: intermediate_rep
                .email
                .into_iter()
                .next()
                .ok_or_else(|| "email missing in RequestRestoreTokenSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RequestRestoreTokenSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RequestRestoreTokenSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<RequestRestoreTokenSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for RequestRestoreTokenSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<RequestRestoreTokenSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <RequestRestoreTokenSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into RequestRestoreTokenSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

//argentum-customization next line
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UserName {
    #[serde(rename = "first")]
    pub first: String,

    #[serde(rename = "last")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,

    #[serde(rename = "patronymic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patronymic: Option<String>,
}

//argentum-customization next line
impl SerializableBody for UserName {}

impl UserName {
    #[allow(clippy::new_without_default)]
    pub fn new(first: String) -> UserName {
        UserName {
            first,
            last: None,
            patronymic: None,
        }
    }
}

/// Converts the UserName value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UserName {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("first".to_string()),
            Some(self.first.to_string()),
            self.last
                .as_ref()
                .map(|last| vec!["last".to_string(), last.to_string()].join(",")),
            self.patronymic
                .as_ref()
                .map(|patronymic| vec!["patronymic".to_string(), patronymic.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UserName value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UserName {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub first: Vec<String>,
            pub last: Vec<String>,
            pub patronymic: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing UserName".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "first" => intermediate_rep.first.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "last" => intermediate_rep.last.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "patronymic" => intermediate_rep.patronymic.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing UserName".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UserName {
            first: intermediate_rep
                .first
                .into_iter()
                .next()
                .ok_or_else(|| "first missing in UserName".to_string())?,
            last: intermediate_rep.last.into_iter().next(),
            patronymic: intermediate_rep.patronymic.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UserName> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UserName>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<UserName>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for UserName - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UserName> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <UserName as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into UserName - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}
