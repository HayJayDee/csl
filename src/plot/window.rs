use glfw::{fail_on_errors, Action, Context, GlfwReceiver, Key, MouseButton, PWindow, WindowEvent};

use super::figure::Figure;

#[cfg(debug_assertions)]
#[allow(dead_code)]
enum PolygonMode {
    Point = gl::POINT as isize,
    Line = gl::LINE as isize,
    Fill = gl::FILL as isize,
}

#[cfg(debug_assertions)]
fn polygon_mode(mode: PolygonMode) {
    unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, mode as u32) };
}

// TODO: Turn this e.g. a trait
pub struct GLFWPlotContext {
    glfw_context: glfw::Glfw,
    window: PWindow,
    window_events: GlfwReceiver<(f64, WindowEvent)>,
}

pub struct PlotWindowProperties {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub resizable: bool,
}

impl Default for PlotWindowProperties {
    fn default() -> Self {
        Self {
            width: 300,
            height: 300,
            title: "CLS plot window".to_string(),
            resizable: true,
        }
    }
}

pub struct PlotWindow {
    plot_context: GLFWPlotContext,
    figures: Vec<Figure>,
}

impl PlotWindow {
    pub fn new(properties: PlotWindowProperties) -> Self {
        ::std::env::set_var("RUST_LOG", "trace");
        env_logger::init();

        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
        glfw.window_hint(glfw::WindowHint::Resizable(properties.resizable));

        // Create a windowed mode window and its OpenGL context
        let (mut window, events) = glfw
            .create_window(
                properties.width,
                properties.height,
                properties.title.as_str(),
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        // Initialize OpenGL
        gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

        unsafe {
            gl::Viewport(0, 0, properties.width as i32, properties.height as i32);
        }

        // Make the window's context current
        window.make_current();
        window.set_key_polling(true);
        window.set_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);

        Self {
            plot_context: GLFWPlotContext {
                window,
                glfw_context: glfw,
                window_events: events,
            },
            figures: vec![],
        }
    }

    pub fn run(&mut self) {
        #[cfg(debug_assertions)]
        polygon_mode(PolygonMode::Fill);

        let mut off_x = 0.0;
        let mut off_y = 0.0;

        let mut init_pos: Option<[f32; 2]> = None;

        // Loop until the user closes the window
        while !self.plot_context.window.should_close() {
            unsafe {
                gl::ClearColor(0.0, 1.0, 1.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            // Poll for and process events
            self.plot_context.glfw_context.poll_events();
            for (_, event) in glfw::flush_messages(&self.plot_context.window_events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.plot_context.window.set_should_close(true)
                    }
                    glfw::WindowEvent::Size(w, h) => {
                        //self.plot_shader.shader.use_program();
                        unsafe {
                            gl::Viewport(0, 0, w, h);
                        }
                        /*translation = glam::Mat4::from_translation(Vec3::new(
                            w as f32 / 2.0,
                            h as f32 / 2.0,
                            0.0,
                        )) * glam::Mat4::from_scale(Vec3::new(
                            w as f32 / 2.0,
                            h as f32 / 2.0,
                            1.0,
                        ));
                        proj =
                            glam::Mat4::orthographic_lh(0.0, w as f32, 0.0, h as f32, 0.01, 100.0);

                        //plot_shader.shader.use_program();
                        //plot_shader.transform.set(proj * translation);
                        graph_shader.use_program();
                        graph_transform_uniform.set(proj * translation);
                        */

                        let pos = self.plot_context.window.get_pos();
                        self.plot_context.window.set_pos(pos.0 + 1, pos.1);
                    }
                    glfw::WindowEvent::MouseButton(btn, action, _) => {
                        if btn == MouseButton::Button1 {
                            if action == Action::Press {
                                if init_pos.is_none() {
                                    let c_pos = self.plot_context.window.get_cursor_pos();
                                    let window_pos = self.plot_context.window.get_pos();
                                    let window_size = self.plot_context.window.get_size();

                                    let x_scaled = (c_pos.0 as f32 - window_pos.0 as f32)
                                        / window_size.0 as f32;
                                    let y_scaled = (c_pos.1 as f32 - window_pos.1 as f32)
                                        / window_size.1 as f32;
                                    init_pos = Some([x_scaled + off_x, y_scaled + off_y]);
                                }
                            } else if action == Action::Release {
                                init_pos = None;
                            }
                        }
                    }
                    glfw::WindowEvent::CursorPos(x, y) => {
                        if let Some(init_pos) = init_pos {
                            let window_pos = self.plot_context.window.get_pos();
                            let window_size = self.plot_context.window.get_size();

                            let x_scaled = (x as f32 - window_pos.0 as f32) / window_size.0 as f32;
                            let y_scaled = (y as f32 - window_pos.1 as f32) / window_size.1 as f32;

                            off_x = init_pos[0] - x_scaled;
                            off_y = init_pos[1] - y_scaled;

                            //plot_shader.shader.use_program();
                            //plot_shader.offset.set([off_x, off_y]);
                        }
                    }
                    _ => {}
                }
            }

            for figure in &mut self.figures {
                figure.render();
            }

            // Swap front and back buffers
            self.plot_context.window.swap_buffers();
        }
    }

    pub fn add_figure(&mut self, figure: Figure) {
        self.figures.push(figure);
    }
}
