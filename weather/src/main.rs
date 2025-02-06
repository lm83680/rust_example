use clap::Parser;
use reqwest::Error;
use serde::Deserialize;
use serde_json::{from_value, Value};
use tokio;

#[derive(Parser, Debug)]
#[command(version, about = "查询指定城市的当日天气信息")]
struct Args {
    /// 要查询的城市名称
    #[arg(index = 1)]
    city: String,

    /// 你的ApiKey
    #[arg(index = 2)]
    api_key: String,

    /// 国家代码
    #[arg(short, default_value = "")]
    country_code: String,
}

#[derive(Deserialize)]
struct WeatherResponse {
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize)]
struct Main {
    temp: f64,
    humidity: f64,
}

#[derive(Deserialize)]
struct Wind {
    speed: f64,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Args = Args::parse();
    let city = &args.city;
    let api_key = &args.api_key;
    let country_code = &args.country_code;

    // 获取城市的地理坐标
    let geo_url = format!(
        "https://api.openweathermap.org/geo/1.0/direct?q={},,{}&limit=1&appid={}",
        city, country_code, api_key
    );
    let geo_response: Vec<GeoResponse> = reqwest::get(&geo_url).await?.json().await?;

    if geo_response.is_empty() {
        eprintln!("未找到城市：{}", city);
        return Ok(());
    }

    let lat = geo_response[0].lat;
    let lon = geo_response[0].lon;

    // 查询天气信息
    let weather_url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&lang=zh_cn&units=metric",
        lat, lon, api_key
    );
    let weather_result: Value = reqwest::get(&weather_url).await?.json().await?;
     // 从 Value 中解析 WeatherResponse from_value具有拷贝到作用所以他会影响原数据，所以需要使用.clone提供一份新的数据。
     let weather_response: WeatherResponse = from_value(weather_result.clone()).expect("Failed to parse WeatherResponse");
     
    // 输出天气信息
    println!("搜索地：{}", city);
    println!("位置坐标：{},{}", lat, lon);
    println!("位置名称：{}", weather_response.name);
    println!("温度：{}°C", weather_response.main.temp);
    println!("湿度：{}%", weather_response.main.humidity);
    println!("风速：{} m/s", weather_response.wind.speed);

    // 格式化并打印天气信息
    let pretty_weather = serde_json::to_string_pretty(&weather_result).unwrap();

    println!("元数据");
    println!("{}", pretty_weather);
    Ok(())
}

#[derive(Deserialize)]
struct GeoResponse {
    lat: f64,
    lon: f64,
}
