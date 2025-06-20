use macroquad::prelude::*;
use macroquad::rand::gen_range;

mod constants;
use constants::*;

#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Menu,
    ModeSelect,
    Playing,
    Paused,
    GameOver,
}

#[derive(Clone, Copy, PartialEq)]
enum GameMode {
    TwoPlayer,
    VsAI,
}

#[derive(Clone)]
struct Particle {
    x: f32,
    y: f32,
    vel_x: f32,
    vel_y: f32,
    life: f32,
    max_life: f32,
    size: f32,
}

impl Particle {
    fn new(x: f32, y: f32, vel_x: f32, vel_y: f32, life: f32, size: f32) -> Self {
        Self {
            x,
            y,
            vel_x,
            vel_y,
            life,
            max_life: life,
            size,
        }
    }
    
    fn update(&mut self, dt: f32) -> bool {
        self.x += self.vel_x * dt;
        self.y += self.vel_y * dt;
        self.life -= dt;
        
        // Add gravity
        self.vel_y += 300.0 * dt;
        
        self.life > 0.0
    }
    
    fn draw(&self) {
        let alpha = self.life / self.max_life;
        let color = Color::new(1.0, 1.0, 1.0, alpha);
        draw_circle(self.x, self.y, self.size * alpha, color);
    }
}

#[macroquad::main("Pong")]
async fn main() {
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    // Paddle positions (Y coordinates, X is constant)
    let mut left_paddle_y = PADDLE_START_Y;
    let mut right_paddle_y = PADDLE_START_Y;

    // Ball position and velocity
    let mut ball_x = BALL_START_X;
    let mut ball_y = BALL_START_Y;
    let mut ball_vel_x = BALL_SPEED;
    let mut ball_vel_y = BALL_SPEED * 0.3; // Start with some angle
    let mut current_ball_speed = BALL_SPEED; // Track current ball speed
    
    // Paddle velocities for smooth movement
    let mut left_paddle_vel = 0.0;
    let mut right_paddle_vel = 0.0;

    // Player scores
    let mut left_score = 0;
    let mut right_score = 0;
    
    // Game state
    let mut game_state = GameState::Menu;
    let mut game_mode = GameMode::TwoPlayer;
    
    // Visual effects
    let mut particles: Vec<Particle> = Vec::new();
    let mut screen_shake = 0.0;
    let mut score_flash_timer = 0.0;

    loop {
        clear_background(BLACK);
        
        match game_state {
            GameState::Menu => {
                // Draw title
                let title = "PONG";
                let title_width = measure_text(title, None, 80, 1.0).width;
                draw_text(
                    title,
                    (SCREEN_WIDTH - title_width) / 2.0,
                    SCREEN_HEIGHT / 2.0 - 100.0,
                    80.0,
                    WHITE,
                );
                
                // Draw instructions
                let start_text = "Press SPACE to Start";
                let start_width = measure_text(start_text, None, 30, 1.0).width;
                draw_text(
                    start_text,
                    (SCREEN_WIDTH - start_width) / 2.0,
                    SCREEN_HEIGHT / 2.0,
                    30.0,
                    GRAY,
                );
                
                let controls_text = "Controls: Left: W/S  Right: Up/Down";
                let controls_width = measure_text(controls_text, None, 20, 1.0).width;
                draw_text(
                    controls_text,
                    (SCREEN_WIDTH - controls_width) / 2.0,
                    SCREEN_HEIGHT / 2.0 + 50.0,
                    20.0,
                    GRAY,
                );
                
                let exit_text = "ESC: Exit";
                let exit_width = measure_text(exit_text, None, 20, 1.0).width;
                draw_text(
                    exit_text,
                    (SCREEN_WIDTH - exit_width) / 2.0,
                    SCREEN_HEIGHT / 2.0 + 80.0,
                    20.0,
                    GRAY,
                );
                
                // Handle input
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::ModeSelect;
                }
            }
            
            GameState::ModeSelect => {
                // Draw title
                let title = "Select Game Mode";
                let title_width = measure_text(title, None, 40, 1.0).width;
                draw_text(
                    title,
                    (SCREEN_WIDTH - title_width) / 2.0,
                    SCREEN_HEIGHT / 2.0 - 80.0,
                    40.0,
                    WHITE,
                );
                
                // Draw mode options
                let mode1_color = if matches!(game_mode, GameMode::TwoPlayer) { YELLOW } else { WHITE };
                let mode1_text = "1: Two Player";
                let mode1_width = measure_text(mode1_text, None, 30, 1.0).width;
                draw_text(
                    mode1_text,
                    (SCREEN_WIDTH - mode1_width) / 2.0,
                    SCREEN_HEIGHT / 2.0 - 20.0,
                    30.0,
                    mode1_color,
                );
                
                let mode2_color = if matches!(game_mode, GameMode::VsAI) { YELLOW } else { WHITE };
                let mode2_text = "2: vs AI";
                let mode2_width = measure_text(mode2_text, None, 30, 1.0).width;
                draw_text(
                    mode2_text,
                    (SCREEN_WIDTH - mode2_width) / 2.0,
                    SCREEN_HEIGHT / 2.0 + 20.0,
                    30.0,
                    mode2_color,
                );
                
                let start_text = "SPACE: Start Game";
                let start_width = measure_text(start_text, None, 20, 1.0).width;
                draw_text(
                    start_text,
                    (SCREEN_WIDTH - start_width) / 2.0,
                    SCREEN_HEIGHT / 2.0 + 80.0,
                    20.0,
                    GRAY,
                );
                
                let back_text = "ESC: Back to Menu";
                let back_width = measure_text(back_text, None, 20, 1.0).width;
                draw_text(
                    back_text,
                    (SCREEN_WIDTH - back_width) / 2.0,
                    SCREEN_HEIGHT / 2.0 + 110.0,
                    20.0,
                    GRAY,
                );
                
                // Handle input
                if is_key_pressed(KeyCode::Key1) {
                    game_mode = GameMode::TwoPlayer;
                } else if is_key_pressed(KeyCode::Key2) {
                    game_mode = GameMode::VsAI;
                } else if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                } else if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Menu;
                }
            }
            
            GameState::Playing => {
                let dt = get_frame_time();
                
                // Handle input and update paddle velocities with immediate response + buildup
                // Left paddle controls (W/S)
                if is_key_down(KeyCode::W) {
                    if left_paddle_vel > -PADDLE_BASE_SPEED {
                        left_paddle_vel = -PADDLE_BASE_SPEED; // Immediate base speed
                    }
                    left_paddle_vel -= PADDLE_ACCELERATION * dt; // Add acceleration buildup
                } else if is_key_down(KeyCode::S) {
                    if left_paddle_vel < PADDLE_BASE_SPEED {
                        left_paddle_vel = PADDLE_BASE_SPEED; // Immediate base speed
                    }
                    left_paddle_vel += PADDLE_ACCELERATION * dt; // Add acceleration buildup
                } else {
                    // Apply friction when no input
                    left_paddle_vel *= PADDLE_FRICTION;
                }
                
                // Right paddle controls (Up/Down arrows or AI)
                match game_mode {
                    GameMode::TwoPlayer => {
                        if is_key_down(KeyCode::Up) {
                            if right_paddle_vel > -PADDLE_BASE_SPEED {
                                right_paddle_vel = -PADDLE_BASE_SPEED;
                            }
                            right_paddle_vel -= PADDLE_ACCELERATION * dt;
                        } else if is_key_down(KeyCode::Down) {
                            if right_paddle_vel < PADDLE_BASE_SPEED {
                                right_paddle_vel = PADDLE_BASE_SPEED;
                            }
                            right_paddle_vel += PADDLE_ACCELERATION * dt;
                        } else {
                            right_paddle_vel *= PADDLE_FRICTION;
                        }
                    }
                    GameMode::VsAI => {
                        // Simple AI: predict where ball will be and move towards it
                        let predicted_ball_y = if ball_vel_x > 0.0 {
                            // Ball moving towards AI paddle
                            let time_to_reach = (RIGHT_PADDLE_X - ball_x) / ball_vel_x.abs();
                            ball_y + ball_vel_y * time_to_reach * AI_PREDICTION
                        } else {
                            ball_y
                        };
                        
                        // Add some random error to make AI beatable
                        let target_y = predicted_ball_y + gen_range(-AI_ERROR, AI_ERROR);
                        let paddle_center = right_paddle_y + PADDLE_HEIGHT / 2.0;
                        
                        // Move AI paddle towards target
                        let diff = target_y - paddle_center;
                        if diff.abs() > 10.0 {
                            if diff < 0.0 {
                                right_paddle_vel = -AI_SPEED;
                            } else {
                                right_paddle_vel = AI_SPEED;
                            }
                        } else {
                            right_paddle_vel *= 0.9; // Slow down when close to target
                        }
                    }
                }
                
                // Clamp paddle velocities to maximum speed
                left_paddle_vel = left_paddle_vel.clamp(-MAX_PADDLE_SPEED, MAX_PADDLE_SPEED);
                right_paddle_vel = right_paddle_vel.clamp(-MAX_PADDLE_SPEED, MAX_PADDLE_SPEED);
                
                // Update paddle positions
                left_paddle_y += left_paddle_vel * dt;
                right_paddle_y += right_paddle_vel * dt;
                
                // Keep paddles within screen bounds
                left_paddle_y = left_paddle_y.clamp(0.0, SCREEN_HEIGHT - PADDLE_HEIGHT);
                right_paddle_y = right_paddle_y.clamp(0.0, SCREEN_HEIGHT - PADDLE_HEIGHT);
                
                // Stop velocity if hitting boundaries
                if left_paddle_y <= 0.0 || left_paddle_y >= SCREEN_HEIGHT - PADDLE_HEIGHT {
                    left_paddle_vel = 0.0;
                }
                if right_paddle_y <= 0.0 || right_paddle_y >= SCREEN_HEIGHT - PADDLE_HEIGHT {
                    right_paddle_vel = 0.0;
                }
                
                // Update ball position
                ball_x += ball_vel_x * dt;
                ball_y += ball_vel_y * dt;
                
                // Ball collision with top and bottom walls
                if ball_y <= BALL_SIZE || ball_y >= SCREEN_HEIGHT - BALL_SIZE {
                    ball_vel_y = -ball_vel_y;
                    ball_y = ball_y.clamp(BALL_SIZE, SCREEN_HEIGHT - BALL_SIZE);
                }
                
                // Ball collision with left paddle
                if ball_x - BALL_SIZE <= LEFT_PADDLE_X + PADDLE_WIDTH
                    && ball_x + BALL_SIZE >= LEFT_PADDLE_X
                    && ball_y + BALL_SIZE >= left_paddle_y
                    && ball_y - BALL_SIZE <= left_paddle_y + PADDLE_HEIGHT
                    && ball_vel_x < 0.0
                {
                    // Increase ball speed
                    current_ball_speed = (current_ball_speed * BALL_SPEED_INCREASE).min(MAX_BALL_SPEED);
                    
                    // Calculate hit position (0.0 = top, 1.0 = bottom)
                    let hit_pos = (ball_y - left_paddle_y) / PADDLE_HEIGHT;
                    
                    // Factor in paddle velocity for more dynamic bounces
                    let paddle_influence = left_paddle_vel * 0.1;
                    
                    // Calculate new velocities with improved angle
                    ball_vel_x = current_ball_speed;
                    ball_vel_y = current_ball_speed * (hit_pos - 0.5) * 2.5 + paddle_influence;
                    
                    // Add screen shake and particles
                    screen_shake = 0.15;
                    for _ in 0..8 {
                        particles.push(Particle::new(
                            LEFT_PADDLE_X + PADDLE_WIDTH,
                            ball_y,
                            gen_range(50.0, 200.0),
                            gen_range(-100.0, 100.0),
                            gen_range(0.3, 0.8),
                            gen_range(2.0, 5.0),
                        ));
                    }
                }
                
                // Ball collision with right paddle
                if ball_x + BALL_SIZE >= RIGHT_PADDLE_X
                    && ball_x - BALL_SIZE <= RIGHT_PADDLE_X + PADDLE_WIDTH
                    && ball_y + BALL_SIZE >= right_paddle_y
                    && ball_y - BALL_SIZE <= right_paddle_y + PADDLE_HEIGHT
                    && ball_vel_x > 0.0
                {
                    // Increase ball speed
                    current_ball_speed = (current_ball_speed * BALL_SPEED_INCREASE).min(MAX_BALL_SPEED);
                    
                    // Calculate hit position (0.0 = top, 1.0 = bottom)
                    let hit_pos = (ball_y - right_paddle_y) / PADDLE_HEIGHT;
                    
                    // Factor in paddle velocity for more dynamic bounces
                    let paddle_influence = right_paddle_vel * 0.1;
                    
                    // Calculate new velocities with improved angle
                    ball_vel_x = -current_ball_speed;
                    ball_vel_y = current_ball_speed * (hit_pos - 0.5) * 2.5 + paddle_influence;
                    
                    // Add screen shake and particles
                    screen_shake = 0.15;
                    for _ in 0..8 {
                        particles.push(Particle::new(
                            RIGHT_PADDLE_X,
                            ball_y,
                            gen_range(-200.0, -50.0),
                            gen_range(-100.0, 100.0),
                            gen_range(0.3, 0.8),
                            gen_range(2.0, 5.0),
                        ));
                    }
                }
                
                // Check for scoring and reset ball
                if ball_x < -BALL_SIZE {
                    right_score += 1;
                    ball_x = BALL_START_X;
                    ball_y = BALL_START_Y;
                    ball_vel_x = BALL_SPEED;
                    ball_vel_y = BALL_SPEED * 0.5;
                    current_ball_speed = BALL_SPEED; // Reset speed
                    score_flash_timer = 0.5; // Flash effect
                    screen_shake = 0.3; // Strong shake for scoring
                    
                    if right_score >= WINNING_SCORE {
                        game_state = GameState::GameOver;
                    }
                } else if ball_x > SCREEN_WIDTH + BALL_SIZE {
                    left_score += 1;
                    ball_x = BALL_START_X;
                    ball_y = BALL_START_Y;
                    ball_vel_x = -BALL_SPEED;
                    ball_vel_y = BALL_SPEED * 0.5;
                    current_ball_speed = BALL_SPEED; // Reset speed
                    score_flash_timer = 0.5; // Flash effect
                    screen_shake = 0.3; // Strong shake for scoring
                    
                    if left_score >= WINNING_SCORE {
                        game_state = GameState::GameOver;
                    }
                }
                
                // Update visual effects
                screen_shake = (screen_shake - dt * 10.0).max(0.0);
                score_flash_timer = (score_flash_timer - dt).max(0.0);
                
                // Update particles
                particles.retain_mut(|particle| particle.update(dt));
                
                // Apply screen shake offset
                let shake_x = if screen_shake > 0.0 { gen_range(-screen_shake * 10.0, screen_shake * 10.0) } else { 0.0 };
                let shake_y = if screen_shake > 0.0 { gen_range(-screen_shake * 10.0, screen_shake * 10.0) } else { 0.0 };
                
                // Draw center line
                for i in 0..20 {
                    let y = i as f32 * 30.0 + 10.0;
                    if y < SCREEN_HEIGHT {
                        draw_rectangle(
                            SCREEN_WIDTH / 2.0 - 2.0 + shake_x,
                            y + shake_y,
                            4.0,
                            15.0,
                            GRAY,
                        );
                    }
                }
                
                // Draw game objects with screen shake
                draw_rectangle(
                    LEFT_PADDLE_X + shake_x,
                    left_paddle_y + shake_y,
                    PADDLE_WIDTH,
                    PADDLE_HEIGHT,
                    WHITE,
                );
                draw_rectangle(
                    RIGHT_PADDLE_X + shake_x,
                    right_paddle_y + shake_y,
                    PADDLE_WIDTH,
                    PADDLE_HEIGHT,
                    WHITE,
                );
                draw_circle(ball_x + shake_x, ball_y + shake_y, BALL_SIZE, WHITE);
                
                // Draw particles
                for particle in &particles {
                    particle.draw();
                }
                
                // Draw scores with flash effect
                let score_text = format!("{}    {}", left_score, right_score);
                let score_width = measure_text(&score_text, None, 60, 1.0).width;
                let score_color = if score_flash_timer > 0.0 {
                    let flash = (score_flash_timer * 10.0).sin().abs();
                    Color::new(1.0, flash, flash, 1.0) // Red flash
                } else {
                    WHITE
                };
                draw_text(
                    &score_text,
                    (SCREEN_WIDTH - score_width) / 2.0 + shake_x,
                    100.0 + shake_y,
                    60.0,
                    score_color,
                );
                
                // Draw control instructions based on mode
                match game_mode {
                    GameMode::TwoPlayer => {
                        draw_text("Left: W/S  Right: Up/Down  P: Pause", 20.0, 20.0, 18.0, GRAY);
                    }
                    GameMode::VsAI => {
                        draw_text("Player: W/S  P: Pause", 20.0, 20.0, 18.0, GRAY);
                    }
                }
                
                // Handle pause
                if is_key_pressed(KeyCode::P) {
                    game_state = GameState::Paused;
                }
            }
            
            GameState::Paused => {
                // Draw game objects (frozen)
                draw_rectangle(LEFT_PADDLE_X, left_paddle_y, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE);
                draw_rectangle(RIGHT_PADDLE_X, right_paddle_y, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE);
                draw_circle(ball_x, ball_y, BALL_SIZE, WHITE);
                
                // Draw scores
                let score_text = format!("{}    {}", left_score, right_score);
                let score_width = measure_text(&score_text, None, 60, 1.0).width;
                draw_text(
                    &score_text,
                    (SCREEN_WIDTH - score_width) / 2.0,
                    100.0,
                    60.0,
                    WHITE,
                );
                
                // Draw pause overlay
                let pause_text = "PAUSED";
                let pause_width = measure_text(pause_text, None, 60, 1.0).width;
                draw_text(
                    pause_text,
                    (SCREEN_WIDTH - pause_width) / 2.0,
                    SCREEN_HEIGHT / 2.0,
                    60.0,
                    YELLOW,
                );
                
                let resume_text = "Press P to Resume";
                let resume_width = measure_text(resume_text, None, 30, 1.0).width;
                draw_text(
                    resume_text,
                    (SCREEN_WIDTH - resume_width) / 2.0,
                    SCREEN_HEIGHT / 2.0 + 60.0,
                    30.0,
                    GRAY,
                );
                
                // Handle resume
                if is_key_pressed(KeyCode::P) {
                    game_state = GameState::Playing;
                }
            }
            
            GameState::GameOver => {
                // Draw game objects (frozen)
                draw_rectangle(LEFT_PADDLE_X, left_paddle_y, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE);
                draw_rectangle(RIGHT_PADDLE_X, right_paddle_y, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE);
                draw_circle(ball_x, ball_y, BALL_SIZE, WHITE);
                
                // Draw scores
                let score_text = format!("{}    {}", left_score, right_score);
                let score_width = measure_text(&score_text, None, 60, 1.0).width;
                draw_text(
                    &score_text,
                    (SCREEN_WIDTH - score_width) / 2.0,
                    100.0,
                    60.0,
                    WHITE,
                );
                
                // Draw winner
                let winner = match game_mode {
                    GameMode::TwoPlayer => {
                        if left_score >= WINNING_SCORE {
                            "Left Player Wins!"
                        } else {
                            "Right Player Wins!"
                        }
                    }
                    GameMode::VsAI => {
                        if left_score >= WINNING_SCORE {
                            "You Win!"
                        } else {
                            "AI Wins!"
                        }
                    }
                };
                let win_width = measure_text(winner, None, 40, 1.0).width;
                draw_text(
                    winner,
                    (SCREEN_WIDTH - win_width) / 2.0,
                    SCREEN_HEIGHT / 2.0,
                    40.0,
                    YELLOW,
                );
                
                let restart_text = "Press R to Restart";
                let restart_width = measure_text(restart_text, None, 30, 1.0).width;
                draw_text(
                    restart_text,
                    (SCREEN_WIDTH - restart_width) / 2.0,
                    SCREEN_HEIGHT / 2.0 + 60.0,
                    30.0,
                    GRAY,
                );
                
                let menu_text = "Press M to Change Mode";
                let menu_width = measure_text(menu_text, None, 30, 1.0).width;
                draw_text(
                    menu_text,
                    (SCREEN_WIDTH - menu_width) / 2.0,
                    SCREEN_HEIGHT / 2.0 + 100.0,
                    30.0,
                    GRAY,
                );
                
                // Handle restart/menu
                if is_key_pressed(KeyCode::R) {
                    // Reset game
                    left_score = 0;
                    right_score = 0;
                    ball_x = BALL_START_X;
                    ball_y = BALL_START_Y;
                    ball_vel_x = BALL_SPEED;
                    ball_vel_y = BALL_SPEED * 0.5;
                    current_ball_speed = BALL_SPEED;
                    left_paddle_y = PADDLE_START_Y;
                    right_paddle_y = PADDLE_START_Y;
                    left_paddle_vel = 0.0;
                    right_paddle_vel = 0.0;
                    particles.clear();
                    screen_shake = 0.0;
                    score_flash_timer = 0.0;
                    game_state = GameState::Playing;
                }
                if is_key_pressed(KeyCode::M) {
                    // Reset game and go to mode select
                    left_score = 0;
                    right_score = 0;
                    ball_x = BALL_START_X;
                    ball_y = BALL_START_Y;
                    ball_vel_x = BALL_SPEED;
                    ball_vel_y = BALL_SPEED * 0.5;
                    current_ball_speed = BALL_SPEED;
                    left_paddle_y = PADDLE_START_Y;
                    right_paddle_y = PADDLE_START_Y;
                    left_paddle_vel = 0.0;
                    right_paddle_vel = 0.0;
                    particles.clear();
                    screen_shake = 0.0;
                    score_flash_timer = 0.0;
                    game_state = GameState::ModeSelect;
                }
            }
        }
        
        // Global exit
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        
        next_frame().await;
    }
}
