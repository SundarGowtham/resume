use yew::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use wasm_bindgen::JsCast;

#[function_component(Simulations)]
pub fn simulations() -> Html {
    let canvas_ref = use_node_ref();
    
    {
        let canvas_ref = canvas_ref.clone();
        use_effect_with(
            (),
            move |_| {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    let context = canvas
                        .get_context("webgl2")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<WebGl2RenderingContext>()
                        .unwrap();
                    
                    // Set up basic WebGL scene
                    context.clear_color(0.0, 0.0, 0.0, 1.0);
                    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
                }
                || ()
            },
        );
    }

    html! {
        <div class="simulations">
            <div class="simulations-header">
                <h1>{"3D Simulations & Visualizations"}</h1>
                <p>{"Interactive mathematical and physical simulations"}</p>
            </div>
            
            <div class="simulation-grid">
                <div class="simulation-card">
                    <h3>{"Fluid Dynamics"}</h3>
                    <canvas 
                        ref={canvas_ref}
                        width="400" 
                        height="300"
                        class="simulation-canvas"
                    />
                    <p>{"Real-time fluid simulation using Navier-Stokes equations"}</p>
                    <button class="sim-btn">{"Start Simulation"}</button>
                </div>
                
                <div class="simulation-card">
                    <h3>{"Wave Propagation"}</h3>
                    <div class="simulation-placeholder">
                        <p>{"Interactive wave equation visualization"}</p>
                    </div>
                    <button class="sim-btn">{"Launch Demo"}</button>
                </div>
                
                <div class="simulation-card">
                    <h3>{"Neural Network Visualization"}</h3>
                    <div class="simulation-placeholder">
                        <p>{"3D visualization of neural network training"}</p>
                    </div>
                    <button class="sim-btn">{"View Network"}</button>
                </div>
                
                <div class="simulation-card">
                    <h3>{"Fractal Explorer"}</h3>
                    <div class="simulation-placeholder">
                        <p>{"Interactive Mandelbrot and Julia set explorer"}</p>
                    </div>
                    <button class="sim-btn">{"Explore Fractals"}</button>
                </div>
            </div>
        </div>
    }
}
