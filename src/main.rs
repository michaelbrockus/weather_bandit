//
// file: main.rs
// author: Michael Brockus
// gmail: <michaelbrockus@gmail.com>
//
extern crate program;
use program::foundation;
use tokio;

// main is where program execution starts
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    foundation().await?;
    Ok(())
}
