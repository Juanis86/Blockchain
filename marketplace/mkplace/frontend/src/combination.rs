// frontend/src/components/combinations.rs

use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Combinations;

impl Component for Combinations {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Combinations
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        // Lógica de actualización si es necesario
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h2>{"Listado de Combinaciones Disponibles"}</h2>
                // Aquí puedes mostrar el listado de combinaciones con enlaces a sus detalles
            </div>
        }
    }
}
