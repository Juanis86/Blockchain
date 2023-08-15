// frontend/src/main.rs

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

mod search;
mod combinations;
mod combination_detail;
// ... (otros módulos que puedas tener)

// Define las rutas y componentes correspondientes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/combinations"]
    Combinations,
    #[to = "/combinations/{id}"]
    CombinationDetail(String),
    #[to = "/search"]
    Search,
    // ... (otras rutas que puedas necesitar)
}

pub struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        // Lógica de actualización si es necesario
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Combinations => html!{<combinations::Combinations />},
                        AppRoute::CombinationDetail(id) => html!{<combination_detail::CombinationDetail id=id />},
                        AppRoute::Search => html!{<search::Search />},
                        // ... (manejo de otras rutas)
                    }
                })
            />
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
