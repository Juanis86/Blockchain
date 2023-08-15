// src/pages/home.rs
use yew::prelude::*;

pub struct HomePage;

impl Component for HomePage {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        HomePage
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"Bienvenido a la Página de Inicio"}</h1>
                <p>{"Aquí verás los contratos y experiencias destacados."}</p>
            </div>
        }
    }
}
