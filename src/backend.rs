use glium::{self, DisplayBuild, Surface};
use glium::glutin;
use traits::DrawContext;

pub use glium::backend::glutin_backend::GlutinFacade as GliumWindow;

pub type GliumWindowError = glium::GliumCreationError<glutin::CreationError>;

pub fn create_window() -> Result<GliumWindow, GliumWindowError>
{
    glutin::WindowBuilder::new()
        .with_visibility(false)
        .build_glium()
}

pub struct GliumDrawContext<'a, T: Surface + 'a>
{
    pub surface: &'a mut T
}

impl<'a, T: Surface> GliumDrawContext<'a, T>
{
    pub fn new(s: &'a mut T) -> Self
    {
        GliumDrawContext{ surface: s }
    }
}

impl<'a, T: Surface> DrawContext for GliumDrawContext<'a, T>
{
    fn clear(&mut self)
    {
        self.surface.clear_color(0.0, 0.0, 0.0, 0.0);
    }
}
