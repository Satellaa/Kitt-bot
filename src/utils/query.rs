use mongodb::bson::{doc, Document};

pub enum QueryHolder {
	Filter(Document),
	Pipeline(Vec<Document>)
}

impl QueryHolder {
	pub fn create_index_pipeline(index: &str, query: &str, path: &str) -> Self {
		let pipeline = vec![
			doc! {
				"$search": {
					"index": index,
					"compound": {
						"should": [
							doc! {
								"text": {
									"query": query,
									"path": path,
									"fuzzy": {
										"maxEdits": 1
									}
								}
							},
							doc! {
								"phrase": {
									"query": query,
									"path": path,
									"slop": 0
								}
							}
						]
					}
				}
			}
		];

		Self::Pipeline(pipeline)
	}

	pub fn create_set_number_pipeline(set_number: &str) -> Self {
		let pipeline = vec![
			doc! {
				"$match": {
					"$expr": {
						"$ne": [
							{
								"$filter": {
									"input": { "$objectToArray": "$card_prices" },
									"cond": {
										"$gt": [
											{
												"$size": {
													"$filter": {
														"input": "$$this.v.set_number",
														"as": "num",
														"cond": { "$regexMatch": { "input": "$$num", "regex": set_number, "options": "i" } }
													}
												}
											},
											0
										]
									}
								}
							},
							[]
						]
					}
				}
			},
			doc! {
				"$addFields": {
					"card_prices": {
						"$arrayToObject": {
							"$map": {
								"input": { "$objectToArray": "$card_prices" },
								"as": "cp",
								"in": {
									"k": "$$cp.k",
									"v": {
										"$filter": {
											"input": "$$cp.v",
											"as": "item",
											"cond": {
												"$regexMatch": {
													"input": "$$item.set_number",
													"regex": set_number,
													"options": "i"
												}
											}
										}
									}
								}
							}
						}
					}
				}
			},
			doc! { "$limit": 1 },
			doc! { "$project": { "_id": 0 } }
		];

		Self::Pipeline(pipeline)
	}
}