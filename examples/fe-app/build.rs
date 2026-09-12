use dotenvy::{from_filename, from_filename_override};
use std::{env, path::Path};

fn export(key: &str) {
    println!(
        "cargo:rustc-env={}={}",
        key,
        env::var(key).unwrap_or_else(|_| panic!("{key} not set"))
    );
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let workspace_root = Path::new(&manifest_dir).ancestors().nth(2).unwrap();

    let env_path = workspace_root.join("./.env");
    let local_path = workspace_root.join("./.env.local");

    from_filename(env_path).unwrap();
    from_filename_override(local_path).unwrap();

    export("AG_GITHUB_CLIENT_ID");
    export("AG_GOOGLE_CLIENT_ID");
    export("AG_OIDC_REDIRECT_URL");

    export("AG_USER_API_URL");
    export("AG_USER_ACCOUNT_API_URL");
    export("AG_OIDC_API_URL");
}
