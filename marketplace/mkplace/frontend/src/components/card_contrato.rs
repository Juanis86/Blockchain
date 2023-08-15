// src/components/card_contrato.rs
use yew::prelude::*;
use crate::models::Contract; // Asume que tienes una estructura Contract definida en otro lugar

pub struct CardContrato {
    contract: Contract,
}

impl Component for CardContrato {
    type Message = ();
    type Properties = Contract;

    fn create(contract: Self::Properties, _: ComponentLink<Self>) -> Self {
        CardContrato { contract }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, contract: Self::Properties) -> ShouldRender {
        if self.contract != contract {
            self.contract = contract;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="card-contrato">
                <h2>{ &self.contract.title }</h2>
                <p>{ &self.contract.description }</p>
                <button>{"Ver Detalles"}</button>
                <button>{"Interactuar"}</button>
            </div>
        }
    }
}
