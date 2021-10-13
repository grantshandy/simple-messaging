use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    let mut req = reqwest::get("http://localhost:8080/events")
        .await
        .unwrap()
        .bytes_stream();

    while let Some(item) = req.next().await {
        let item = item.unwrap();
        
        let string = String::from_utf8(item.to_vec()).unwrap();

        println!("{}", string);
    }
}
