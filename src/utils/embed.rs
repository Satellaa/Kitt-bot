use poise::serenity_prelude as serenity;
use serenity::{
	CreateEmbed,
	Colour,
	model::Timestamp
};

use crate::utils::card::{CardPrice, Card};


pub async fn embeds_from_card_prices(card: &Card) -> Vec<CreateEmbed> {
	let mut embeds: Vec::<CreateEmbed> = Vec::new();
	let mut fields: Vec<(String, String, bool)> = Vec::with_capacity(6);
	let mut embed = create_base_embed(&card);
	
	for (i, card_price) in card.card_prices.iter().enumerate() {
		fields.push((get_name(&card_price), get_value(&card_price), true));
		
		if (i + 1) % 6 == 0 {
			embed = embed.fields(fields.clone());
			embeds.push(embed);
			embed = create_base_embed(&card);
			
			fields.clear();
		}
	}
	
	if fields.len() > 0 {
		embed = embed.fields(fields);
		embeds.push(embed);
	}
	
	embeds
}

fn create_base_embed(card: &Card) -> CreateEmbed {
	CreateEmbed::new()
		.title(&card.name.en)
		.thumbnail(format!("https://images.ygoprodeck.com/images/cards_cropped/{}.jpg", &card.password))
		.color(Colour::PURPLE)
		.timestamp(Timestamp::now())
		.url(format!("https://yugipedia.com/wiki/{}", &card.konami_id))
}

fn get_name(card_price: &CardPrice) -> String {
	format!("{} ({})",
		card_price.set_number,
		card_price.rarity
	)
}

fn get_value(card_price: &CardPrice) -> String {
	let status: &str = if card_price.is_hidden_price { "Sold Out" } else { "For Sale" };
	
	format!("YEN: {}\nVND: {}\nCondition: {}\nStatus: {}\nLast modified: <t:{}:R>",
		&card_price.price,
		yen_to_vnd(card_price.price),
		card_price.condition,
		status,
		card_price.last_modified
	)
}

fn yen_to_vnd(yen: i32) -> String {
	(yen * 180)
		.abs()
		.to_string()
		.as_bytes()
		.rchunks(3)
		.rev()
		.map(std::str::from_utf8)
		.collect::<Result<Vec<&str>, _>>()
		.unwrap()
		.join(",")
}