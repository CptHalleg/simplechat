use crate::connection;
use crate::connection::connect;
use crate::core;
use crate::core::CoreMsg;
use crate::core::CoreState;
use crate::login;
use crate::login::LoginAction;
use crate::login::LoginMsg;
use crate::login::LoginState;
use iced::Element;
use iced::Subscription;
use iced::Task;
use iced::Theme;
use iced::futures::Stream;
use iced::widget::container;

pub fn run() {
    iced::application(init, update, view)
        .title("Lumes")
        .theme(Theme::Dark)
        .subscription(socket_subscription)
        .run()
        .expect("failed to start applciation");
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    Test,
    Login(LoginMsg),
    Core(CoreMsg),
}

#[derive(Debug, Clone)]
pub enum AppState {
    Login(LoginState),
    Core(CoreState),
}

fn socket_subscription(state: &AppState) -> Subscription<AppMsg> {
    match state {
        AppState::Core(_) => println!("subscription: core"),
        AppState::Login(_) => println!("subscription: login"),
    }

    Subscription::run(connection::connect).map(|_| AppMsg::Test)
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
        AppMsg::Test => Task::none(),
        AppMsg::Login(m) => match state {
            AppState::Login(s) => {
                let (task, action) = login::update(s, m);
                match action {
                    None => (),
                    Some(LoginAction::ToCore(core_state)) => *state = AppState::Core(core_state),
                }
                task.map(|x| AppMsg::Login(x))
            }
            s => panic!("Invalid State and Message: {:#?}, Login", s),
        },
        AppMsg::Core(m) => match state {
            AppState::Core(s) => core::update(s, m),
            s => panic!("Invalid State and Message: {:#?}, Core", s),
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
