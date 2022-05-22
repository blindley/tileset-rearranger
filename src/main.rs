mod tileset_renderer;
mod rectangle_renderer;

use tileset_renderer::TilesetRenderer;
use rectangle_renderer::RectangleRenderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
    .with_title("fuzzy pickles");
    
    let wc = glutin::ContextBuilder::new().build_windowed(wb, &el).unwrap();
    let wc = unsafe { wc.make_current().unwrap() };
    
    gl::load_with(|p| wc.get_proc_address(p) as *const _);
    
    let mut app_data = {
        AppData::new()
    };

    el.run(move |event, _, control_flow| {
        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;

        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,

            Event::NewEvents(cause) => match cause {
                _ => (),
            },

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    wc.resize(physical_size);
                    app_data.resize_window([physical_size.width as _, physical_size.height as _]);
                },

                WindowEvent::KeyboardInput { input, .. } => {
                    use glutin::event::VirtualKeyCode::{Escape};
                    match input.virtual_keycode {
                        Some(Escape) => *control_flow = ControlFlow::Exit,
                        _ => (),
                    }
                },

                _ => (),
            },
            Event::RedrawRequested(_) => {
                app_data.redraw();
                wc.swap_buffers().unwrap();
            },

            _ => (),
        }
    });
}

#[derive(Debug)]
struct AppData {
    window_size: [i32;2],
    tileset_renderer: TilesetRenderer,
    rectangle_renderer: RectangleRenderer,
}

impl AppData {
    fn new() -> AppData {
        let tileset_renderer = TilesetRenderer::new();

        let rect = rectangle_renderer::Rectangle {
            coords: [-0.75, -0.5, 0.75, 0.5],
            color: [1.0, 1.0, 0.2, 1.0],
            style: rectangle_renderer::RectStyle::Solid,
        };

        let rectangle_renderer = RectangleRenderer::new(rect);
    
        let app_data = AppData {
            window_size: [1,1],
            tileset_renderer,
            rectangle_renderer,
        };

        app_data
    }

    fn redraw(&self) {
        self.tileset_renderer.render();
        self.rectangle_renderer.render();
    }

    fn resize_window(&mut self, size: [i32;2]) {
        self.window_size = size;
        self.tileset_renderer.on_window_resize(size);
        self.rectangle_renderer.on_window_resize(size);
        
        unsafe { gl::Viewport(0, 0, size[0], size[1]); }
    }

}
