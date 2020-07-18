use yew::format::Nothing;
use yew::prelude::Callback;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub struct Fetcher {}

impl Fetcher {
    pub fn build_get<OUT: 'static>(path: &str, callback: Callback<Response<OUT>>) -> FetchTask
    where
        OUT: From<yew::format::Text>,
    {
        FetchService::fetch(Request::get(path).body(Nothing).unwrap(), callback).unwrap()
    }

    pub fn build_post<IN, OUT: 'static>(
        path: &str,
        body: IN,
        callback: Callback<Response<OUT>>,
    ) -> FetchTask
    where
        IN: Into<yew::format::Text>,
        OUT: From<yew::format::Text>,
    {
        FetchService::fetch(Request::post(path).body(body).unwrap(), callback).unwrap()
    }

    pub fn build_put<IN, OUT: 'static>(
        path: &str,
        body: IN,
        callback: Callback<Response<OUT>>,
    ) -> FetchTask
    where
        IN: Into<yew::format::Text>,
        OUT: From<yew::format::Text>,
    {
        FetchService::fetch(Request::put(path).body(body).unwrap(), callback).unwrap()
    }

    pub fn build_delete<IN, OUT: 'static>(
        path: &str,
        body: IN,
        callback: Callback<Response<OUT>>,
    ) -> FetchTask
    where
        IN: Into<yew::format::Text>,
        OUT: From<yew::format::Text>,
    {
        FetchService::fetch(Request::delete(path).body(body).unwrap(), callback).unwrap()
    }
}
