use std::collections::HashMap;
use poise::serenity_prelude as serenity;
use serenity::{
	CreateEmbed,
	CreateEmbedFooter,
	CreateEmbedAuthor,
	Colour,
	model::Timestamp
};

use crate::models::{CardPrice, Card};
use crate::utils::global::EXCHANGE_RATE;

pub type EmbedsMap = HashMap<String, Vec<CreateEmbed>>;

pub fn create_embeds_map(card: &Card) -> EmbedsMap {
	card.card_prices.iter()
		.map(|(k, prices)| (k.clone(), create_embeds_for_card_prices(card, prices, k)))
		.collect()
}

fn create_embeds_for_card_prices(card: &Card, card_prices: &[CardPrice], market: &str) -> Vec<CreateEmbed> {
	let exchange_rate: f32 = *EXCHANGE_RATE.lock().unwrap();
	card_prices
		.chunks(6)
		.map(|chunk| {
			let fields = chunk.iter().map(|card_price| {
				(get_name(card_price), get_value(card_price, exchange_rate, market), true)
			}).collect::<Vec<_>>();

			create_base_embed(card, exchange_rate, market).fields(fields)
		})
		.collect()
}

fn create_base_embed(card: &Card, exchange_rate: f32, market: &str) -> CreateEmbed {
	let mut embed = CreateEmbed::new()
		.author(CreateEmbedAuthor::new(market.to_uppercase()))
		.title(&card.name.en)
		.thumbnail(format!("https://images.ygoprodeck.com/images/cards_cropped/{}.jpg", &card.password))
		.color(Colour::from_rgb(238, 190, 184))
		.timestamp(Timestamp::now())
		.url(format!("https://yugipedia.com/wiki/{}", &card.konami_id));

	if market == "tcg_corner" {
		embed = embed.footer(CreateEmbedFooter::new(format!("1 VND = {} JPY", exchange_rate)));
	}

	embed
}

fn get_name(card_price: &CardPrice) -> String {
	format!("{} ({})", card_price.set_number, card_price.rarity)
}

fn get_value(card_price: &CardPrice, exchange_rate: f32, market: &str) -> String {
	let vnd: i32 = if market == "tcg_corner" {
		(card_price.price as f32 / exchange_rate) as i32
	}
	else {
		card_price.price * 180
	};
	let jpy: i32 = if market == "tcg_corner" {
		((card_price.price as f32 / exchange_rate) * exchange_rate) as i32
	}
	else {
		card_price.price
	};

	format!(
		"JPY: {}\nVND: {}\nCondition: {}\nStatus: {}\nLast modified: <t:{}:R>",
		jpy,
		format_vnd(vnd),
		card_price.condition,
		card_price.status,
		card_price.last_modified
	)
}

fn format_vnd(vnd: i32) -> String {
	vnd.abs()
		.to_string()
		.chars()
		.rev()
		.collect::<Vec<_>>()
		.chunks(3)
		.map(|chunk| chunk.iter().collect::<String>())
		.collect::<Vec<_>>()
		.join(",")
		.chars()
		.rev()
		.collect()
}