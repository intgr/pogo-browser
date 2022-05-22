use crate::json_stream::iter_json_array;
use std::io;
use std::io::Read;

use crate::schema::{
    FormSettings, GameMasterTemplate, GenderSettings, PokemonFamily, PokemonSettings, TemplateData,
};

mod json_stream;
mod schema;

#[derive(Debug)]
pub struct PogoData {
    pub monsters: Vec<PokemonSettings>,
    pub families: Vec<PokemonFamily>,
    pub forms: Vec<FormSettings>,
    pub genders: Vec<GenderSettings>,
}

pub fn parse_json<R: Read>(reader: R) -> io::Result<PogoData> {
    let mut monsters: Vec<PokemonSettings> = Vec::new();
    let mut families: Vec<PokemonFamily> = Vec::new();
    let mut forms: Vec<FormSettings> = Vec::new();
    let mut genders: Vec<GenderSettings> = Vec::new();

    for result in iter_json_array::<GameMasterTemplate, R>(reader) {
        let item = result?;
        match item.data {
            TemplateData { pokemon_settings: Some(val), .. } => monsters.push(val),
            TemplateData { pokemon_family: Some(val), .. } => families.push(val),
            TemplateData { form_settings: Some(val), .. } => forms.push(val),
            TemplateData { gender_settings: Some(val), .. } => genders.push(val),
            _ => {}
        }
    }

    Ok(PogoData { monsters, families, forms, genders })
}
