use std::sync::Arc;

use iced::Element;
use iced::Font;
use iced::Length;
use iced::Task;
use iced::font::Weight;
use iced::widget::column;
use iced::widget::mouse_area;
use iced::widget::row;
use iced::widget::scrollable;
use iced::widget::text;
use iced::widget::text_input;
use shared::types::ClientToServerFrame;

use crate::app::AppMsg;
use crate::websocket::Instruction;
use crate::websocket::WebSocket;
use shared::types::ChatMessage;
use shared::types::Room;
use shared::types::User;

#[derive(Debug, Clone)]
pub enum CoreMsg {
    ChangedMessageInput(String),
    SendMessage(),
    RecievedMessage(ChatMessage),
    SelectConversation(usize),
}

#[derive(Debug, Clone)]
pub struct CoreState {
    pub websocket: WebSocket,
    pub user: Arc<User>,
    pub conversations: Vec<Room>,
    pub current_converstaion: usize,
}

pub fn update(state: &mut CoreState, message: CoreMsg) -> Task<AppMsg> {
    match message {
        CoreMsg::ChangedMessageInput(message) => {
            match state.conversations.get_mut(state.current_converstaion) {
                Option::Some(c) => {
                    c.input_text = message;
                }
                Option::None => (),
            }
            Task::none()
        }
        CoreMsg::RecievedMessage(message) => {
            match state.conversations.get_mut(state.current_converstaion) {
                Option::Some(c) => {
                    c.messages.push(message);
                    Task::none()
                }
                Option::None => Task::none(),
            }
        }
        CoreMsg::SendMessage() => match state.conversations.get_mut(state.current_converstaion) {
            Option::Some(c) => {
                let new_message = ChatMessage {
                    content: c.input_text.clone(),
                    author: state.user.handle.clone(),
                };
                c.messages.push(new_message.clone());
                c.input_text.clear();

                let mut websocket_clone = state.websocket.clone();
                let new_message_clone = new_message.clone();
                Task::future(async move {
                    websocket_clone
                        .send(Instruction::SendMessage(ClientToServerFrame::SendMessage(
                            new_message_clone,
                        )))
                        .await;
                })
                .discard()
            }
            Option::None => Task::none(),
        },
        CoreMsg::SelectConversation(i) => {
            state.current_converstaion = i;
            Task::none()
        }
    }
}

pub fn view(state: &CoreState) -> Element<'_, AppMsg> {
    let chat = match state.conversations.get(state.current_converstaion) {
        Option::Some(c) => chat_panel(c),
        Option::None => text("please select a converstation")
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
    };
    return row([conversations_panel(state), chat]).into();
}

fn chat_panel(conversation: &Room) -> Element<'_, AppMsg> {
    let message_elements: Vec<Element<AppMsg>> =
        conversation.messages.iter().map(message).collect();

    let message_history: Element<AppMsg> = scrollable(column(message_elements))
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

    let input = text_input("Send: ", &conversation.input_text)
        .on_input(|x| AppMsg::Core(CoreMsg::ChangedMessageInput(x)))
        .on_submit(AppMsg::Core(CoreMsg::SendMessage()))
        .into();

    let header: Element<AppMsg> = text(&conversation.participant.name).into();
    return column([header, message_history, input]).into();
}

fn message(message: &ChatMessage) -> Element<'_, AppMsg> {
    let name_label = text("message.author.name.clone() :").font(Font {
        weight: Weight::Bold,
        ..Font::DEFAULT
    });
    return row([name_label.into(), text(&message.content).into()])
        .spacing(10)
        .into();
}

fn converstaion(conversation: &Room, selected: bool, index: usize) -> Element<'_, AppMsg> {
    let user = &conversation.participant;
    let title = if selected {
        ["<", &user.name, ">"].concat()
    } else {
        user.name.clone()
    };

    return mouse_area(text(title))
        .on_press(AppMsg::Core(CoreMsg::SelectConversation(index)))
        .into();
}

fn conversations_panel(state: &CoreState) -> Element<'_, AppMsg> {
    let converstaion_elements: Vec<Element<AppMsg>> = state
        .conversations
        .iter()
        .enumerate()
        .map(|(i, c)| converstaion(c, i == state.current_converstaion, i))
        .collect();
    return scrollable(column(converstaion_elements))
        .height(Length::Fill)
        .width(Length::Fixed(100.0))
        .into();
}
