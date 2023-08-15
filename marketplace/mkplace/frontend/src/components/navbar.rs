// src/components/navbar.rs
use yew::prelude::*;

pub struct Navbar {}

impl Component for Navbar {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Navbar {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <nav class="navbar">
                <ul class="nav-links">
                    <li><a href="/">{"Inicio"}</a></li>
                    <li><a href="/exploracion">{"Exploración"}</a></li>
                    <li><a href="/perfil">{"Perfil"}</a></li>
                </ul>
            </nav>
        }
    }
}
