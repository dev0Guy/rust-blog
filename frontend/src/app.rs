use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew::{function_component, html, Html};
use crate::router::{switch,Route};
use yew_router::prelude::*;
use yew::prelude::*;


#[derive(Properties, PartialEq, Debug)]
pub struct ServerAppProps {
    pub url: String,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html{
    let history = AnyHistory::from(MemoryHistory::new());
    history.push(&*props.url);
    return html! (
        <Router history={history}>
            <Switch<Route> render={switch} />
        </Router>
    );
}


#[function_component(ClientApp)]
pub fn server_app() -> Html{
    html!(
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    )
}




