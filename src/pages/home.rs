use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="home">
            <section class="hero">
                <div class="hero-content">
                    <h1 class="hero-title">{"Gowtham Sundar"}</h1>
                    <h2 class="hero-subtitle">{"AI Engineer, Software Engineer & Developer"}</h2>
                    <p class="hero-description">
                        {"Welcome to my digital space where I share my journey through AI, software development, "}
                        {"and complex mathematical simulations. Explore my blog posts, interactive 3D simulations, "}
                        {"and professional experience."}
                    </p>
                    <div class="hero-buttons">
                        <a href="/resume" class="btn btn-primary">{"View Resume"}</a>
                        <a href="/blog" class="btn btn-secondary">{"Read Blog"}</a>
                        <a href="/simulations" class="btn btn-tertiary">{"Try Simulations"}</a>
                    </div>
                </div>
            </section>
            
            <section class="featured">
                <h3>{"Featured Content"}</h3>
                <div class="featured-grid">
                    <div class="featured-item">
                        <h4>{"Latest Blog Posts"}</h4>
                        <p>{"Deep dives into AI, mathematics, and software engineering"}</p>
                    </div>
                    <div class="featured-item">
                        <h4>{"3D Simulations"}</h4>
                        <p>{"Interactive visualizations of complex mathematical concepts"}</p>
                    </div>
                    <div class="featured-item">
                        <h4>{"Technical Notes"}</h4>
                        <p>{"My notes and insights from textbooks and research papers"}</p>
                    </div>
                </div>
            </section>
        </div>
    }
}
