use github_site::app::App;

#[tokio::main]
async fn main() {
    App:new().await.run().await;
}