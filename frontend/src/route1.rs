use anyhow::Error;
use api::Hello;
use log::{error, info};
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub struct Route1 {
    _link: ComponentLink<Self>,
    data: Option<Hello>,
    _ft: FetchTask,
}

pub enum Msg {
    FetchReady(Result<Hello, Error>),
    Ignore,
}

impl Component for Route1 {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let task = Route1::fetch_data(&_link);
        Route1 {
            _link,
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
                <h2>{"Route 1"}</h2>
                {
                    match &self.data {
                        None => html!{},
                        Some(data) => html!{<h2>{&data.text}</h2>}
                    }
                }
            </div>
        }
    }
}

impl Route1 {
    fn fetch_data(link: &ComponentLink<Self>) -> FetchTask {
        let callback = link.callback(move |response: Response<Json<Result<Hello, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            info!("META: {:?}, {:?}", meta, data);
            if meta.status.is_success() {
                Msg::FetchReady(data)
            } else {
                Msg::Ignore
            }
        });
        let request = Request::get("/test").body(Nothing).unwrap();
        FetchService::fetch(request, callback).unwrap()
    }
}
