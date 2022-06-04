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
            <p class="subtitle">{ &mon.pokemon_id }</p>
        </div>
    }
}
