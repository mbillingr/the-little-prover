//! A 2D character-tile buffer for displaying text and ASCII art

use crate::styles::Style;

pub trait RenderTarget {
    type Error;
    fn prepare(&mut self) -> Result<(), Self::Error>;
    fn finalize(&mut self) -> Result<(), Self::Error>;
    fn write_char(&mut self, ch: char, s: Style) -> Result<(), Self::Error>;
}

pub struct TextBuffer {
    text: Vec2D<char>,
    style: Vec2D<Style>,
}

impl TextBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        TextBuffer {
            text: Vec2D::new(width, height),
            style: Vec2D::new(width, height),
        }
    }

    pub fn width(&self) -> usize {
        self.text.width()
    }

    pub fn height(&self) -> usize {
        self.text.height()
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        let text = std::mem::replace(&mut self.text, Vec2D::new(width, height));
        let style = std::mem::replace(&mut self.style, Vec2D::new(width, height));
        let old_buffer = TextBuffer { text, style };
        self.copy_rect(
            0,
            0,
            &old_buffer,
            0,
            0,
            old_buffer.width().min(width),
            old_buffer.height().min(height),
        );
    }

    pub fn clear(&mut self, ch: char, style: Style) {
        self.text.fill(ch);
        self.style.fill(style);
    }

    pub fn set_char(&mut self, x: usize, y: usize, ch: char, style: Style) {
        self.text.set(x, y, ch);
        self.style.set(x, y, style);
    }

    pub fn render<T: RenderTarget>(&self, target: &mut T) -> Result<(), T::Error> {
        target.prepare()?;
        for (text_row, style_row) in self.text.iter_rows().zip(self.style.iter_rows()) {
            for (&ch, s) in text_row.iter().zip(style_row) {
                target.write_char(ch, *s)?;
            }
        }
        target.finalize()
    }

    pub fn fill_rect(
        &mut self,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        ch: char,
        style: Style,
    ) {
        self.text.set_rect(x0, y0, x1, y1, ch);
        self.style.set_rect(x0, y0, x1, y1, style);
    }

    pub fn draw_hline(&mut self, y: usize, x0: usize, x1: usize, ch: char, style: Style) {
        for x in x0..=x1 {
            self.set_char(x, y, ch, style.clone());
        }
    }

    pub fn draw_vline(&mut self, x: usize, y0: usize, y1: usize, ch: char, style: Style) {
        for y in y0..=y1 {
            self.set_char(x, y, ch, style.clone());
        }
    }

    pub fn copy_rect(
        &mut self,
        x: usize,
        y: usize,
        src: &Self,
        sx: usize,
        sy: usize,
        w: usize,
        h: usize,
    ) {
        for j in 0..h {
            for i in 0..w {
                let ch = *src.text.get(sx + i, sy + j);
                if ch != '\u{0}' {
                    self.text.set(x + i, y + j, ch);

                    let st = src.style.get(sx + i, sy + j);
                    self.style.set(x + i, y + j, *st);
                }
            }
        }
    }
}

struct Vec2D<T> {
    data: Vec<T>,
    shape: (usize, usize),
}

impl<T> Vec2D<T> {
    pub fn from_vec(width: usize, height: usize, data: Vec<T>) -> Self {
        let size = width * height;
        assert_eq!(data.len(), size);

        Vec2D {
            data,
            shape: (width, height),
        }
    }

    pub fn width(&self) -> usize {
        self.shape.0
    }

    pub fn height(&self) -> usize {
        self.shape.1
    }

    pub fn get(&self, col: usize, row: usize) -> &T {
        &self.data[self.index(col, row)]
    }

    pub fn set(&mut self, col: usize, row: usize, value: T) {
        let idx = self.index(col, row);
        self.data[idx] = value
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
        self.data.chunks(self.width())
    }

    fn index(&self, col: usize, row: usize) -> usize {
        row * self.width() + col
    }
}

impl<T: Clone> Vec2D<T> {
    pub fn fill(&mut self, value: T) {
        for x in &mut self.data {
            *x = value.clone();
        }
    }

    pub fn set_rect(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, value: T) {
        let mut idx = self.index(0, y0);
        for _ in y0..y1 {
            for x in x0..x1 {
                self.data[idx + x] = value.clone();
            }
            idx += self.width();
        }
    }
}

impl<T: Clone + Default> Vec2D<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self::from_vec(width, height, vec![Default::default(); width * height])
    }
}
