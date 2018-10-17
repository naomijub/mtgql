use juniper::{FieldResult};
use rayon::prelude::*;

pub struct Query;

graphql_object!(Query: Card |&self| {
    field all_cards(&executor) -> FieldResult<Vec<CardBody>> {
        Ok(executor.context().cards.to_owned())
    }

    field card_by_name(&executor, name: String) -> FieldResult<Vec<CardBody>> {
        let cards = executor.context().cards.clone();
        Ok(cards.into_par_iter().filter(|card| card.name.contains(name.as_str())).collect::<Vec<CardBody>>())
    }
});

#[derive(Serialize, Deserialize, Debug, Clone, GraphQLObject)]
#[graphql(description="Cards Vector")]
pub struct Card {
    pub cards: Vec<CardBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone, GraphQLObject)]
#[graphql(description="Card Fields")]
#[allow(non_snake_case)]
pub struct CardBody {
    name: String,
    manaCost: Option<String>,
    cmc: i32,
    colors: Vec<String>,
    colorIdentity: Option<Vec<String>>,
    types: Vec<String>,
    subtypes: Option<Vec<String>>,
    rarity: String,
    set: String,
    setName: Option<String>,
    text: String,
    artist: String,
    number: String,
    power: Option<String>,
    toughness: Option<String>,
    layout: String,
    multiverseid: i32,
    imageUrl: Option<String>,
    rulings: Option<Vec<Rulings>>,
    printings: Vec<String>,
    originalText: Option<String>,
    originalType: Option<String>,
    id: String
}

#[derive(Serialize, Deserialize, Debug, Clone, GraphQLObject)]
#[graphql(description="Card Rulings")]
struct Rulings {
    date: String,
    text: String,
}
