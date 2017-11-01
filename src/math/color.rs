use std::f32;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { red: r, green: g, blue: b }
    }

    pub fn grey(r: f32) -> Color {
        Color { red: r, green: r, blue: r }
    }

    pub const BLACK:        Color = Color { red: 0.0, green: 0.0, blue: 0.0 };
    pub const WHITE:        Color = Color { red: 1.0, green: 1.0, blue: 1.0 };
    pub const MIDDLE_GREY:  Color = Color { red: 0.5, green: 0.5, blue: 0.5 };

    pub const RED:          Color = Color { red: 1.0, green: 0.0, blue: 0.0 };
    pub const GREEN:        Color = Color { red: 0.0, green: 1.0, blue: 0.0 };
    pub const BLUE:         Color = Color { red: 0.0, green: 0.0, blue: 1.0 };
    pub const YELLOW:       Color = Color { red: 1.0, green: 1.0, blue: 0.0 };
    pub const MAGENTA:      Color = Color { red: 1.0, green: 0.0, blue: 1.0 };
    pub const CYAN:         Color = Color { red: 0.0, green: 1.0, blue: 1.0 };
}

impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, b: Color) -> Color {
        Color {
            red: self.red + b.red,
            green: self.green + b.green,
            blue: self.blue + b.blue
        }
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, b: Color) {
        *self = *self + b;
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, x: f32) -> Color {
        Color { red: self.red * x, green: self.green * x, blue: self.blue * x }
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, v: Color) -> Color {
        Color { red: self * v.red, green: self * v.green, blue: self * v.blue }
    }
}

impl ops::Div<f32> for Color {
    type Output = Color;
    fn div(self, x: f32) -> Color {
        let inv_x = 1.0 / x;
        Color { red: self.red * inv_x, green: self.green * inv_x, blue: self.blue * inv_x }
    }
}
