use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BlogPost {
    pub title: String,
    pub date: String,
    pub excerpt: String,
    pub content: String,
    pub tags: Vec<String>,
}

#[function_component(Blog)]
pub fn blog() -> Html {
    let blog_posts = vec![
        BlogPost {
            title: "Understanding Transformer Architecture".to_string(),
            date: "2024-01-15".to_string(),
            excerpt: "Deep dive into the attention mechanism and its applications".to_string(),
            content: "# Understanding Transformer Architecture\n\nThe transformer architecture...".to_string(),
            tags: vec!["AI".to_string(), "Deep Learning".to_string()],
        },
        BlogPost {
            title: "3D Fluid Dynamics Simulation".to_string(),
            date: "2024-01-10".to_string(),
            excerpt: "Building interactive fluid simulations in the browser with WebGL".to_string(),
            content: "# 3D Fluid Dynamics Simulation\n\nFluid dynamics...".to_string(),
            tags: vec!["WebGL".to_string(), "Simulation".to_string()],
        },
    ];

    html! {
        <div class="blog">
            <div class="blog-header">
                <h1>{"Blog & Technical Notes"}</h1>
                <p>{"Thoughts on AI, mathematics, and software engineering"}</p>
            </div>
            
            <div class="blog-grid">
                {blog_posts.iter().map(|post| {
                    html! {
                        <article class="blog-card">
                            <div class="blog-card-content">
                                <h2 class="blog-title">{&post.title}</h2>
                                <div class="blog-meta">
                                    <span class="blog-date">{&post.date}</span>
                                </div>
                                <p class="blog-excerpt">{&post.excerpt}</p>
                                <div class="blog-tags">
                                    {post.tags.iter().map(|tag| {
                                        html! { <span class="tag">{tag}</span> }
                                    }).collect::<Html>()}
                                </div>
                                <a href="#" class="read-more">{"Read More"}</a>
                            </div>
                        </article>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}
