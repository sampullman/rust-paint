use conrod::{widget, Colorable, Positionable, Widget, Scalar};
use conrod::text::font::Id;
use conrod::widget::{Common, CommonBuilder, UpdateArgs, PointPath, id};
use conrod::widget::id::Generator;
use conrod::{input, Ui, color};
use conrod::event::Widget::{Press, Drag, Release};
use conrod::event::Button::Mouse;
use conrod::input::MouseButton::Left;
use conrod::position::Point;
use std::cell::RefCell;

pub struct PaintArea {
    /// Handles some of the dirty work of rendering a GUI.
    common: CommonBuilder,
    /// See the Style struct below.
    style: Style
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
}

widget_ids! {
    struct Ids {
        background,
        active,
    }
}

/// Represents the unique, cached state for our PaintArea widget.
pub struct State {
    ids: Ids,
    line_ids: id::List,
    lines: Vec<Vec<[f64; 2]>>,
    points: Vec<[f64; 2]>,
}

#[derive(PartialEq)]
enum PaintAction {
    Press(Point),
    Drag(Point),
    Release,
}

impl PaintArea {

    /// Create a button context to be built upon.
    pub fn new() -> Self {
        PaintArea {
            common: CommonBuilder::default(),
            style: Style::default(),
        }
    }

    fn handle_input(&self, ui: &Ui, id: id::Id) -> Option<PaintAction> {
        let mut input = ui.widget_input(id);

        let drag_option = input.drags().left().next();
        if let Some(drag) = drag_option {
            println!("Drag {:?}", drag.to);
            return Some(PaintAction::Drag(drag.to));
        }

        let press_option = input.presses().mouse().left().next();
        if let Some(press) = press_option {
            println!("Press {:?}", press.0);
            return Some(PaintAction::Press(press.0));
            return None
        }

        let release_option = input.releases().mouse().left().next();
        if let Some(release) = release_option {
            println!("Release!");
            return Some(PaintAction::Release)
        }
        None
    }

    fn resize(&self, list: &mut id::List, new_id: id::Id) {

    }

    fn handle_action(&self, state: &mut widget::State<<PaintArea as Widget>::State>,
                    action: PaintAction) {
        match action {
            PaintAction::Press(point) => {
                state.update(|state| {
                    state.points.push(point);
                });
            },
            PaintAction::Drag(point) => {
                state.update(|state| {
                    state.points.push(point);
                });
            },
            PaintAction::Release => {

                println!("Added line2!");
                state.update(|state| {
                    state.lines.push(state.points.clone());
                    state.points.clear();
                });
            }
        }
    }
}

impl Common for PaintArea {

    fn common(&self) -> &CommonBuilder {
        &self.common
    }

    fn common_mut(&mut self) -> &mut CommonBuilder {
        &mut self.common
    }
}

/// A custom Conrod widget must implement the Widget trait
impl Widget for PaintArea {
    type State = State;
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when clicked, otherwise `None`.
    type Event = Option<()>;

    fn init_state<'b>(&self, id_gen: Generator) -> Self::State {
        State {
                ids: Ids::new(id_gen),
                line_ids: id::List::new(),
                lines: vec![vec![]],
                points: vec![] }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    fn update(self, args: UpdateArgs<Self>) -> Self::Event {
        let UpdateArgs { id, mut state, mut ui, .. } = args;

        if let Some(action) = self.handle_input(&ui, id) {
            if action == PaintAction::Release {

                println!("Added line1!");
                state.update(|state| {
                    let len = state.lines.len()+1;
                    state.line_ids.resize(len, &mut ui.widget_id_generator());
                });
            }
            self.handle_action(&mut state, action);
        }
        /*
        widget::Rectangle::fill([rect.w(), rect.h()])
            .middle_of(id)
            .graphics_for(id)
            .color(color)
            .set(state.ids.background, ui);
        */

        let mut actions: Vec<PaintAction> = vec![];
        for i in 0..state.line_ids.len() {

            let line_id = state.line_ids[i];
            let line = &state.lines[i];

            if let Some(action) = self.handle_input(&ui, line_id) {
                actions.push(action);
            }                         
        }
        for action in actions.into_iter() {
            if action == PaintAction::Release {
                state.update(|state| {
                    let len = state.lines.len()+1;
                    state.line_ids.resize(len, &mut ui.widget_id_generator());
                });
            }
            self.handle_action(&mut state, action);
        }
        for (line, &line_id) in state.lines.iter().zip(state.line_ids.iter()) {
            PointPath::abs(line.clone())
                .middle_of(id)
                .color(color::BLACK)
                .set(line_id, ui);
        }
        PointPath::abs(state.points.clone())
                .middle_of(id)
                .color(color::BLACK)
                .set(state.ids.active, ui);

        None
    }

}
