/// This example demonstrates low-level usage of scopes.

#[macro_use]
extern crate yew;

use yew::html::*;

#[derive(Default)]
pub struct Context {
    pub activators: Vec<Activator<Context, Model>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            activators: Vec::new(),
        }
    }
}

impl AsMut<Context> for Context {
    fn as_mut(&mut self) -> &mut Context {
        self
    }
}

pub struct Model {
    activator: Activator<Context, Model>,
    selector: &'static str,
    title: String,
}

pub enum Msg {
    SendToOpposite(String),
    SetTitle(String),
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<Context>,
{
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, env: &mut Env<CTX, Self>) -> Self {
        let activator = env.context().as_mut().activators.pop().unwrap();
        Model {
            // TODO Use properties to set activator...
            activator,
            selector: "",
            title: "Nothing".into(),
        }
    }

    fn update(&mut self, msg: Msg, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::SendToOpposite(title) => {
                self.activator.send_message(Msg::SetTitle(title));
            }
            Msg::SetTitle(title) => {
                let context = env.context();
                match title.as_ref() {
                    "Ping" => {
                        self.activator.send_message(Msg::SetTitle("Pong".into()));
                    }
                    "Pong" => {
                        self.activator.send_message(Msg::SetTitle("Pong Done".into()));
                    }
                    "Pong Done" => {
                        self.activator.send_message(Msg::SetTitle("Ping Done".into()));
                    }
                    _ => {
                    }
                }
                self.title = title;
                drop(context);
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<Context> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div>
                <h3>{ format!("{} received <{}>", self.selector, self.title) }</h3>
                <button onclick=|_| Msg::SendToOpposite("One".into()),>{ "One" }</button>
                <button onclick=|_| Msg::SendToOpposite("Two".into()),>{ "Two" }</button>
                <button onclick=|_| Msg::SendToOpposite("Three".into()),>{ "Three" }</button>
                <button onclick=|_| Msg::SendToOpposite("Ping".into()),>{ "Ping" }</button>
            </div>
        }
    }
}
