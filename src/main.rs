
#[macro_use] extern crate conrod;
extern crate find_folder;
mod paint_area;

#[cfg(all(feature="winit", feature="glium"))] mod support;

use conrod::backend::glium::Renderer;
use conrod::glium::glutin::{WindowBuilder};
use conrod::glium::backend::glutin_backend::GlutinFacade;
use conrod::glium::texture::Texture2d;
use conrod::glium::glutin::Event::KeyboardInput;
use conrod::glium::glutin::Event;
use conrod::glium::glutin::VirtualKeyCode;
use conrod::image::Map;
use conrod::backend::glium::glium::{DisplayBuild, Surface};
use conrod::backend::winit::convert as convert_event;
use support::EventLoop;

#[cfg(all(feature="winit", feature="glium"))]
fn main() {
    use conrod::{self, color, widget, Colorable, Positionable, Sizeable, Widget};
    use self::paint_area;

    const WIDTH: u32 = 1000;
    const HEIGHT: u32 = 600;

    // Build the window.
    let display = WindowBuilder::new()
        .with_vsync()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("Rust Paint")
        .build_glium()
        .unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // The `widget_ids` macro is a easy, safe way of generating a type for producing `widget::Id`s.
    widget_ids! {
        struct Ids {
            // An ID for the background widget, upon which we'll place our custom button.
            background,
            // The WidgetId we'll use to plug our widget into the `Ui`.
            paint,
        }
    }
    let ids = Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    let regular = ui.fonts.insert_from_file(font_path).unwrap();

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = Map::<Texture2d>::new();

    // Poll events from the window.
    let mut event_loop = EventLoop::new();
    'main: loop {

        if handle_events(&mut ui, &display, &mut event_loop) {
            break 'main
        }

        // Instantiate the widgets.
        {
           let ui = &mut ui.set_widgets();

            // Sets a color to clear the background with before the Ui draws our widget.
            widget::Canvas::new()
                .color(color::WHITE)
                .set(ids.background, ui);

            // Instantiate of our custom widget.
            for _click in paint_area::PaintArea::new()
                .middle_of(ids.background)
                .w_h(WIDTH as f64, HEIGHT as f64)
                .set(ids.paint, ui)
            {
                println!("Click!");
            }
        }

        render(&mut ui, &mut renderer, &display, &image_map);
    }
}

fn handle_events(ui: &mut conrod::Ui, display: &GlutinFacade, event_loop: &mut EventLoop)
        -> bool {
    // Handle all events.
    for event in event_loop.next(display) {

        // Use the `winit` backend feature to convert the winit event to a conrod one.
        if let Some(event) = convert_event(event.clone(), display) {
            ui.handle_event(event);
            event_loop.needs_update();
        }

        match event {
            // Break from the loop upon `Escape`.
            KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) |
            Event::Closed =>
                return true,
            _ => {},
        }
    }
    return false
}

fn render(ui: &mut conrod::Ui, renderer: &mut Renderer, display: &GlutinFacade,
            image_map: &Map<Texture2d>) {
                
    if let Some(primitives) = ui.draw_if_changed() {
        renderer.fill(&display, primitives, &image_map);
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        renderer.draw(display, &mut target, image_map).unwrap();
        target.finish().unwrap()
    }
}

#[cfg(not(all(feature="winit", feature="glium")))]
fn main() {
    println!("This example requires the `winit` and `glium` features. \
             Try running `cargo run --release --features=\"winit glium\"`");
}