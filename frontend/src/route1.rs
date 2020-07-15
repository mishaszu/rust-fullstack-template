use yew::prelude::*;

pub struct Route1 {
    _link: ComponentLink<Self>,
}

impl Component for Route1 {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Route1 { _link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
