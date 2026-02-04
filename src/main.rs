use std::sync::Arc;
use std::num::NonZeroU32;

use winit::event_loop::{EventLoop, ActiveEventLoop};
use winit::window::{Window, WindowId};
use winit::dpi::{LogicalSize};
use winit::event::WindowEvent;
use winit::application::ApplicationHandler;

use softbuffer::{Context, Surface};

struct App {
    state: Option<GraphicsState>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(
            Window::default_attributes()
                .with_title("Window App")
                .with_inner_size(LogicalSize::new(800.0, 600.0)),
        ).expect("failed to create window");

        let state = GraphicsState::new(window);
        state.window.request_redraw();
        self.state = Some(state);

    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let Some(state) = self.state.as_mut() else { return };
        if state.window.id() != window_id { return }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                state.render();
            }
            WindowEvent::Resized(_) => {
                state.window.request_redraw();
            }
            _ => {
                println!("Got event: {:?}", event);
            } // ignore all other events
        }
    }
}

struct GraphicsState {
    window: Arc<Window>,
    _context: Context<Arc<Window>>,
    surface: Surface<Arc<Window>, Arc<Window>>,
}

impl GraphicsState {
    fn new(window: Window) -> Self {
        let window = Arc::new(window);
        let context = Context::new(window.clone())
            .expect("failed to create context");
        let surface = Surface::new(&context, window.clone())
            .expect("failed to create window surface");

        Self {
            window,
            _context: context,
            surface,
        }
    }

    fn render(&mut self) {
        let size = self.window.inner_size();
        let w = NonZeroU32::new(size.width.max(1)).unwrap();
        let h = NonZeroU32::new(size.height.max(1)).unwrap();

        self.surface.resize(w, h).expect("resize failed");

        let mut buffer = self.surface.buffer_mut().expect("buffer failed");
        buffer.fill(0x00202020);

        let width = w.get() as usize;
        let height = h.get() as usize;
        
        let mw = width * 10 / 100;
        let mh = height * 10 / 100;

        for y in 0..mh {
            for x in 0..mw {
                buffer[y * width + x] = 0x00FF00FF;
            }
        }

        buffer.present().expect("present failed");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App { state: None };
    let event_loop = EventLoop::new()?;

    event_loop.run_app(&mut app)?;

    Ok(())
}
