use leptos::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Recipe {
    pub id: String,
    pub dish_name: String,
    pub ingredients: String,
    pub time_to_prepare: String,
    pub source: String,
    pub tags: Vec<String>,
}

#[component]
pub fn RecipeLoader() -> impl IntoView {
    let recipe = create_rw_signal::<Option<Recipe>>(None);
    let loading = create_rw_signal(false);
    let error = create_rw_signal::<Option<String>>(None);

    let load_recipe = move |_| {
        loading.set(true);
        error.set(None);

        spawn_local(async move {
            let response = reqwasm::http::Request::get("http://localhost:3000/api/v1/random-recipe")
                .send()
                .await;

            match response {
                Ok(resp) => {
                    if resp.ok() {
                        let json = resp.json::<Recipe>().await.ok();
                        recipe.set(json);
                    } else {
                        error.set(Some(format!("API error: {}", resp.status())));
                    }
                }
                Err(e) => error.set(Some(format!("Fetch error: {e}"))),
            }
            loading.set(false);
        });
    };

    view! {
        <div class="recipe-container" style="max-width: 600px; margin: auto; padding: 1em;">
            <Show
                when=move || error.get().is_some()
                fallback=|| view! {}
            >
                <p class="error" style="color: red; font-weight: bold;">
                    {move || error.get().unwrap_or_default()}
                </p>
            </Show>

            <Show
                when=move || recipe.get().is_some()
                fallback=|| view! { <p style="font-style: italic;">"Click to load a recipe!"</p> }
            >
                {move || recipe.get().map(|r| view! {
                    <div class="recipe-card" style="border: 1px solid #ccc; border-radius: 8px; padding: 1em; background-color: #f9f9f9; text-align: left;">
                        <h2 style="margin-top: 0;">{r.dish_name}</h2>
                        <p><strong>Ingredients:</strong> {r.ingredients}</p>
                        <p><strong>Preparation Time:</strong> {r.time_to_prepare}</p>
                        <p>
                            <strong>Source:</strong>
                            <a href={r.source.clone()} target="_blank" style="margin-left: 0.5em;">
                                {r.source}
                            </a>
                        </p>
                        <p><strong>Tags:</strong> {
                            if r.tags.is_empty() {
                                "None".to_string()
                            } else {
                                r.tags.join(", ")
                            }
                        }</p>
                    </div>
                })}
            </Show>

            <div style="text-align: center; margin-top: 1em;">
                <button
                    class="load-btn"
                    on:click=load_recipe
                    disabled=move || loading.get()
                    style="padding: 0.5em 1em; font-size: 1em;"
                >
                    {move || if loading.get() { "Loading..." } else { "Load Random Recipe" }}
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main style="font-family: sans-serif; text-align: center; padding: 2em;">
            <h1 style="font-size: 2em;">"Recipe App"</h1>
            <RecipeLoader />
        </main>
    }
}
