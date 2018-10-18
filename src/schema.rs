use juniper::{FieldResult};
use rayon::prelude::*;

pub struct Query;

graphql_object!(Query: Card |&self| {
    field all_cards(&executor) -> FieldResult<Vec<CardBody>> {
        Ok(executor.context().cards.to_owned())
    }

    field cards_by_name(&executor, name: String) -> FieldResult<Vec<CardBody>> {
        let cards = executor.context().cards.clone();
        Ok(cards.into_par_iter()
                .filter(|card| card.name.contains(name.as_str()))
                .collect::<Vec<CardBody>>())
    }

    field mana_type_cards(&executor, color: Option<String>, colors: Option<Vec<String>>)
                        -> FieldResult<Vec<CardBody>> {
        let cards = executor.context().cards.clone();
        match color {
            Some(c) => Ok(cards.into_par_iter()
                            .filter(|card| card.colors.contains(&c.to_owned()))
                            .collect::<Vec<CardBody>>()),
            None => match colors {
                Some(cs) => Ok(cards.into_par_iter()
                                .filter(|card|
                                    cs.iter().fold(true, |value, x| value
                                        && card.colors.contains(&x.to_owned())))
                                .collect::<Vec<CardBody>>()),
                None => Ok(vec![]),
            }

        }
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
