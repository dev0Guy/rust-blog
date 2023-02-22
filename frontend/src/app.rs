use yew::{function_component, html, Html};
use crate::components::{Navbar,ArticleGrid};

#[function_component(App)]
pub fn app() -> Html{
    html!(
        <>
            <Navbar/>
            <ArticleGrid/>
        </>
    )
}
