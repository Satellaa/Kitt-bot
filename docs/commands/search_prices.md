## Definitions

[**Password**](https://yugipedia.com/wiki/Password): the number printed in the bottom-left corner of a _Yu-Gi-Oh!_ card.

[**Database ID**](https://yugipedia.com/wiki/List_of_cards_by_Konami_index_number_(4007%E2%80%935000)): the ID assigned to the card in the [official card database](https://www.db.yugioh-card.com/).

[**Set Number**](https://yugipedia.com/wiki/Card_Number): the set number printed below the artwork of a _Yu-Gi-Oh!_ card.

## Slash Commands

- [`/prices_by_name`](#prices_by_name)
- [`/prices_by_password`](#prices_by_password)
- [`/prices_by_database_id`](#prices_by_database_id)
- [`/prices_by_set_number`](#prices_by_set_number)

## `/prices_by_name`

Find all prices for the card with this English name.

> The prefix version of this command is `$cp`

### Parameters

Name | Required? | Description | Type
--- | --- | --- | ---
`name` | ✔ | Card's English name to search by, fuzzy matching supported. | text

## `/prices_by_password`

Find all prices for the card with this password.

> The prefix version of this command is `$cpp`

### Parameters

Name | Required? | Description | Type
--- | --- | --- | ---
`password` | ✔ | The password you're searching by. | number

## `/prices_by_database_id`

Find all prices for the card with this official database ID.

> The prefix version of this command is `$cpi`

### Parameters

Name | Required? | Description | Type
--- | --- | --- | ---
`database_id` | ✔ | The Database ID you're searching by. | number

## `/prices_by_set_number`

Find all prices for this set number

> The prefix version of this command is `$pp`

### Parameters

Name | Required? | Description | Type
--- | --- | --- | ---
`set_number` | ✔ | The set number you're searching by. | text

## Current behaviour

The public reply will either be a no-match message or the card prices presented in
Discord embeds.

The following information is displayed:

- card name, hyperlinked to the Yugipedia data source
- frameless card artwork, if available
- card prices
  - set number
  - rarity
  - price in Japanese YEN
  - price in Vietnam dong
  - condition
  - sold out or for sale
  - last modified