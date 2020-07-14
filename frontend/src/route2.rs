use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub text: String,
}

pub struct Route2 {
    link: ComponentLink<Self>,
    text: String,
}

impl Component for Route2 {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Route2 {
            link,
            text: props.text,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <h2>{format!("Route 2, text: {}", self.text)}</h2>
        }
    }
}
