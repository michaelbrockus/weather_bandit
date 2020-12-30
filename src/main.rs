//
// file: main.rs
// author: Michael Brockus
// gmail: <michaelbrockus@gmail.com>
//
use tokio;
use std::env;
use dotenv::dotenv;
use openweather_async::{ OpenWeather, Units};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("No env file found");
    let token = env::var("OPENWEATHER_API_KEY").unwrap();
    let weather: OpenWeather = OpenWeather::new(&token, Units::Metric).await?;
    let report = weather.get_by_city("Tokyo").await?;
    println!("{:?}\n", report);
    println!("{:?}\n", report.main);
    println!("{:?}\n", report.wind.speed);
   Ok(())
}