use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use serde_json::from_str;
use super::schema::Card;

fn read_fake() -> Result<String> {
    let mut file = File::open("./body.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn fake_ctx() -> Card {
    let v: Card = from_str(read_fake().unwrap().as_str()).unwrap();
    v
}
