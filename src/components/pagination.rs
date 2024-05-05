use std::collections::BTreeMap;
use poise::serenity_prelude as serenity;
use poise::reply::CreateReply;
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
	ButtonStyle::Secondary
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
	card_prices: &'a EmbedsMap,
	component: CreateComponent,
	components: ComponentsMap,
	state: PaginationState
}

impl<'a> Pagination<'a> {
	pub fn new(ctx: &'a Context<'a>, card_prices: &'a EmbedsMap) -> Self {
		let ctx_id = ctx.id();
		let component = CreateComponent::new(&ctx_id);
		let mut components = ComponentsMap::new();
		
		if card_prices.len() > 1 {
			components.insert(SELECT_MENU_KEY, component.create_select_menu_component(card_prices, "Select a vendor"));
		}
		else {
			let mut comps = component.create_button_component();
			Self::update_page_count(&mut comps, &component.page_counter_id, 1, card_prices.values().next().unwrap().len());
			components.insert(BUTTONS_KEY, comps);
		};
		
		Self {
			ctx,
			card_prices,
			component,
			components,
			state: PaginationState::new(),
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
	
	fn create_reply(&mut self) -> CreateReply {
		let mut reply = CreateReply::default()
			.components(vec![self.components.values().next().unwrap().clone()])
			.reply(true);
		
		if self.card_prices.len() == 1 {
			let market = self.card_prices.keys().next().unwrap().to_string();
			reply = reply.embed(self.card_prices[&market][0].clone());
			self.state.update_market(market);
		}
		
		reply
	}
	
	async fn handle_interaction(&mut self, interaction: &ComponentInteraction) -> Result<()> {
		if interaction.user.id != self.ctx.author().id {
			cannot_control(self.ctx.serenity_context(), interaction, "You cannot control this pagination").await?;
			return Ok(());
		}
		
		// my personal opinion is that `if, else if, else` looks better here than `match`
		if interaction.data.custom_id == self.component.select_menu_id {
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
		else if interaction.data.custom_id == self.component.next_id {
			self.state.next_page(self.card_prices[&self.state.market].len());
		}
		else if interaction.data.custom_id == self.component.prev_id {
			self.state.prev_page(self.card_prices[&self.state.market].len());
		}
		else {
			return Ok(());
		}
		
		Self::update_page_count(self.components.get_mut(&BUTTONS_KEY).unwrap(), &self.component.page_counter_id, self.state.current_page + 1, self.card_prices[&self.state.market].len());
		self.update_message(interaction, self.card_prices[&self.state.market][self.state.current_page].clone()).await?;
		
		Ok(())
	}
	
	fn update_page_count(
		buttons: &mut CreateActionRow,
		page_counter_id: &String,
		current: usize,
		total: usize
	) {
		if let CreateActionRow::Buttons(buttons) = buttons {
			let page_count = format!("{}/{}", current, total);
			buttons[1] = CreateButton::new(page_counter_id)
				.label(page_count)
				.style(Secondary)
				.disabled(true);
		}
	}
	
	async fn update_message(&self, interaction: &ComponentInteraction, page: CreateEmbed) -> Result<()> {
		interaction.create_response(
			self.ctx.serenity_context(),
			CreateInteractionResponse::UpdateMessage(
				CreateInteractionResponseMessage::new()
					.embed(page)
					.components(self.components.values().cloned().collect()),
			),
		).await?;
		
		Ok(())
	}
}

struct CreateComponent {
	select_menu_id: String,
	prev_id: String,
	page_counter_id: String,
	next_id: String,
}

impl CreateComponent {
	
	fn new(ctx_id: &u64) -> Self {
		Self {
			select_menu_id: format!("{}select_menu", ctx_id),
			prev_id: format!("{}prev", ctx_id),
			page_counter_id: format!("{}page_count", ctx_id),
			next_id: format!("{}next", ctx_id),
		}
	}
	
	fn create_select_menu_component(&self, card_prices: &EmbedsMap, placeholder: &str) -> CreateActionRow {
		let mut select_menu_options = Vec::<CreateSelectMenuOption>::new();
		select_menu_options.extend(
			card_prices.keys().map(|key| {
				let description = if key == "tcg_corner" { "Asian English" } else { "Japanese" };
				CreateSelectMenuOption::new(key.to_uppercase(), key).description(description)
			})
		);
		
		let select_menu_kind = CreateSelectMenuKind::String {
			options: select_menu_options
		};
	
		let select_menu = CreateSelectMenu::new(&self.select_menu_id, select_menu_kind)
			.placeholder(placeholder);
	
		CreateActionRow::SelectMenu(select_menu)
	}
	
	fn create_button_component(&self) -> CreateActionRow {
		CreateActionRow::Buttons(vec![
			CreateButton::new(&self.prev_id)
				.emoji('◀')
				.style(Secondary),
			CreateButton::new(&self.page_counter_id)
				.label("")
				.style(Secondary)
				.disabled(true),
			CreateButton::new(&self.next_id)
				.emoji('▶')
				.style(Secondary),
		])
	}
}

struct PaginationState {
	market: String,
	current_page: usize,
}

impl PaginationState {
	fn new() -> Self {
		PaginationState {
			market: String::new(),
			current_page: 0,
		}
	}

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
