extern crate reqwest;
use reqwest::header;

fn main(){

    let mut headers = header::HeaderMap::new();
    headers.insert("Accept", "*/*".parse().unwrap());
    headers.insert("Accept-Language", "pl-PL,pl;q=0.6".parse().unwrap());
    headers.insert("Connection", "keep-alive".parse().unwrap());
    headers.insert("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap());
    headers.insert(header::COOKIE, "ActListPageSize=10; timezoneOffset=3600,0; Steam_Language=english; browserid=2834483470340916546; steamCurrencyId=6; recentlyVisitedAppHubs=730; strInventoryLastContext=730_2; sessionid=d85c6c67e407f87cd7c2a3ea; webTradeEligibility=%7B%22allowed%22%3A1%2C%22allowed_at_time%22%3A0%2C%22steamguard_required_days%22%3A15%2C%22new_device_cooldown_days%22%3A0%2C%22time_checked%22%3A1676800280%7D; app_impressions=730@2_100100_100101_100102; steamCountry=PL%7Cfa11d370fcc12db9b571c58a3563a4e9; steamLoginSecure=76561198183864150%7C%7CeyAidHlwIjogIkpXVCIsICJhbGciOiAiRWREU0EiIH0.eyAiaXNzIjogInI6MENGOV8yMjFDQUY1RF80QUQ4MCIsICJzdWIiOiAiNzY1NjExOTgxODM4NjQxNTAiLCAiYXVkIjogWyAid2ViIiBdLCAiZXhwIjogMTY3Njk2MzMxNywgIm5iZiI6IDE2NjgyMzY5MTIsICJpYXQiOiAxNjc2ODc2OTEyLCAianRpIjogIjBDRjFfMjIxQzcwOTZfMjA2MzEiLCAib2F0IjogMTY3Njg3NjkxMiwgInJ0X2V4cCI6IDE2OTUwNTIwNjgsICJwZXIiOiAwLCAiaXBfc3ViamVjdCI6ICI0NS4xNTguMTA5LjE5MSIsICJpcF9jb25maXJtZXIiOiAiNDUuMTU4LjEwOS4xOTEiIH0.x3p_Z_Gm8SUH0dQtQKXsMS1Vr-ndRCrtm8iFzw9s9RMyw9oL7L2xyOoBwmX81h8dQpqHvrXNPsi6rHhTRW2cBQ; tsTradeOffersLastRead=1657996922".parse().unwrap());
    headers.insert("Origin", "https://steamcommunity.com".parse().unwrap());
    headers.insert("Referer", "https://steamcommunity.com/market/listings/730/Tec-9%20%7C%20Army%20Mesh%20(Field-Tested)".parse().unwrap());
    headers.insert("Sec-Fetch-Dest", "empty".parse().unwrap());
    headers.insert("Sec-Fetch-Mode", "cors".parse().unwrap());
    headers.insert("Sec-Fetch-Site", "same-origin".parse().unwrap());
    headers.insert("Sec-GPC", "1".parse().unwrap());
    headers.insert("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("sec-ch-ua", "\"Chromium\";v=\"110\", \"Not A(Brand\";v=\"24\", \"Brave\";v=\"110\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Linux\"".parse().unwrap());

    let client = reqwest::blocking::Client::new();
    let res = client.post("https://steamcommunity.com/market/buylisting/4249745875846429894")
        .headers(headers)
        .body("sessionid=d85c6c67e407f87cd7c2a3ea&currency=6&subtotal=7&fee=2&total=9&quantity=1&first_name=Ziemowit&last_name=Maciejewski&billing_address=Warszawa&billing_address_two=Lanowa&billing_country=PL&billing_city=Poland&billing_state=&billing_postal_code=03-107&save_my_address=1")
        .send().unwrap()
        .text().unwrap();
    println!("{}", res);

}

// curl 'https://steamcommunity.com/market/buylisting/4251997675657036556' \
//   -H 'Accept: */*' \
//   -H 'Accept-Language: pl-PL,pl;q=0.6' \
//   -H 'Connection: keep-alive' \
//   -H 'Content-Type: application/x-www-form-urlencoded; charset=UTF-8' \
//   -H 'Cookie: ActListPageSize=10; timezoneOffset=3600,0; Steam_Language=english; browserid=2834483470340916546; steamCurrencyId=6; recentlyVisitedAppHubs=730; strInventoryLastContext=730_2; sessionid=d85c6c67e407f87cd7c2a3ea; webTradeEligibility=%7B%22allowed%22%3A1%2C%22allowed_at_time%22%3A0%2C%22steamguard_required_days%22%3A15%2C%22new_device_cooldown_days%22%3A0%2C%22time_checked%22%3A1676800280%7D; app_impressions=730@2_100100_100101_100102; steamCountry=PL%7Cfa11d370fcc12db9b571c58a3563a4e9; steamLoginSecure=76561198183864150%7C%7CeyAidHlwIjogIkpXVCIsICJhbGciOiAiRWREU0EiIH0.eyAiaXNzIjogInI6MENGOV8yMjFDQUY1RF80QUQ4MCIsICJzdWIiOiAiNzY1NjExOTgxODM4NjQxNTAiLCAiYXVkIjogWyAid2ViIiBdLCAiZXhwIjogMTY3Njk2MzMxNywgIm5iZiI6IDE2NjgyMzY5MTIsICJpYXQiOiAxNjc2ODc2OTEyLCAianRpIjogIjBDRjFfMjIxQzcwOTZfMjA2MzEiLCAib2F0IjogMTY3Njg3NjkxMiwgInJ0X2V4cCI6IDE2OTUwNTIwNjgsICJwZXIiOiAwLCAiaXBfc3ViamVjdCI6ICI0NS4xNTguMTA5LjE5MSIsICJpcF9jb25maXJtZXIiOiAiNDUuMTU4LjEwOS4xOTEiIH0.x3p_Z_Gm8SUH0dQtQKXsMS1Vr-ndRCrtm8iFzw9s9RMyw9oL7L2xyOoBwmX81h8dQpqHvrXNPsi6rHhTRW2cBQ; tsTradeOffersLastRead=1657996922' \
//   -H 'Origin: https://steamcommunity.com' \
//   -H 'Referer: https://steamcommunity.com/market/listings/730/Tec-9%20%7C%20Army%20Mesh%20(Field-Tested)' \
//   -H 'Sec-Fetch-Dest: empty' \
//   -H 'Sec-Fetch-Mode: cors' \
//   -H 'Sec-Fetch-Site: same-origin' \
//   -H 'Sec-GPC: 1' \
//   -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36' \
//   -H 'sec-ch-ua: "Chromium";v="110", "Not A(Brand";v="24", "Brave";v="110"' \
//   -H 'sec-ch-ua-mobile: ?0' \
//   -H 'sec-ch-ua-platform: "Linux"' \
//   --data-raw 'sessionid=d85c77cd7c2a3ea&currency=6&subtotal=7&fee=2&total=9&quantity=1&first_name=Ziemowit&last_name=Maciejewski&billing_address=Warszawa&billing_address_two=Lanowa&billing_country=PL&billing_city=Poland&billing_state=&billing_postal_code=03-107&save_my_address=1' \
//   --compressed
