use conrod::{color, Colorable, Positionable, widget, Widget};
use conrod::widget::{Common, CommonBuilder, UpdateArgs};
use conrod::widget::id::Generator;
use super::PaintArea;

#[derive(PartialEq)]
pub enum WindowAction {
    None,
    Quit,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
}

pub struct PaintWindow {
    /// Handles some of the dirty work of rendering a GUI.
    common: CommonBuilder,
    /// See the Style struct below.
    style: Style,
}

widget_ids! {
    struct Ids {
        background,
        paint_area,
    }
}

pub struct State {
    ids: Ids,
}

impl PaintWindow {

    pub fn new() -> Self {
        PaintWindow {
            common: CommonBuilder::default(),
            style: Style::default(),
        }
    }
}

impl Common for PaintWindow {

    fn common(&self) -> &CommonBuilder {
        &self.common
    }

    fn common_mut(&mut self) -> &mut CommonBuilder {
        &mut self.common
    }
}

impl Widget for PaintWindow {
    type State = State;
    type Style = Style;
    type Event = Option<WindowAction>;

    fn init_state<'b>(&self, id_gen: Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    fn update(self, args: UpdateArgs<Self>) -> Self::Event {
        let UpdateArgs { id, state, mut ui, .. } = args;

        widget::Canvas::new()
            .color(color::WHITE)
            .set(state.ids.background, ui);

        let event = PaintArea::new()
            .middle_of(state.ids.background)
            .set(state.ids.paint_area, ui);
        
        event
    }
}