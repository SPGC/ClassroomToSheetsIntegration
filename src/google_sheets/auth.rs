use reqwest::Client;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize)]
struct Claims {
    iss: String,       // Email of service account
    scope: String,     // Access scope
    aud: String,       // Audience
    exp: usize,        // Token expiration time
    iat: usize,        // Token issue time
}

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: usize,
    pub token_type: String,
}

pub async fn get_access_token(
    service_account_email: &str,
    private_key: &str,
    scope: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // JWT
    let iat = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() as usize;
    let exp = iat + 3600; // Token is valid for 1 hour

    let claims = Claims {
        iss: service_account_email.to_string(),
        scope: scope.to_string(),
        aud: "https://oauth2.googleapis.com/token".to_string(),
        exp,
        iat,
    };

    let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes())?;

    let jwt = encode(&Header::new(Algorithm::RS256), &claims, &encoding_key)?;

    // Request to get access token
    let client = Client::new();
    let params = [
        ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
        ("assertion", &jwt),
    ];

    let resp = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await?;

    if resp.status().is_success() {
        let token_response: TokenResponse = resp.json().await?;
        Ok(token_response.access_token)
    } else {
        let error_text = resp.text().await?;
        Err(Box::from(error_text))
    }
}
