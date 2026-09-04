use std::sync::Arc;
use std::sync::Mutex;

use iced::Element;
use iced::Length;
use iced::Task;
use iced::widget::button;
use iced::widget::column;
use iced::widget::container;
use iced::widget::text;
use iced::widget::text_input;

use crate::app::AppMsg;
use crate::app::AppState;
use crate::core::CoreState;
use shared::constants::PORT;
use shared::types::Room;
use shared::types::User;
use std::net::TcpStream;

pub enum LoginAction {
    ToCore(CoreState),
}

#[derive(Debug, Clone)]
pub enum LoginMsg {
    ChangedNameInput(String),
    ChangedIpInput(String),
    Connect(),
    Login(Arc<Mutex<TcpStream>>),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct LoginState {
    pub error: Option<String>,
    pub name_input_value: String,
    pub ip_input_value: String,
}

pub fn update(state: &mut LoginState, message: LoginMsg) -> (Task<LoginMsg>, Option<LoginAction>) {
    match message {
        LoginMsg::ChangedNameInput(s) => {
            state.name_input_value = s;
            return (Task::none(), None);
        }
        LoginMsg::ChangedIpInput(s) => {
            state.ip_input_value = s;
            return (Task::none(), None);
        }
        LoginMsg::Connect() => {
            if state.name_input_value.len() < 1 {
                return (
                    Task::done(LoginMsg::Error(String::from("Name cant be Empty"))),
                    None,
                );
            }
            let ip = state.ip_input_value.clone();
            let future = async move {
                match TcpStream::connect(format!("{}:{}", ip, PORT)) {
                    Ok(stream) => {
                        return LoginMsg::Login(Arc::new(Mutex::new(stream)));
                    }
                    Err(error) => {
                        return LoginMsg::Error(String::from(error.to_string()));
                    }
                };
            };
            return (Task::future(future), None);
        }
        LoginMsg::Login(stream) => {
            let converstations = vec![
                Room {
                    participant: Arc::new(User {
                        name: String::from("Peter"),
                    }),
                    messages: Vec::new(),
                    input_text: String::new(),
                },
                Room {
                    participant: Arc::new(User {
                        name: String::from("Jonas"),
                    }),
                    messages: Vec::new(),
                    input_text: String::new(),
                },
                Room {
                    participant: Arc::new(User {
                        name: String::from("Olaf"),
                    }),
                    messages: Vec::new(),
                    input_text: String::new(),
                },
            ];
            let new_state = CoreState {
                stream: stream,
                user: Arc::new(User {
                    name: String::from(&state.name_input_value),
                }),
                conversations: converstations,
                current_converstaion: 0,
            };
            return (Task::none(), Some(LoginAction::ToCore(new_state)));
        }
        LoginMsg::Error(e) => {
            state.error = Some(e);
            return (Task::none(), None);
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
        .on_press(AppMsg::Login(LoginMsg::Connect()))
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
    ]))
    .center(Length::Fill)
    .into();
}
