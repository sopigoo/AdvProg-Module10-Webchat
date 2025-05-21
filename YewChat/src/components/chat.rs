use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
    online: bool,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    username: String,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
            username,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.clone(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                ),
                                online: true,
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        if let Some(data_str) = &msg.data {
                            let message_data: MessageData =
                                serde_json::from_str(data_str).unwrap();
                            self.messages.push(message_data);
                            return true;
                        }
                        false
                    }
                    _ => false,
                }
            }
            Msg::SubmitMessage => {
                if let Some(input) = self.chat_input.cast::<HtmlInputElement>() {
                    let val = input.value();
                    if !val.trim().is_empty() {
                        let message = WebSocketMessage {
                            message_type: MsgTypes::Message,
                            data: Some(val.clone()),
                            data_array: None,
                        };
                        if let Err(e) = self
                            .wss
                            .tx
                            .clone()
                            .try_send(serde_json::to_string(&message).unwrap())
                        {
                            log::debug!("error sending to channel: {:?}", e);
                        }
                        input.set_value("");
                        return false;
                    }
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
            <div class="flex w-screen h-screen bg-gradient-to-r from-purple-300 via-pink-300 to-red-300 font-sans">
                <aside class="flex-none w-64 h-full bg-white shadow-lg p-4 overflow-y-auto">
                    <h2 class="text-2xl font-semibold mb-4 text-purple-700">{"🧑‍🤝‍🧑 Active Users"}</h2>
                    {
                        self.users.iter().map(|u| {
                            html! {
                                <div key={u.name.clone()} class="flex items-center mb-3 p-2 rounded-lg hover:bg-purple-100 transition-colors cursor-pointer">
                                    <img
                                        class="w-12 h-12 rounded-full mr-3 shadow"
                                        src={u.avatar.clone()}
                                        alt="avatar"
                                        />
                                    <div class="flex flex-col">
                                        <span class="font-semibold text-purple-900">{&u.name}</span>
                                        <span class="text-xs text-green-600 flex items-center">
                                            <span class="inline-block w-2 h-2 rounded-full bg-green-400 mr-1 animate-pulse"></span>
                                            {"Online"}
                                        </span>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </aside>
                <main class="flex-grow flex flex-col h-full">
                    <header class="flex-none h-16 bg-white shadow flex items-center px-6 border-b border-purple-300">
                        <h1 class="text-xl font-bold text-purple-700">{"💬 Let's Chat!"}</h1>
                    </header>
                    <section class="flex-grow overflow-auto p-6 bg-purple-50">
                        {
                            self.messages.iter().map(|m| {
                                let is_current_user = m.from == self.username;
                                let user = self.users.iter().find(|u| u.name == m.from);
                                let avatar = user.map(|u| u.avatar.clone()).unwrap_or_default();
                                let is_gif = m.message.to_lowercase().ends_with(".gif");

                                html! {
                                        <div key={format!("{}-{}", m.from, m.message)} class={classes!(
                                            "flex", "items-start", "mb-6",
                                            if is_current_user { "justify-end" } else { "justify-start" }
                                        )}>
                                        {
                                            if !is_current_user {
                                                html! {
                                                    <img class="w-10 h-10 rounded-full mr-3 shadow-md" src={avatar.clone()} alt="avatar"/>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }

                                        <div class={classes!(
                                            "rounded-lg", "shadow", "p-3", "max-w-xs", "break-words",
                                            if is_current_user {
                                                "bg-purple-600 text-white"
                                            } else {
                                                "bg-white text-gray-800"
                                            }
                                        )}>
                                            {
                                                if !is_current_user {
                                                    html! { <div class="text-sm text-gray-500">{ &m.from }</div> }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                            <div class="font-semibold mb-1">{ &m.from }</div>
                                            {
                                                if is_gif {
                                                    html! { <img class="rounded-md max-w-xs" src={m.message.clone()} alt="gif" /> }
                                                } else {
                                                    html! { <p class="whitespace-pre-wrap">{ &m.message }</p> }
                                                }
                                            }
                                        </div>

                                        {
                                            if is_current_user {
                                                html! {
                                                    <img class="w-10 h-10 rounded-full ml-3 shadow-md" src={avatar.clone()} alt="avatar"/>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </section>
                    <footer class="flex-none bg-white border-t border-purple-300 p-4 flex items-center space-x-4">
                        <input
                            ref={self.chat_input.clone()}
                            type="text"
                            placeholder="Write your message..."
                            class="flex-grow px-4 py-2 rounded-full border border-purple-300 focus:outline-none focus:ring-2 focus:ring-purple-500"
                        />
                        <button onclick={submit} class="bg-purple-600 hover:bg-purple-700 text-white p-3 rounded-full shadow-md flex items-center justify-center" aria-label="Send message">
                            <svg viewBox="0 0 24 24" class="w-6 h-6 fill-white" xmlns="http://www.w3.org/2000/svg">
                                <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z" />
                            </svg>
                        </button>
                    </footer>
                </main>
            </div>
        }
    }
}
