use crate::Error;
use reqwest::Client;
use serde::Serialize;
use serde_json::json;
use serde::Deserialize;

#[derive(Serialize)]
struct QueryParams {
    fid_cond_mrkt_div_code:String,
    fid_input_iscd:String,
}
#[derive(serde::Deserialize, Debug)]
struct Search {
    rt_cd: String,       // 성공 실패 코드
    msg_cd: String,      // 응답코드
    msg1: String,        // 응답메시지
    output:Option<Vec<Output>>,
}
#[derive(serde::Deserialize, Debug)]
struct Output {
    bstp_kor_isnm:String, //업종 한글 종목명
    stck_prpr:String, //주식 현재가
    stck_hgpr:String, //주식 최고가
    stck_lwpr:String, //주식 최저가
    stck_mxpr:String, //주식 상한가
    stck_llam:String, //주식 하한가
}
pub async fn run(access_token:&str,code:&str) -> Result<(), Error>{
    
    let client = Client::new();
    let appkey = "APPKEY";
    let appsecret = "APPSECRET";
    let base_url = "https://openapivts.koreainvestment.com:29443";
    let endpoint = "/uapi/domestic-stock/v1/quotations/inquire-price";
    let url = format!("{}{}?apiKey={}", base_url, endpoint, appkey);
    //[국내주식]>기본시세>주식현재가 시세 - GET

    let params = QueryParams {
        fid_cond_mrkt_div_code:"J".to_string(),
        fid_input_iscd:code.to_string(),
    };
    let response = client.get(&url)
    .header("Content-Type", "application/json; charset=utf-8")
    .header("Authorization", format!("Bearer {}", access_token))
    .header("appkey", appkey)
    .header("appsecret",appsecret)
    .header("tr_id","FHKST01010100")
    .query(&params)
    .send().await?; 

    println!("Response Status: {}", response.status());
    let response_text = response.text().await?;
    // 응답이 콤마로 구분된 경우 처리
    let lines: Vec<&str> = response_text.split('\n').collect();
    let mut checks:Vec<&str> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        for part in parts {
            let sub_parts: Vec<&str> = part.split(':').collect();
            checks.extend(sub_parts); // sub_parts를 checks에 추가
        }
    }
    if let Some(value) = checks.get(8) {
        let cleaned_value = value
        .replace("Some(", "")
        .replace(")", "")
        .replace("\\\"", "")
        .replace("\"", "");
    
        println!("업종 한글 종목명 {}", cleaned_value);
    } else {
        println!("값이 없습니다.");
    }
    if let Some(value) = checks.get(22) {
        let cleaned_value = value
        .replace("Some(", "")
        .replace(")", "")
        .replace("\\\"", "")
        .replace("\"", "");
    
        println!("주식 현재가 {}", cleaned_value);
    } else {
        println!("값이 없습니다.");
    }
    if let Some(value) = checks.get(38) {
        let cleaned_value = value
        .replace("Some(", "")
        .replace(")", "")
        .replace("\\\"", "")
        .replace("\"", "");
    
        println!("주식 최고가 {}", cleaned_value);
    } else {
        println!("값이 없습니다.");
    }
    if let Some(value) = checks.get(40) {
        let cleaned_value = value
        .replace("Some(", "")
        .replace(")", "")
        .replace("\\\"", "")
        .replace("\"", "");
    
        println!("주식 최저가 {}", cleaned_value);
    } else {
        println!("값이 없습니다.");
    }
    if let Some(value) = checks.get(42) {
        let cleaned_value = value
        .replace("Some(", "")
        .replace(")", "")
        .replace("\\\"", "")
        .replace("\"", "");
    
        println!("주식 상한가 {}", cleaned_value);
    } else {
        println!("값이 없습니다.");
    }
    if let Some(value) = checks.get(44) {
        let cleaned_value = value
        .replace("Some(", "")
        .replace(")", "")
        .replace("\\\"", "")
        .replace("\"", "");
    
        println!("주식 하한가 {}", cleaned_value);
    } else {
        println!("값이 없습니다.");
    }
    Ok(())
}