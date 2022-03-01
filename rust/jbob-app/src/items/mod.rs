use crate::textbuffer::TextBuffer;

mod framed;
mod sexpr_view;

pub use framed::Framed;
pub use sexpr_view::SexprView;

pub trait Item {
    fn size(&self) -> (usize, usize);
    fn resize(&mut self, width: usize, height: usize);
    fn draw(&self, buf: &mut TextBuffer, x: usize, y: usize);
}

pub trait EventHandler<E> {
    fn handle_event(&mut self, event: &E) -> bool;
}
