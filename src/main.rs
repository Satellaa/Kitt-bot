use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use std::sync::Arc;
use poise::serenity_prelude as serenity;
use anyhow::Context as _;
use mongodb::{Client, options::ClientOptions};

mod utils;
mod components;
mod commands;

use utils::{
	card::Card,
	types::Data,
	global::update_exchange_rate_periodically
};

use commands::{
	prices::{
		prices_name,
		prices_database_id,
		prices_password,
		prices_set_number
	},
	help::help,
	event_handler::event_handler
};

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
	
	
	tokio::spawn(async {
		update_exchange_rate_periodically().await;
	});
	
	let uri = secrets
		.get("ANON")
		.context("'ANON' was not found")?;
	
	let mut client_options = ClientOptions::parse(&uri).await.expect("Bad connection");
	client_options.app_name = Some("Kitt".to_string());
	
	let client = Client::with_options(client_options).expect("Bad connection");
	let database = client.database("data");
	let coll = database.collection::<Card>("cards");
	
	let token = secrets
		.get("DISCORD_TOKEN")
		.context("'DISCORD_TOKEN' was not found")?;
	
	let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
	
	let framework = poise::Framework::builder()
		.options(poise::FrameworkOptions {
			commands: vec![prices_name(), prices_database_id(), prices_password(), prices_set_number(), help()],
			prefix_options: poise::PrefixFrameworkOptions {
				prefix: Some("$".into()),
				edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(std::time::Duration::from_secs(3600)))),
				case_insensitive_commands: true,
				.. Default::default()
			},
			event_handler: |ctx, event, framework, data| {
				Box::pin(event_handler(ctx, event, framework, data))
			},
			..Default::default()
		})
		.setup(|ctx, _ready, framework| {
			Box::pin(async move {
				poise::builtins::register_globally(ctx, &framework.options().commands).await?;
				Ok(Data { coll })
			})
		})
		.build();

	let client = serenity::ClientBuilder::new(&token, intents)
		.framework(framework)
		.await
		.unwrap();
	
	Ok(client.into())
}
