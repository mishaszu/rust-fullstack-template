use yew::prelude::*;

pub struct Route1 {
    link: ComponentLink<Self>,
}

impl Component for Route1 {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Route1 { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <h2>{"Route 1"}</h2>
        }
    }
}
