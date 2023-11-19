use super::{color::Color, consts::BLACK};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub canvas: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            canvas: vec![vec![BLACK.clone(); width]; height],
        }
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.canvas[y][x] = color;
    }
    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.canvas[y][x]
    }

    pub fn to_ppm(&self) {
        print!("P3\n{} {}\n255\n", self.width, self.height);
        for line in self.canvas.clone().into_iter() {
            for pixel in line {
                print!("{}", pixel.clamp().rgb.as_str())
            }
        }
    }
}
