use tokio::task::JoinHandle;

async fn async_fn(knife_name: &str) {
    println!("knife_name: {}", knife_name);
}

fn main() {
    test("");
}

fn test(knife_name: &str){
    // let knife_name = "chef_knife";
    let mut tasks: Vec<JoinHandle<()>> = Vec::new();

    for _ in 0..5 {
        let name = knife_name.to_string();
        let task = tokio::spawn(async move {
            async_fn(name.as_str()).await;
        });
        tasks.push(task);
    }

    let x = 324;
    let y = x.to_owned();

    // Wait for all tasks to complete
}
