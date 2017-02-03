
#[macro_use] extern crate conrod;
extern crate find_folder;
mod paint_area;

#[cfg(all(feature="winit", feature="glium"))] mod support;

#[cfg(all(feature="winit", feature="glium"))]
fn main() {
    use conrod::{self, widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    use conrod::backend::glium::glium;
    use conrod::backend::glium::glium::{DisplayBuild, Surface};
    use support;
    use self::paint_area;

    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = 800;

    // Build the window.
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("Control Panel")
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
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    // Poll events from the window.
    let mut event_loop = support::EventLoop::new();
    'main: loop {

        // Handle all events.
        for event in event_loop.next(&display) {

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                // Break from the loop upon `Escape`.
                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                glium::glutin::Event::Closed =>
                    break 'main,
                _ => {},
            }
        }

        // Instantiate the widgets.
        {
           let ui = &mut ui.set_widgets();

            // Sets a color to clear the background with before the Ui draws our widget.
            widget::Canvas::new().color(conrod::color::BLACK).set(ids.background, ui);

            // Instantiate of our custom widget.
            for _click in paint_area::PaintArea::new()
                .color(conrod::color::WHITE)
                .middle_of(ids.background)
                .w_h(WIDTH as f64, HEIGHT as f64)
                .label_font_id(regular)
                .label_color(conrod::color::BLACK)
                .label("Paint Area")
                // Add the widget to the conrod::Ui. This schedules the widget it to be
                // drawn when we call Ui::draw.
                .set(ids.paint, ui)
            {
                println!("Click!");
            }
        }

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

#[cfg(not(all(feature="winit", feature="glium")))]
fn main() {
    println!("This example requires the `winit` and `glium` features. \
             Try running `cargo run --release --features=\"winit glium\" --example <example_name>`");
}