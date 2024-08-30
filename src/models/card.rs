use serde::{Deserialize, Serialize, Deserializer};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CardName {
	#[serde(deserialize_with = "deserialize_default_string")]
	pub en: String,
	#[serde(deserialize_with = "deserialize_default_string")]
	pub ja: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Set {
	pub set_number: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CardPrice {
	pub id: i32,
	pub set_number: String,
	pub price: i32,
	pub rarity: String,
	pub condition: String,
	pub status: String,
	pub last_modified: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Card {
	pub name: CardName,
	#[serde(deserialize_with = "deserialize_default_i32")]
	pub password: i32,
	pub konami_id: i32,
	pub sets: HashMap<String, Vec<Set>>,
	pub card_prices: HashMap<String, Vec<CardPrice>>
}

fn deserialize_default_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
	D: Deserializer<'de>,
{
	Option::deserialize(deserializer)
		.map(|opt_value| opt_value.unwrap_or(0))
}

fn deserialize_default_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
	D: Deserializer<'de>,
{
	Option::deserialize(deserializer)
		.map(|opt_value| opt_value.unwrap_or_default())
}