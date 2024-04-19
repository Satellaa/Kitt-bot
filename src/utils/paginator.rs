use poise::serenity_prelude as serenity;
use poise::reply::CreateReply;
use serenity::{
	collector::ComponentInteractionCollector,
	ComponentInteraction,
	CreateActionRow,
	CreateButton,
	CreateEmbed,
	CreateInteractionResponse,
	CreateInteractionResponseMessage,
	Colour,
	ButtonStyle::Secondary
};

use crate::utils::types::{Context, Result};

pub async fn paginate(ctx: &Context<'_>, pages: &Vec<CreateEmbed>) -> Result<()> {
	let ctx_id = ctx.id();
	let pagination_buttons = PaginationButtons::new(&ctx.id()); 
	let mut components = pagination_buttons.create_action_row(&format!("1/{}", pages.len()));
	let reply = create_reply(pages[0].clone(), &components).await;
	
	ctx.send(reply).await?;
	
	
	let mut current_page = 0;
	while let Some(press) = ComponentInteractionCollector::new(ctx)
		.filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
		.timeout(std::time::Duration::from_secs(86_400))
		.await
	{
		if press.user.id != ctx.author().id {
			press.create_response(ctx.serenity_context(),
				CreateInteractionResponse::Message(
				CreateInteractionResponseMessage::new()
					.embed(CreateEmbed::new()
						.description("You cannot control this pagination!")
						.color(Colour::RED)
					)
					.ephemeral(true)
				)
			).await?;
			break;
		}
		
		if press.data.custom_id == pagination_buttons.next_id {
			current_page = (current_page + 1) % pages.len();
		}
		else if press.data.custom_id == pagination_buttons.prev_id {
			current_page = current_page.checked_sub(1).unwrap_or(pages.len() - 1);
		}
		else {
			continue;
		}
		
		update_page_count(&mut components, current_page + 1, pages.len(), &pagination_buttons).await;
        update_message(ctx.serenity_context(), &press, pages[current_page].clone(), &components).await?;
	}
	Ok(())
}

async fn create_reply(page: CreateEmbed, components: &CreateActionRow) -> CreateReply {
	CreateReply::default()
		.embed(page)
		.components(vec![components.clone()])
		.reply(true)
}

async fn update_page_count(
	components: &mut CreateActionRow,
	current: usize,
	total: usize,
	pagination_buttons: &PaginationButtons,
) {
	if let CreateActionRow::Buttons(buttons) = components {
		let page_count = format!("{}/{}", current, total);
		buttons[1] = CreateButton::new(&pagination_buttons.page_counter_id)
			.label(page_count)
			.style(Secondary)
			.disabled(true);
	}
}

async fn update_message(
	ctx: &serenity::Context,
	press: &ComponentInteraction,
	page: CreateEmbed,
	components: &CreateActionRow,
) -> Result<()> {
	press
		.create_response(
			ctx,
			CreateInteractionResponse::UpdateMessage(
			CreateInteractionResponseMessage::new()
				.embed(page)
				.components(vec![components.clone()]),
			),
		).await?;
	
	Ok(())
}

struct PaginationButtons {
	prev_id: String,
	page_counter_id: String,
	next_id: String,
}

impl PaginationButtons {
	
	fn new(ctx_id: &u64) -> Self {
		Self {
			prev_id: format!("{}prev", ctx_id),
			page_counter_id: format!("{}page_count", ctx_id),
			next_id: format!("{}next", ctx_id),
		}
	}
	
	fn create_action_row(&self, page_count: &str) -> CreateActionRow {
		CreateActionRow::Buttons(vec![
			CreateButton::new(&self.prev_id)
				.emoji('◀')
				.style(Secondary),
			CreateButton::new(&self.page_counter_id)
				.label(page_count)
				.style(Secondary)
				.disabled(true),
			CreateButton::new(&self.next_id)
				.emoji('▶')
				.style(Secondary),
		])
	}
}