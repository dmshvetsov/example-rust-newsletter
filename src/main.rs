use newsletter_backend_example::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
