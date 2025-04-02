use crate::cards::Card;
use crate::game::{GamePhase, GameState, CardAnimationState};
use crate::cards::{Suit, Rank};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::ttf::{self, Font};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::path::Path;

pub struct Renderer<'a> {
    card_textures: HashMap<String, Texture<'a>>,
    ui_textures: HashMap<String, Texture<'a>>,
    text_textures: HashMap<String, Texture<'a>>, // Store pre-rendered text
    ttf_context: ttf::Sdl2TtfContext,
    texture_creator: &'a TextureCreator<WindowContext>,
}

impl<'a> Renderer<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let ttf_context = ttf::init().map_err(|e| e.to_string())?;
        
        let mut card_textures = HashMap::new();
        let ui_textures = HashMap::new();
        let text_textures = HashMap::new();

        for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in [
                Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six,
                Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
                Rank::Jack, Rank::Queen, Rank::King, Rank::Ace
            ]
            {
                let card = Card { suit, rank, back: 1 }; // Assuming back is always 1 for simplicity
                let filename = card.image_filename(); // Get the image filename for the card
                let path = format!("assets/images/cards/{}", filename);

                let texture = sdl2::image::LoadTexture::load_texture(
                    texture_creator,
                    Path::new(&path)
                ).map_err(|e| format!("Failed to load {}: {}", path, e))?;

                card_textures.insert(filename, texture);
            }
        }
            
        for i in 1..=8 {
            let filename = format!("back{:02}.png", i);
            let path = format!("assets/images/cards/{}", filename);

            let texture = sdl2::image::LoadTexture::load_texture(
                texture_creator,
                Path::new(&path)
            ).map_err(|e| format!("Failed to load {}: {}", path, e))?;

            card_textures.insert(filename, texture);
        }

        Ok(Renderer {
            card_textures,
            ui_textures,
            text_textures,
            ttf_context,
            texture_creator,
        })
    }

    pub fn create_text_texture(&mut self, text: &str, color: Color, size: u16) -> Result<&Texture<'a>, String> {
        let key = format!("{}:{}:{}:{}:{}", text, color.r, color.g, color.b, size);

        if !self.text_textures.contains_key(&key) {

            let font = self.ttf_context.load_font(Path::new("assets/fonts/arial.ttf"), size)
                .map_err(|e| e.to_string())?;

            let surface = font.render(text)
                .solid(color)
                .map_err(|e| e.to_string())?;

            let texture = self.texture_creator.create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            self.text_textures.insert(key.clone(), texture);
        }

        Ok(self.text_textures.get(&key).unwrap())
    }


    pub fn render(&mut self, canvas: &mut WindowCanvas, game_state: &GameState) -> Result<(), String> {

        canvas.set_draw_color(Color::RGB(0, 100, 0));
        canvas.clear();

        match game_state.phase {
            GamePhase::Betting => self.render_betting_ui(canvas, game_state)?,
            GamePhase::SelectingHolds => self.render_selecting_holds_ui(canvas, game_state)?,
            GamePhase::ShowResults => self.render_result_ui(canvas, game_state)?,
            _ => {}
        }   

        self.render_cards(canvas, game_state)?;
        self.render_player_stats(canvas, game_state)?;

        Ok(())
    }

    fn render_betting_ui(&mut self, canvas: &mut WindowCanvas, game_state: &GameState) -> Result<(), String> {
        
        // Draw bet down button
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.fill_rect(Rect::new(50, 400, 50, 30))?;
        
        // Add text to bet down button
        let down_text = self.create_text_texture("-", Color::RGB(0, 0, 0), 20)?;
        let query = down_text.query();
        canvas.copy(down_text, None, Rect::new(65, 405, query.width, query.height))?;
        
        // Draw bet up button
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.fill_rect(Rect::new(200, 400, 50, 30))?;
        
        // Add text to bet up button
        let up_text = self.create_text_texture("+", Color::RGB(0, 0, 0), 20)?;
        let query = up_text.query();
        canvas.copy(up_text, None, Rect::new(215, 405, query.width, query.height))?;
        
        // Draw deal button with more distinct color
        canvas.set_draw_color(Color::RGB(150, 150, 255));
        canvas.fill_rect(Rect::new(350, 400, 100, 40))?;
        
        // Add text to deal button
        let deal_text = self.create_text_texture("DEAL", Color::RGB(0, 0, 0), 18)?;
        let query = deal_text.query();
        canvas.copy(deal_text, None, Rect::new(380, 410, query.width, query.height))?;

        Ok(())
    }

    fn render_selecting_holds_ui(&mut self, canvas: &mut WindowCanvas, game_state: &GameState) -> Result<(), String> {
        // Render the UI for selecting holds
        for i in 0..game_state.hand.len() {
            if game_state.held[i] {
                canvas.set_draw_color(Color::RGB(255, 200, 0));
                canvas.fill_rect(Rect::new(100 + i as i32 * 120, 350, 80, 20))?;
                
                // Add "HOLD" text
                let hold_text = self.create_text_texture("HOLD", Color::RGB(0, 0, 0), 16)?;
                let query = hold_text.query();
                canvas.copy(hold_text, None, Rect::new(120 + i as i32 * 120, 352, query.width, query.height))?;
            }
        }

        // Draw draw button with distinct color
        canvas.set_draw_color(Color::RGB(150, 150, 255));
        canvas.fill_rect(Rect::new(350, 400, 100, 40))?;
        
        // Add text to draw button
        let draw_text = self.create_text_texture("DRAW", Color::RGB(0, 0, 0), 18)?;
        let query = draw_text.query();
        canvas.copy(draw_text, None, Rect::new(380, 410, query.width, query.height))?;

        Ok(())
    }

    fn render_result_ui(&mut self, canvas: &mut WindowCanvas, game_state: &GameState) -> Result<(), String> {
        if let Some(rank) = game_state.hand_rank {
            // Draw a more visible result box
            canvas.set_draw_color(Color::RGB(255, 215, 0));
            canvas.fill_rect(Rect::new(250, 100, 300, 50))?;
            
            // Show the hand rank result text
            let result_text = self.create_text_texture(&format!("{}", rank.to_string()), Color::RGB(0, 0, 0), 24)?;
            let query = result_text.query();
            canvas.copy(result_text, None, Rect::new(400 - (query.width as i32)/2, 115, query.width, query.height))?;
            
            // Show win amount if > 0
            if game_state.win_amount > 0 {
                let win_text = self.create_text_texture(&format!("You win: {}", game_state.win_amount), Color::RGB(0, 0, 0), 20)?;
                let query = win_text.query();
                canvas.copy(win_text, None, Rect::new(400 - (query.width as i32)/2, 145, query.width, query.height))?;
            }
        }

        // Draw new game button
        canvas.set_draw_color(Color::RGB(150, 255, 150));
        canvas.fill_rect(Rect::new(350, 400, 100, 40))?;
        
        // Add text to new game button
        let new_game_text = self.create_text_texture("NEW GAME", Color::RGB(0, 0, 0), 16)?;
        let query = new_game_text.query();
        canvas.copy(new_game_text, None, Rect::new(365, 410, query.width, query.height))?;

        Ok(())
    }

    fn render_cards(&self, canvas: &mut WindowCanvas, game_state: &GameState) -> Result<(), String> {
        // Render the cards in the player's hand
        for (i, card) in game_state.hand.iter().enumerate() {
            match game_state.card_animations[i] {
                CardAnimationState::FlipToFront(progress) => {
                    // Calculate rotation angle - pi radians when progress = 0.5
                    let rotation_progress = if progress < 0.5 {
                        // First half - show back of card while rotating
                        progress * 2.0 // 0 -> 1 as progress goes 0 -> 0.5
                    } else {
                        // Second half - show front of card while rotating
                        (1.0 - progress) * 2.0 // 1 -> 0 as progress goes 0.5 -> 1
                    };
                    
                    // Only show card width based on rotation
                    let visible_width = (80.0 * rotation_progress.sin()).max(5.0) as u32;
                    let x_offset = (80 - visible_width) / 2;
                    
                    if progress < 0.5 {
                        // Draw back of card
                        if let Some(texture) = self.card_textures.get(&format!("back01.png")) {
                            canvas.copy(
                                texture, 
                                None, 
                                Rect::new(100 + i as i32 * 120 + x_offset as i32, 200, visible_width, 120)
                            )?;
                        }
                    } else {
                        // Draw front of card
                        let filename = card.image_filename();
                        if let Some(texture) = self.card_textures.get(&filename) {
                            canvas.copy(
                                texture, 
                                None, 
                                Rect::new(100 + i as i32 * 120 + x_offset as i32, 200, visible_width, 120)
                            )?;
                        }
                    }
                },
                CardAnimationState::FlipToBack(progress) => {
                    // Calculate rotation angle - pi radians when progress = 0.5
                    let rotation_progress = if progress < 0.5 {
                        // First half - show front of card while rotating
                        progress * 2.0 // 0 -> 1 as progress goes 0 -> 0.5
                    } else {
                        // Second half - show back of card while rotating
                        (1.0 - progress) * 2.0 // 1 -> 0 as progress goes 0.5 -> 1
                    };
                    
                    // Only show card width based on rotation
                    let visible_width = (80.0 * rotation_progress.sin()).max(5.0) as u32;
                    let x_offset = (80 - visible_width) / 2;
                    
                    if progress < 0.5 {
                        // Draw front of card
                        let filename = card.image_filename();
                        if let Some(texture) = self.card_textures.get(&filename) {
                            canvas.copy(
                                texture, 
                                None, 
                                Rect::new(100 + i as i32 * 120 + x_offset as i32, 200, visible_width, 120)
                            )?;
                        }
                    } else {
                        // Draw back of card
                        if let Some(texture) = self.card_textures.get(&format!("back01.png")) {
                            canvas.copy(
                                texture, 
                                None, 
                                Rect::new(100 + i as i32 * 120 + x_offset as i32, 200, visible_width, 120)
                            )?;
                        }
                    }
                },
                CardAnimationState::None => {
                    // Normal rendering without animation
                    let filename = card.image_filename();
                    if let Some(texture) = self.card_textures.get(&filename) {
                        canvas.copy(texture, None, Rect::new(100 + i as i32 * 120, 200, 80, 120))?;
                    } else {
                        // Fallback rendering if texture not found
                        canvas.set_draw_color(Color::RGB(255, 255, 255));
                        canvas.fill_rect(Rect::new(100 + i as i32 * 120, 200, 80, 120))?;

                        canvas.set_draw_color(Color::RGB(0, 0, 0));
                        canvas.draw_rect(Rect::new(100 + i as i32 * 120, 200, 80, 120))?;

                        let suit_color = match card.suit {
                            crate::cards::Suit::Hearts | crate::cards::Suit::Diamonds =>
                                Color::RGB(255, 0, 0),
                            _ => Color::RGB(0,0,0),
                        };
                        canvas.set_draw_color(suit_color);
                        canvas.fill_rect(Rect::new(110 + i as i32 * 120, 210, 20, 20))?;
                    }
                }
            }
        }

        Ok(())
    }

    fn render_player_stats(&mut self, canvas: &mut WindowCanvas, game_state: &GameState) -> Result<(), String> {
        // Render credits
        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.fill_rect(Rect::new(50, 50, 150, 40))?;
        
        // Add this to render the credits text:
        let credits_text = &format!("Credits: {}", game_state.credits);
        let credits_texture = self.create_text_texture(credits_text, Color::RGB(255, 255, 255), 16)?;
        let query = credits_texture.query();
        canvas.copy(credits_texture, None, Rect::new(60, 60, query.width, query.height))?;
        
        // Render bet
        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.fill_rect(Rect::new(250, 50, 150, 40))?;
        
        // Add this to render the bet text:
        let bet_text = &format!("Bet: {}", game_state.bet);
        let bet_texture = self.create_text_texture(bet_text, Color::RGB(255, 255, 255), 16)?;
        let query = bet_texture.query();
        canvas.copy(bet_texture, None, Rect::new(260, 60, query.width, query.height))?;
        
        Ok(())
    }

    pub fn is_point_in_bet_up_button(&self, x: i32, y: i32) -> bool {
        Rect::new(200, 400, 50, 30).contains_point((x, y))
    }

    pub fn is_point_in_bet_down_button(&self, x: i32, y: i32) -> bool {
        Rect::new(50, 400, 50, 30).contains_point((x, y))
    }

    pub fn is_point_in_deal_button(&self, x: i32, y: i32) -> bool {
        Rect::new(350, 400, 100, 40).contains_point((x, y))
    }

    pub fn is_point_in_draw_button(&self, x: i32, y: i32) -> bool {
        Rect::new(350, 400, 100, 40).contains_point((x, y))
    }

    pub fn is_point_in_new_game_button(&self, x: i32, y: i32) -> bool {
        Rect::new(350, 400, 100, 40).contains_point((x, y))
    }

    pub fn is_point_in_card(&self, index: usize, x: i32, y: i32) -> bool {
        Rect::new(100 + index as i32 * 120, 200, 80, 120).contains_point((x, y))
    }
}
