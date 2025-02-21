//mod search;
//mod buy;
//mod sell;
mod stock_holding_check;
mod transaction_recode_check;
mod trading_ranking;

use std::io; 
use reqwest::Error;
use serde::{Deserialize};

#[tokio::main]
async fn main() -> Result<(), Error>{

    let choice = '1';
    while choice !='0'{
        let mut choice = String::new();

        println!("주식 관리 시스템");
        println!("1. 주식 검색");
        println!("2. 주식 매입");
        println!("3. 주식 매도");
        println!("4. 보유 주식 조회");
        println!("5. 거래 기록 조회");
        println!("6. 거래량 순위");
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
            //1=> search::run().await.expect("오류"),
            //2=> buy::run().await.expect("오류"),
            //3=> sell::run().await.expect("오류"),
            4=> stock_holding_check::run().await.expect("오류"),
            5=> transaction_recode_check::run().await.expect("오류"),
            6=> trading_ranking::run().await.expect("오류"),
            0=> {
                println!("주식 관리 시스템을 종료합니다.");
                break;
            }
            _ => println!("올바른 선택이 아닙니다."),
        }
    }
    Ok(())
}

// 기능1. 주식검색 search
// 기능2. 주식매입 buy
// 기능3. 주식매도 sell
// 기능4. 보유주식 조회 stock holding check
// 기능5. 거래기록 조회 transaction recode check(일별)
// 기능6. 거래량 순위 trading ranking