use serde::{Deserialize, Serialize};

pub enum TimeZone {
    AmericaAnchorage,
    AmericaLosAngeles,
    AmericaDenver,
    AmericaChicago,
    AmericaNewYork,
    AmericaSaoPaulo,
    GMT0,
    Auto,
    EuropeLondon,
    EuropeBerlin,
    EuropeMoscow,
    AfricaCairo,
    AsiaBangkok,
    AsiaSingapore,
    AsiaTokyo,
    AustraliaSydney,
    PacificAuckland
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWeather {
    pub temperature: f32,
    pub windspeed: f32,
    pub winddirection: f32,
    pub weathercode: f32,
    pub is_day: f32,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HourlyUnits {
    pub time: String,
    pub temperature_2m: String,
    pub relativehumidity_2m: String,
    pub dewpoint_2m:	String,
    pub apparent_temperature: String,
    pub precipitation_probability: String,
    pub precipitation: String,	
    pub rain: String,	
    pub showers: String,	
    pub snowfall: String,	
    pub snow_depth: String,	
    pub weathercode: String,	
    pub pressure_msl: String,	
    pub surface_pressure: String,
    pub cloudcover: String,	
    pub cloudcover_low: String,	
    pub cloudcover_mid: String,
    pub cloudcover_high: String,	
    pub visibility: String,	
    pub evapotranspiration: String,	
    pub et0_fao_evapotranspiration: String,
    pub vapor_pressure_deficit: String,
    pub windspeed_10m: String,
    pub windspeed_80m: String,
    pub windspeed_120m: String,
    pub windspeed_180m: String,
    pub winddirection_10m: String,
    pub winddirection_80m: String,
    pub winddirection_120m: String,
    pub winddirection_180m: String,
    pub windgusts_10m: String,
    pub temperature_80m: String,
    pub temperature_120m: String,
    pub temperature_180m: String,
    pub soil_temperature_0cm: String,
    pub soil_temperature_6cm: String,
    pub soil_temperature_18cm: String,
    pub soil_temperature_54cm: String,
    pub soil_moisture_0_1cm: String,
    pub soil_moisture_1_3cm: String,
    pub soil_moisture_3_9cm: String,
    pub soil_moisture_9_27cm: String,
    pub soil_moisture_27_81cm: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hourly {
    pub time: Vec<String>,
    pub temperature_2m: Vec<Option<f32>>,
    pub relativehumidity_2m: Vec<Option<f32>>,
    pub dewpoint_2m: Vec<Option<f32>>,
    pub apparent_temperature: Vec<Option<f32>>,
    pub precipitation_probability: Vec<Option<f32>>,
    pub precipitation: Vec<Option<f32>>,
    pub rain: Vec<Option<f32>>,	
    pub showers: Vec<Option<f32>>,	
    pub snowfall: Vec<Option<f32>>,	
    pub snow_depth: Vec<Option<f32>>,
    pub weathercode: Vec<Option<f32>>,
    pub pressure_msl: Vec<Option<f32>>,	
    pub surface_pressure: Vec<Option<f32>>,
    pub cloudcover: Vec<Option<f32>>,
    pub cloudcover_low: Vec<Option<f32>>,
    pub cloudcover_mid: Vec<Option<f32>>,
    pub cloudcover_high: Vec<Option<f32>>,
    pub visibility: Vec<Option<f32>>,	
    pub evapotranspiration: Vec<Option<f32>>,
    pub et0_fao_evapotranspiration: Vec<Option<f32>>,
    pub vapor_pressure_deficit: Vec<Option<f32>>, 
    pub windspeed_10m: Vec<Option<f32>>,
    pub windspeed_80m: Vec<Option<f32>>,
    pub windspeed_120m: Vec<Option<f32>>,
    pub windspeed_180m: Vec<Option<f32>>,
    pub winddirection_10m: Vec<Option<f32>>,
    pub winddirection_80m: Vec<Option<f32>>,
    pub winddirection_120m: Vec<Option<f32>>,
    pub winddirection_180m: Vec<Option<f32>>,
    pub windgusts_10m: Vec<Option<f32>>,
    pub temperature_80m: Vec<Option<f32>>,
    pub temperature_120m: Vec<Option<f32>>,
    pub temperature_180m: Vec<Option<f32>>,
    pub soil_temperature_0cm: Vec<Option<f32>>,
    pub soil_temperature_6cm: Vec<Option<f32>>,
    pub soil_temperature_18cm: Vec<Option<f32>>,
    pub soil_temperature_54cm: Vec<Option<f32>>,
    pub soil_moisture_0_1cm: Vec<Option<f32>>,
    pub soil_moisture_1_3cm: Vec<Option<f32>>,
    pub soil_moisture_3_9cm: Vec<Option<f32>>,
    pub soil_moisture_9_27cm: Vec<Option<f32>>,
    pub soil_moisture_27_81cm: Vec<Option<f32>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyUnits {
    pub time: String,
    pub weathercode: String,
    pub temperature_2m_max: String,
    pub temperature_2m_min: String,
    pub apparent_temperature_max: String,
    pub apparent_temperature_min: String,
    pub sunrise: String,
    pub sunset: String,
    pub uv_index_max: String,
    pub uv_index_clear_sky_max: String,
    pub precipitation_sum: String,
    pub rain_sum: String,
    pub showers_sum: String,
    pub snowfall_sum: String,
    pub precipitation_hours: String,
    pub precipitation_probability_max: String,
    pub windspeed_10m_max: String,
    pub windgusts_10m_max: String,
    pub winddirection_10m_dominant: String,
    pub shortwave_radiation_sum: String,
    pub et0_fao_evapotranspiration: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Daily {
    pub time: Vec<String>,
    pub weathercode: Vec<Option<f32>>,
    pub temperature_2m_max: Vec<Option<f32>>,
    pub temperature_2m_min: Vec<Option<f32>>,
    pub apparent_temperature_max: Vec<Option<f32>>,
    pub apparent_temperature_min: Vec<Option<f32>>,
    pub sunrise: Vec<String>,
    pub sunset: Vec<String>,
    pub uv_index_max: Vec<Option<f32>>,
    pub uv_index_clear_sky_max: Vec<Option<f32>>,
    pub precipitation_sum: Vec<Option<f32>>,
    pub rain_sum: Vec<Option<f32>>,
    pub showers_sum: Vec<Option<f32>>,
    pub snowfall_sum: Vec<Option<f32>>,
    pub precipitation_hours: Vec<Option<f32>>,
    pub precipitation_probability_max: Vec<Option<f32>>,
    pub windspeed_10m_max: Vec<Option<f32>>,
    pub windgusts_10m_max: Vec<Option<f32>>,
    pub winddirection_10m_dominant: Vec<Option<f32>>,
    pub shortwave_radiation_sum: Vec<Option<f32>>,
    pub et0_fao_evapotranspiration: Vec<Option<f32>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenMeteoData {
    pub latitude: f32,
    pub longitude: f32,
    pub generationtime_ms: f32,
    pub utc_offset_seconds: f32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f32,
    pub current_weather: Option<CurrentWeather>,
    pub hourly_units: Option<HourlyUnits>,
    pub hourly: Option<Hourly>,
    pub daily_units: Option<DailyUnits>,
    pub daily: Option<Daily>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenMeteoError {
    pub error: bool,
    pub reason: String
}