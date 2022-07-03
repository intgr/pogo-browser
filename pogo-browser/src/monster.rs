use crate::common::pretty_name;
use crate::types::TypePair;
use pogo_data::schema::{MonsterBaseStats, PokemonSettings};
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

fn render_moves_list(title: &'static str, moves: &Vec<String>) -> Html {
    // TODO: Display the type of moves too!
    html! {
        <>
            <h5>{ title }</h5>
            <ul>
                {
                    moves.iter().map(|item| {
                        html! { <li>{ pretty_name(item) }</li> }
                    }).collect::<Html>()
                }
            </ul>
        </>
    }
}

#[function_component(Monster)]
pub fn render_monster(props: &MonsterProps) -> Html {
    let mon: &PokemonSettings = props.mon.borrow();

    let stats = match mon.stats {
        MonsterBaseStats::Some { base_attack, base_defense, base_stamina } => {
            html! {<div>{format!("ATK: {base_attack}, DEF: {base_defense}, HP: {base_stamina}")}</div>}
        }
        _ => html! {},
    };

    html! {
        <div class="box">
            <p class="subtitle">
                { pretty_name(&mon.pokemon_id) }
                <TypePair typ1={mon.type1} typ2={mon.type2} />
            </p>
            { stats }
            { render_moves_list("Fast moves", &mon.quick_moves) }
            { render_moves_list("Elite fast moves", &mon.elite_quick_move) }
            { render_moves_list("Charged moves", &mon.cinematic_moves) }
            { render_moves_list("Elite charged moves", &mon.elite_cinematic_move) }
        </div>
    }
}
