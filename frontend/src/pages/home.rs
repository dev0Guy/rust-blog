use crate::components::{ArticleGrid};
use yew::{function_component, html, Html};


#[function_component(HomePage)]
pub fn home_page() -> Html{
    html!(
        <ArticleGrid/>
    )
}
