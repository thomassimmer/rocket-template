use rocket_cors::{AllowedOrigins, Cors, CorsOptions};

#[macro_export]
macro_rules! json_string {
    ($value:tt) => {
        serde_json::to_string(&serde_json::json!($value)).expect("cannot json stringify")
    };
}

// * Handles CORS config so that server and client can communicate
pub fn cors_fairing() -> Cors {
    CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .to_cors()
        .unwrap()
}
