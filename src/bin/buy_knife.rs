extern crate reqwest;

use dotenv::dotenv;
use reqwest::header;
use rusty_sapphire::config::{buylisting_params, Currency, dummy_headers};

fn main() {
    // buy_knife("PP-Bizon | Sand Dashed (Well-Worn)".to_string(), "4266636276820175139".to_string());
    for param in buylisting_params("e5c810f940360ddf3e8714c5", 100, 10, 110).iter() {
        println!("{:?}", param);
    }


}

fn buy_knife(name: String, listingid: String, converted_price: u64, converted_publisher_fee: u64, converted_steam_fee: u64) {
    dotenv().ok();
    let COOKIE = std::env::var("COOKIE").expect("COOKIE variable not found");
    let sessionid = COOKIE.split("; ").find(|&x| x.starts_with("sessionid=")).unwrap().split("=").nth(1).unwrap();
    let mut headers = dummy_headers();
    headers.insert("Accept", "*/*".parse().unwrap());
    headers.insert("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap());
    headers.insert("Origin", "https://steamcommunity.com".parse().unwrap());
    headers.insert("Referer", format!("https://steamcommunity.com/market/listings/730/{name}").parse().unwrap());

    headers.insert(header::COOKIE, COOKIE.parse().unwrap());

    let subtotal = converted_price;
    let fee = converted_publisher_fee + converted_steam_fee;
    let total = subtotal + fee;

    let params = buylisting_params(sessionid, subtotal, fee, total);

    let client = reqwest::blocking::Client::new();
    let res = client.post(format!("https://steamcommunity.com/market/buylisting/{listingid}"))
        .headers(headers)
        .body(params.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<String>>().join("&"))
        .send().unwrap();
    // .text().unwrap();
    println!("{}", res.status());
}

// curl 'https://steamcommunity.com/market/buylisting/4273391676262600401' \
//   -H 'Accept: */*' \
//   -H 'Accept-Language: pl-PL,pl;q=0.9' \
//   -H 'Connection: keep-alive' \
//   -H 'Content-Type: application/x-www-form-urlencoded; charset=UTF-8' \
//   -H 'Cookie: ActListPageSize=10; timezoneOffset=3600,0; Steam_Language=english; browserid=2834483470340916546; recentlyVisitedAppHubs=730; strInventoryLastContext=730_2; steamCurrencyId=6; sessionid=e5c810f940360ddf3e8714c5; webTradeEligibility=%7B%22allowed%22%3A1%2C%22allowed_at_time%22%3A0%2C%22steamguard_required_days%22%3A15%2C%22new_device_cooldown_days%22%3A0%2C%22time_checked%22%3A1678223725%7D; steamCountry=PL%7Cfa11d370fcc12db9b571c58a3563a4e9; steamLoginSecure=76561198183864150%7C%7CeyAidHlwIjogIkpXVCIsICJhbGciOiAiRWREU0EiIH0.eyAiaXNzIjogInI6MEQwMF8yMjMxODBGM18wOTNBOCIsICJzdWIiOiAiNzY1NjExOTgxODM4NjQxNTAiLCAiYXVkIjogWyAid2ViIiBdLCAiZXhwIjogMTY3ODM1MzgyNiwgIm5iZiI6IDE2Njk2MjczNDAsICJpYXQiOiAxNjc4MjY3MzQwLCAianRpIjogIjBEMEFfMjIzMTgwRjJfMUNEQkEiLCAib2F0IjogMTY3ODI2NzM0MCwgInJ0X2V4cCI6IDE2OTY0ODUxNzYsICJwZXIiOiAwLCAiaXBfc3ViamVjdCI6ICI0NS4xNTguMTA5LjE5MSIsICJpcF9jb25maXJtZXIiOiAiNDUuMTU4LjEwOS4xOTEiIH0.0kpsyb_YlZa41wXT9KhoacGNZViQpY6GGkf7HuHPcHWv25HV55-Cuy0CYri9sV25kVw3_3h-M1cwOlbBNCjKAQ' \
//   -H 'Origin: https://steamcommunity.com' \
//   -H 'Referer: https://steamcommunity.com/market/listings/730/PP-Bizon%20%7C%20Sand%20Dashed%20%28Field-Tested%29' \
//   -H 'Sec-Fetch-Dest: empty' \
//   -H 'Sec-Fetch-Mode: cors' \
//   -H 'Sec-Fetch-Site: same-origin' \
//   -H 'Sec-GPC: 1' \
//   -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36' \
//   -H 'sec-ch-ua: "Chromium";v="110", "Not A(Brand";v="24", "Brave";v="110"' \
//   -H 'sec-ch-ua-mobile: ?0' \
//   -H 'sec-ch-ua-platform: "Linux"' \
//   --data-raw 'sessionid=e5c810f940360ddf3e8714c5&currency=6&subtotal=7&fee=2&total=9&quantity=1&first_name=Ziemowit&last_name=Maciejewski&billing_address=sgdrg&billing_address_two=sdrgsrdg&billing_country=PL&billing_city=sdgrsdrg&billing_state=&billing_postal_code=03-107&save_my_address=1' \
//   --compressed


// curl 'https://steamcommunity.com/market/buylisting/4264384476997126397' \
//   -H 'Accept: */*' \
//   -H 'Accept-Language: pl-PL,pl;q=0.9' \
//   -H 'Connection: keep-alive' \
//   -H 'Content-Type: application/x-www-form-urlencoded; charset=UTF-8' \
//   -H 'Cookie: ActListPageSize=10; timezoneOffset=3600,0; Steam_Language=english; browserid=2834483470340916546; recentlyVisitedAppHubs=730; strInventoryLastContext=730_2; steamCurrencyId=6; sessionid=8c2b6d788b066cab1b8b92d8; webTradeEligibility=%7B%22allowed%22%3A1%2C%22allowed_at_time%22%3A0%2C%22steamguard_required_days%22%3A15%2C%22new_device_cooldown_days%22%3A0%2C%22time_checked%22%3A1678307651%7D; steamCountry=PL%7Cfa11d370fcc12db9b571c58a3563a4e9; steamLoginSecure=76561198183864150%7C%7CeyAidHlwIjogIkpXVCIsICJhbGciOiAiRWREU0EiIH0.eyAiaXNzIjogInI6MEQwNF8yMjMxODFFOF81QkFBMCIsICJzdWIiOiAiNzY1NjExOTgxODM4NjQxNTAiLCAiYXVkIjogWyAid2ViIiBdLCAiZXhwIjogMTY3ODQ2NDU5OSwgIm5iZiI6IDE2Njk3Mzc3MTAsICJpYXQiOiAxNjc4Mzc3NzEwLCAianRpIjogIjBEMEFfMjIzMTgxRTdfMDVCRkQiLCAib2F0IjogMTY3ODM3NzcwOSwgInJ0X2V4cCI6IDE2OTY2MzAxMTMsICJwZXIiOiAwLCAiaXBfc3ViamVjdCI6ICI0NS4xNTguMTA5LjE5MSIsICJpcF9jb25maXJtZXIiOiAiNDUuMTU4LjEwOS4xOTEiIH0.ZEFGm2XM7WiQF-dFZqP5eM_s3gPbXPMsgD2tz6ZgoHw3gF0v0QvRheNQGGd8YbjYNSsYpNLMlhjjX2eiqcjyAQ' \
//   -H 'Origin: https://steamcommunity.com' \
//   -H 'Referer: https://steamcommunity.com/market/listings/730/Galil%20AR%20%7C%20Cold%20Fusion%20%28Field-Tested%29' \
//   -H 'Sec-Fetch-Dest: empty' \
//   -H 'Sec-Fetch-Mode: cors' \
//   -H 'Sec-Fetch-Site: same-origin' \
//   -H 'Sec-GPC: 1' \
//   -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36' \
//   -H 'sec-ch-ua: "Chromium";v="110", "Not A(Brand";v="24", "Brave";v="110"' \
//   -H 'sec-ch-ua-mobile: ?0' \
//   -H 'sec-ch-ua-platform: "Linux"' \
//   --data-raw 'sessionid=8c2b6d788b066cab1b8b92d8&currency=6&subtotal=38&fee=4&total=42&quantity=1&first_name=Ziemowit&last_name=Maciejewski&billing_address=Warszawa&billing_address_two=Lanowa&billing_country=PL&billing_city=Poland&billing_state=&billing_postal_code=03-107&save_my_address=1' \
//   --compressed
