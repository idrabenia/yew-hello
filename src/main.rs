
use serde::Deserialize;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

enum Msg {
    AddOne,
    GetUsers,
    ReceiveResponse(Result<User, anyhow::Error>),
}

struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub active: bool,
    pub sign_in_count: i64
}

struct Model {
    value: i64,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    users: Vec<User>
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            users: vec![],
            fetch_task: None,
            link: Self.link
        };

        Self.link.callback(|_| Msg::GetUsers);

        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetUsers => {
                let request = Request::get("http://localhost:8000/")
                    .body(Nothing)
                    .expect("Could not build request.");
                // 2. construct a callback
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<User, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::ReceiveResponse(data)
                        });
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
                // we want to redraw so that the page displays a 'fetching...' message to the user
                // so return 'true'
                true
            },
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            },
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(users) => {
                        self.users = Some(users);
                    }
                    Err(error) => {
                        //self.error = Some(error.to_string())
                    }
                }
                self.fetch_task = None;
                // we want to redraw so that the page displays the location of the ISS instead of
                // 'fetching...'
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <>
                <div>
                    <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                    <p>{ self.value }</p>
                </div>
                <div>
                    <ul>
                        {self.users.iter().map(|cur_user| {
                            html! {
                                <li>{cur_user.name} {cur_user.email}</li>
                            }
                        }).collect::<Html>()}
                    </ul>
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
