// src/components/card_experiencia.rs
use yew::prelude::*;
use crate::models::Experience; // Asume que tienes una estructura Experience definida en otro lugar

pub struct CardExperiencia {
    experience: Experience,
}

impl Component for CardExperiencia {
    type Message = ();
    type Properties = Experience;

    fn create(experience: Self::Properties, _: ComponentLink<Self>) -> Self {
        CardExperiencia { experience }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, experience: Self::Properties) -> ShouldRender {
        if self.experience != experience {
            self.experience = experience;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="card-experiencia">
                <h2>{ &self.experience.title }</h2>
                <p>{ &self.experience.description }</p>
                <p>{"Calificación: "}{ &self.experience.rating }</p>
                <button>{"Ver Detalles"}</button>
                <button>{"Interactuar"}</button>
            </div>
        }
    }
}
