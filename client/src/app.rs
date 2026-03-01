use crate::core;
use crate::core::CoreMsg;
use crate::core::CoreState;
use crate::login;
use crate::login::LoginMsg;
use crate::login::LoginState;
use iced::Element;
use iced::Task;
use iced::Theme;
use iced::widget::container;

pub fn run() {
    iced::application(init, update, view)
        .title("Lumes")
        .theme(Theme::Dark)
        .run()
        .expect("failed to start applciation");
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    ChangeState(AppState),
    Login(LoginMsg),
    Core(CoreMsg),
}

#[derive(Debug, Clone)]
pub enum AppState {
    Login(LoginState),
    Core(CoreState),
}

fn init() -> (AppState, Task<AppMsg>) {
    let state: AppState = AppState::Login(LoginState {
        error: Option::None,
        ip_input_value: String::from("127.0.0.1"),
        name_input_value: String::new(),
    });
    return (state, Task::none());
}

fn update(state: &mut AppState, message: AppMsg) -> Task<AppMsg> {
    match message {
        AppMsg::ChangeState(n) => {
            *state = n;
            return Task::none();
        }
        _ => match (state, message) {
            (AppState::Login(s), AppMsg::Login(m)) => login::update(s, m),
            (AppState::Core(s), AppMsg::Core(m)) => core::update(s, m),
            (s, m) => panic!("Invalid State and Message: {:#?}, {:#?} ", s, m),
        },
    }
}

fn view(state: &AppState) -> Element<'_, AppMsg> {
    let ele = match state {
        AppState::Login(s) => login::view(s),
        AppState::Core(s) => core::view(s),
    };
    return container(ele).padding(10).into();
}
