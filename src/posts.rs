use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct Post {
    pub by: String,
    pub descendants: usize,
    pub id: usize,
    pub score: usize,
    pub time: usize,
    pub title: String,
    #[serde(alias = "type")]
    pub post_type: String,
    pub url: String,
}

#[derive(Properties, PartialEq)]
pub struct PostsListProps {
    pub posts: Vec<Post>,
}

#[function_component(PostsList)]
pub fn posts_list(PostsListProps { posts }: &PostsListProps) -> Html {
    html! {
        <div class={classes!(String::from("container"))}>
            <div class={classes!(String::from("flex flex-wrap"))}>
                {posts
                    .iter()
                    .map(|p| {
                        html! {
                            <div key={p.id} class={classes!(String::from("bg-white dark:bg-slate-700 rounded-md shadow-md bg-white m-2 p-5"))}>
                                <h3>{p.title.clone()}</h3>
                                <span>{p.by.clone()}</span>
                                <span>{format!(" - {}", p.time)}</span>
                            </div>
                        }
                    })
                    .collect::<Html>()
                }
            </div>
        </div>
    }
}
