use poise::builtins::{HelpConfiguration, help as poise_help};

use crate::utils::types::{Context, Result};

/// Show this menu
#[poise::command(
	slash_command,
	prefix_command,
	track_edits
)]
pub async fn help(
	ctx: Context<'_>,
	#[description = "Specific command to show help about"] command: Option<String>,
) -> Result<()> {
	let config = HelpConfiguration {
		extra_text_at_bottom: "\
Type $help command for more info on a command.
You can edit your message to the bot and the bot will edit its response.",
		..Default::default()
	};
	poise_help(ctx, command.as_deref(), config).await?;
	Ok(())
}