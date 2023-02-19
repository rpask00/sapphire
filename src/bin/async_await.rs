use std::time::Duration;
use tokio::time::sleep;
#[tokio::main]
async fn main() {
    let mut handles = vec![];

    for i in 0..4 {

        let handle = tokio::spawn(async move {
            my_fun(i).await;
        });

        println!("[{i}] after spawn");


        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn my_fun(i: i32) {
    println!("[{i}] im an async func");
    let s1 = read_from_db().await;
    println!("[{i}] first result: {}", s1);
    let s2 = read_from_db().await;
    println!("[{i}] second result: {}", s2);
}

async fn read_from_db() -> String {
    sleep(Duration::from_secs(5)).await;

    "DB result".to_owned()
}
