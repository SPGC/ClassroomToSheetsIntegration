use reqwest::Client;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize)]
struct Claims {
    iss: String,       // Email сервисного аккаунта
    scope: String,     // Область доступа
    aud: String,       // Аудитория
    exp: usize,        // Время истечения токена
    iat: usize,        // Время выпуска токена
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
    // Создание JWT
    let iat = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() as usize;
    let exp = iat + 3600; // Токен действителен 1 час

    let claims = Claims {
        iss: service_account_email.to_string(),
        scope: scope.to_string(),
        aud: "https://oauth2.googleapis.com/token".to_string(),
        exp,
        iat,
    };

    let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes())?;

    let jwt = encode(&Header::new(Algorithm::RS256), &claims, &encoding_key)?;

    // Запрос на получение токена доступа
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
