use mongodb::bson::{doc, from_document};
use futures::stream::TryStreamExt;

use crate::models::Card;
use crate::utils::types::{Context, Result};
use crate::utils::query::QueryHolder;

pub async fn find_card_by_name(ctx: &Context<'_>, query: &str) -> Result<Option<Card>> {
	get_card(ctx, QueryHolder::create_index_pipeline("name_search", query, "name.en")).await
}

pub async fn find_card_by_konami_id(ctx: &Context<'_>, konami_id: i32) -> Result<Option<Card>> {
	get_card(ctx, QueryHolder::Filter(doc! { "konami_id": konami_id })).await
}

pub async fn find_card_by_password(ctx: &Context<'_>, password: i32) -> Result<Option<Card>> {
	get_card(ctx, QueryHolder::Filter(doc! { "password": password })).await
}

pub async fn find_card_by_set_number(ctx: &Context<'_>, set_number: &str) -> Result<Option<Card>> {
	get_card(ctx, QueryHolder::create_set_number_pipeline(set_number)).await
}

async fn get_card(ctx: &Context<'_>, query: QueryHolder) -> Result<Option<Card>> {
	let card_collection = &ctx.data().card_collection;

	let mut card = match query {
		QueryHolder::Pipeline(pipeline) => {
			card_collection.aggregate(pipeline).await?
				.try_next().await?
				.map(from_document::<Card>)
				.transpose()?
		},
		QueryHolder::Filter(filter) => card_collection.find_one(filter).await?,
	};

	if let Some(ref mut card) = card {
		for card_prices in card.card_prices.values_mut() {
			card_prices.sort_unstable_by(|a, b| a.price.cmp(&b.price));
		}
	}

	Ok(card)
}