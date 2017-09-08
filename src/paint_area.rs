
use conrod::{self, widget, Colorable, Dimensions, Labelable, Point,
                Positionable, Widget, Color, FontSize, Scalar};
use conrod::text::font::Id;
use conrod::widget::{CommonBuilder, CommonState, UpdateArgs, PointPath, Text};
use conrod::{input, Ui};

/// The type upon which we'll implement the `Widget` trait.
pub struct PaintArea<'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    common: CommonBuilder,
    /// Optional label string for the button.
    maybe_label: Option<&'a str>,
    /// See the Style struct below.
    style: Style,
    /// Whether the button is currently enabled, i.e. whether it responds to
    /// user input.
    enabled: bool
}

// We use the `widget_style!` macro to vastly simplify the definition and implementation of the
// widget's associated `Style` type. This generates both a `Style` struct, as well as an
// implementation that automatically retrieves defaults from the provided theme.
//
// See the documenation of the macro for a more details.
widget_style!{
    /// Represents the unique styling for our PaintArea widget.
    style Style {
        /// Color of the button.
        - color: Color { theme.shape_color }
        /// Color of the button's label.
        - label_color: Color { theme.label_color }
        /// Font size of the button's label.
        - label_font_size: FontSize { theme.font_size_medium }
        /// Specify a unique font for the label.
        - label_font_id: Option<Id> { theme.font_id }
    }
}

widget_ids! {
    struct Ids {
        text,
        path,
        background,
    }
}

/// Represents the unique, cached state for our PaintArea widget.
pub struct State {
    ids: Ids,
    points: Vec<[f64; 2]>,
}

/// Return whether or not a given point is over a circle at a given point on a
/// Cartesian plane. We use this to determine whether the mouse is over the button.
pub fn is_over_circ(circ_center: Point, mouse_point: Point, dim: Dimensions) -> bool {
    // Offset vector from the center of the circle to the mouse.
    let offset = conrod::utils::vec2_sub(mouse_point, circ_center);

    // If the length of the offset vector is less than or equal to the circle's
    // radius, then the mouse is inside the circle. We assume that dim is a square
    // bounding box around the circle, thus 2 * radius == dim[0] == dim[1].
    let distance = (offset[0].powf(2.0) + offset[1].powf(2.0)).sqrt();
    let radius = dim[0] / 2.0;
    distance <= radius
}

impl<'a> PaintArea<'a> {

    /// Create a button context to be built upon.
    pub fn new() -> Self {
        PaintArea {
            common: CommonBuilder::new(),
            maybe_label: None,
            style: Style::new(),
            enabled: true,
        }
    }

    /// Specify the font used for displaying the label.
    pub fn label_font_id(mut self, font_id: Id) -> Self {
        self.style.label_font_id = Some(Some(font_id));
        self
    }

    /// If true, will allow user inputs.  If false, will disallow user inputs.
    #[allow(dead_code)]
    pub fn enabled(mut self, flag: bool) -> Self {
        self.enabled = flag;
        self
    }

    fn handle_input(&self, ui: &Ui, input: input::Widget, state: &mut widget::State<<PaintArea<'a> as Widget>::State>, style: &Style)
                 -> (Color, Option<()>) {
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
        let color = style.color(&ui.theme);

        (color, event)
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a> Widget for PaintArea<'a> {
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

        let (_, event) = {
            let input = ui.widget_input(id);
            self.handle_input(&ui, input, state, style)
        };
        /*
        widget::Rectangle::fill([rect.w(), rect.h()])
            .middle_of(id)
            .graphics_for(id)
            .color(color)
            .set(state.ids.background, ui);
        */

        const SHAPE_GAP: Scalar = 50.0;

        let (_, path_event) = {
            let point_path_input = ui.widget_input(state.ids.path);
            self.handle_input(ui, point_path_input, state, style)
        };

        PointPath::abs(state.points.clone())
            .middle_of(id)
            .set(state.ids.path, ui);

        // Now we'll instantiate our label using the **Text** widget.
        if let Some(ref label) = self.maybe_label {
            let label_color = style.label_color(&ui.theme);
            let font_size = style.label_font_size(&ui.theme);
            let font_id = style.label_font_id(&ui.theme).or(ui.fonts.ids().next());
            Text::new(label)
                .and_then(font_id, Text::font_id)
                .middle_of(id)
                .font_size(font_size)
                .graphics_for(id)
                .color(label_color)
                .set(state.ids.text, ui);
        }

        event
    }

}

/// Provide the chainable color() configuration method.
impl<'a> Colorable for PaintArea<'a> {
    fn color(mut self, color: Color) -> Self {
        self.style.color = Some(color);
        self
    }
}

/// Provide the chainable label(), label_color(), and label_font_size()
/// configuration methods.
impl<'a> Labelable<'a> for PaintArea<'a> {
    fn label(mut self, text: &'a str) -> Self {
        self.maybe_label = Some(text);
        self
    }
    fn label_color(mut self, color: Color) -> Self {
        self.style.label_color = Some(color);
        self
    }
    fn label_font_size(mut self, size: FontSize) -> Self {
        self.style.label_font_size = Some(size);
        self
    }
}