use crate::Error;
use reqwest::Client;
use serde::Serialize;
use serde_json::json;
use serde::Deserialize;

#[derive(serde::Deserialize, Debug)]
struct Output1 {
    ord_dt:String, //주문날짜
    ord_gno_brno:String, //주문 번호
    pdno:String, //상품 번호
    prdt_name:String, //상품 이름
    ord_qty:String, //주문 수량
    ord_unpr:String, //주문 단가
    tot_ccld_qty:String, //총 체결 수량
    tot_ccld_amt:String, //총 체결 금액
    rmn_qty:String, //잔여 수량
}
#[derive(serde::Deserialize, Debug)]
struct Output2 {
    tot_ord_qty:String, //총 주문 수량
    tot_ccld_qty:String, // 총 체결 수량
    tot_ccld_amt:String, //총 체결 금액
}
#[derive(serde::Deserialize, Debug)]
struct TransactionRecord {
    rt_cd:String,
    msg_cd:String,
    msg1:String,
    ctx_area_fk100: String,
    ctx_area_nk100: String,
    output1: Option<Vec<Output1>>, // 여러 개의 상품 정보를 담기 위해 Vec 사용
    output2: Option<Output2>, // 여러 개의 상품 정보를 담기 위해 Vec 사용
}
#[derive(Serialize)]
struct QueryParams {
    cano:String,
    acnt_prdt_cd:String,
    inqr_strt_dt:String,
    inqr_end_dt:String,
    sll_buy_dvsn_cd:String,
    inqr_dvsn:String,
    pdno:String,
    ccld_dvsn:String,
    ord_gno_brno:String,
    odno:String,
    inqr_dvsn_3:String,
    inqr_dvsn_1:String,
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
    let base_url ="https://openapivts.koreainvestment.com:29443";
    let endpoint = "/uapi/domestic-stock/v1/trading/inquire-daily-ccld";
    let url = format!("{}{}?apiKey={}", base_url, endpoint, appkey);

    let params = QueryParams {
        cano: "--------".to_string(),
        acnt_prdt_cd: "01".to_string(),
        inqr_strt_dt:"20250218".to_string(),
        inqr_end_dt:"20250221".to_string(),
        sll_buy_dvsn_cd:"00".to_string(),
        inqr_dvsn:"01".to_string(),
        pdno:"".to_string(),
        ccld_dvsn:"01".to_string(),
        ord_gno_brno:"".to_string(),
        odno:"".to_string(),
        inqr_dvsn_3:"00".to_string(),
        inqr_dvsn_1:"".to_string(),
        ctx_area_fk100:"".to_string(),
        ctx_area_nk100:"".to_string(),
    };
    
    //[국내주식]>주문/계좌>주식일별주문체결조회 -GET
    let response = client.get(&url)
    .header("Content-Type", "application/json; charset=utf-8")
    .header("Authorization", format!("Bearer {}", access_token))
    .header("appkey", appkey)
    .header("appsecret",appsecret)
    .header("tr_id","VTTC8001R")
    .query(&params)
    .send()
    .await?;

    let transaction: TransactionRecord= response.json().await?; //JSON 응답을 TransactionRecord 구조체로 파싱
    //println!("{:?}",transaction);

    // 각 필드를 한 줄씩 출력
    println!("rt_cd: {}", transaction.rt_cd);
    println!("msg_cd: {}", transaction.msg_cd);
    println!("msg1: {}", transaction.msg1);
    println!("ctx_area_fk100: {}", transaction.ctx_area_fk100);
    println!("ctx_area_nk100: {}", transaction.ctx_area_nk100);
    println!("");

    if let Some(output1) = transaction.output1 {
        for item in output1 {
            println!("주문 날짜 {}", item.ord_dt);
            println!("주문 번호 {}", item.ord_gno_brno);
            println!("상품 번호 {}", item.pdno);
            println!("상품 이름 {}", item.prdt_name);
            println!("주문 수량 {}", item.ord_qty);
            println!("주문 단가 {}", item.ord_unpr);
            println!("총 체결 수량 {}", item.tot_ccld_qty);
            println!("총 체결 금액 {}", item.tot_ccld_amt);
            println!("잔여 수량 {}", item.rmn_qty);
            println!("");
        }
    } else {
        println!("output1이 없습니다.");
        println!("");
    }

    if let Some(output2) = transaction.output2 {
        println!("총 주문 수량 {}", output2.tot_ord_qty);
        println!("총 체결 수량 {}", output2.tot_ccld_qty);
        println!("총 체결 금액 {}", output2.tot_ccld_amt);
        println!("");
    } else {
        println!("output2가 없습니다.");
        println!("");
    }
    Ok(())
}
