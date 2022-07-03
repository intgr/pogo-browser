use monster::Monster;
use pogo_data::get_embedded_data;
use pogo_data::schema::PokemonSettings;
use std::sync::Arc;
use yew::prelude::*;

mod common;
mod monster;
mod types;

enum Msg {}

struct Model {
    mons: Vec<Arc<PokemonSettings>>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let data = get_embedded_data();
        Self {
            mons: data
                .monsters
                .into_iter()
                .filter(|p| p.form.is_none())
                .map(Arc::new)
                .collect(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let monsters = &self.mons;
        let brand = html! {
            <h1 style="font-size: 20pt">{"Pogo Browser"}</h1>
        };
        html! {
            <>
            <ybc::Navbar navbrand={brand} />
            <ybc::Container fluid={true}>
                <div  style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 0 20px">
                    {
                        monsters.iter().map(|item| {
                            html!{<Monster mon={item.clone()} />}
                        }).collect::<Html>()
                    }
                </div>
            </ybc::Container>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
