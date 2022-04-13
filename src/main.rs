use yew::prelude::*;

enum Msg {
    AddOne,
    IncrementIncreaseBy,
}

struct Model {
    value: i64,
    increase_by: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            increase_by: 1,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += self.increase_by;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::IncrementIncreaseBy => {
                self.increase_by += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <h1>{"Minimal Yew App"}</h1>
                <div>
                    <button onclick={link.callback(|_| Msg::AddOne)}>{ format!("+{}", self.increase_by) }</button>
                    <p>{ self.value }</p>
                </div>
                <div>
                    <button onclick={link.callback(|_| Msg::IncrementIncreaseBy)}>{ "Increment" }</button>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
