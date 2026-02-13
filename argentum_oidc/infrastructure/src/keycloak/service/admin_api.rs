use std::{
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use argentum_log_business::LoggerTrait;
use argentum_standard_business::error::PoisonedLockError;
use reqwest::{RequestBuilder, StatusCode};
use serde::Serialize;

use crate::keycloak::{dto::AdminTokenResponse, service::error::AdminApiError};

type TokenCache = RwLock<Option<(String, Instant)>>;

pub struct AdminApi<L>
where
    L: LoggerTrait,
{
    base_url: String,
    realm: String,
    client_id: String,
    client_secret: String,

    logger: Arc<L>,
    client: Arc<reqwest::Client>,
    token_cache: TokenCache,
}

impl<L> AdminApi<L>
where
    L: LoggerTrait,
{
    pub fn new(
        base_url: &str,
        realm: &str,
        client_id: &str,
        client_secret: &str,
        logger: Arc<L>,
        client: Arc<reqwest::Client>,
    ) -> Self {
        let token_cache = RwLock::new(None);

        Self {
            base_url: base_url.into(),
            realm: realm.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            logger,
            client,
            token_cache,
        }
    }

    async fn get_admin_token(&self) -> Result<AdminTokenResponse, AdminApiError> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.base_url, self.realm
        );

        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
        ];

        let resp = self
            .client
            .post(&token_url)
            .form(&params)
            .send()
            .await?
            .error_for_status()?
            .json::<AdminTokenResponse>()
            .await?;

        Ok(resp)
    }

    async fn get_cached_admin_token(&self) -> Result<String, AdminApiError> {
        {
            let read_guard = self.token_cache.read().map_err(|_| PoisonedLockError)?;

            let now = Instant::now();
            if let Some((token, expires_at)) = read_guard.as_ref()
                && *expires_at > now
            {
                return Ok(token.clone());
            }
        }

        self.logger
            .info("Admin token not found in cache, fetching new one.");

        let resp = self.get_admin_token().await?;

        self.update_cache(resp.clone()).await?;

        Ok(resp.access_token)
    }

    async fn update_cache(&self, token_resp: AdminTokenResponse) -> Result<(), AdminApiError> {
        let now = Instant::now();
        let expires_in = token_resp.expires_in as u64;
        let expires_at = now + Duration::from_secs(expires_in) - Duration::from_secs(120);
        let mut write_guard = self.token_cache.write().map_err(|_| PoisonedLockError)?;

        *write_guard = Some((token_resp.access_token.clone(), expires_at));
        drop(write_guard);

        Ok(())
    }

    async fn build_get<T: Serialize + ?Sized>(
        &self,
        path: &str,
        query: Option<&T>,
    ) -> Result<RequestBuilder, AdminApiError> {
        let token = self.get_cached_admin_token().await?;

        let url = format!("{}/admin/realms/{}{}", self.base_url, self.realm, path,);
        let mut builder = self.client.get(url).bearer_auth(token);

        if let Some(q) = query {
            builder = builder.query(q);
        }

        Ok(builder)
    }

    pub async fn get<T: Serialize + ?Sized>(
        &self,
        path: &str,
        query: Option<&T>,
    ) -> Result<reqwest::Response, AdminApiError> {
        let builder = self.build_get(path, query).await?;

        let resp = builder.send().await?;

        match resp.status() {
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                self.logger.info("Admin token expired, fetching new one.");

                let builder = self.build_get(path, query).await?;
                Ok(builder.send().await?)
            }
            _ => Ok(resp),
        }
    }

    async fn build_post<T: Serialize + ?Sized>(
        &self,
        path: &str,
        json: Option<&T>,
    ) -> Result<RequestBuilder, AdminApiError> {
        let token = self.get_cached_admin_token().await?;

        let url = format!("{}/admin/realms/{}{}", self.base_url, self.realm, path,);

        let mut builder = self.client.post(url).bearer_auth(token);

        if let Some(j) = json {
            builder = builder.json(j);
        }

        Ok(builder)
    }

    pub async fn post<T: Serialize + ?Sized>(
        &self,
        path: &str,
        json: Option<&T>,
    ) -> Result<reqwest::Response, AdminApiError> {
        let builder = self.build_post(path, json).await?;

        let resp = builder.send().await?;

        match resp.status() {
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                self.logger.info("Admin token expired, fetching new one.");

                let builder = self.build_post(path, json).await?;
                Ok(builder.send().await?)
            }
            _ => Ok(resp),
        }
    }

    async fn build_put<T: Serialize + ?Sized>(
        &self,
        path: &str,
        json: Option<&T>,
    ) -> Result<RequestBuilder, AdminApiError> {
        let token = self.get_cached_admin_token().await?;

        let url = format!("{}/admin/realms/{}{}", self.base_url, self.realm, path,);
        let mut builder = self.client.put(url).bearer_auth(token);

        if let Some(j) = json {
            builder = builder.json(j);
        }

        Ok(builder)
    }

    pub async fn put<T: Serialize + ?Sized>(
        &self,
        path: &str,
        json: Option<&T>,
    ) -> Result<reqwest::Response, AdminApiError> {
        let builder = self.build_put(path, json).await?;

        let resp = builder.send().await?;

        match resp.status() {
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                self.logger.info("Admin token expired, fetching new one.");

                let builder = self.build_put(path, json).await?;
                Ok(builder.send().await?)
            }
            _ => Ok(resp),
        }
    }
}
