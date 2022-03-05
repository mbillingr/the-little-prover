use crate::items::Item;
use crate::styles::Style;
use crate::textbuffer::TextBuffer;

const DEFAULT_FRAME: [char; 9] = ['╔', '═', '╗', '║', ' ', '║', '╚', '═', '╝'];

pub struct Framed<T: Item> {
    tiles: &'static [char],
    style: Style,
    inner: T,
}

impl<T: Item> Framed<T> {
    pub fn new(inner: T) -> Self {
        Framed {
            tiles: &DEFAULT_FRAME,
            style: Style::Frame,
            inner,
        }
    }
}

impl<T: Item> Item for Framed<T> {
    fn draw(&self, buf: &mut TextBuffer, x: usize, y: usize, width: usize, height: usize) {
        let left = x;
        let right = x + width - 1;
        let top = y;
        let bottom = y + height - 1;

        let left_inner = x + 1;
        let right_inner = x + width - 2;
        let top_inner = y + 1;
        let bottom_inner = y + height - 2;

        // corners
        buf.set_char(left, top, self.tiles[0], self.style);
        buf.set_char(right, top, self.tiles[2], self.style);
        buf.set_char(left, bottom, self.tiles[6], self.style);
        buf.set_char(right, bottom, self.tiles[8], self.style);

        // edges
        buf.draw_hline(top, left_inner, right_inner, self.tiles[1], self.style);
        buf.draw_hline(bottom, left_inner, right_inner, self.tiles[7], self.style);
        buf.draw_vline(left, top_inner, bottom_inner, self.tiles[3], self.style);
        buf.draw_vline(right, top_inner, bottom_inner, self.tiles[5], self.style);

        //inside
        buf.fill_rect(
            left_inner,
            top_inner,
            right,
            bottom,
            self.tiles[4],
            self.style,
        );
        self.inner.draw(buf, x + 1, y + 1, width - 2, height - 2)
    }
}
