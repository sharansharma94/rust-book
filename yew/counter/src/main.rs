use yew::prelude::*;

// enum Msg can be dispatched from ui events such as on click

enum Msg {
    AddOne,
    MinusOne,
}

struct Model {
    value: i64,
}

//life cycle methods
impl Component for Model {
    //type of msg we can pass
    type Message = Msg;

    // props
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }

            Msg::MinusOne => {
                self.value -=1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div class="container">
                <div class="buttons">
                    <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                    <button onclick={link.callback(|_| Msg::MinusOne)}>{ "-1" }</button>
                </div>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}