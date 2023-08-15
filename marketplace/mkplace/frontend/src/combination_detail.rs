// frontend/src/components/combination_detail.rs

use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct CombinationDetail {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub combination_id: u64,
}

impl Component for CombinationDetail {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        CombinationDetail { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        // Lógica de actualización si es necesario
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // Lógica para manejar cambios en las propiedades
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h2>{ format!("Detalle de Combinación {}", self.props.combination_id) }</h2>
                // Aquí puedes mostrar los detalles de la combinación, los servicios, componentes, etc.
            </div>
        }
    }
}
