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
    fn size(&self) -> (usize, usize) {
        let (w, h) = self.inner.size();
        (w + 2, h + 2)
    }

    fn resize(&mut self, width: usize, height: usize) {
        self.inner.resize(width - 2, height - 2)
    }

    fn draw(&self, buf: &mut TextBuffer, x: usize, y: usize) {
        let (width, height) = self.size();

        // corners
        buf.set_char(x, y, self.tiles[0], self.style);
        buf.set_char(x + width, y, self.tiles[2], self.style);
        buf.set_char(x, y + height, self.tiles[6], self.style);
        buf.set_char(x + width, y + height, self.tiles[8], self.style);

        // edges
        buf.draw_hline(y, x + 1, x + width - 1, self.tiles[1], self.style);
        buf.draw_hline(y + height, x + 1, x + width - 1, self.tiles[7], self.style);
        buf.draw_vline(x, y + 1, y + height - 1, self.tiles[3], self.style);
        buf.draw_vline(x + width, y + 1, y + height - 1, self.tiles[5], self.style);

        //inside
        buf.fill_rect(
            x + 1,
            y + 1,
            x + width,
            y + height,
            self.tiles[4],
            self.style,
        );
        self.inner.draw(buf, x + 1, y + 1)
    }
}
