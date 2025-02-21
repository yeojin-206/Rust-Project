use crate::Error;
use reqwest::Client;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;

#[derive(serde::Deserialize, Debug)]
struct Stock {
    code: String,          // 주식 코드
    name: String,          // 주식 이름
    current_price: f64,    // 현재 가격
    change: f64,           // 변동률
    volume: u64,           // 거래량
}
pub async fn run() -> Result<(), Error>{

    let api_key = "APIKEY";
    let base_url = "ws://ops.koreainvestment.com:31000";
    let endpoint = "/tryitout/H0STASP0";
    let url = Url::parse("{}{}?apiKey={}", base_url, endpoint, api_key).unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("연결오류");
    //[국내주식]>실시간시세> 국내주식 실시간호가

    // 메시지 전송
    let message = Message::Text("".to_string());
    ws_stream.send(message).await.expect("Failed to send message");

    // 응답 수신
    while let Some(message) = ws_stream.next().await {
        match message {
            Ok(msg) => println!("Received: {}", msg),
            Err(e) => eprintln!("Error receiving message: {}", e),
        }
    }

    //let response = reqwest::get(&url).await?; // 비동기 HTTP GET 요청
    //let stock: Stock= response.json().await?; //JSON 응답을 Stock 구조체로 파싱

    //println!("{:?}",stock);
    Ok(())
}