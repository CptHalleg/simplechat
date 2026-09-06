use crate::core;
use crate::core::CoreMsg;
use crate::core::CoreState;
use crate::login;
use crate::login::LoginAction;
use crate::login::LoginMsg;
use crate::login::LoginState;
use crate::websocket;
use crate::websocket::Event;
use iced::Element;
use iced::Subscription;
use iced::Task;
use iced::Theme;
use iced::widget::container;
use log::debug;
use log::info;
use shared::types::ServerToClientFrame;

pub fn run() {
    info!("App starting!");

    iced::application(init, update, view)
        .title("Lumes")
        .theme(Theme::Light)
        .subscription(socket_subscription)
        .run()
        .expect("failed to start applciation");
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    Login(LoginMsg),
    Core(CoreMsg),
}

#[derive(Debug, Clone)]
pub enum AppState {
    Login(LoginState),
    Core(CoreState),
}

fn socket_subscription(_state: &AppState) -> Subscription<AppMsg> {
    Subscription::run(websocket::connect).map(|event| match event {
        websocket::Event::Initialized(websocket) => {
            AppMsg::Login(LoginMsg::WebSocketInitialized(websocket))
        }
        Event::Connected => AppMsg::Login(LoginMsg::WebsocketConnected),
        Event::Disconnected => todo!(),
        Event::MessageReceived(frame) => match frame {
            ServerToClientFrame::RecivedMessage(message) => {
                AppMsg::Core(CoreMsg::RecievedMessage(message))
            }
        },
        Event::Error(error) => AppMsg::Login(LoginMsg::Error(error)),
    })
}

fn init() -> (AppState, Task<AppMsg>) {
    debug!("initializing state");
    let state: AppState = AppState::Login(LoginState {
        websocket: None,
        error: Option::None,
        ip_input_value: String::from("127.0.0.1:6767"),
        name_input_value: String::new(),
    });
    return (state, Task::none());
}

fn update(state: &mut AppState, message: AppMsg) -> Task<AppMsg> {
    debug!(
        "processing message:\n{:#?}\ncurrent state:\n{:#?}",
        message, state
    );
    match message {
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
    debug!("render view");
    let ele = match state {
        AppState::Login(s) => login::view(s),
        AppState::Core(s) => core::view(s),
    };
    return container(ele).padding(10).into();
}
