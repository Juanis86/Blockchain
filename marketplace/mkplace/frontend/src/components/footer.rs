// src/components/footer.rs
use yew::prelude::*;

pub struct Footer {}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Footer {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <footer class="footer">
                <p>{"© 2023 Tu App. Todos los derechos reservados."}</p>
                <ul class="footer-links">
                    <li><a href="/privacidad">{"Política de Privacidad"}</a></li>
                    <li><a href="/terminos">{"Términos y Condiciones"}</a></li>
                </ul>
            </footer>
        }
    }
}
