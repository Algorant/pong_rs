// Screen dimensions
pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;

// Paddle properties
pub const PADDLE_WIDTH: f32 = 15.0;
pub const PADDLE_HEIGHT: f32 = 100.0;
pub const PADDLE_SPEED: f32 = 300.0;
pub const PADDLE_MARGIN: f32 = 50.0; // Distance from screen edge

// Ball properties
pub const BALL_SIZE: f32 = 15.0;
pub const BALL_SPEED: f32 = 250.0;

// Paddle positions
pub const LEFT_PADDLE_X: f32 = PADDLE_MARGIN;
pub const RIGHT_PADDLE_X: f32 = SCREEN_WIDTH - PADDLE_MARGIN - PADDLE_WIDTH;
pub const PADDLE_START_Y: f32 = (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0;

// Ball starting position
pub const BALL_START_X: f32 = SCREEN_WIDTH / 2.0;
pub const BALL_START_Y: f32 = SCREEN_HEIGHT / 2.0;

// Game settings
pub const WINNING_SCORE: i32 = 6;

// Enhanced physics
pub const BALL_SPEED_INCREASE: f32 = 1.05; // 5% speed increase per paddle hit
pub const MAX_BALL_SPEED: f32 = BALL_SPEED * 2.0; // Maximum ball speed
pub const PADDLE_BASE_SPEED: f32 = PADDLE_SPEED * 0.8; // Immediate response speed
pub const PADDLE_ACCELERATION: f32 = 600.0; // Additional acceleration buildup
pub const PADDLE_FRICTION: f32 = 0.85; // How much velocity is retained each frame
pub const MAX_PADDLE_SPEED: f32 = PADDLE_SPEED * 1.4; // Maximum paddle velocity

// AI settings
pub const AI_SPEED: f32 = PADDLE_SPEED * 0.7; // AI paddle speed (slightly slower than human)
pub const AI_PREDICTION: f32 = 0.3; // How far ahead AI predicts ball position
pub const AI_ERROR: f32 = 10.0; // Random error in AI positioning for difficulty

