use crate::{blocks::Block, ui::Message};
use iced::{
    Color, Point, Rectangle, Renderer, Size, Theme, Vector,
    widget::canvas::{self, Cache, Frame, Program},
};

#[derive(Debug, Default)]
pub struct CanvasState {
    pub blocks: Vec<Block>,
    pub drag_state: DragState,
    cache: Cache,
}

#[derive(Debug, Default)]
pub enum DragState {
    #[default]
    Idle,
    DraggingBlock(String, Vector),
}

impl CanvasState {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            drag_state: DragState::Idle,
            cache: Cache::default(),
        }
    }
}

impl Program<Message> for CanvasState {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let geometry = self
            .cache
            .draw(renderer, bounds.size(), |frame: &mut Frame| {
                for block in &self.blocks {
                    let position = Point::new(block.position.0, block.position.1);
                    frame.fill_rectangle(
                        position,
                        Size::new(100.0, 60.0),
                        Color::from_rgb8(0x70, 0x80, 0x90),
                    );
                }
            });

        vec![geometry]
    }
}
