use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::posts::{Post, PostsList};

#[function_component(App)]
pub fn app() -> Html {
    let posts = use_state(|| vec![]);

    {
        let posts = posts.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let results: Vec<usize> =
                        Request::get("https://hacker-news.firebaseio.com/v0/topstories.json")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();

                    let mut post_results: Vec<Post> = vec![];

                    for r in results.iter().take(10) {
                        post_results.push(
                            Request::get(
                                format!("https://hacker-news.firebaseio.com/v0/item/{r}.json")
                                    .as_str(),
                            )
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap(),
                        )
                    }

                    posts.set(post_results);
                });

                || ()
            },
            (),
        );
    }

    html! {
        <main class={classes!(String::from("bg-white dark:bg-slate-900 w-screen h-screen"))}>
            <PostsList posts={(*posts).clone()} />
        </main>
    }
}
