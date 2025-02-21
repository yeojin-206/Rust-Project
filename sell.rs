use crate::Error;

#[derive(serde::Deserialize, Debug)]
struct SellResponse {
    success: bool,         // 매도 성공 여부
    transaction_id: String, // 거래 ID
    message: Option<String>, // 메시지 (에러 시)
}
pub async fn run() -> Result<(), Error>{

    let client = Client::new();
    let api_key = "APIKEY";
    let url = "API_URL";

    let response = reqwest::get(&url).await?; //비동기 HTTP GET 요청
    let movie: Movie= response.json().await?; //JSON 응답을 Movie 구조체로 파싱

    Ok(())
}