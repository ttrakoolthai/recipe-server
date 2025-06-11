//! JWT authentication module for the recipe server.
//!
//! Provides utilities for generating and validating JWTs,
//! handling user registration, and defining authentication-related
//! data structures and errors.

use crate::*;

/// Holds the JWT encoding and decoding keys.
pub struct JwtKeys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl JwtKeys {
    /// Creates new JWT keys from a secret.
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

/// Reads a secret value from the environment or from a fallback file.
pub async fn read_secret(
    env_var: &str,
    default: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let secretf = std::env::var(env_var).unwrap_or_else(|_| default.to_owned());
    let secret = tokio::fs::read_to_string(secretf).await?;
    Ok(secret.trim().to_string())
}

/// Constructs JWT keys from a secret file.
pub async fn make_jwt_keys() -> Result<JwtKeys, Box<dyn std::error::Error>> {
    let secret = read_secret("JWT_SECRETFILE", "secrets/jwt_secret.txt").await?;
    Ok(JwtKeys::new(secret.as_bytes()))
}

/// Enumeration of authentication-related errors.
#[derive(Debug, thiserror::Error, Serialize)]
pub enum AuthError {
    /// The token provided is invalid.
    #[error("Invalid token")]
    InvalidToken,

    /// An internal error occurred during token creation.
    #[error("Internal Error: Token creation")]
    TokenCreation,

    /// The registration failed.
    #[error("Registration error")]
    Registration,
}

impl utoipa::PartialSchema for AuthError {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::Schema> {
        serde_json::json!({
            "Status":"401","Error":"Wrong credentials"
        })
        .into()
    }
}

impl utoipa::ToSchema for AuthError {}

/// A JSON Web Token response body returned to the client.
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

impl IntoResponse for AuthBody {
    fn into_response(self) -> axum::response::Response {
        Json(serde_json::json!(self)).into_response()
    }
}

impl axum::extract::FromRequestParts<SharedAppState> for Claims {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &SharedAppState,
    ) -> Result<Self, Self::Rejection> {
        use jsonwebtoken::{Algorithm, Validation, decode};

        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let appstate = state.read().await;
        let decoding_key = &appstate.jwt_keys.decoding;
        let validation = Validation::new(Algorithm::HS512);
        let result = decode::<Claims>(bearer.token(), decoding_key, &validation);
        let token_data = result.map_err(|_| AuthError::Registration)?;
        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AuthError::Registration => (StatusCode::UNAUTHORIZED, "Invalid registration"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
        };
        let body = Json(serde_json::json!({
            "Status": status.as_u16(),
            "Error": error_message,
        }));
        (status, body).into_response()
    }
}

/// A struct used to register a user.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct Registration {
    /// The full name of the user.
    #[schema(example = "John Smith")]
    full_name: String,

    /// The email address of the user.
    #[schema(example = "johnsmith@example.org")]
    email: String,

    /// The password provided by the user.
    #[schema(example = "password123")]
    password: String,
}

/// JWT Claims for authenticated sessions.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    /// The issuer of the token.
    #[schema(example = "recipe-server.po8.org")]
    iss: String,

    /// The subject of the token (user info).
    #[schema(example = "John Smith <johnsmith@example.org>")]
    sub: String,

    /// Expiration time as a Unix timestamp.
    #[schema(example = "1717630066")]
    exp: u64,
}

/// Generates a JWT token from the given registration information.
pub fn make_jwt_token(
    appstate: &AppState,
    registration: &Registration,
) -> Result<AuthBody, AuthError> {
    use jsonwebtoken::{Algorithm, Header, encode};

    if registration.password != appstate.reg_key {
        return Err(AuthError::Registration);
    }

    let iss = "recipe-server.po8.org".to_string();
    let sub = format!("{} <{}>", registration.full_name, registration.email);
    let exp = (Utc::now() + TimeDelta::days(1)).timestamp();
    let exp = u64::try_from(exp).unwrap();
    let claims = Claims { iss, sub, exp };
    let header = Header::new(Algorithm::HS512);
    let token = encode(&header, &claims, &appstate.jwt_keys.encoding)
        .map_err(|_| AuthError::TokenCreation)?;
    Ok(AuthBody::new(token))
}
