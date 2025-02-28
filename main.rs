mod search;
mod buy;
mod sell;
mod stock_holding_check;
mod transaction_recode_check;
mod cancel;
mod get_access_token;
mod discard_token;

use std::io; 
use reqwest::Error;
use serde::{Deserialize};
use get_access_token::TokenResponse;

#[tokio::main]
async fn main() -> Result<(), Error>{

    let token_response:TokenResponse = get_access_token::get_access_token().await?;
    let access_token = token_response.access_token;

    let choice = '1';
    while choice !='0'{
        let mut choice = String::new();
        let mut code = String::new();
        println!("주식 관리 시스템");
        println!("1. 주식 검색");
        println!("2. 주식 매입");
        println!("3. 주식 매도");
        println!("4. 자산 조회");
        println!("5. 거래 기록 조회");
        println!("6. 주식 정정 취소");
        println!("0. 종료");
        println!("원하는 메뉴를 입력하세요");
        io::stdin().read_line(&mut choice).expect("입력오류");
        let choice: i32 = match choice.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("올바른 숫자를 입력하세요.");
                continue;
            }
        };
        match choice {
            1=> {println!("원하는 종목코드를 입력하세요.");
                io::stdin().read_line(&mut code).expect("입력오류");
                let code_trimmed = code.trim();
                search::run(&access_token, &code_trimmed).await.expect("오류")},
            2=> {println!("원하는 종목코드를 입력하세요.");
                io::stdin().read_line(&mut code).expect("입력오류");
                let code_trimmed = code.trim();
                buy::run(&access_token, &code_trimmed).await.expect("오류")},
            3=> {println!("원하는 종목코드를 입력하세요.");
                io::stdin().read_line(&mut code).expect("입력오류");
                let code_trimmed = code.trim();
                sell::run(&access_token, &code_trimmed).await.expect("오류")},
            4=> stock_holding_check::run(&access_token).await.expect("오류"),
            5=> transaction_recode_check::run(&access_token).await.expect("오류"),
            6=> {println!("원주문번호를 입력하세요.(원주문번호 확인은 5번 메뉴를 통해 가능합니다)");
                io::stdin().read_line(&mut code).expect("입력오류");
                let code_trimmed = code.trim();
                cancel::run(&access_token, &code_trimmed).await.expect("오류")},
            0=> {
                println!("주식 관리 시스템을 종료합니다.");
                break;
            }
            _ => println!("올바른 선택이 아닙니다."),
        }
    }
    discard_token::discard_token(&access_token);
    Ok(())
}

// 기능1. 주식검색 search
// 기능2. 주식매입 buy
// 기능3. 주식매도 sell
// 기능4. 자산 조회 stock holding check
// 기능5. 거래기록 조회 transaction recode check(일별)
// 기능6. 주식정정취소 cancel