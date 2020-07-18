use anyhow::Error;
use api::Hello;
use helpers::Fetcher;
use log::{error, info};
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchTask, Response};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub text: String,
}

pub enum Msg {
    FetchReady(Result<Hello, Error>),
    Ignore,
}

pub struct Route2 {
    _link: ComponentLink<Self>,
    text: String,
    data: Option<Hello>,
    _ft: FetchTask,
}

impl Component for Route2 {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let task = Fetcher::build_get("/api/hello", Self::hello_callback(&_link));
        Route2 {
            _link,
            text: props.text,
            data: None,
            _ft: task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchReady(data) => match data {
                Ok(data) => self.data = Some(data),
                _ => self.data = None,
            },
            Msg::Ignore => error!("Can't get request"),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h2>{format!("Route 2, text from props: {}", self.text)}</h2>
                {
                    match &self.data {
                        None => html!{},
                        Some(data) => html!{<h2>{data.text.clone()}</h2>}
                    }
                }
            </div>
        }
    }
}

impl Route2 {
    fn hello_callback(
        link: &ComponentLink<Self>,
    ) -> Callback<Response<Json<Result<Hello, Error>>>> {
        link.callback(move |response: Response<Json<Result<Hello, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            info!("META: {:?}, {:?}", meta, data);
            if meta.status.is_success() {
                Msg::FetchReady(data)
            } else {
                Msg::Ignore
            }
        })
    }
}
