use super::colour::Colour;

use util::vector;
use util::vector::Vector2D;

use super::sprite::Sprite;
use std::collections::HashMap;

pub mod colours {
    use graphics::colour::Colour;
    define_colour!(CLEAR_COLOUR, 25, 20, 70);
    define_colour!(BORDER, 20, 20, 20);
}

pub struct Panel {
    start_point: Vector2D<u32>,
    size: Vector2D<u32>,
}

impl Panel {
    pub fn new(start_point: Vector2D<u32>, size: Vector2D<u32>) -> Panel {
        Panel { start_point, size }
    }

    pub fn clear(&self, colour: &Colour) {
        Renderer::set_bg_colour(&colour);

        let row_str = &" ".repeat(self.size.x as usize);
        for y in 0..self.size.y {
            self.draw_string(row_str, Vector2D::new(0, y));
        }
    }

    /// Creates a border around the panel area
    pub fn border(&self) {
        let bg_col = colours::BORDER;

        let Vector2D { x: w, y: h } = self.size;

        // top
        self.draw_line_h(&bg_col, self.start_point, w + 2);
        // left
        self.draw_line_v(&bg_col, self.start_point, h + 2);
        self.draw_line_v(&bg_col, self.start_point + Vector2D::new(1, 0), h + 2);
        // bottom
        self.draw_line_h(&bg_col, self.start_point + Vector2D::new(0, h + 1), w + 2);
        // right
        self.draw_line_v(&bg_col, self.start_point + Vector2D::new(w + 2, 0), h + 2);
        self.draw_line_v(&bg_col, self.start_point + Vector2D::new(w + 3, 0), h + 2);
    }

    /// Draws a string to a render panel.
    /// Note: The function does not handle the length of strings going outside of the render panel (for now?)
    pub fn draw_string(&self, string: &str, start_position: Vector2D<u32>) {
        if start_position.y >= self.size.y {
            panic!();
        }

        self.set_cursor(start_position);
        print!("{}", string);
    }

    /// Draws a sprite (duh)
    pub fn draw_sprite(&self, sprite: &Sprite) {
        self.set_cursor(sprite.position);
        let data = sprite.render_data();

        for (line_num, line) in data.iter().enumerate() {
            self.draw_string(line, sprite.position + Vector2D::new(0, line_num as u32));
        }
    }

    pub fn set_cursor(&self, position: Vector2D<u32>) {
        Renderer::set_cursor_location(self.start_point + position + Vector2D::new(2, 1));
    }

    /// Draws a solid horizontal line
    pub fn draw_line_h(&self, colour: &Colour, begin_position: Vector2D<u32>, length: u32) {
        Renderer::set_bg_colour(colour);
        Renderer::set_cursor_location(begin_position);
        for _x in 0..length {
            print!(" ");
        }
        Renderer::set_bg_colour(&colours::CLEAR_COLOUR);
    }

    /// Draws a solid vertical line
    pub fn draw_line_v(&self, colour: &Colour, begin_position: Vector2D<u32>, height: u32) {
        Renderer::set_bg_colour(colour);
        for y in 0..height {
            Renderer::set_cursor_location(begin_position + Vector2D::new(0, y));
            print!(" ");
        }
        Renderer::set_bg_colour(&colours::CLEAR_COLOUR);
    }
}

pub struct Renderer {
    size: Vector2D<u32>,
    panels: HashMap<String, Panel>,
}

impl Renderer {
    pub fn new(size: Vector2D<u32>) -> Renderer {
        let mut renderer = Renderer {
            size,
            panels: HashMap::new(),
        };

        let main_panel = Panel::new(vector::ZERO, size);
        main_panel.clear(&colours::CLEAR_COLOUR);
        main_panel.border();

        let debug_panel = Panel::new(Vector2D::new(size.x + 2, 0), Vector2D::new(20, size.y));
        debug_panel.clear(&colours::CLEAR_COLOUR);
        debug_panel.border();

        renderer.add_panel("main", main_panel);
        renderer.add_panel("debug", debug_panel);

        renderer
    }

    pub fn panel(&self, name: &str) -> &Panel {
        &self.panels[name]
    }

    pub fn add_panel(&mut self, name: &str, panel: Panel) {
        self.panels.insert(name.to_string(), panel);
    }

    /// Set the foreground colour for text printed to the terminal
    pub fn set_text_colour(colour: &Colour) {
        print!("{}", colour.ansi_text_string());
    }

    /// Set the background colour for text printed to the terminal
    pub fn set_bg_colour(colour: &Colour) {
        print!("{}", colour.ansi_bg_string());
    }

    /// Sets cursor location in the renderer
    pub fn set_cursor_location(pos: Vector2D<u32>) {
        print!("\x1b[{};{}H", pos.y + 1, pos.x + 1);
    }
}
