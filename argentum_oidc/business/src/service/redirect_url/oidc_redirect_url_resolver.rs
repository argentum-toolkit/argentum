pub struct OidcRedirectUrlResolver {
    base_url: String,
}

impl OidcRedirectUrlResolver {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub fn resolve(&self, provider: &str) -> String {
        format!("{}?provider={}", self.base_url, provider)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_appends_provider_to_base_url() {
        let resolver =
            OidcRedirectUrlResolver::new("https://argentum.example.com/oidc/auth/callback");

        assert_eq!(
            resolver.resolve("Google"),
            "https://argentum.example.com/oidc/auth/callback?provider=Google"
        );
        assert_eq!(
            resolver.resolve("GitHub"),
            "https://argentum.example.com/oidc/auth/callback?provider=GitHub"
        );
    }
}
