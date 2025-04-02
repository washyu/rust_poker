use crate::cards::{Card, Deck, HandRank, evaluate_hand};
use crate::render::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GamePhase {
    Betting,
    FirstDeal,
    SelectingHolds,
    FinalDeal,
    ShowResults,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CardAnimationState {
    None,
    FlipToFront(f32), // Progress 0.0 to 1.0
    FlipToBack(f32),  // Progress 0.0 to 1.0
}

pub struct GameState {
    pub deck: Deck,
    pub hand: Vec<Card>,
    pub credits: i32,
    pub bet: i32,
    pub held: [bool; 5],
    pub phase: GamePhase,
    pub hand_rank: Option<HandRank>,
    pub win_amount: i32,
    animation_timer: f32,
    pub card_animations: [CardAnimationState; 5],
}

impl GameState {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();

        GameState {
            deck,
            hand: Vec::with_capacity(5),
            credits: 100,
            bet: 5,
            held: [false; 5],
            phase: GamePhase::Betting,
            hand_rank: None,
            win_amount: 0,
            animation_timer: 0.0,
            card_animations: [CardAnimationState::None; 5],
        }
    }

    pub fn handle_event(&mut self, event: &Event, renderer: &mut Renderer) {
        match event {
            Event::KeyDown { keycode: Some(keycode), .. } => {
                match self.phase {
                    GamePhase::Betting => self.handle_betting_keys(*keycode),
                    GamePhase::SelectingHolds => self.handle_selecting_holds_keys(*keycode),
                    GamePhase::ShowResults => {
                        if *keycode == Keycode::Return || *keycode == Keycode::Space {
                            self.reset_for_new_hand();
                        }
                    },
                    _ => {}
                }
            },
            Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                self.handle_mouse_click(*x, *y, renderer);
            },
            _ => {}
        }
    }

    fn handle_betting_keys(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Up => self.increase_bet(),
            Keycode::Down => self.decrease_bet(),
            Keycode::Return | Keycode::Space => self.deal_initial_hand(),
            _ => {}
        }
    }

    fn handle_selecting_holds_keys(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Num1 => self.toggle_hold(0),
            Keycode::Num2 => self.toggle_hold(1),
            Keycode::Num3 => self.toggle_hold(2),
            Keycode::Num4 => self.toggle_hold(3),
            Keycode::Num5 => self.toggle_hold(4),
            Keycode::Return | Keycode::Space => self.deal_final_hand(),
            _ => {}
        }
    }

    fn handle_mouse_click(&mut self, x: i32, y: i32, renderer: &Renderer) {
        match self.phase {
            GamePhase::Betting => {
                if renderer.is_point_in_bet_up_button(x, y) {
                    self.increase_bet();
                } else if renderer.is_point_in_bet_down_button(x, y) {
                    self.decrease_bet();
                } else if renderer.is_point_in_deal_button(x, y) {
                    self.deal_initial_hand();
                }
            },
            GamePhase::SelectingHolds => {
                for i in 0..self.hand.len() {
                    if renderer.is_point_in_card(i, x, y) {
                        self.toggle_hold(i);
                        break;
                    }
                }

                if renderer.is_point_in_draw_button(x, y) {
                    self.deal_final_hand();
                }
            },
            GamePhase::ShowResults => {
                if self.bet < 5 && self.credits >= self.bet + 1 {
                    self.bet += 1;
                }
                if renderer.is_point_in_new_game_button(x, y) {
                    self.reset_for_new_hand();
                }
            },
            _ => {}
        }
    }

    fn increase_bet(&mut self) {
        if self.bet < 5 && self.credits >= self.bet + 1 {
            self.bet += 1;
        }
    }

    fn decrease_bet(&mut self) {
        if self.bet > 1 {
            self.bet -= 1;  // Fixed: was incorrectly adding 1
        }
    }

    pub fn deal_initial_hand(&mut self) {
        // Clear any existing cards in hand first
        self.hand.clear();
        
        // Reset held cards
        for i in 0..5 {
            self.held[i] = false;
            self.card_animations[i] = CardAnimationState::FlipToFront(0.0);
        }
        
        // Deduct the bet amount from credits
        self.credits -= self.bet;
        
        // Draw 5 cards from the deck
        for _ in 0..5 {
            if let Some(card) = self.deck.draw_card() {
                self.hand.push(card);
            } else {
                // Reshuffle if deck is empty
                self.deck = Deck::new();
                self.deck.shuffle();
                // Try again to draw a card
                if let Some(card) = self.deck.draw_card() {
                    self.hand.push(card);
                }
            }
        }
        
        // Set to FirstDeal to enable animation
        self.phase = GamePhase::FirstDeal;
        self.animation_timer = 0.0;
    }

    fn toggle_hold(&mut self, index: usize) {
        if index < self.hand.len() {
            self.held[index] = !self.held[index];
        }
    }

    fn deal_final_hand(&mut self) {
        // Start flip animations for cards that will be replaced
        for i in 0..self.hand.len() {
            if !self.held[i] {
                self.card_animations[i] = CardAnimationState::FlipToBack(0.0);
            }
        }
        
        // Set to FinalDeal to enable animation
        self.phase = GamePhase::FinalDeal;
        self.animation_timer = 0.0;
    }

    fn reset_for_new_hand(&mut self) {
        self.deck.reset();
        self.hand.clear();
        self.held = [false; 5];
        self.hand_rank = None;
        self.win_amount = 0;
        self.phase = GamePhase::Betting;
        self.card_animations = [CardAnimationState::None; 5];
    }

    pub fn update(&mut self, dt: Duration) {
        self.animation_timer += dt.as_secs_f32();
        let dt_seconds = dt.as_secs_f32();

        // Update card animations
        for i in 0..5 {
            match self.card_animations[i] {
                CardAnimationState::FlipToFront(progress) => {
                    let new_progress = progress + dt_seconds * 2.0; // Adjust speed as needed
                    if new_progress >= 1.0 {
                        self.card_animations[i] = CardAnimationState::None;
                    } else {
                        self.card_animations[i] = CardAnimationState::FlipToFront(new_progress);
                    }
                },
                CardAnimationState::FlipToBack(progress) => {
                    let new_progress = progress + dt_seconds * 2.0; // Adjust speed as needed
                    if new_progress >= 1.0 {
                        // When back flip is complete, replace the card and start front flip
                        if self.phase == GamePhase::FinalDeal {
                            // Replace this card with a new one from the deck
                            if let Some(card) = self.deck.draw_card() {
                                self.hand[i] = card;
                            } else {
                                // Add this: Reshuffle if deck is empty
                                self.deck = Deck::new();
                                self.deck.shuffle();
                                // Try again to draw a card
                                if let Some(card) = self.deck.draw_card() {
                                    self.hand[i] = card;
                                }
                            }
                            self.card_animations[i] = CardAnimationState::FlipToFront(0.0);
                        } else {
                            self.card_animations[i] = CardAnimationState::None;
                        }
                    } else {
                        self.card_animations[i] = CardAnimationState::FlipToBack(new_progress);
                    }
                },
                CardAnimationState::None => {}
            }
        }

        match self.phase {
            GamePhase::FirstDeal => {
                // Check if all card animations are done
                let all_animations_done = self.card_animations.iter()
                    .all(|state| matches!(state, CardAnimationState::None));
                    
                if all_animations_done {
                    self.phase = GamePhase::SelectingHolds;
                    
                    // Now that we can see the cards, evaluate the hand
                    if self.hand.len() == 5 {
                        let hand_array: [Card; 5] = self.hand.clone().try_into().unwrap();
                        self.hand_rank = Some(evaluate_hand(&hand_array));
                    }
                }
            },
            GamePhase::FinalDeal => {
                // Check if all card animations are done
                let all_animations_done = self.card_animations.iter()
                    .all(|state| matches!(state, CardAnimationState::None));
                    
                // Add a safety timeout to prevent freezes - move to next phase after 3 seconds
                let timeout_reached = self.animation_timer > 3.0;
                    
                if all_animations_done || timeout_reached {
                    // Reset any stuck animations
                    if timeout_reached {
                        for i in 0..5 {
                            self.card_animations[i] = CardAnimationState::None;
                        }
                    }
                
                    // Evaluate the final hand
                    if self.hand.len() == 5 {
                        let hand_array: [Card; 5] = self.hand.clone().try_into().unwrap();
                        self.hand_rank = Some(evaluate_hand(&hand_array));
                        
                        if let Some(rank) = self.hand_rank {
                            self.win_amount = rank.payout() * self.bet;
                            self.credits += self.win_amount;
                        }
                    }
                    
                    self.phase = GamePhase::ShowResults;
                    self.animation_timer = 0.0;
                }
            },
            _ => {}
        }
    }
}




