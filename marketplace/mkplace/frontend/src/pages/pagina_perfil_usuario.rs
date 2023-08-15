// Página de Perfil de Usuario (pagina_perfil_usuario.rs)
use yew::prelude::*;

pub struct PaginaPerfilUsuario {
    // Añade los campos necesarios para mostrar la información del perfil de usuario y el historial de transacciones
}

impl Component for PaginaPerfilUsuario {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        PaginaPerfilUsuario {
            // Inicializa los campos necesarios
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                // Renderiza la información del perfil de usuario y el historial de transacciones
            </div>
        }
    }
}

// Página de Notificaciones (pagina_notificaciones.rs)
use yew::prelude::*;

pub struct PaginaNotificaciones {
    // Añade los campos necesarios para mostrar las notificaciones recientes para el usuario
}

impl Component for PaginaNotificaciones {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        PaginaNotificaciones {
            // Inicializa los campos necesarios
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                // Renderiza las notificaciones recientes para el usuario
            </div>
        }
    }
}
