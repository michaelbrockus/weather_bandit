//
// file: program.rs
// author: Michael Brockus
// gmail: <michaelbrockus@gmail.com>
//
use dotenv::dotenv;
use openweather_async::{OpenWeather, Units, Weather};
use std::env;

//
// foundation of the program and related
// application logic must be implemented
// in the foundation.
//
pub async fn show_report(report: Weather) {
    println!("The weather report at {}", report.name);
    println!("Degrees    | {:?}", report.wind.deg);
    println!("Wind Speed | {:?}", report.wind.speed);
}

pub async fn show_help() {
    println!("Usage: weather-bandit <command> <switch>");
    println!("forcast | Show forcast from a city");
    println!("help    | Show this help message\n");
}

//
// foundation of the program and related
// application logic must be implemented
// in the foundation.
//
pub async fn foundation() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("No env file found");
    let args: Vec<String> = env::args().collect();

    let this_command = &args[1];

    match this_command.as_str() {
        "forcast" => {
            let from_city = &args[2];
            let token = env::var("OPENWEATHER_API_KEY").unwrap();
            let weather: OpenWeather = OpenWeather::new(&token, Units::Metric).await?;
            let report: Weather = weather.get_by_city(from_city).await?;
            show_report(report).await;
        }
        "help" => show_help().await,
        _ => println!("No sure what you mean by {}?", this_command),
    }

    Ok(())
}
