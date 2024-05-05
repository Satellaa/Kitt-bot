use poise::serenity_prelude as serenity;
use serenity::{
	ComponentInteraction,
	CreateEmbed,
	CreateInteractionResponse,
	CreateInteractionResponseMessage,
	Colour
};

use crate::utils::types::Result;


pub async fn cannot_control(
	ctx: &serenity::Context,
	interaction: &ComponentInteraction,
	content: &str
) -> Result<()> {
	interaction.create_response(ctx,
		CreateInteractionResponse::Message(
		CreateInteractionResponseMessage::new()
			.embed(CreateEmbed::new()
				.description(content)
				.color(Colour::RED)
			)
			.ephemeral(true)
		)
	).await?;
	
	Ok(())
}