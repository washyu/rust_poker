# Rust Poker

A video poker game implemented in Rust using SDL2.

![Rust Poker Screenshot](screenshots/game_screenshot.png) <!-- Replace with an actual screenshot once available -->

## Features

- Classic video poker gameplay
- Card flip animations
- Hand evaluation
- Betting system
- Multiple pay ranks based on poker hand combinations

## Requirements

- Rust (latest stable version recommended)
- SDL2 and its extension libraries:
  - SDL2_image
  - SDL2_ttf

### SDL2 Setup

#### Windows
1. Download the SDL2, SDL2_image, and SDL2_ttf development libraries from [SDL's website](https://www.libsdl.org/download-2.0.php)
2. Place the DLL files in your system PATH or in the project directory

#### Linux
```bash
# Ubuntu/Debian
sudo apt-get install libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev

# Fedora
sudo dnf install SDL2-devel SDL2_image-devel SDL2_ttf-devel

# Arch Linux
sudo pacman -S sdl2 sdl2_image sdl2_ttf
```

#### macOS
```bash
brew install sdl2 sdl2_image sdl2_ttf
```

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rust_poker.git
cd rust_poker
```

2. Create the assets directory structure:
```bash
mkdir -p assets/images/cards assets/fonts
```

3. Add required assets:
   - Add card images to `assets/images/cards/` (format: `hearts_02.png`, `diamonds_king.png`, etc.)
   - Add card back images to `assets/images/cards/` (format: `back01.png`, `back02.png`, etc.)
   - Add at least one font (e.g., `arial.ttf`) to `assets/fonts/`

4. Build and run the game:
```bash
cargo build --release
cargo run --release
```

## How to Play

1. **Betting Phase**: 
   - Use the "-" and "+" buttons to adjust your bet amount
   - Click "DEAL" to start the game

2. **Selecting Holds**:
   - Click on cards you want to hold
   - Selected cards will be marked "HOLD"
   - Click "DRAW" to replace non-held cards

3. **Results**:
   - The game will display your hand rank
   - Winnings will be added to your credits
   - Click "NEW GAME" to play again

## Controls

### Keyboard Controls
- **Betting Phase:**
  - Up/Down Arrows: Increase/decrease bet
  - Space/Enter: Deal cards

- **Card Selection Phase:**
  - Number keys 1-5: Toggle hold status for corresponding cards
  - Space/Enter: Draw new cards

- **Results Phase:**
  - Space/Enter: Start new game
  - Escape: Quit game (any phase)

### Mouse Controls
- Click on cards to hold/unhold them
- Click buttons to perform actions
- Click "NEW GAME" after a round to start a new hand

## Poker Hand Rankings and Payouts

| Hand            | Payout Multiplier |
|-----------------|-------------------|
| High Card       | 0                 |
| One Pair        | 1                 |
| Two Pairs       | 2                 |
| Three of a Kind | 3                 |
| Straight        | 4                 |
| Flush           | 6                 |
| Full House      | 9                 |
| Four of a Kind  | 25                |
| Straight Flush  | 50                |

## Project Structure

- `src/main.rs` - The entry point for the application
- `src/cards.rs` - Card definitions, deck management, and hand evaluation
- `src/game.rs` - Game state and logic
- `src/render.rs` - Rendering functions using SDL2
- `src/ui.rs` - UI elements like buttons

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- The SDL2 team for their excellent graphics libraries
- The Rust community for their support and documentation
