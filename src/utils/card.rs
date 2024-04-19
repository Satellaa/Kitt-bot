use serde::{Deserialize, Serialize, Deserializer};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CardPrice {
	pub bigweb_id: i32,
	pub set_number: String,
	pub price: i32,
	pub rarity: String,
	pub condition: String,
	pub is_hidden_price: bool,
	pub last_modified: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CardName {
	#[serde(deserialize_with = "default_on_null_string")]
	pub en: String,
	#[serde(deserialize_with = "default_on_null_string")]
	pub ja: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Set {
	pub set_number: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sets {
	pub ja: Vec<Set>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Card {
	pub name: CardName,
	#[serde(deserialize_with = "default_on_null_i32")]
	pub password: i32,
	pub konami_id: i32,
	pub sets: Sets,
	pub card_prices: Vec<CardPrice>
}

fn default_on_null_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
	D: Deserializer<'de>,
{
	Ok(Deserialize::deserialize(deserializer).unwrap_or(0))
}

fn default_on_null_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
	D: Deserializer<'de>,
{
	Ok(Deserialize::deserialize(deserializer).unwrap_or(String::new()))
}