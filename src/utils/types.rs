use mongodb::Collection;

use crate::utils::card::Card;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
	pub coll: Collection<Card>
}