use conrod::{widget, Colorable, Positionable, Widget};
use conrod::widget::{Common, CommonBuilder, UpdateArgs, PointPath, id};
use conrod::widget::id::{Id, Generator};
use conrod::{Ui, color};
use conrod::position::Point;
use conrod::event;
use conrod::input::Key;
use support::id::IdPool;

pub struct PaintArea {
    /// Handles some of the dirty work of rendering a GUI.
    common: CommonBuilder,
    /// See the Style struct below.
    style: Style,
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

#[derive(Copy, Clone, PartialEq)]
enum MouseState {
    None,
    Pressed,
    Cancelled,
}

/// Represents the unique, cached state for our PaintArea widget.
pub struct State {
    mouse_state: MouseState,
    id_pool: IdPool,
    ids: Ids,
    line_ids: Vec<Id>,
    lines: Vec<Vec<[f64; 2]>>,
    points: Vec<[f64; 2]>,
}

#[derive(PartialEq)]
enum PaintAction {
    Cancel,
    Press(Point),
    Drag(Point),
    Release,
}

impl PaintArea {

    pub fn new() -> Self {
        PaintArea {
            common: CommonBuilder::default(),
            style: Style::default(),
        }
    }

    fn handle_input(&self, ui: &Ui, mouse_state: MouseState) -> Option<PaintAction> {

        if let Some(key_id) = ui.global_input().current.widget_capturing_keyboard {
            'events: for widget_event in ui.widget_input(key_id).events() {
                match widget_event {
                    event::Widget::Press(press) => match press.button {
                        event::Button::Keyboard(key) => match key {
                        
                            Key::Escape => {
                                return Some(PaintAction::Cancel)
                            },
                            _ => ()
                        },
                        _ => (),
                    },
                    _ => ()
                }
            }
        }

        let press_option = ui.global_input().current.mouse.buttons.pressed().next();
        if press_option.is_some() {

            let xy = ui.global_input().current.mouse.xy;
            if mouse_state == MouseState::Cancelled {
                return None
            } else if mouse_state == MouseState::Pressed {
                return Some(PaintAction::Drag(xy))
            } else {
                return Some(PaintAction::Press(xy))
            }
        } else if (mouse_state == MouseState::Pressed || mouse_state == MouseState::Cancelled) && 
                ui.global_input().current.mouse.buttons.left().is_up() {
            return Some(PaintAction::Release)
        }
        None
    }

    fn handle_action(&self, state: &mut widget::State<<PaintArea as Widget>::State>,
                    action: PaintAction) {
        match action {
            PaintAction::Cancel => {
                state.update(|state| {
                    state.points.clear();
                    state.mouse_state = MouseState::Cancelled;
                });
            },
            PaintAction::Press(point) => {
                state.update(|state| {
                    state.points.push(point);
                    state.mouse_state = MouseState::Pressed;
                });
            },
            PaintAction::Drag(point) => {
                state.update(|state| {
                    state.points.push(point);
                });
            },
            PaintAction::Release => {

                state.update(|state| {

                    if let Some(new_id) = state.id_pool.get() {
                        if state.points.len() > 1 && state.mouse_state == MouseState::Pressed {
                            println!("Added line! {}", state.points.len());
                            state.line_ids.push(new_id);
                            state.lines.push(state.points.clone());
                        }
                    } else {
                        println!("No ids left!");
                        // TODO -- should probably panic, or alert user
                    }
                    state.mouse_state = MouseState::None;
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
        State { mouse_state: MouseState::None,
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

        if let Some(action) = self.handle_input(&ui, state.mouse_state) {

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
