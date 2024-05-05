use mongodb::bson::{doc, from_document};
use futures::stream::TryStreamExt;

use crate::utils::{
	card::Card,
	types::{Context, Result},
	query::QueryHolder
};

pub async fn card_by_name(ctx: &Context<'_>, query: &str) -> Result<Option<Card>> {
	get_card(ctx, QueryHolder::create_index_pipeline("name_search", query, "name.en")).await
}

pub async fn card_by_konami_id(ctx: &Context<'_>, konami_id: &i32) -> Result<Option<Card>> {
	get_card(ctx, QueryHolder::Filter(doc! { "konami_id": konami_id })).await
}

pub async fn card_by_password(ctx: &Context<'_>, password: &i32) -> Result<Option<Card>> {
	get_card(ctx, QueryHolder::Filter(doc! { "password": password })).await
}

pub async fn card_by_set_number(ctx: &Context<'_>, set_number: &str) -> Result<Option<Card>> {
	get_card(ctx, QueryHolder::create_set_number_pipeline(set_number)).await
}

async fn get_card(ctx: &Context<'_>, query: QueryHolder) -> Result<Option<Card>> {
	let data = &ctx.data();
	let coll = &data.coll;
	
	let mut card = None;
	
	if let QueryHolder::Pipeline(pipeline) = query {
		let mut cursor = coll.aggregate(pipeline, None).await?;
		if let Some(doc) = cursor.try_next().await? {
			card = Some(from_document::<Card>(doc)?);
		}
	}
	else if let QueryHolder::Filter(filter) = query {
		card = coll.find_one(filter, None).await?;
	}
	
	if let Some(ref mut unsorted_card) = card {
		for (_, card_prices) in &mut unsorted_card.card_prices {
			card_prices.sort_unstable_by(|a, b| a.price.cmp(&b.price));
		}
	}
	
	Ok(card)
}