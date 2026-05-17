use std::env;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub google_client_email: String,
    pub google_private_key: String,
    pub google_delegated_user: String,
    pub google_user_domain: String,
    pub google_default_password: String,
    pub odoo_base_url: String,
    pub odoo_username: String,
    pub odoo_password: String,
    pub default_user_role: String,
    pub default_start_date: String,
}

impl Config {
    pub fn from_env() -> Self {
        let (google_client_email, google_private_key) = load_google_credentials();

        Self {
            port: env_var("PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .expect("PORT must be a valid number"),
            google_client_email,
            google_private_key,
            google_delegated_user: require_env("GOOGLE_DELEGATED_USER"),
            google_user_domain: require_env("GOOGLE_USER_DOMAIN"),
            google_default_password: require_env("GOOGLE_DEFAULT_PASSWORD"),
            odoo_base_url: require_env("ODOO_BASE_URL"),
            odoo_username: require_env("ODOO_USERNAME"),
            odoo_password: require_env("ODOO_PASSWORD"),
            default_user_role: env_var("DEFAULT_USER_ROLE")
                .unwrap_or_else(|_| "employee".into()),
            default_start_date: require_env("DEFAULT_START_DATE"),
        }
    }
}

fn load_google_credentials() -> (String, String) {
    if let Ok(path) = env::var("GOOGLE_SERVICE_ACCOUNT_KEY_PATH") {
        let key_file =
            std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {path}: {e}"));
        return parse_service_account_json(&key_file);
    }

    if let Ok(json) = env::var("GOOGLE_SERVICE_ACCOUNT_JSON") {
        return parse_service_account_json(&json);
    }

    let client_email = require_env("GOOGLE_CLIENT_EMAIL");
    let private_key = normalize_private_key(&require_env("GOOGLE_PRIVATE_KEY"));
    (client_email, private_key)
}

fn parse_service_account_json(json: &str) -> (String, String) {
    let value: serde_json::Value =
        serde_json::from_str(json).expect("invalid Google service account JSON");
    let client_email = value["client_email"]
        .as_str()
        .expect("service account JSON missing client_email")
        .to_string();
    let private_key = normalize_private_key(
        value["private_key"]
            .as_str()
            .expect("service account JSON missing private_key"),
    );
    (client_email, private_key)
}

fn normalize_private_key(key: &str) -> String {
    key.replace("\\n", "\n")
}

fn require_env(name: &str) -> String {
    env_var(name).unwrap_or_else(|_| panic!("missing required env var: {name}"))
}

fn env_var(name: &str) -> Result<String, env::VarError> {
    env::var(name).map(|v| v.trim().to_string())
}
