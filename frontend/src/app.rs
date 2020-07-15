use crate::route1::Route1;
use crate::route2::Route2;
use log::*;
use yew::prelude::*;
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;

pub enum Msg {
    ChangeRoute(AppRoute),
}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/route2/{id}"]
    Example(String),
    #[to = "/"]
    Index,
}

pub struct App {
    link: ComponentLink<Self>,
    route_service: RouteAgentDispatcher,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_service = RouteAgentDispatcher::new();
        Self {
            link,
            route_service,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeRoute(route) => self
                .route_service
                .send(RouteRequest::ChangeRoute(Route::from(route))),
        };
        info!("update test");
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        let test = AppRoute::Example(String::from("test"));
        info!("Hello from main component");
        html! {
            <div class="main">
                <div class="app">
                <h1>{"Fullstack with Yew"}</h1>
                    <div>
                        <button onclick=&self.change_route(AppRoute::Index)>{"Route 1"}</button>
                        <button onclick=&self.change_route(test)>{"Route 2"}</button>
                    </div>
                    <div class="routes">
                        {App::create_router()}
                    </div>
                </div>
            </div>
        }
    }
}

impl App {
    fn create_router() -> Html {
        html! {
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Index => html!{<Route1 />},
                        AppRoute::Example(text) => html!{<Route2 text=text />},
                    }
                })
            />
        }
    }
    fn change_route(&self, app_route: AppRoute) -> Callback<MouseEvent> {
        self.link.callback(move |_| {
            let route = app_route.clone();
            Msg::ChangeRoute(route)
        })
    }
}
