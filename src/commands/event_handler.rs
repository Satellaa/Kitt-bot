use poise::FrameworkContext;
use poise::serenity_prelude as serenity;
use serenity::{
	FullEvent,
	Context,
	Message,
	CreateMessage,
	CreateEmbed,
	CreateEmbedAuthor,
	Colour,
	cache::Cache,
	gateway::ActivityData,
	model::user::OnlineStatus
};

use crate::utils::{
	types::{Result, Error, Data}
};

pub async fn event_handler(
	ctx: &Context,
	event: &FullEvent,
	_framework: FrameworkContext<'_, Data, Error>,
	_data: &Data,
) -> Result<()> {
	match event {
		FullEvent::Ready { data_about_bot } => {
			println!("Logged in as {}", data_about_bot.user.name);
			
			let activity = ActivityData::playing("/help");
			let status = OnlineStatus::Online;
			
			ctx.set_presence(Some(activity), status);
		}
		FullEvent::Message { new_message } => {
			if new_message.content.contains(&ctx.cache.as_ref().current_user().id.get().to_string()) && !is_own(ctx, &new_message)
				&& new_message.mentions_me(ctx).await?
			{
				send_info(ctx, new_message).await?;
			}
		}
		_ => {},
	}
	
	Ok(())
}

async fn send_info(ctx: &Context, message: &Message) -> Result<()> {
	let desc = "\
❓ Help documentation on [GitHub](https://github.com/Satellaa/Smol-Lilac-bot), or use `/help`.
💡 Kitt is the character appearing in the artwork of the card [Tri-Brigade Kitt](https://yugipedia.com/wiki/Tri-Brigade_Kitt).
🟢 Licence: [GNU AGPL 3.0+](https://choosealicense.com/licenses/agpl-3.0/).";
	
	let embed_author = CreateEmbedAuthor::new("Kitt")
		.icon_url("https://cdn.discordapp.com/avatars/1082275634757242890/42488ede859a7383ccbaa7e4065a1ead.png")
		.url("https://github.com/Satellaa/Kitt-bot");
	
	let embed = CreateEmbed::new()
		.title("A free _Yu-Gi-Oh!_ bot")
		.description(desc)
		.color(Colour::from_rgb(238, 190, 184))
		.author(embed_author);
		
	message.channel_id.send_message(ctx, CreateMessage::new().add_embed(embed)).await?;
	
	Ok(())
}

fn is_own(cache: impl AsRef<Cache>, message: &Message) -> bool {
	message.author.id == cache.as_ref().current_user().id
}