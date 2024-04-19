use poise::command;

use crate::utils::{
	card::Card,
	types::{Context, Result},
	search::{card_by_name, card_by_konami_id, card_by_password, card_by_set_number},
	embed::embeds_from_card_prices,
	paginator::paginate
};

/// find the prices of a card by its name
#[command(
	slash_command,
	prefix_command,
	track_edits,
	aliases("cp")
)]
pub async fn prices_by_name(
	ctx: Context<'_>,
	#[rest]
	#[description = "for example: Sky Striker Ace - Raye"]
	name: String,
) -> Result<()> {
	ctx.defer().await?;
	respond(&ctx, card_by_name(&ctx, &name).await, &name).await?;
	
	Ok(())
}

/// find the prices of a card by its database id
#[command(
	slash_command,
	prefix_command,
	track_edits,
	aliases("cpi")
)]
pub async fn prices_by_database_id(
	ctx: Context<'_>,
	#[description = "for example: 13670"]
	database_id: i32,
) -> Result<()> {
	ctx.defer().await?;
	respond(&ctx, card_by_konami_id(&ctx, &database_id).await, &database_id.to_string()).await?;
	
	Ok(())
}

/// find the prices of a card by its password
#[command(
	slash_command,
	prefix_command,
	track_edits,
	aliases("cpp")
)]
pub async fn prices_by_password(
	ctx: Context<'_>,
	#[description = "for example: 26077387"]
	password: i32,
) -> Result<()> {
	ctx.defer().await?;
	respond(&ctx, card_by_password(&ctx, &password).await, &password.to_string()).await?;
	
	Ok(())
}

/// find the prices of a set number
#[command(
	slash_command,
	prefix_command,
	track_edits,
	aliases("pp")
)]
pub async fn prices_by_set_number(
	ctx: Context<'_>,
	#[description = "for example: 20CP-JPC02"]
	set_number: String,
) -> Result<()> {
	ctx.defer().await?;
	respond(&ctx, card_by_set_number(&ctx, &set_number).await, &set_number).await?;
	
	Ok(())
}

async fn respond(
	ctx: &Context<'_>,
	card_result: Result<Option<Card>>,
	identifier: &str,
) -> Result<()> {
	if let Some(card) = card_result? {
		if !card.card_prices.is_empty() {
			let embeds = embeds_from_card_prices(&card).await;
			paginate(ctx, &embeds).await?;
		} else {
			ctx.say(format!("Oops! `{}` is not in stock.", card.name.en)).await?;
		}
	} else {
		ctx.say(format!("Could not find a card matching `{}`!", identifier)).await?;
	}

	Ok(())
}