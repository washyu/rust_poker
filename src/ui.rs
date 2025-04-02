// This file will contain UI-specific code if needed
// For now, we're handling most UI in the render.rs
// We'll leave this as a placeholder for future expansion

pub struct Button {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub text: String,
}

impl Button {
    pub fn new(x: i32, y: i32, width: u32, height: u32, text: &str) -> Self {
        Button {
            x,
            y,
            width,
            height,
            text: text.to_string(),
        }
    }
    
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        x >= self.x && x <= self.x + self.width as i32 &&
        y >= self.y && y <= self.y + self.height as i32
    }
}