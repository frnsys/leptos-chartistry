use crate::examples::*;
use js_sys::wasm_bindgen::JsCast;
use leptos::{html::Dialog, *};
use web_sys::{HtmlDialogElement, MouseEvent};

#[component]
pub fn Examples() -> impl IntoView {
    view! {
        <article id="examples">
            <h1>"Examples"</h1>
            <nav>
                <ul class="background-box">
                    <li>
                        <a href="#series">"By chart series"</a>": "
                        <ul>
                            <li><a href="#series-line">"Line charts"</a></li>
                            <li><a href="#series-bar">"Bar charts"</a></li>
                            <li><a href="#series-scatter">"Scatter charts"</a></li>
                        </ul>
                    </li>
                    <li>
                        <a href="#edge">"By edge layout"</a>": "
                        <ul>
                            <li><a href="#edge-legend">"Legend"</a></li>
                            <li><a href="#edge-text">"Text label"</a></li>
                            <li><a href="#edge-ticks">"Tick labels"</a></li>
                        </ul>
                    </li>
                    <li>
                        <a href="#inner">"By inner layout"</a>": "
                        <ul>
                            <li><a href="#inner-axis">"Axis marker"</a></li>
                            <li><a href="#inner-grid">"Grid line"</a></li>
                            <li><a href="#inner-guide">"Guide line"</a></li>
                            <li><a href="#inner-legend">"Legend"</a></li>
                        </ul>
                    </li>
                    <li>
                        <a href="#feature">"By feature"</a>": "
                        <ul>
                            <li><a href="#feature-colour">"Colours"</a></li>
                            <li><a href="#feature-width">"Line widths"</a></li>
                        </ul>
                    </li>
                </ul>
            </nav>

            <div id="series">
                <div id="series-line">
                    <h2>"Line charts"</h2>
                    <div class="card">
                        "todo"
                    </div>
                </div>

                <div id="series-bar">
                    <h2>"Bar charts"</h2>
                    <p>"Planned"</p>
                </div>

                <div id="series-scatter">
                    <h2>"Scatter charts"</h2>
                    <p>"Planned"</p>
                </div>
            </div>

            <h2 id="edge">"Edge layout options"</h2>
            <div class="cards">
                <EdgeLayoutFigures />
            </div>

        </article>
    }
}

#[component]
fn ShowCode(#[prop(into)] code: String) -> impl IntoView {
    let dialog = create_node_ref::<Dialog>();
    let on_open = move |ev: MouseEvent| {
        ev.prevent_default();
        if let Some(dialog) = dialog.get() {
            dialog
                .show_modal()
                .expect("unable to show example code dialog");
        }
    };
    let on_close = move |ev: MouseEvent| {
        ev.prevent_default();
        if let Some(dialog) = dialog.get() {
            // Close dialogue (it covers the whole page) on interaction unless user clicks on text inside
            if let Some(target) = ev.target() {
                if target.dyn_ref::<HtmlDialogElement>().is_some() {
                    dialog.close()
                }
            }
        }
    };
    view! {
        <a href="#" on:click=on_open>"Show example code"</a>
        <dialog node_ref=dialog on:click=on_close>
            <pre><code>{code}</code></pre>
        </dialog>
    }
}

#[component]
fn EdgeLayoutFigures() -> impl IntoView {
    let data = load_data();
    view! {
        <figure id="edge-legend" class="background-box">
            <figcaption>
                <h3>"Legend"</h3>
                <p>
                    "Add legends to your chart. "
                    <ShowCode code=include_str!("../examples/edge_layout.rs") />
                </p>
            </figcaption>
            <edge_layout::Example data=data />
        </figure>
    }
}
