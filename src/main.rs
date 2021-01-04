// file: main.rs
// author: Michael Brockus
// gmail: <michaelbrockus@gmail.com>

use weather_bandit::foundation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    foundation().await?;
    Ok(())
}
