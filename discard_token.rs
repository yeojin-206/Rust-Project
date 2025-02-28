use crate::Error;
use reqwest::Client;
use serde::Serialize;
use serde_json::json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String, // 접근 토큰
    pub token_type: String,   // 토큰 유형
    pub expires_in: u64,      // 만료 시간
}

pub async fn discard_token(access_token:&str) -> Result<TokenResponse, Error> {
    let client = Client::new();
    let url = "https://openapivts.koreainvestment.com:29443/oauth2/revokeP";
    let appkey = "APPKEY";
    let appsecret = "APPSECRET";
    
    // 요청 본문 생성
    let params = json!({
        "appkey": app_key,
        "appsecret": app_secret,
        "token":access_token,
    });

    // POST 요청 보내기
    let response = client.post(url)
        .header("Content-Type", "application/json; charset=utf-8")
        .json(&params) // JSON 형식으로 본문 추가
        .send()
        .await?;

    // JSON 응답 파싱
    let token_response: TokenResponse = response.json().await?;

    Ok(token_response)
}
