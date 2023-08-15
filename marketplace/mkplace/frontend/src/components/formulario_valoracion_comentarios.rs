// src/components/formulario_valoracion_comentarios.rs
use yew::prelude::*;

pub struct FormularioValoracionComentarios {
    link: ComponentLink<Self>,
    rating: u32,
    comment: String,
}

pub enum Msg {
    UpdateRating(u32),
    UpdateComment(String),
    Submit,
}

impl Component for FormularioValoracionComentarios {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        FormularioValoracionComentarios {
            link,
            rating: 0,
            comment: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateRating(rating) => self.rating = rating,
            Msg::UpdateComment(comment) => self.comment = comment,
            Msg::Submit => {
                // Lógica para enviar la valoración y comentario al backend
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
                <div class="rating">
                    { for (1..=5).map(|i| self.render_star(i)) }
                </div>
                <textarea placeholder="Deja tu comentario" value=&self.comment
                    oninput=self.link.callback(|e: InputData| Msg::UpdateComment(e.value)) />
                <button type="submit">{"Enviar Valoración y Comentario"}</button>
            </form>
        }
    }
}

impl FormularioValoracionComentarios {
    fn render_star(&self, value: u32) -> Html {
        let active = if value <= self.rating { "active" } else { "" };
        html! {
            <span class=("star", active) onclick=self.link.callback(move |_| Msg::UpdateRating(value))>{"★"}</span>
        }
    }
}
