use mongodb::{bson::{doc, from_document, Document}};
use futures::stream::TryStreamExt;

use crate::utils::card::Card;
use crate::utils::types::{Context, Result};


pub async fn card_by_name(ctx: &Context<'_>, query: &str) -> Result<Option<Card>> {
	get_card(ctx, FilterHolder::with_filters(vec![create_index_filter("en_name_search", query, "name.en")])).await
}

pub async fn card_by_konami_id(ctx: &Context<'_>, konami_id: &i32) -> Result<Option<Card>> {
	get_card(ctx, FilterHolder::with_filter(doc! { "konami_id": konami_id })).await
}

pub async fn card_by_password(ctx: &Context<'_>, password: &i32) -> Result<Option<Card>> {
	get_card(ctx, FilterHolder::with_filter(doc! { "password": password })).await
}

pub async fn card_by_set_number(ctx: &Context<'_>, set_number: &str) -> Result<Option<Card>> {
	let filters = vec![
		doc! { "$match": { "card_prices.set_number": { "$regex": set_number, "$options": "i" } } },
		doc! { "$addFields": {
			"card_prices": {
				"$filter": {
					"input": "$card_prices",
					"as": "item",
					"cond": {
						"$regexMatch": {
							"input": "$$item.set_number",
							"regex": set_number,
							"options": "i"
						}
					}
				}
			}
		} },
		doc! { "$project": { "_id": 0 } }
	];
	
	get_card(ctx, FilterHolder::with_filters(filters)).await
}

async fn get_card(ctx: &Context<'_>, filter_holder: FilterHolder) -> Result<Option<Card>> {
	let data = &ctx.data();
	let coll = &data.coll;
	
	let mut card = None;
	
	if let Some(filters) = filter_holder.filters {
		let mut cursor = coll.aggregate(filters, None).await?;
		if let Some(doc) = cursor.try_next().await? {
			card = Some(from_document::<Card>(doc)?);
		}
	}
	else if let Some(filter) = filter_holder.filter {
		card = coll.find_one(filter, None).await?;
	}
	
	if let Some(ref mut unsorted_card) = card {
		unsorted_card.card_prices.sort_unstable_by(|a, b| a.price.cmp(&b.price));
	}
	
	Ok(card)
}

fn create_index_filter(index: &str, query: &str, path: &str) -> Document {
	doc! {
		"$search": {
			"index": index,
			"text": {
				"query": query,
				"path": path,
				"fuzzy": {
					"maxEdits": 1
				}
			}
		}
	}
}

struct FilterHolder {
	filter: Option<Document>,
	filters: Option<Vec<Document>>
}

impl FilterHolder {
	fn with_filter(filter: Document) -> Self {
		Self { filter: Some(filter), filters: None }
	}
	
	fn with_filters(filters: Vec<Document>) -> Self {
		Self { filter: None, filters: Some(filters) }
	}
}