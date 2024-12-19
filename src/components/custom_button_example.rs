use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::border;
use iced::mouse;
use iced::{Color, Element, Length, Rectangle, Size};

pub struct ProgramsButton {
    border_radius: f32,
    width: f32,
    color: Color,
}

impl ProgramsButton {
    pub fn new(border_radius: f32, width: f32, color: Color) -> Self {
        Self {
            border_radius,
            width,
            color,
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for ProgramsButton
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }
    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(self.width, self.width))
    }
    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: border::rounded(self.border_radius),
                ..renderer::Quad::default()
            },
            self.color,
        );
    }
}

impl<Message, Theme, Renderer> From<ProgramsButton> for Element<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(programs_button: ProgramsButton) -> Self {
        Self::new(programs_button)
    }
}
