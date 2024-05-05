use once_cell::sync::Lazy;
use tokio::time::{self, Duration};
use std::sync::Mutex;
use serde::Deserialize;

pub static EXCHANGE_RATE: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(0.0));

#[derive(Deserialize)]
struct Rates {
	#[serde(rename = "JPY")]
	jpy: f32
}

#[derive(Deserialize)]
struct ExchangeRates {
	rates: Rates
}

pub async fn update_exchange_rate_periodically() {
	let mut interval = time::interval(Duration::from_secs(60 * 120));
	loop {
		interval.tick().await;
		// Anyway, it's a simple and intermittent task, so there's no need to create a RequestBuilder
		match reqwest::get("https://open.er-api.com/v6/latest/VND").await {
			Ok(resp) => {
				match resp.json::<ExchangeRates>().await {
					Ok(exchange_rates_map) => {
						let mut exchange_rate = EXCHANGE_RATE.lock().unwrap();
						*exchange_rate = exchange_rates_map.rates.jpy;
					},
					Err(e) => println!("{:?}", e),
				}
			},
			Err(e) => println!("{:?}", e),
		}
	}
}