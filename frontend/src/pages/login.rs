use yew::{function_component, html, Html};


#[function_component(LoginPage)]
pub fn login_page() -> Html{
    html!(
        <form>  
            <div class="container">   
                <label>{"Username :"} </label>   
                <input type="text" placeholder="Enter Username" name="username" required=true/>  
                <label>{"Password :"} </label>   
                <input type="password" placeholder="Enter Password" name="password" required=true/>  
                <button type="submit">{"Login"}</button>     
            </div>   
        </form>     
    )
}
