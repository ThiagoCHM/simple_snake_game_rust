use piston_window::*;

pub struct Score {
    points: u32,
}

impl Score {
    pub fn new() -> Self {
        Self { points: 0 }
    }

    pub fn increase(&mut self) {
        self.points += 100;
    }

    pub fn get(&self) -> u32 {
        self.points
    }

    pub fn draw(
        &self,
        context: &Context,
        graphics: &mut G2d,
        glyphs: &mut Glyphs,
        game_over: bool,
    ) {
        let text_color = if game_over {
            [1.0, 0.0, 0.0, 1.0]
        } else {
            [1.0, 1.0, 1.0, 1.0]
        };
        let text_position = if game_over {
            (10.0, 100.0)
        } else {
            (10.0, 30.0)
        };
        let game_over_text = if game_over {
            format!("Game Over - Score: {}", self.points)
        } else {
            format!("Score: {}", self.points)
        };

        text::Text::new_color(text_color, 32)
            .draw(
                &game_over_text,
                glyphs,
                &context.draw_state,
                context.transform.trans(text_position.0, text_position.1),
                graphics,
            )
            .unwrap();
    }
}
