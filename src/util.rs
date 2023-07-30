use tetra::math::Vec2;

pub const WINDOW_WIDTH: f32 = 640.0;
pub const WINDOW_HEIGHT: f32 = 480.0;
pub const PADDLE_SPEED_NORMAL: f32 = 8.0;
pub const PADDLE_SPEED_HIGH: f32 = 13.0;
pub const BALL_SPEED_NORMAL: f32 = 5.0;
pub const BALL_SPEED_HARD: f32 = 9.0;
pub const PADDLE_SPIN: f32 = 4.0;
pub const BALL_ACC: f32 = 0.05;
pub const SCORE_TEXT_OFFSET: Vec2<f32> = Vec2::new(32.0, 16.0);
pub const SCORE_TEXT_SIZE: f32 = 21.0;
pub const CENTER_LINE_SIZE: f32 = 18.0;
pub const MAIN_MENU_HEADER_SIZE: f32 = 20.0;
pub const MAIN_MENU_USAGE_SIZE: f32 = 15.0;
pub const FREEZE_UPDATE_RATE: f64 = 0.000001;
pub const DEFAULT_UPDATE_RATE: f64 = 60.0;
pub const SCORE_LIMIT: u8 = 15;
pub const PONG_GAME_FONT: &str = "./resources/pong.ttf";
pub const MAIN_MENU_FONT: &str = "./resources/comic.ttf";
pub const DASHED_MIDDLE_LINE: &str = "|
|
|
|
|
|
|
|
|
|
|
|
|
|
|
|
|
|
|
|
|
|
|
|
|
|
|";

pub const MAIN_MENU_HEADER: &str = ">---- The Pong-Game ----<";
pub const MAIN_MENU_USAGE: &str = "ESC             =>  Quit game
P                => Pause/Resume
Backspace => Main menu

Player 1 (Left hand side):
- Use 'W' and 'S' to move the paddle

Player 2 (Right hand side):
- Use 'UP' and 'DOWN' to move the paddle

Start playing, choose game play mode:
--------------------------------------
N    =>  Normal
H    =>  Hard";

pub const WIN_MESSAGE: &str = "   > You win the game <

Backspace => Main menu";
