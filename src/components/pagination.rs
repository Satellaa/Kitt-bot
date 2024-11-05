use std::collections::BTreeMap;
use poise::serenity_prelude as serenity;
use serenity::{
	collector::ComponentInteractionCollector,
	ComponentInteraction,
	ComponentInteractionDataKind,
	CreateActionRow,
	CreateButton,
	CreateSelectMenu,
	CreateSelectMenuKind,
	CreateSelectMenuOption,
	CreateEmbed,
	CreateInteractionResponse,
	CreateInteractionResponseMessage,
	ButtonStyle,
};

use crate::utils::{
	types::{Context, Result},
	embed::EmbedsMap
};
use crate::components::helper::cannot_control;

const SELECT_MENU_KEY: i32 = 1;
const BUTTONS_KEY: i32 = 2;
type ComponentsMap = BTreeMap<i32, CreateActionRow>;

pub struct Pagination<'a> {
	ctx: &'a Context<'a>,
	owner_name: &'a str,
	card_prices: &'a EmbedsMap,
	component: ComponentIds,
	components: ComponentsMap,
	state: PaginationState,
}

impl<'a> Pagination<'a> {
	pub fn new(ctx: &'a Context<'a>, owner_name: &'a str, card_prices: &'a EmbedsMap) -> Self {
		let ctx_id = ctx.id();
		let component = ComponentIds::new(&ctx_id);
		let components = Self::initialize_components(&component, card_prices);
		
		Self {
			ctx,
			owner_name,
			card_prices,
			component,
			components,
			state: PaginationState::default(),
		}
	}
	
	pub async fn start(&mut self) -> Result<()> {
		let reply = self.create_reply();
		self.ctx.send(reply).await?;
		
		let ctx_id = self.ctx.id();
		while let Some(interaction) = ComponentInteractionCollector::new(self.ctx)
			.filter(move |interaction| interaction.data.custom_id.starts_with(&ctx_id.to_string()))
			.timeout(std::time::Duration::from_secs(86_400))
			.await
		{
			self.handle_interaction(&interaction).await?;
		}
		
		Ok(())
	}
	
	fn initialize_components(component: &ComponentIds, card_prices: &EmbedsMap) -> ComponentsMap {
		let mut components = ComponentsMap::new();
		
		if card_prices.len() > 1 {
			components.insert(SELECT_MENU_KEY, component.create_select_menu_component(card_prices, "Select a vendor"));
		}
		else {
			let mut buttons = component.create_button_component();
			Self::update_page_count(&mut buttons, &component.page_counter_id, 1, card_prices.values().next().unwrap().len());
			components.insert(BUTTONS_KEY, buttons);
		}
		
		components
	}
	
	fn create_reply(&mut self) -> poise::CreateReply {
		let mut reply = poise::CreateReply::default()
			.content(format!("**{}**", self.owner_name))
			.components(vec![self.components.values().next().unwrap().clone()])
			.reply(true);
		
		if self.card_prices.len() == 1 {
			let market = self.card_prices.keys().next().unwrap();
			reply = reply.embed(self.card_prices[market][0].clone());
			self.state.update_market(market.to_string());
		}
		
		reply
	}
	
	async fn handle_interaction(&mut self, interaction: &ComponentInteraction) -> Result<()> {
		if interaction.user.id != self.ctx.author().id {
			cannot_control(self.ctx.serenity_context(), interaction, "You cannot control this pagination").await?;
			return Ok(());
		}
		
		match interaction.data.custom_id.as_str() {
			id if id == self.component.select_menu_id => self.handle_select_menu(interaction),
			id if id == self.component.next_id => self.state.next_page(self.card_prices[&self.state.market].len()),
			id if id == self.component.prev_id => self.state.prev_page(self.card_prices[&self.state.market].len()),
			_ => return Ok(()),
		}
		
		self.update_components();
		self.update_message(interaction, self.card_prices[&self.state.market][self.state.current_page].clone()).await?;
		
		Ok(())
	}
	
	fn handle_select_menu(&mut self, interaction: &ComponentInteraction) {
		if let ComponentInteractionDataKind::StringSelect { values } = &interaction.data.kind {
			if self.components.len() == 1 {
				self.components.insert(BUTTONS_KEY, self.component.create_button_component());
			}
			else {
				self.state.reset();
			}
			self.state.update_market(values[0].clone());
		}
	}
	
	fn update_components(&mut self) {
		if let Some(buttons) = self.components.get_mut(&BUTTONS_KEY) {
			Self::update_page_count(buttons, &self.component.page_counter_id, self.state.current_page + 1, self.card_prices[&self.state.market].len());
		}
	}
	
	fn update_page_count(
		buttons: &mut CreateActionRow,
		page_counter_id: &str,
		current: usize,
		total: usize
	) {
		if let CreateActionRow::Buttons(buttons) = buttons {
			let page_count = format!("{}/{}", current, total);
			buttons[1] = CreateButton::new(page_counter_id)
				.label(page_count)
				.style(ButtonStyle::Secondary)
				.disabled(true);
		}
	}
	
	async fn update_message(&self, interaction: &ComponentInteraction, page: CreateEmbed) -> Result<()> {
		interaction.create_response(
			self.ctx.serenity_context(),
			CreateInteractionResponse::UpdateMessage(
				CreateInteractionResponseMessage::new()
					.content("")
					.embed(page)
					.components(self.components.values().cloned().collect()),
			),
		).await?;
		
		Ok(())
	}
}

struct ComponentIds {
	select_menu_id: String,
	prev_id: String,
	page_counter_id: String,
	next_id: String,
}

impl ComponentIds {
	fn new(ctx_id: &u64) -> Self {
		Self {
			select_menu_id: format!("{}select_menu", ctx_id),
			prev_id: format!("{}prev", ctx_id),
			page_counter_id: format!("{}page_count", ctx_id),
			next_id: format!("{}next", ctx_id),
		}
	}
	
	fn create_select_menu_component(&self, card_prices: &EmbedsMap, placeholder: &str) -> CreateActionRow {
		let options = card_prices.keys().map(|key| {
			let description = if key == "tcg_corner" { "Asian English" } else { "Japanese" };
			CreateSelectMenuOption::new(key.to_uppercase(), key).description(description)
		}).collect::<Vec<_>>();
		
		let select_menu = CreateSelectMenu::new(&self.select_menu_id, CreateSelectMenuKind::String { options })
			.placeholder(placeholder);
	
		CreateActionRow::SelectMenu(select_menu)
	}
	
	fn create_button_component(&self) -> CreateActionRow {
		CreateActionRow::Buttons(vec![
			CreateButton::new(&self.prev_id)
				.emoji('◀')
				.style(ButtonStyle::Secondary),
			CreateButton::new(&self.page_counter_id)
				.label("")
				.style(ButtonStyle::Secondary)
				.disabled(true),
			CreateButton::new(&self.next_id)
				.emoji('▶')
				.style(ButtonStyle::Secondary),
		])
	}
}

#[derive(Default)]
struct PaginationState {
	market: String,
	current_page: usize,
}

impl PaginationState {
	fn reset(&mut self) {
		self.current_page = 0;
	}

	fn update_market(&mut self, new_market: String) {
		self.market = new_market;
	}

	fn next_page(&mut self, total_pages: usize) {
		self.current_page = (self.current_page + 1) % total_pages;
	}

	fn prev_page(&mut self, total_pages: usize) {
		self.current_page = self.current_page.checked_sub(1).unwrap_or(total_pages - 1);
	}
}