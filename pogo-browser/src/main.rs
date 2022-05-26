use ybc::NavbarFixed::Top;
use ybc::TileCtx::{Ancestor, Child, Parent};
use ybc::TileSize::Three;
use yew::prelude::*;

enum Msg {}

struct Model {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let brand = html! {<h1 style="font-size: 20pt">{"Pogo Browser"}</h1> };
        html! {
          <>
          <ybc::Navbar navbrand={brand}>
            // <ybc::NavbarItem href={"/test"}>
            //     {"Test link"}
            // </ybc::NavbarItem>
          </ybc::Navbar>
          <ybc::Container fluid={true}>
            <ybc::Tile ctx={Ancestor}>
              <ybc::Tile ctx={Parent} vertical={true} size={Three}>
                {"Item 1"}
              </ybc::Tile>
              <ybc::Tile ctx={Parent} vertical={true} size={Three}>
                {"Item 2"}
              </ybc::Tile>
            </ybc::Tile>
          </ybc::Container>
          </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
