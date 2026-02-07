use moldudp64_engine::MOLDPRODUCER;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mold = MOLDPRODUCER::initialize("127.0.0.1:8080").await?;

    loop {
        mold.produce("TEST".as_bytes(), "127.0.0.1:8081".parse().unwrap())
            .await?;
    }
}
