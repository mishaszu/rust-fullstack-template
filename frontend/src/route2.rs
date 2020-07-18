use anyhow::Error;
use api::Hello;
use log::{error, info};
use std::collections::HashMap;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub text: String,
}

pub enum Msg {
    FetchReady(Result<Hello, Error>),
    Ignore,
}

pub struct Route2 {
    link: ComponentLink<Self>,
    text: String,
    data: Option<Hello>,
    _ft: FetchTask,
}

impl Component for Route2 {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let task = Route2::fetch_hello(&link);
        Route2 {
            link,
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

enum ReqType {
    GET,
}

pub struct Fetcher {
    pub tasks: HashMap<String, FetchTask>,
}

impl FetchFactory for Fetcher {}

impl Fetcher {
    pub fn register<T, OUT: 'static>(
        &mut self,
        name: String,
        path: &str,
        callback: Callback<Response<OUT>>,
    ) where
        T: Into<yew::format::Text>,
        OUT: From<yew::format::Text>,
    {
        self.tasks.insert(name, Self::get(path, callback));
    }
}

trait FetchFactory {
    fn get<OUT: 'static>(path: &str, callback: Callback<Response<OUT>>) -> FetchTask
    where
        OUT: From<yew::format::Text>,
    {
        FetchService::fetch(Request::get(path).body(Nothing).unwrap(), callback).unwrap()
    }
}

// impl FetchTaskFactory<Json<Result<Hello, Error>>, Request<Nothing>> for Route2 {}

impl FetchFactory for Route2 {}

impl Route2 {
    fn fetch_hello(link: &ComponentLink<Self>) -> FetchTask {
        Route2::get("/api/hello", Route2::build_hello_callback(link))
    }
    fn build_hello_callback(
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
