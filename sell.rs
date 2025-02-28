use crate::Error;
use reqwest::Client;
use serde::Serialize;
use serde_json::json;
use serde::Deserialize;

#[derive(Serialize)]
struct QueryParams {
    CANO:String,
    ACNT_PRDT_CD:String,
    PDNO:String,
    ORD_DVSN:String,
    ORD_QTY:String,
    ORD_UNPR:String,
}
#[derive(serde::Deserialize, Debug)]
struct SellResponse {
    rt_cd: String,       // 성공 실패 코드
    msg_cd: String,      // 응답코드
    msg1: String,        // 응답메시지
    output:Option<Output>,
}
#[derive(serde::Deserialize, Debug)]
struct Output {
    KRX_FWDG_ORD_ORGNO:String, //한국거래소전송주문조직번호
    ODNO:String, //주문번호
    ORD_TMD:String, //주문시각
}
pub async fn run(access_token:&str,code:&str) -> Result<(), Error>{

    let client = Client::new();
    let appkey = "APPKEY";
    let appsecret = "APPSECRET";
    let base_url = "https://openapivts.koreainvestment.com:29443";
    let endpoint = "/uapi/domestic-stock/v1/trading/order-cash";
    let url = format!("{}{}?apiKey={}", base_url, endpoint, appkey);
    //[국내주식]>주문/계좌>주식주문(현금) - POST

    let params = QueryParams {
        CANO:"계좌번호".to_string(),
        ACNT_PRDT_CD:"01".to_string(),
        PDNO:code.to_string(),
        ORD_DVSN:"03".to_string(), //주문 구분 03 -> 최유리 지정가
        ORD_QTY:"10".to_string(), // 주문 수량
        ORD_UNPR:"0".to_string(), //주문 단가
    };
    let response = client.post(&url)
    .header("Content-Type", "application/json; charset=utf-8")
    .header("Authorization", format!("Bearer {}", access_token))
    .header("appkey", appkey)
    .header("appsecret",appsecret)
    .header("tr_id","VTTC0801U")
    .json(&params).send().await?; 

    
    let check: SellResponse= response.json().await?; 
    println!("rt_cd: {}", check.rt_cd);
    println!("msg_cd: {}", check.msg_cd);
    println!("msg1: {}", check.msg1);
    println!("");

    if let Some(output) = check.output {
        
        println!("한국거래소전송주문조직번호 {}", output.KRX_FWDG_ORD_ORGNO);
        println!("주문번호 {}", output.ODNO);
        println!("주문시각 {}", output.ORD_TMD);
        println!("");
        
    } else {
        println!("output이 없습니다.");
        println!("");
    }
    Ok(())
}