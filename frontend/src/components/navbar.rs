use yew::{function_component, html, Html};

#[function_component(Navbar)]
pub fn navbar() -> Html{
    html!(
        <nav class="navbar">
            <div class="navbar-item navbar-logo"><a href="#">{"Dev0Guy"}</a></div>
            <ul class="navbar-list">
                <li class="navbar-item"><a><img src="dist/assets/icons/linkedin.svg" alt="Linkedin"  id="linkedin"/></a></li>
                <li class="navbar-item"><a href="https://github.com/dev0Guy"><img src="dist/assets/icons/github.svg" alt="github" /></a></li>
            </ul>
        </nav>
    )
}