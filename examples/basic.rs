extern crate dd_gui;

use dd_gui::winit;
use dd_gui::glutin;
use dd_gui::glium::{ DisplayBuild };

use dd_gui::{ Point, Rect };
use dd_gui::widgets::{ Triangle, Knob };

use dd_gui::color;

fn main() {
    let wb = winit::WindowBuilder::new()
        .with_dimensions(640, 480)
//        .with_visibility(true)
        .with_transparency(false);

    let display = glutin::WindowBuilder::from_winit_builder(wb)
        .with_decorations(true)
//        .with_dimensions(640, 480)
//         .with_vsync()
//         .with_multisampling(8)
//         .with_visibility(true)
//         .with_transparency(false)
//         .with_gl_robustness(Robustness::RobustLoseContextOnReset)
        .build_glium()
        .unwrap();

    let mut renderer = dd_gui::Renderer::new(display);

    let mut last_update = std::time::Instant::now();
    'main: loop {
        let now = std::time::Instant::now();
        let duration_since_last_update = now.duration_since(last_update);

        let sixteen_ms = std::time::Duration::from_millis(50);

        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update)
        } else {
            for event in renderer.display.poll_events() {
                match event {
                    glutin::Event::Closed => break 'main,
                    glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Escape)) => break 'main,
                    _ => ()
                }
            }

            Triangle::new(Rect{ origin: Point{ x:100.0, y:100.0 }, width:10.0, height:100.0 })
                .color(color::GREEN)
                .set(&mut renderer);

            Triangle::new(Rect{ origin: Point{ x:10.0, y:10.0 }, width:50.0, height:40.0 })
                .set(&mut renderer);

            Knob::new(Rect{ origin: Point{ x:20.0, y:20.0 }, width:80.0, height:50.0 })
                .color(color::GREEN)
                .set(&mut renderer);

            Knob::new(Rect{ origin: Point{ x:150.0, y:190.0 }, width:400.0, height:400.0 })
                .color(color::PINK)
                .set(&mut renderer);

            renderer.render();

            last_update = now;
        }
    }
}
