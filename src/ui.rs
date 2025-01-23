use crate::{blocks::Block, canvas::CanvasState, connections::Connection};
use iced::{
    Command, Element, Length, Settings, Theme,
    widget::{Canvas, button, column, container, row, text, text_input},
};
use iced_aw::menu::{Menu, MenuBar, MenuItem};

#[derive(Debug, Clone)]
pub enum Message {
    AddBlock(Block),
    SelectBlock(String),
    UpdateProperty(String, String),
    ConnectBlocks(String, String),
    Save,
    Load,
}

#[derive(Default)]
pub struct RpaEditor {
    blocks: Vec<Block>,
    connections: Vec<Connection>,
    selected_block: Option<String>,
    canvas_state: CanvasState,
}

impl RpaEditor {
    pub fn new() -> (Self, Command<Message>) {
        (
            Self {
                blocks: vec![],
                connections: vec![],
                selected_block: None,
                canvas_state: CanvasState::new(),
            },
            Command::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AddBlock(block) => {
                self.blocks.push(block);
                self.canvas_state.blocks = self.blocks.clone();
            }
            Message::SelectBlock(id) => {
                self.selected_block = Some(id);
            }
            Message::UpdateProperty(key, value) => {
                if let Some(block) = self
                    .blocks
                    .iter_mut()
                    .find(|b| b.id == self.selected_block.as_ref().unwrap())
                {
                    block.properties.insert(key, value);
                }
            }
            Message::ConnectBlocks(from, to) => {
                self.connections.push(Connection { from, to });
            }
            _ => {}
        }
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        let menu = MenuBar::new(vec![
            Menu::new("File")
                .add(MenuItem::new("Save").on_press(Message::Save))
                .add(MenuItem::new("Load").on_press(Message::Load)),
        ]);

        let blocks_panel = column![
            text("Blocks"),
            button("Click").on_press(Message::AddBlock(Block::new_click())),
            button("Mouse Move").on_press(Message::AddBlock(Block::new_mouse_move())),
            button("Screenshot").on_press(Message::AddBlock(Block::new_screenshot())),
            button("Coordinates").on_press(Message::AddBlock(Block::new_coordinates())),
        ]
        .width(200);

        let properties_editor = if let Some(id) = &self.selected_block {
            self.blocks
                .iter()
                .find(|b| &b.id == id)
                .map(|block| {
                    column![
                        text("Properties"),
                        text_input("X", &block.position.0.to_string())
                            .on_input(|v| Message::UpdateProperty("x".into(), v)),
                        text_input("Y", &block.position.1.to_string())
                            .on_input(|v| Message::UpdateProperty("y".into(), v)),
                    ]
                })
                .unwrap_or_else(|| text("Block not found").into())
        } else {
            text("Select a block").into()
        }
        .width(200);

        let canvas = Canvas::new(&self.canvas_state)
            .width(Length::Fill)
            .height(Length::Fill);

        container(column![menu, row![blocks_panel, canvas, properties_editor]])
            .padding(10)
            .into()
    }
}

pub fn run() -> iced::Result {
    let (initial_state, initial_command) = RpaEditor::new();

    iced::Settings::default().run(|state: &mut RpaEditor, message: Message| {
        let command = state.update(message);
        (state, command, |state| state.view())
    })
}
