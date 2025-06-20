# Pong Clone in Rust

A classic Pong game implementation using Rust and the Macroquad game engine. This project follows an iterative development approach, starting with a barebones game loop and gradually adding features.

## 🎯 Project Goals

- Learn game development patterns in Rust
- Build a complete Pong clone from scratch
- Practice iterative development methodology
- Create a terminal-executable game

## 🛠️ Technology Stack

- **Language**: Rust 2021 Edition
- **Game Engine**: [Macroquad](https://github.com/not-fl3/macroquad) - Simple and easy-to-use 2D game framework
- **Target**: Cross-platform desktop executable

## 🚀 Getting Started

### Prerequisites
- Rust (latest stable version)
- Cargo

### Running the Game
```bash
cargo run
```

### Building for Release
```bash
cargo build --release
```

## 📋 Development Roadmap

### Phase 1: Foundation ✅
- [x] Project setup with Macroquad
- [x] Basic project structure
- [ ] Basic window and game loop

### Phase 2: Core Gameplay 🔄
- [ ] Render basic paddles (rectangles)
- [ ] Render basic ball (circle)
- [ ] Implement ball movement
- [ ] Add paddle input controls (W/S for left, Up/Down for right)
- [ ] Implement ball-paddle collision
- [ ] Implement ball-wall collision
- [ ] Add basic scoring system

### Phase 3: Polish & Features 📋
- [ ] Add game states (menu, playing, paused, game over)
- [ ] Improve graphics and animations
- [ ] Add sound effects
- [ ] Add particle effects
- [ ] Implement AI opponent option
- [ ] Add difficulty settings
- [ ] Add customizable controls

### Phase 4: Advanced Features 📋
- [ ] Add power-ups
- [ ] Multiple ball modes
- [ ] Tournament mode
- [ ] Save/load high scores
- [ ] Custom themes/skins

## 🎮 Game Design

### Core Mechanics
- **Paddles**: Vertical rectangles that move up/down
- **Ball**: Circular object that bounces off paddles and walls
- **Scoring**: Points awarded when ball passes opponent's paddle
- **Win Condition**: First to reach target score (default: 11 points)

### Controls
- **Player 1 (Left)**: W (up), S (down)
- **Player 2 (Right)**: Arrow Up, Arrow Down
- **General**: ESC (pause/menu), Space (serve ball)

### Game Physics
- Ball speed increases slightly after each paddle hit
- Ball angle changes based on where it hits the paddle
- Paddle movement has momentum/smoothing

## 🏗️ Architecture

### Game Loop Pattern
```
Initialize → Update → Render → Repeat
```

### Key Components
- **Game State**: Manages current game phase
- **Paddle**: Player-controlled game objects
- **Ball**: Physics-driven game object
- **Collision System**: Handles ball-paddle and ball-wall interactions
- **Scoring System**: Tracks and displays player scores
- **Input Handler**: Processes keyboard input

### File Structure
```
src/
├── main.rs           # Entry point and main game loop
├── game.rs           # Core game state and logic
├── paddle.rs         # Paddle entity and behavior
├── ball.rs           # Ball entity and physics
├── collision.rs      # Collision detection and response
├── input.rs          # Input handling
└── constants.rs      # Game constants and configuration
```

## 🎨 Visual Design

### Color Scheme
- Background: Dark blue/black
- Paddles: White
- Ball: White
- UI Text: White
- Score: Large, centered at top

### Screen Layout
```
     SCORE 1    SCORE 2
         0         0

|                      |
|                      |
|  ▌        ●         ▐|
|                      |
|                      |
```

## 🔧 Configuration

Game settings can be modified in `src/constants.rs`:
- Window dimensions
- Paddle size and speed
- Ball size and speed
- Scoring target
- Colors and styling

## 📚 Learning Notes

### Game Development Patterns Learned
- **Game Loop**: The fundamental pattern of update → render
- **Entity-Component Pattern**: Separating data from behavior
- **State Management**: Handling different game states
- **Physics Integration**: Basic collision detection and response
- **Input Handling**: Responsive controls and input buffering

### Rust-Specific Patterns
- **Ownership in Game Objects**: Managing mutable references
- **Performance Considerations**: Avoiding allocations in hot loops
- **Error Handling**: Graceful handling of game errors
- **Module Organization**: Structuring game code effectively

## 🐛 Known Issues

- None yet! 🎉

## 🤝 Contributing

This is a learning project, but suggestions and improvements are welcome!

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

---

**Current Status**: 🚧 In Development - Phase 1 (Foundation)