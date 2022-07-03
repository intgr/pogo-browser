use crate::common::pretty_name;
use crate::types::TypePair;
use pogo_data::schema::PokemonSettings;
use std::borrow::Borrow;
use std::sync::Arc;
use yew::prelude::*;

#[derive(Properties)]
pub struct MonsterProps {
    pub mon: Arc<PokemonSettings>,
}

impl PartialEq for MonsterProps {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.mon, &other.mon)
    }
}

#[function_component(Monster)]
pub fn render_monster(props: &MonsterProps) -> Html {
    let mon: &PokemonSettings = props.mon.borrow();
    html! {
        <div class="box">
            <p class="subtitle">
                { pretty_name(&mon.pokemon_id) }
                <TypePair typ1={mon.type1} typ2={mon.type2} />
            </p>
        </div>
    }
}
