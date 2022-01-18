use youdao::{fanyi};


#[tokio::main]
async fn main() {
    fanyi().await.expect("");
}
