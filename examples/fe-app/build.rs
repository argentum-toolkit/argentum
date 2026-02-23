use dotenvy::{from_filename, from_filename_override};
use std::env;

fn export(key: &str) {
    println!(
        "cargo:rustc-env={}={}",
        key,
        env::var(key).expect(&format!("{key} not set"))
    );
}

fn main() {
    from_filename("../.env").ok();
    from_filename_override("../.env.local").ok();

    export("GITHUB_CLIENT_ID");
    export("OIDC_REDIRECT_URL");
}
