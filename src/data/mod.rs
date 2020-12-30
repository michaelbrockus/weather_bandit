
/// Contains the result of an aggregate query.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WeatherAggregate {
    /// Search accuracy. Possible values: "accurate", "like".
    pub message: Option<String>,
    /// HTTP status code for the request
    pub cod: Option<i32>,
    /// Number of items in the list
    pub count: Option<i32>,
    /// List of weather info
    pub list: Option<Vec<WeatherInfo>>,
}

/// Contains the result of a bounding-box query.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WeatherBoxAggregate {
    /// HTTP status code for the request
    pub cod: Option<i32>,
    /// Time elapsed server-side to handle the request
    pub calctime: Option<f32>,
    /// Number of items in the list
    pub cnt: Option<i32>,
    /// List of weather info
    pub list: Option<Vec<WeatherInfo>>,
}

/// Contains all the weather-related information.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WeatherInfo {
    /// City geographic coordinates
    pub coord: Option<Coordinates>,
    /// Weather conditions
    pub weather: Option<Vec<Weather>>,
    /// Internal parameter
    pub base: Option<String>,
    /// General weather parameters
    pub main: Option<Main>,
    /// Wind-related information
    pub wind: Option<Wind>,
    /// Cloud-related information
    pub clouds: Option<Clouds>,
    /// Rain-related information
    pub rain: Option<Rain>,
    /// Snow-related information
    pub snow: Option<Snow>,
    /// Time of data calculation, Unix, UTC
    pub dt: Option<i64>,
    /// Internal parameter
    pub sys: Option<Sys>,
    /// City ID
    pub id: Option<i64>,
    /// City name
    pub name: Option<String>,
    /// Internal parameter
    pub cod: Option<i32>,
}

/// Contains the geographic coordinates of the location.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Coordinates {
    /// Longitude
    pub lon: Option<f32>,
    /// Latitude
    pub lat: Option<f32>,
}

/// Represents OpenWeatherMap's weather condition codes.
/// See http://openweathermap.org/weather-conditions for details.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Weather {
    /// Weather condition ID
    pub id: Option<i32>,
    /// Group of weather parameters
    pub main: Option<String>,
    /// Weather condition within the group
    pub description: Option<String>,
    /// Weather icon ID
    pub icon: Option<String>,
}

/// Contains weather information not tied to particular weather conditions.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Main {
    /// Current temperature. Unit Default: [K], Metric: [°C], Imperial: [°F]
    pub temp: Option<f32>,
    /// Atmospheric pressure [hPa] (at sea level, if there is no sea_level or grnd_level data)
    pub pressure: Option<i32>,
    /// Humidity [%]
    pub humidity: Option<i32>,
    /// Minimum temperature at the moment (deviation from current temperature, significant for large areas)
    pub temp_min: Option<f32>,
    /// Maximum temperature at the moment (deviation from current temperature, significant for large areas)
    pub temp_max: Option<f32>,
    /// Atmospheric pressure on the sea level [hPa]
    pub sea_level: Option<i32>,
    /// Atmospheric pressure on the ground level [hPa]
    pub grnd_level: Option<i32>,
}

/// Contains wind-related information.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Wind {
    /// Wind speed. Unit default: [m/s], Metric: [m/s], Imperial: [miles/h]
    pub speed: Option<f32>,
    /// Wind direction [deg] (meteorological)
    pub deg: Option<i32>,
    /// Wind gust. Same units as speed.AsMut
    pub gust: Option<f32>,
}

/// Contains cloud-related information.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Clouds {
    /// Cloudiness [%]
    pub all: Option<i32>,
}

/// Contains rain-related information.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rain {
    /// Rain volume for the last 3 hours
    #[serde(rename="3h")]
    pub three_hours: Option<i32>,
}

/// Contains snow-related information.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Snow {
    /// Snow volume for the last 3 hours
    #[serde(rename="3h")]
    pub three_hours: Option<i32>,
}

/// Contains internal API parameters.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Sys {
    /// Internal parameter
    #[serde(rename="type")]
    pub type_: Option<i32>,
    /// Internal parameter
    pub id: Option<i32>,
    /// Internal parameter
    pub message: Option<f32>,
    /// Country code
    pub country: Option<String>,
    /// Sunrise time, Unix, UTC
    pub sunrise: Option<i64>,
    /// Sunset time, Unix, UTC
    pub sunset: Option<i64>,
}

/// Represents an error message sent by the API server in response to a bad request.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// HTTP status code for the requested resource
    pub cod: Option<i32>,
    /// Human-readable error message
    pub message: Option<String>,
}
