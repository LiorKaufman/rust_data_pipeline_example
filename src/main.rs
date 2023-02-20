use reqwest::Error;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io;
use std::io::Write;
use url::Url;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "generationtime_ms")]
    pub generationtime_ms: f64,
    #[serde(rename = "utc_offset_seconds")]
    pub utc_offset_seconds: i64,
    pub timezone: String,
    #[serde(rename = "timezone_abbreviation")]
    pub timezone_abbreviation: String,
    pub elevation: f64,
    #[serde(rename = "current_weather")]
    pub current_weather: CurrentWeather,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentWeather {
    pub temperature: f64,
    pub windspeed: f64,
    pub winddirection: f64,
    pub weathercode: i64,
    pub time: String,
}

struct WeatherRequest {
    lat: f32,
    lon: f32,
    currentweather: bool,
}

impl Default for WeatherRequest {
    fn default() -> Self {
        WeatherRequest {
            lat: 52.52,
            lon: 13.41,
            currentweather: true,
        }
    }
}
impl WeatherRequest {
    fn new() -> WeatherRequest {
        let mut lat = String::new();
        let mut lon = String::new();
        let mut currentweather = String::new();

        println!("Enter the latitude");
        io::stdin().read_line(&mut lat).unwrap();
        let lat = match lat.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input for latitude. Defaulting to 0.0");
                0.0
            }
        };

        println!("Enter the longitude: ");
        io::stdin().read_line(&mut lon).unwrap();
        let lon = match lon.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input for longitude. Defaulting to 0.0");
                0.0
            }
        };

        println!("Do you want current weather information? (yes/no)");
        io::stdin().read_line(&mut currentweather).unwrap();
        let currentweather = match currentweather.trim().to_lowercase().as_ref() {
            "yes" => true,
            "no" => false,
            _ => {
                println!("Invalid input for current weather info. Defaulting to false.");
                false
            }
        };

        WeatherRequest {
            lat,
            lon,
            currentweather,
        }
    }
    fn get_url(&self) -> String {
        let base_url = Url::parse("https://api.open-meteo.com/v1/");
        let lat_lon = format!(
            "forecast?latitude={}&longitude={}&current_weather={}",
            self.lat, self.lon, self.currentweather
        );
        let url = base_url.unwrap().join(&lat_lon).unwrap().to_string();
        return url;
    }
    async fn call_the_weather_api(self, url: String) -> Result<WeatherResponse, Error> {
        let res = reqwest::get(url).await?.text().await?;
        let root: WeatherResponse = serde_json::from_str(&res).unwrap();

        println!("{:?}", root);
        Ok(root)
    }
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    let req = WeatherRequest::default();
    let url = req.get_url();
    let weather = req.call_the_weather_api(url.to_string()).await?;
    save_weather_response(&weather);
    Ok(())
}
fn save_weather_response(weather_response: &WeatherResponse) {
    let data = serde_json::to_string(weather_response).unwrap();
    let file = File::create("weather_data.json");
    file.expect("no json file")
        .write_all(data.as_bytes())
        .ok();
}
