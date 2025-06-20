# CLAUDE.md - Pong Clone Learning Project

## Project Overview
This is a learning project focused on game development using Rust and the Macroquad game engine. The goal is to build a complete Pong clone through iterative development, learning both Rust gaming libraries and fundamental game programming concepts.

## Learning Objectives
- Master game development patterns and architecture
- Learn Rust-specific game programming techniques
- Understand physics simulation and collision detection
- Practice iterative development methodology
- Explore game state management and user input handling

## Build Commands
- `cargo run` - Build and run the game
- `cargo build` - Build the project  
- `cargo build --release` - Build optimized release version
- `cargo check` - Quick syntax/type check
- `cargo clippy` - Run linter for code quality
- `cargo fmt` - Format code

## Iterative Development Checklist

### Iteration 1: Basic Window & Game Loop ✅
- [x] Project setup with Macroquad
- [x] Basic window creation
- [x] Game loop with clear background
- [x] ESC key to exit
- **Learning Focus**: Understanding async game loops, basic windowing

### Iteration 2: Static Game Objects
- [ ] Draw two paddles as white rectangles
- [ ] Draw ball as white circle
- [ ] Position objects correctly on screen
- [ ] Add basic game constants (dimensions, positions)
- **Learning Focus**: 2D rendering, coordinate systems, constants organization

### Iteration 3: Player Input
- [ ] Implement paddle movement with keyboard input
- [ ] W/S keys for left paddle
- [ ] Up/Down arrows for right paddle
- [ ] Constrain paddle movement to screen bounds
- **Learning Focus**: Input handling, boundary checking, real-time controls

### Iteration 4: Ball Physics
- [ ] Add ball velocity and movement
- [ ] Implement basic ball bouncing off top/bottom walls
- [ ] Ball resets when it goes off screen
- **Learning Focus**: Basic physics, velocity, collision with boundaries

### Iteration 5: Paddle-Ball Collision
- [ ] Detect collision between ball and paddles
- [ ] Ball bounces off paddles
- [ ] Change ball direction based on paddle hit location
- **Learning Focus**: Collision detection algorithms, vector math

### Iteration 6: Scoring System
- [ ] Track player scores
- [ ] Display scores on screen
- [ ] Reset ball position after scoring
- [ ] Basic game over condition
- **Learning Focus**: Game state management, UI rendering

### Iteration 7: Game States
- [ ] Implement game state enum (Menu, Playing, Paused, GameOver)
- [ ] Add start screen
- [ ] Add pause functionality
- [ ] Add game over screen with restart option
- **Learning Focus**: State machines, menu systems

### Iteration 8: Enhanced Physics
- [ ] Ball speed increases after paddle hits
- [ ] Smooth paddle movement with acceleration
- [ ] Improved ball angle calculation based on paddle hit position
- **Learning Focus**: Advanced physics, game feel, momentum

### Iteration 9: Visual Polish
- [ ] Add particle effects for ball hits
- [ ] Screen shake on paddle hits
- [ ] Better typography and UI layout
- [ ] Visual feedback for scoring
- **Learning Focus**: Juice, visual effects, game polish

### Iteration 10: Audio & Final Features
- [ ] Add sound effects for paddle hits, scoring, wall bounces
- [ ] Add background music
- [ ] Implement simple AI opponent option
- [ ] Add settings/options menu
- **Learning Focus**: Audio integration, AI behavior, complete game experience

## Key Learning Concepts by Iteration

### Core Programming Concepts
- **Game Loop Pattern**: Update → Render cycle
- **State Management**: Tracking game objects and states
- **Input Handling**: Real-time user input processing
- **Collision Detection**: Rectangle-circle and rectangle-rectangle collision
- **Vector Math**: 2D movement, velocity, and direction calculations

### Rust-Specific Concepts
- **Ownership**: Managing mutable game state
- **Structs & Enums**: Modeling game entities and states
- **Pattern Matching**: Handling different game states
- **Error Handling**: Graceful error management in games
- **Performance**: Avoiding allocations in hot game loops

### Game Development Concepts
- **Entity Design**: Modeling paddles, ball, and game objects
- **Physics Simulation**: Movement, collision, and response
- **Game Feel**: Making interactions satisfying (juice)
- **State Machines**: Managing different game phases
- **Audio-Visual Feedback**: Creating engaging player experience

## Architecture Evolution
As the project progresses, the architecture will evolve from a single main.rs file to a modular structure:

```
src/
├── main.rs           # Entry point and main game loop
├── game.rs           # Core game state and logic  
├── paddle.rs         # Paddle entity and behavior
├── ball.rs           # Ball entity and physics
├── collision.rs      # Collision detection and response
├── input.rs          # Input handling
├── audio.rs          # Sound effects and music
└── constants.rs      # Game constants and configuration
```

## Success Metrics
- Functional Pong game with all basic mechanics
- Clean, well-organized Rust code following best practices
- Understanding of fundamental game programming concepts
- Ability to extend the game with additional features
- Solid grasp of Macroquad framework capabilities

## Current Status
**Iteration 1** - Basic window and game loop established. Ready to begin Iteration 2 with static game object rendering.