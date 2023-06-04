use core::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _keys = ["key", "key", "key"];
    let client = reqwest::Client::builder().build()?;

    for i in 0..20 {
        let client = client.clone();
        tokio::spawn(async move {
            let url = "https://api.ipify.org";
            // moved actual request into inner block...
            match async move {
                let res = client.get(url).send().await?.text().await?;
                // ...returning Result with explicit error type,
                // so that the wholeinner async block is treated as "try"-block...
                Ok::<_, Box<dyn std::error::Error + Send + Sync>>(res)
            }
                .await
            {
                // ...and matching on the result, to have either text or error
                // (here it'll always be error, due to invalid URL)
                Ok(res) => println!("{i} - {:?}", res),
                Err(er) => println!("{}", er),
            }
        });
    }

    // just to wait for responses
    sleep(Duration::from_millis(10000)).await;
    Ok(())
}