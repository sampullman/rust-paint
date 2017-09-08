
use conrod::{widget, Colorable, Positionable, Widget, Scalar};
use conrod::text::font::Id;
use conrod::widget::{CommonBuilder, UpdateArgs, PointPath};
use conrod::{input, Ui, color};

pub struct PaintArea {
    /// Handles some of the dirty work of rendering a GUI.
    common: CommonBuilder,
    /// See the Style struct below.
    style: Style
}

// This generates both a `Style` struct and animplementation that automatically
// retrieves defaults from the provided theme.
widget_style!{
    /// Represents the unique styling for our PaintArea widget.
    style Style {
    }
}

widget_ids! {
    struct Ids {
        path,
        background,
    }
}

/// Represents the unique, cached state for our PaintArea widget.
pub struct State {
    ids: Ids,
    points: Vec<[f64; 2]>,
}

impl PaintArea {

    /// Create a button context to be built upon.
    pub fn new() -> Self {
        PaintArea {
            common: CommonBuilder::new(),
            style: Style::new(),
        }
    }

    fn handle_input(&self, input: input::Widget, state: &mut widget::State<<PaintArea as Widget>::State>)
                 -> Option<()> {
        // If the button was clicked, produce `Some` event.
        let event = input.clicks().left().next().map(|_| ());

        let drag_option = input.drags().left().next();
        if let Some(drag) = drag_option {

            state.update(|state| {
                state.points.push(drag.from);
                state.points.push(drag.to);
            });
            println!("Drag {:?}", drag.to);
        }

        event
    }
}

/// A custom Conrod widget must implement the Widget trait
impl Widget for PaintArea {
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when clicked, otherwise `None`.
    type Event = Option<()>;

    fn common(&self) -> &CommonBuilder {
        &self.common
    }

    fn common_mut(&mut self) -> &mut CommonBuilder {
        &mut self.common
    }

    fn init_state<'b>(&self, id_gen: widget::id::Generator) -> Self::State {
        State { ids: Ids::new(id_gen), points: vec![] }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    fn update(self, args: UpdateArgs<Self>) -> Self::Event {
        let UpdateArgs { id, mut state, rect, mut ui, style, .. } = args;

        let event = {
            let input = ui.widget_input(id);
            self.handle_input(input, state)
        };
        /*
        widget::Rectangle::fill([rect.w(), rect.h()])
            .middle_of(id)
            .graphics_for(id)
            .color(color)
            .set(state.ids.background, ui);
        */

        const SHAPE_GAP: Scalar = 50.0;

        let _ = {
            let point_path_input = ui.widget_input(state.ids.path);
            self.handle_input(point_path_input, state)
        };

        PointPath::abs(state.points.clone())
            .middle_of(id)
            .color(color::BLACK)
            .set(state.ids.path, ui);

        event
    }

}
