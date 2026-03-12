use std::{
    error::Error,
    fmt::{self, Display},
    sync::Arc,
};

use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::{
    data_type::builder::NameBuilder,
    entity::user::Member,
    repository::user_repository::{ExternalUserError, MemberRepositoryTrait},
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::keycloak::service::AdminApi;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct KeycloakUser {
    id: Uuid,
    email: String,
    first_name: String,
    last_name: Option<String>,
}

#[derive(Debug)]
struct KeycloakUserError(String);
impl Error for KeycloakUserError {}

impl Display for KeycloakUserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NewUser {
    id: String,
    email: String,
    username: String,
    first_name: String,
    last_name: Option<String>,
    enabled: bool,
}

#[derive(Serialize, Debug)]
struct PartialImportRepresentation {
    users: Vec<NewUser>,
    #[serde(rename = "ifResourceExists")]
    if_resource_exists: String, // "FAIL", "SKIP", "OVERWRITE"
}
pub struct MemberRepository<L>
where
    L: LoggerTrait,
{
    admin_api: Arc<AdminApi<L>>,
    logger: Arc<L>,
}

impl<L> MemberRepository<L>
where
    L: LoggerTrait,
{
    pub fn new(admin_api: Arc<AdminApi<L>>, logger: Arc<L>) -> Self {
        Self { admin_api, logger }
    }
}

#[async_trait]
impl<L> MemberRepositoryTrait for MemberRepository<L>
where
    L: LoggerTrait,
{
    type Id = UniqueId;

    async fn find(&self, id: Self::Id) -> Result<Option<Member<Self::Id>>, ExternalUserError> {
        let path = format!("/users/{}", id);

        let u: KeycloakUser = self
            .admin_api
            .get(&path, None::<&()>)
            .await
            .map_err(|e| ExternalUserError::Member(Some(Arc::new(e))))?
            .error_for_status()
            .map_err(|e| ExternalUserError::Member(Some(Arc::new(e))))?
            .json()
            .await
            .map_err(|e| ExternalUserError::Member(Some(Arc::new(e))))?;

        let email = EmailAddress::try_new(&u.email)
            .map_err(|e| ExternalUserError::Member(Some(Arc::new(e))))?;
        let name = NameBuilder::new(&u.first_name)
            .last(u.last_name.as_deref())
            .try_build()
            .map_err(|e| ExternalUserError::Member(Some(Arc::new(e))))?;

        Ok(Some(Member::new(u.id.into(), name, email, true)))
    }

    async fn find_by_email(
        &self,
        email: &EmailAddress,
    ) -> Result<Option<Member<Self::Id>>, ExternalUserError> {
        let resp = self
            .admin_api
            .get("/users", Some(&[("email", email.as_string())]))
            .await
            .map_err(|e| ExternalUserError::Member(Some(Arc::new(e))))?
            .error_for_status()
            .map_err(|e| ExternalUserError::Member(Some(Arc::new(e))))?;

        let users: Vec<KeycloakUser> = resp
            .json()
            .await
            .map_err(|e| ExternalUserError::Member(Some(Arc::new(e))))?;

        match users.into_iter().next() {
            Some(u) => {
                let email = EmailAddress::try_new(&u.email)
                    .map_err(|e| ExternalUserError::Member(Some(Arc::new(e))))?;
                let name = NameBuilder::new(&u.first_name)
                    .last(u.last_name.as_deref())
                    .try_build()
                    .map_err(|e| ExternalUserError::Member(Some(Arc::new(e))))?;

                Ok(Some(Member::new(u.id.into(), name, email, true)))
            }
            None => Ok(None),
        }
    }

    async fn create(&self, user: &Member<Self::Id>) -> Result<(), ExternalUserError> {
        let last = user.name.last.clone().map(|l| l.to_string());
        let new_user = NewUser {
            id: user.id.to_string(),
            first_name: user.name.first.to_string(),
            last_name: last,
            email: user.email.as_string(),
            username: user.email.as_string(),
            enabled: true,
        };

        let partial_import = PartialImportRepresentation {
            users: vec![new_user],
            if_resource_exists: "FAIL".to_string(),
        };

        let resp = self
            .admin_api
            .post("/partialImport", Some(&partial_import))
            .await
            .map_err(|e| ExternalUserError::Member(Some(Arc::new(e))))?;

        if resp.status().is_success() || resp.status() == reqwest::StatusCode::CREATED {
            let actions = ["VERIFY_EMAIL"];
            let path = format!("/users/{}/execute-actions-email", user.id);

            let resp = self
                .admin_api
                .put(&path, Some(&actions))
                .await
                .map_err(|e| ExternalUserError::Member(Some(Arc::new(e))))?;

            if !resp.status().is_success() {
                self.logger.error(format!(
                    "Cant send email verification: {}",
                    resp.text()
                        .await
                        .unwrap_or("Also can't get response text.".into())
                ));
            }

            Ok(())
        } else {
            self.logger.error(format!(
                "Failed to create user: {}",
                resp.text()
                    .await
                    .unwrap_or("Also can't get response text.".into())
            ));

            Err(ExternalUserError::Member(Some(Arc::new(
                KeycloakUserError(
                    "Failed to create user. Wrong keycloak response status code".into(),
                ),
            ))))
        }
    }
}
