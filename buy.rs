use crate::Error;

#[derive(serde::Deserialize, Debug)]
struct BuyResponse {
    success: bool,         // 매수 성공 여부
    transaction_id: String, // 거래 ID
    message: Option<String>, // 메시지 (에러 시)
}
pub async fn run() -> Result<(), Error>{

    let client = Client::new();
    let api_key = "APIKEY";
    let base_url = "https://openapivts.koreainvestment.com:29443";
    let url = "/uapi/domestic-stock/v1/trading/order-cash";
    //[국내주식]>주문/계좌>주식주문(현금) - POST

    // JSON 데이터 생성
    let new_post = json!({
        "title": "foo",
        "body": "bar",
        "userId": 1,
    });

    let response = client.post(url).json(&new_post).send().await?; 
    let movie: Movie= response.json().await?; //JSON 응답을 Movie 구조체로 파싱
    // 응답 본문을 문자열로 변환
    let body = response.text().await?;
    Ok(())
}