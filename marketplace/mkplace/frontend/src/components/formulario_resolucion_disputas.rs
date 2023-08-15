// src/components/formulario_resolucion_disputas.rs
use yew::prelude::*;

pub struct FormularioResolucionDisputas {
    link: ComponentLink<Self>,
    description: String,
    vote: Option<bool>,
    result: Option<bool>,
}

pub enum Msg {
    UpdateDescription(String),
    Vote(bool),
    Submit,
}

impl Component for FormularioResolucionDisputas {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        FormularioResolucionDisputas {
            link,
            description: String::new(),
            vote: None,
            result: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateDescription(description) => self.description = description,
            Msg::Vote(vote) => self.vote = Some(vote),
            Msg::Submit => {
                // Lógica para enviar la resolución de disputa al backend
                // Calcula el resultado basado en los votos y actualiza self.result
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <form onsubmit=self.link.callback(|e| {
                e.prevent_default();
                Msg::Submit
            })>
                <textarea placeholder="Descripción de la disputa" value=&self.description
                    oninput=self.link.callback(|e: InputData| Msg::UpdateDescription(e.value)) />
                <div class="vote-buttons">
                    <button class=("vote-button", self.vote_class(true)) onclick=self.link.callback(|_| Msg::Vote(true))>{"A favor"}</button>
                    <button class=("vote-button", self.vote_class(false)) onclick=self.link.callback(|_| Msg::Vote(false))>{"En contra"}</button>
                </div>
                { self.render_result() }
                <button type="submit">{"Enviar Resolución de Disputa"}</button>
            </form>
        }
    }
}

impl FormularioResolucionDisputas {
    fn vote_class(&self, vote: bool) -> &str {
        if self.vote == Some(vote) { "active" } else { "" }
    }

    fn render_result(&self) -> Html {
        match self.result {
            Some(true) => html! { <div class="result positive">{"Resolución: A favor"}</div> },
            Some(false) => html! { <div class="result negative">{"Resolución: En contra"}</div> },
            None => html! {},
        }
    }
}
