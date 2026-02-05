use std::num::NonZeroU32;
use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use softbuffer::{Context, Surface};

struct App {
    state: Option<GraphicsState>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Window App")
                    .with_inner_size(LogicalSize::new(800.0, 600.0)),
            )
            .expect("failed to create window");

        let state = GraphicsState::new(window);
        state.window.request_redraw();
        self.state = Some(state);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = self.state.as_mut() else {
            return;
        };
        if state.window.id() != window_id {
            return;
        }

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
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state.is_pressed() {
                    match event.physical_key {
                        PhysicalKey::Code(KeyCode::KeyW) => {
                            state.square_pos_y =
                                state.square_pos_y.saturating_sub(20);
                            state.window.request_redraw();
                        }
                        PhysicalKey::Code(KeyCode::KeyA) => {
                            state.square_pos_x =
                                state.square_pos_x.saturating_sub(20);
                            state.window.request_redraw();
                        }
                        PhysicalKey::Code(KeyCode::KeyS) => {
                            let (_, max_pos_y) = state.max_square_pos();

                            state.square_pos_y += 20;
                            state.square_pos_y =
                                state.square_pos_y.min(max_pos_y);
                            state.window.request_redraw();
                        }
                        PhysicalKey::Code(KeyCode::KeyD) => {
                            let (max_pos_x, _) = state.max_square_pos();

                            state.square_pos_x += 20;
                            state.square_pos_x =
                                state.square_pos_x.min(max_pos_x);
                            state.window.request_redraw();
                        }
                        _ => {}
                    }
                }
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
    square_pos_x: usize,
    square_pos_y: usize,
}

impl GraphicsState {
    fn new(window: Window) -> Self {
        let window = Arc::new(window);
        let context =
            Context::new(window.clone()).expect("failed to create context");
        let surface = Surface::new(&context, window.clone())
            .expect("failed to create window surface");

        let size = window.inner_size();
        let w = NonZeroU32::new(size.width.max(1)).unwrap();
        let h = NonZeroU32::new(size.height.max(1)).unwrap();

        let width = w.get() as usize;
        let height = h.get() as usize;

        let mw = width * 10 / 100;
        let mh = height * 10 / 100;

        let square_pos_x = width / 2 - mw / 2;
        let square_pos_y = height / 2 - mh / 2;

        Self {
            window,
            _context: context,
            surface,
            square_pos_x: square_pos_x,
            square_pos_y: square_pos_y,
        }
    }

    fn render(&mut self) {
        let size = self.window.inner_size();
        let w = NonZeroU32::new(size.width.max(1)).unwrap();
        let h = NonZeroU32::new(size.height.max(1)).unwrap();

        self.surface
            .resize(w, h)
            .expect("resize failed");

        let mut buffer = self
            .surface
            .buffer_mut()
            .expect("buffer failed");
        buffer.fill(0x00202020);

        let width = w.get() as usize;
        let height = h.get() as usize;

        let mw = width * 10 / 100;
        let mh = height * 10 / 100;

        let square_start_x = self.square_pos_x.min(width);
        let square_end_x = (square_start_x + mw).min(width);

        let square_start_y = self.square_pos_y.min(height);
        let square_end_y = (square_start_y + mh).min(height);

        for y in square_start_y..square_end_y {
            for x in square_start_x..square_end_x {
                buffer[y * width + x] = 0x00FF00FF;
            }
        }

        buffer
            .present()
            .expect("present failed");
    }

    fn max_square_pos(&self) -> (usize, usize) {
        let size = self.window.inner_size();
        let w = NonZeroU32::new(size.width.max(1)).unwrap();
        let h = NonZeroU32::new(size.height.max(1)).unwrap();

        let width = w.get() as usize;
        let height = h.get() as usize;

        let mw = width * 10 / 100;
        let mh = height * 10 / 100;

        let max_pos_x = width - mw;
        let max_pos_y = height - mh;

        return (max_pos_x, max_pos_y);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App { state: None };
    let event_loop = EventLoop::new()?;

    event_loop.run_app(&mut app)?;

    Ok(())
}
