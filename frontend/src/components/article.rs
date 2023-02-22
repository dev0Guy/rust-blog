use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ArticlePreviewProps {
    pub id: i32,
}

#[function_component(ArticlePreview)]
pub fn article_preview(props: &ArticlePreviewProps) -> Html{
    html!{
        <div class="article-preview"
             id={format!("item-{}",props.id)}>
            <div class="article-preview-background"
                 style="background-image: url(dist/assets/images/deepLearning.jpeg)"
                >
            </div>
        </div>
    }
}

#[function_component(ArticleGrid)]
pub fn article_grid() -> Html{
    let counter = (1..14).collect::<Vec<_>>();
    html!{
        <section class="article-grid">
            {
                counter.iter().map(|i| {
                    html!{<ArticlePreview id={i}/>}
                }).collect::<Html>()
            }
        </section>
    }
}
