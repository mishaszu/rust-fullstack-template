use log::*;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

pub struct App {
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <div class="main">
                <div class="app">
                <h1>{"Fullstack with Yew"}</h1>
                    <div>
                        <button>{"Route 1"}</button>
                        <button>{"Route 2"}</button>
                    </div>
                    <div class="routes">
                        <p>{"routes"}</p>
                    </div>
                </div>
            </div>
        }
    }
}
