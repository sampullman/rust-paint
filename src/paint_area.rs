use conrod::{widget, Colorable, Positionable, Widget};
use conrod::widget::{Common, CommonBuilder, UpdateArgs, PointPath, id};
use conrod::widget::id::{Id, Generator};
use conrod::{Ui, color};
use conrod::position::Point;
use support::id::IdPool;

pub struct PaintArea {
    /// Handles some of the dirty work of rendering a GUI.
    common: CommonBuilder,
    /// See the Style struct below.
    style: Style,
    action: Action,
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
    mouse_pressed: bool,
    id_pool: IdPool,
    ids: Ids,
    line_ids: Vec<Id>,
    lines: Vec<Vec<[f64; 2]>>,
    points: Vec<[f64; 2]>,
}

#[derive(PartialEq)]
enum PaintAction {
    Press(Point),
    Drag(Point),
    Release,
}

#[derive(PartialEq)]
pub enum Action {
    None,
    Quit,
    Cancel,
}

impl PaintArea {

    pub fn new(action: Action) -> Self {
        PaintArea {
            common: CommonBuilder::default(),
            style: Style::default(),
            action: action,
        }
    }

    fn handle_input(&self, ui: &Ui, mouse_pressed: bool) -> Option<PaintAction> {

        let press_option = ui.global_input().current.mouse.buttons.pressed().next();
        if press_option.is_some() {
            let xy = ui.global_input().current.mouse.xy;
            if mouse_pressed {
                return Some(PaintAction::Drag(xy))
            } else {
                return Some(PaintAction::Press(xy))
            }
        } else if mouse_pressed && ui.global_input().current.mouse.buttons.left().is_up() {
            return Some(PaintAction::Release)
        }
        None
    }

    fn handle_action(&self, state: &mut widget::State<<PaintArea as Widget>::State>,
                    action: PaintAction) {
        match action {
            PaintAction::Press(point) => {
                state.update(|state| {
                    state.points.push(point);
                    state.mouse_pressed = true;
                });
            },
            PaintAction::Drag(point) => {
                state.update(|state| {
                    state.points.push(point);
                });
            },
            PaintAction::Release => {

                println!("Added line!");
                state.update(|state| {
                    state.mouse_pressed = false;

                    if let Some(new_id) = state.id_pool.get() {
                        state.line_ids.push(new_id);
                        state.lines.push(state.points.clone());
                    } else {
                        println!("No ids left!");
                        // TODO -- should probably panic, or alert user
                    }
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
        State { mouse_pressed: false,
                id_pool: IdPool::new(),
                ids: Ids::new(id_gen),
                line_ids: vec![],
                lines: vec![],
                points: vec![] }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    fn update(self, args: UpdateArgs<Self>) -> Self::Event {
        let UpdateArgs { id, mut state, mut ui, .. } = args;

        if self.action == Action::Cancel {
            println!("CANCEL");
        }

        if let Some(action) = self.handle_input(&ui, state.mouse_pressed) {

            // Make sure we have enough Ids in the pool, in case a Widget is created
            state.update(|state| {
                state.id_pool.repopulate(&mut ui.widget_id_generator());
            });

            self.handle_action(&mut state, action);
        }
        /*
        widget::Rectangle::fill([rect.w(), rect.h()])
            .middle_of(id)
            .graphics_for(id)
            .color(color)
            .set(state.ids.background, ui);
        */

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
