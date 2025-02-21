use crate::Error;
use reqwest::Client;
use serde::Serialize;
use serde_json::json;
use serde::Deserialize;

#[derive(serde::Deserialize, Debug)]
struct Output {
    prvs_rcdl_excc_amt:String, //정산액
    thdt_buy_amt:String, //해당 날짜 매입 금액
    scts_evlu_amt:String, //총 평가 금액
    nass_amt:String, //총 순 자산 금액
    evlu_pfls_smtl_amt:String, //평가 손익 합계
}
#[derive(serde::Deserialize, Debug)]
struct Holding {
    rt_cd: String,       // 성공 실패 코드
    msg_cd: String,      // 응답코드
    msg1: String,        // 응답메시지
    ctx_area_fk100: String,
    ctx_area_nk100: String,
    output2: Option<Vec<Output>>, // 여러 개의 상품 정보를 담기 위해 Vec 사용
}

#[derive(Serialize)]
struct QueryParams {
    cano:String,
    acnt_prdt_cd:String,
    afhr_flpr_yn:String,
    ofl_yn:String,
    inqr_dvsn:String,
    unpr_dvsn:String,
    fund_sttl_icld_yn:String,
    fncg_amt_auto_rdpt_yn:String,
    prcs_dvsn:String,
    ctx_area_fk100:String,
    ctx_area_nk100:String,
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
    let endpoint = "/uapi/domestic-stock/v1/trading/inquire-balance";
    let url = format!("{}{}?apiKey={}", base_url, endpoint, appkey);
    //[국내주식]>주문/계좌>주식잔고조회 - GET

    let params = QueryParams {
        cano: "--------".to_string(),
        acnt_prdt_cd: "01".to_string(),
        afhr_flpr_yn: "N".to_string(),
        ofl_yn: "Y".to_string(),
        inqr_dvsn: "02".to_string(),
        unpr_dvsn: "01".to_string(),
        fund_sttl_icld_yn: "N".to_string(),
        fncg_amt_auto_rdpt_yn: "N".to_string(),
        prcs_dvsn: "01".to_string(),
        ctx_area_fk100: "100".to_string(),
        ctx_area_nk100: "100".to_string(),
    };

    let response = client.get(&url)
    .header("Content-Type", "application/json; charset=utf-8")
    .header("Authorization", format!("Bearer {}", access_token))
    .header("appkey", appkey)
    .header("appsecret",appsecret)
    .header("tr_id","VTTC8434R")
    .header("Accept", "application/json")
    .query(&params)
    .send().await?; //비동기 HTTP GET 요청

    let check: Holding= response.json().await?; //JSON 응답을 Holding 구조체로 파싱
    //println!("{:?}",check);
    println!("rt_cd: {}", check.rt_cd);
    println!("msg_cd: {}", check.msg_cd);
    println!("msg1: {}", check.msg1);
    println!("ctx_area_fk100: {}", check.ctx_area_fk100);
    println!("ctx_area_nk100: {}", check.ctx_area_nk100);
    println!("");

    if let Some(output) = check.output2 {
        for item in output {
            println!("정산액 {}", item.prvs_rcdl_excc_amt);
            println!("해당 날짜 매입 금액 {}", item.thdt_buy_amt);
            println!("총 평가 금액 {}", item.scts_evlu_amt);
            println!("총 순 자산 금액 {}", item.nass_amt);
            println!("평가 손익 합계 {}", item.evlu_pfls_smtl_amt);
            println!("");
        }
    } else {
        println!("output2가 없습니다.");
        println!("");
    }
    Ok(())
}