use std::sync::Arc;

use iced::Element;
use iced::Length;
use iced::Task;
use iced::widget::button;
use iced::widget::column;
use iced::widget::container;
use iced::widget::text;
use iced::widget::text_input;
use shared::types::UserHandle;

use crate::app::AppMsg;
use crate::core::CoreState;
use crate::websocket::Instruction::Connect;
use crate::websocket::WebSocket;
use shared::types::Room;
use shared::types::User;

pub enum LoginAction {
    ToCore(CoreState),
}

#[derive(Debug, Clone)]
pub enum LoginMsg {
    WebSocketInitialized(WebSocket),
    ChangedNameInput(String),
    ChangedIpInput(String),
    ConnectClicked,
    RegisterClicked,
    WebsocketConnected,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct LoginState {
    pub websocket: Option<WebSocket>,
    pub error: Option<String>,
    pub name_input_value: String,
    pub ip_input_value: String,
}

pub fn update(state: &mut LoginState, message: LoginMsg) -> (Task<LoginMsg>, Option<LoginAction>) {
    match message {
        LoginMsg::WebSocketInitialized(websocket) => {
            state.websocket = Some(websocket);
            (Task::none(), None)
        }
        LoginMsg::ChangedNameInput(s) => {
            state.name_input_value = s;
            (Task::none(), None)
        }
        LoginMsg::ChangedIpInput(s) => {
            state.ip_input_value = s;
            (Task::none(), None)
        }
        LoginMsg::ConnectClicked => match &state.websocket {
            Some(inst) => {
                let mut websocket = inst.clone();
                let address = state.ip_input_value.clone();

                (
                    Task::future(async move {
                        websocket.send(Connect(address)).await;
                    })
                    .discard(),
                    None,
                )
            }
            None => {
                state.error = Some(String::from("Websocket was not initialized"));
                (Task::none(), None)
            }
        },
        LoginMsg::RegisterClicked => match &state.websocket {
            Some(inst) => {
                let mut websocket = inst.clone();
                let address = state.ip_input_value.clone();

                (
                    Task::future(async move {
                        websocket.send(Connect(address)).await;
                    })
                    .discard(),
                    None,
                )
            }
            None => {
                state.error = Some(String::from("Websocket was not initialized"));
                (Task::none(), None)
            }
        },
        LoginMsg::WebsocketConnected => match &state.websocket {
            Some(inst) => {
                let converstations = vec![
                    Room {
                        participant: User {
                            handle: UserHandle::new_random(),
                            name: String::from("Peter"),
                        },
                        messages: Vec::new(),
                        input_text: String::new(),
                    },
                    Room {
                        participant: User {
                            handle: UserHandle::new_random(),
                            name: String::from("Jonas"),
                        },
                        messages: Vec::new(),
                        input_text: String::new(),
                    },
                    Room {
                        participant: User {
                            handle: UserHandle::new_random(),
                            name: String::from("Olaf"),
                        },
                        messages: Vec::new(),
                        input_text: String::new(),
                    },
                ];
                let new_state = CoreState {
                    websocket: inst.clone(),
                    user: Arc::new(User {
                        handle: UserHandle::new_random(),
                        name: String::from(&state.name_input_value),
                    }),
                    conversations: converstations,
                    current_converstaion: 0,
                };
                (Task::none(), Some(LoginAction::ToCore(new_state)))
            }
            None => {
                state.error = Some(String::from("Websocket was not initialized"));
                (Task::none(), None)
            }
        },
        LoginMsg::Error(e) => {
            state.error = Some(e);
            (Task::none(), None)
        }
    }
}

pub fn view(state: &LoginState) -> Element<'_, AppMsg> {
    let name_input: Element<AppMsg> = text_input("Name: ", &state.name_input_value)
        .on_input(|x| AppMsg::Login(LoginMsg::ChangedNameInput(x)))
        .into();

    let ip_input: Element<AppMsg> = text_input("ip: ", &state.ip_input_value)
        .on_input(|x| AppMsg::Login(LoginMsg::ChangedIpInput(x)))
        .into();

    let confirm_button: Element<AppMsg> = button("Connect")
        .on_press(AppMsg::Login(LoginMsg::ConnectClicked))
        .into();

    let register_button: Element<AppMsg> = button("Register")
        .on_press(AppMsg::Login(LoginMsg::RegisterClicked))
        .into();

    let error_display: Element<AppMsg> = match &state.error {
        Option::Some(s) => text(s).into(),
        Option::None => text("").into(),
    };

    return container(column([
        text("Hallo Welt").size(24).into(),
        error_display,
        ip_input,
        name_input,
        confirm_button,
        register_button,
    ]))
    .center(Length::Fill)
    .into();
}
