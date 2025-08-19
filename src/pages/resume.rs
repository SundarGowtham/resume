use yew::prelude::*;

#[function_component(Resume)]
pub fn resume() -> Html {
    html! {
        <div class="resume">
            <div class="resume-header">
                <h1>{"Resume"}</h1>
                <p>{"Professional experience and qualifications"}</p>
            </div>
            
            <div class="resume-content">
                <div class="resume-actions">
                    <a href="/Gowtham-Sundar-Resume.pdf" 
                       target="_blank" 
                       class="btn btn-primary">
                        {"📄 View Resume"}
                    </a>
                    <a href="/Gowtham-Sundar-Resume.pdf" 
                       download="Gowtham-Sundar-Resume.pdf"
                       class="btn btn-secondary">
                        {"💾 Download Resume"}
                    </a>
                </div>
                
                <div class="resume-preview">
                    <div class="resume-section">
                        <h2>{"Professional Summary"}</h2>
                        <p>
                            {"AI Engineer and Software Developer with expertise in machine learning, "}
                            {"web development, and complex system design. Passionate about creating "}
                            {"innovative solutions that bridge theoretical knowledge with practical applications."}
                        </p>
                    </div>
                    
                    <div class="resume-section">
                        <h2>{"Key Skills"}</h2>
                        <div class="skills-grid">
                            <div class="skill-category">
                                <h3>{"AI & ML"}</h3>
                                <ul>
                                    <li>{"Deep Learning"}</li>
                                    <li>{"Neural Networks"}</li>
                                    <li>{"Computer Vision"}</li>
                                    <li>{"NLP"}</li>
                                </ul>
                            </div>
                            <div class="skill-category">
                                <h3>{"Programming"}</h3>
                                <ul>
                                    <li>{"Rust"}</li>
                                    <li>{"Python"}</li>
                                    <li>{"JavaScript/TypeScript"}</li>
                                    <li>{"WebAssembly"}</li>
                                </ul>
                            </div>
                            <div class="skill-category">
                                <h3>{"Web Technologies"}</h3>
                                <ul>
                                    <li>{"WebGL/WebGPU"}</li>
                                    <li>{"React/Yew"}</li>
                                    <li>{"Node.js"}</li>
                                    <li>{"Three.js"}</li>
                                </ul>
                            </div>
                        </div>
                    </div>
                    
                    <div class="contact-section">
                        <h2>{"Contact Information"}</h2>
                        <div class="contact-grid">
                            <div class="contact-item">
                                <span class="icon">{"📧"}</span>
                                <a href="mailto:your.email@example.com">{"your.email@example.com"}</a>
                            </div>
                            <div class="contact-item">
                                <span class="icon">{"🔗"}</span>
                                <a href="https://github.com/SundarGowtham" target="_blank">{"GitHub"}</a>
                            </div>
                            <div class="contact-item">
                                <span class="icon">{"💼"}</span>
                                <a href="https://linkedin.com/in/yourprofile" target="_blank">{"LinkedIn"}</a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
