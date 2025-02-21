use crate::Error;
use reqwest::Client;
use serde::Serialize;
use serde_json::json;
use serde::Deserialize;

#[derive(serde::Deserialize, Debug)]
struct Trading_Ranking {
    rt_cd:String,
    msg_cd:String,
    msg1:String,
}

#[derive(Serialize)]
struct QueryParams {
    fid_cond_mrkt_div_code:String,
    fid_input_iscd:String,
    fid_input_data_1:String,
    find_input_data_2:String,
    fid_period_div_code:String,
    fid_org_adj_prc:String,
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String, // 접근 토큰
    token_type: String,   // 토큰 유형
    expires_in: u64,      // 만료 시간
}

pub async fn get_access_token() -> Result<TokenResponse, Error> {
    let client = Client::new();
    let url = "https://openapi.koreainvestment.com:9443/oauth2/tokenP";
    let app_key = "APIKEY";
    let app_secret = "SECRET";

    // 요청 본문 생성
    let params = json!({
        "grant_type": "client_credentials",
        "appkey": app_key,
        "appsecret": app_secret
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

pub async fn run() -> Result<(), Error>{
    
    let token_response = get_access_token().await?;
    let access_token = token_response.access_token;
    let client = Client::new();
    let appkey = "APIKEY";
    let appsecret = "SECRET";
    let base_url = "https://openapivts.koreainvestment.com:29443";
    let endpoint = "/uapi/domestic-stock/v1/quotations/volume-rank";
    //[국내주식]>기본시세>국내주식기간별시세 - GET
    let url = format!("{}{}?apiKey={}", base_url, endpoint, appkey);

    let params = QueryParams {
        fid_cond_mrkt_div_code:"J".to_string(),
        fid_input_iscd:"Q500001".to_string(),
        fid_input_data_1:"20250218".to_string(),
        find_input_data_2:"20250221".to_string(),
        fid_period_div_code:"D".to_string(),
        fid_org_adj_prc:"1".to_string(),
    };

    let response = client.get(&url)
    .header("Content-Type", "application/json; charset=utf-8")
    .header("Authorization", format!("Bearer {}", access_token))
    .header("appkey", appkey)
    .header("appsecret",appsecret)
    .header("tr_id","FHKST03010100")
    .header("custtype", "P")
    .query(&params)
    .send().await?; //비동기 HTTP GET 요청

    println!("Response Status: {}", response.status());
    let body = response.text().await?;
    println!("Response Body: {}", body);
    //let rank: String= response.text().await?;
    //println!("{:?}",rank);
    
    Ok(())
}