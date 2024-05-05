use poise::command;

use crate::utils::{
	card::Card,
	types::{Context, Result},
	search::{card_by_name, card_by_konami_id, card_by_password, card_by_set_number},
	embed::create_embeds_map,
};

use crate::components::pagination::Pagination;

/// Find the prices of a card by its name.
/// The prefix version of this command is `$cp`.
#[command(
	slash_command,
	prefix_command,
	track_edits,
	aliases("cp"),
	required_bot_permissions = "SEND_MESSAGES | SEND_MESSAGES_IN_THREADS | EMBED_LINKS | READ_MESSAGE_HISTORY"
)]
pub async fn prices_name(
	ctx: Context<'_>,
	#[rest]
	#[description = "for example: Sky Striker Ace - Raye"]
	name: String,
) -> Result<()> {
	ctx.defer().await?;
	respond(&ctx, card_by_name(&ctx, &name).await, &name).await?;
	
	Ok(())
}

/// Find the prices of a card by its database id.
/// The prefix version of this command is `$cpi`.
#[command(
	slash_command,
	prefix_command,
	track_edits,
	aliases("cpi"),
	required_bot_permissions = "SEND_MESSAGES | SEND_MESSAGES_IN_THREADS | EMBED_LINKS | READ_MESSAGE_HISTORY"
)]
pub async fn prices_database_id(
	ctx: Context<'_>,
	#[description = "for example: 13670"]
	database_id: i32,
) -> Result<()> {
	ctx.defer().await?;
	respond(&ctx, card_by_konami_id(&ctx, &database_id).await, &database_id.to_string()).await?;
	
	Ok(())
}

/// Find the prices of a card by its password.
/// The prefix version of this command is `$cpp`.
#[command(
	slash_command,
	prefix_command,
	track_edits,
	aliases("cpp"),
	required_bot_permissions = "SEND_MESSAGES | SEND_MESSAGES_IN_THREADS | EMBED_LINKS | READ_MESSAGE_HISTORY"
)]
pub async fn prices_password(
	ctx: Context<'_>,
	#[description = "for example: 26077387"]
	password: i32,
) -> Result<()> {
	ctx.defer().await?;
	respond(&ctx, card_by_password(&ctx, &password).await, &password.to_string()).await?;
	
	Ok(())
}

/// Find the prices of a set number.
/// The prefix version of this command is `$pp`.
#[command(
	slash_command,
	prefix_command,
	track_edits,
	aliases("pp"),
	required_bot_permissions = "SEND_MESSAGES | SEND_MESSAGES_IN_THREADS | EMBED_LINKS | READ_MESSAGE_HISTORY"
)]
pub async fn prices_set_number(
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
	if let Some(mut card) = card_result? {
		card.card_prices.retain(|_, arr| !arr.is_empty());
		if card.card_prices.is_empty() {
			ctx.say(format!("Oops! `{}` is not in stock.", card.name.en)).await?;
		}
		else {
			let embeds_map = create_embeds_map(&card);
			let mut pagination = Pagination::new(ctx, &embeds_map);
			pagination.start().await?;
		}
	}
	else {
		ctx.say(format!("Could not find a card matching `{}`!", identifier)).await?;
	}

	Ok(())
}