use reqwest::header::{HeaderMap, ACCEPT_LANGUAGE, CONNECTION, USER_AGENT};

pub static COMBINED_COLLECTION_NAME: &str = "combined";

pub enum Currency {
    PLN = 6
}

impl From<Currency> for i32 {
    fn from(c: Currency) -> i32 {
        c as i32
    }
}


pub fn get_image_url(phase_key: impl Into<String>) -> String {
    format!(
        "https://community.cloudflare.steamstatic.com/economy/image/{}/62fx62f",
        phase_key.into()
    )
}


pub fn dummy_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT_LANGUAGE, "pl-PL,pl;q=0.5".parse().unwrap());
    headers.insert(CONNECTION, "keep-alive".parse().unwrap());
    headers.insert("Sec-Fetch-Dest", "empty".parse().unwrap());
    headers.insert("Sec-Fetch-Mode", "cors".parse().unwrap());
    headers.insert("Sec-Fetch-Site", "same-origin".parse().unwrap());
    headers.insert("Sec-GPC", "1".parse().unwrap());
    headers.insert(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("sec-ch-ua", "\"Chromium\";v=\"110\", \"Not A(Brand\";v=\"24\", \"Brave\";v=\"110\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Linux\"".parse().unwrap());

    headers
}


pub fn buylisting_params(sessionid: &String, subtotal: u64, fee: u64, total: u64) -> Vec<(&'static str, String)> {
    vec![
        // --------------------->>>
        ("sessionid", sessionid.to_string()),
        ("currency", i32::from(Currency::PLN).to_string()),
        // --------------------->>>
        ("subtotal", subtotal.to_string()),
        ("fee", fee.to_string()),
        ("total", total.to_string()),
        // --------------------->>>
        ("quantity", "1".to_string()),
        ("first_name", "Ziemowit".to_string()),
        ("last_name", "Maciejewski".to_string()),
        ("billing_address", "Warszawa".to_string()),
        ("billing_address_two", "Lanolin".to_string()),
        ("billing_country", "PL".to_string()),
        ("billing_city", "Poland".to_string()),
        ("billing_state", "".to_string()),
        ("billing_postal_code", "03-107".to_string()),
        ("save_my_address", "1".to_string()),
    ]
}

