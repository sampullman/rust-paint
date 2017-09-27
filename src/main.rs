
#[macro_use] extern crate conrod;
#[macro_use] extern crate conrod_derive;
extern crate find_folder;

#[cfg(all(feature="winit", feature="glium"))] mod support;

pub mod paint;

use conrod::backend::glium::Renderer;
use conrod::glium;
use conrod::glium::Display;
use conrod::glium::texture::Texture2d;
use conrod::glium::glutin::{ContextBuilder, WindowBuilder, EventsLoop, VirtualKeyCode};
use conrod::image::Map;
use conrod::input::keyboard;
use conrod::backend::glium::glium::Surface;
use conrod::backend::winit::convert_event;
use support::EventLoop;
use paint::PaintWindow;
use paint::WindowAction;

#[cfg(all(feature="winit", feature="glium"))]
fn main() {
    use conrod::{self, Sizeable, Widget};

    const WIDTH: u32 = 1000;
    const HEIGHT: u32 = 600;

    // Build the window.
    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("Rust Paint");
    let context = ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);

    let display = Display::new(window, context, &events_loop).unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // The `widget_ids` macro is a easy, safe way of generating a type for producing `widget::Id`s.
    widget_ids! {
        struct Ids {
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

    let mut cmd_pressed = false; // TODO1 -- horribly hacky way of dealing with Mac cmd modifier
    'main: loop {

        let action = handle_events(&mut ui, &display, &mut events_loop, cmd_pressed);
        match action {
            WindowAction::Quit => {
                println!("QUIT");
                break 'main
            },
            // TODO1 -- remove when possible
            WindowAction::CmdPress => { cmd_pressed = true; },
            WindowAction::CmdRelease => { cmd_pressed = false; },
            _ => ()
        }

        {
           let ui = &mut ui.set_widgets();

            for action in PaintWindow::new()
                .w_h(WIDTH as f64, HEIGHT as f64)
                .set(ids.paint, ui)
            {
                println!("Click! {:?}", action);
            }
        }

        render(&mut ui, &mut renderer, &display, &image_map);
    }
}

fn handle_events(ui: &mut conrod::Ui, display: &Display, mut events_loop: &mut EventsLoop, cmd_pressed: bool)
        -> WindowAction {
    // Handle all events.
    let mut event_loop = EventLoop::new();
    for event in event_loop.next(&mut events_loop) {

        // Use the `winit` backend feature to convert the winit event to a conrod one.
        if let Some(event) = convert_event(event.clone(), display) {
            ui.handle_event(event);
            event_loop.needs_update();
        }

        match event {
            glium::glutin::Event::WindowEvent { event, .. } => match event {
                // Break from the loop upon `Escape`.
                glium::glutin::WindowEvent::Closed => {
                    return WindowAction::Quit
                },
                glium::glutin::WindowEvent::KeyboardInput {
                    input: glium::glutin::KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        state: glium::glutin::ElementState::Pressed,
                        ..
                    },
                    ..
                } => (),
                glium::glutin::WindowEvent::KeyboardInput {
                    input: glium::glutin::KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Q),
                        state: glium::glutin::ElementState::Pressed,
                        ..
                    },
                    ..
                } => {
                    println!("{:?}", ui.global_input().current.touch);
                    if ui.global_input().current.modifiers.contains(keyboard::CTRL) || cmd_pressed {
                        return WindowAction::Quit
                    }
                    ()
                },
                // TODO1 -- remove when possible
                glium::glutin::WindowEvent::KeyboardInput {
                    input: glium::glutin::KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::LWin),
                        state,
                        ..
                    },
                    ..
                } => {
                    if state == glium::glutin::ElementState::Pressed {
                        return WindowAction::CmdPress
                    } else {
                        return WindowAction::CmdRelease
                    }   
                }
                _ => (),
            },
            _ => (),
        }
    }
    return WindowAction::None
}

fn render(ui: &mut conrod::Ui, renderer: &mut Renderer, display: &Display,
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