use std::io::Read;

use crate::schema::GameMasterTemplate;

mod schema;

pub fn parse_json<R: Read>(reader: R) -> serde_json::Result<Vec<GameMasterTemplate>> {
    serde_json::from_reader::<R, Vec<GameMasterTemplate>>(reader)
}
