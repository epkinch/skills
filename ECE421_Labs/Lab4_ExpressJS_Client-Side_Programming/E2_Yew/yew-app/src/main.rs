use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;

#[derive(Clone, PartialEq, Deserialize)]
    struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[function_component(App)]
fn app() -> Html {

    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with_deps(move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> =
                Request::get("/tutorial/data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                videos.set(fetched_videos);
            });
            || ()
        }, ());
    }

    html! {
        <>
        <h1>{ "RustConf Explorer" }</h1>
        <div>
        <h3>{"Videos to watch"}</h3>
        - <VideosList videos={videos}
        on_click={on_video_select.clone()} />
        <VideosList videos={(*videos).clone()}
        on_click={on_video_select.clone()} />
        </div>
        { for details }
        </>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}