use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Instant;

use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use softbuffer::{Context, Surface};

struct App {
    gfx_state: Option<GraphicsState>,
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
        self.gfx_state = Some(state);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(gfx_state) = self.gfx_state.as_mut() else {
            return;
        };
        if gfx_state.window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                gfx_state.render();
            }
            WindowEvent::Resized(_) => {
                gfx_state.window.request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state.is_pressed() {
                    match event.physical_key {
                        PhysicalKey::Code(KeyCode::KeyW) => {
                            gfx_state.square_pos_y -= 20.0;
                            gfx_state.square_pos_y =
                                gfx_state.square_pos_y.max(0.0);
                            gfx_state.window.request_redraw();
                        }
                        PhysicalKey::Code(KeyCode::KeyA) => {
                            gfx_state.square_pos_x -= 20.0;
                            gfx_state.square_pos_x =
                                gfx_state.square_pos_x.max(0.0);
                            gfx_state.window.request_redraw();
                        }
                        PhysicalKey::Code(KeyCode::KeyS) => {
                            let (_, max_pos_y) = gfx_state.max_square_pos();

                            gfx_state.square_pos_y += 20.0;
                            gfx_state.square_pos_y =
                                gfx_state.square_pos_y.min(max_pos_y);
                            gfx_state.window.request_redraw();
                        }
                        PhysicalKey::Code(KeyCode::KeyD) => {
                            let (max_pos_x, _) = gfx_state.max_square_pos();

                            gfx_state.square_pos_x += 20.0;
                            gfx_state.square_pos_x =
                                gfx_state.square_pos_x.min(max_pos_x);
                            gfx_state.window.request_redraw();
                        }
                        _ => {}
                    }
                }
            }
            WindowEvent::MouseInput { button, state, .. } => {
                if state.is_pressed() {
                    match button {
                        MouseButton::Left => {
                            let x = gfx_state.cursor_x as usize;
                            let y = gfx_state.cursor_y as usize;

                            let size = gfx_state.window.inner_size();
                            let w = NonZeroU32::new(size.width.max(1)).unwrap();
                            let h =
                                NonZeroU32::new(size.height.max(1)).unwrap();

                            let width = w.get() as usize;
                            let height = h.get() as usize;

                            let mw = width * 10 / 100;
                            let mh = height * 10 / 100;

                            gfx_state.square_pos_x =
                                (x as f64) - (mw as f64 / 2.0);
                            gfx_state.square_pos_y =
                                (y as f64) - (mh as f64 / 2.0);
                            gfx_state.window.request_redraw();
                        }
                        _ => {}
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                gfx_state.cursor_x = position.x;
                gfx_state.cursor_y = position.y;
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
    square_pos_x: f64,
    square_pos_y: f64,
    velocity_x: f64,
    velocity_y: f64,
    last_time_frame: Instant,
    cursor_x: f64,
    cursor_y: f64,
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

        let square_pos_x = (width / 2 - mw / 2) as f64;
        let square_pos_y = (height / 2 - mh / 2) as f64;

        Self {
            window,
            _context: context,
            surface,
            square_pos_x: square_pos_x,
            square_pos_y: square_pos_y,
            velocity_x: 100.0,
            velocity_y: 100.0,
            last_time_frame: Instant::now(),
            cursor_x: 0.0,
            cursor_y: 0.0,
        }
    }

    fn render(&mut self) {
        let dt = self
            .last_time_frame
            .elapsed()
            .as_secs_f64();
        self.last_time_frame = Instant::now();

        let (max_pos_x, max_pos_y) = self.max_square_pos();

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

        self.square_pos_x = self.square_pos_x + (self.velocity_x * dt);
        self.square_pos_y = self.square_pos_y + (self.velocity_y * dt);

        let square_start_x = (self.square_pos_x as usize).min(width);
        let square_end_x = (square_start_x + mw).min(width);

        let square_start_y = (self.square_pos_y as usize).min(height);
        let square_end_y = (square_start_y + mh).min(height);

        if square_end_x >= width || square_start_x <= 0 {
            self.velocity_x = -self.velocity_x;
            self.square_pos_x = self.square_pos_x.clamp(0.0, max_pos_x);
        }
        if square_end_y >= height || square_start_y <= 0 {
            self.velocity_y = -self.velocity_y;
            self.square_pos_y = self.square_pos_y.clamp(0.0, max_pos_y);
        }

        for y in square_start_y..square_end_y {
            for x in square_start_x..square_end_x {
                buffer[y * width + x] = 0x00FF00FF;
            }
        }

        buffer
            .present()
            .expect("present failed");

        self.window.request_redraw();
    }

    fn max_square_pos(&self) -> (f64, f64) {
        let size = self.window.inner_size();
        let w = NonZeroU32::new(size.width.max(1)).unwrap();
        let h = NonZeroU32::new(size.height.max(1)).unwrap();

        let width = w.get() as f64;
        let height = h.get() as f64;

        let mw = width * 0.1;
        let mh = height * 0.1;

        let max_pos_x = width - mw;
        let max_pos_y = height - mh;

        return (max_pos_x, max_pos_y);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App { gfx_state: None };
    let event_loop = EventLoop::new()?;

    event_loop.run_app(&mut app)?;

    Ok(())
}
