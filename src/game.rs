use crate::util::{
    BALL_ACC, BALL_SPEED_HARD, BALL_SPEED_NORMAL, CENTER_LINE_SIZE, DASHED_MIDDLE_LINE,
    DEFAULT_UPDATE_RATE, FREEZE_UPDATE_RATE, MAIN_MENU_FONT, MAIN_MENU_HEADER,
    MAIN_MENU_HEADER_SIZE, MAIN_MENU_USAGE, MAIN_MENU_USAGE_SIZE, PADDLE_SPEED_HIGH,
    PADDLE_SPEED_NORMAL, PADDLE_SPIN, PONG_GAME_FONT, SCORE_LIMIT, SCORE_TEXT_OFFSET,
    SCORE_TEXT_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH, WIN_MESSAGE,
};
use rand::random;
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{time, Context, Event, State};

/// Interactive game structure to hold game objects
struct SpriteEntity {
    /// Sprite
    sprite: Texture,
    /// Position (2D coordinates - x and y)
    position: Vec2<f32>,
    /// velocity
    speed: Vec2<f32>,
}

impl SpriteEntity {
    // Constructor for a game object without a speed
    // - use the underscore to have no compiler warnings
    fn _new(sprite: Texture, position: Vec2<f32>) -> Self {
        Self {
            sprite,
            position,
            speed: Vec2::zero(),
        }
    }

    // Constructor for a game object that has a speed
    fn with_velocity(sprite: Texture, position: Vec2<f32>, speed: Vec2<f32>) -> Self {
        Self {
            sprite,
            position,
            speed,
        }
    }

    // GETTER
    fn width(&self) -> f32 {
        self.sprite.width() as f32
    }

    fn height(&self) -> f32 {
        self.sprite.height() as f32
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }

    fn centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0),
        )
    }
}

/// Game score structure
struct TextEntity {
    text_to_display: Text,
    value: u8,
    position: Vec2<f32>,
}

impl TextEntity {
    // Constructor for normal text object
    fn new(text_to_display: Text, pos: Vec2<f32>) -> Self {
        Self {
            text_to_display,
            value: 0,
            position: pos,
        }
    }

    // Constructor for the game score
    fn with_score_value(text_to_display: Text, value: u8, pos: Vec2<f32>) -> Self {
        Self {
            text_to_display,
            value,
            position: pos,
        }
    }
}

/// GameState object to hold all required things for the game.
pub struct GameState {
    player_1: SpriteEntity,
    player_2: SpriteEntity,
    score_player_1: TextEntity,
    score_player_2: TextEntity,
    ball: SpriteEntity,
    center_line: TextEntity,
    main_menu_header: TextEntity,
    main_menu_usage: TextEntity,
    winner_msg: TextEntity,
    is_paused: bool,
    is_main_menu_showing: bool,
    is_to_end: bool,
}

impl GameState {
    // constructor for the GameState struct
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        // game starts for the first time -> so showing the main menu
        let is_main_menu_showing = true;

        // Game is running
        let is_paused = false;

        // No player wins at this point, therefore the game is not to end
        let is_to_end = false;

        // create the main menu header style
        let main_menu_header_text_style = Text::new(
            MAIN_MENU_HEADER,
            Font::vector(ctx, MAIN_MENU_FONT, MAIN_MENU_HEADER_SIZE)?,
        );

        // create the main menu usage style
        let main_menu_usage_text_style = Text::new(
            MAIN_MENU_USAGE,
            Font::vector(ctx, MAIN_MENU_FONT, MAIN_MENU_USAGE_SIZE)?,
        );

        // create main menu header object
        let main_menu_header = TextEntity::new(main_menu_header_text_style, Vec2::new(160.0, 40.0));

        // create the main menu usage object
        let main_menu_usage = TextEntity::new(main_menu_usage_text_style, Vec2::new(160.0, 100.0));

        // create the dashed center line style
        let center_line_text_style = Text::new(
            DASHED_MIDDLE_LINE,
            Font::vector(ctx, PONG_GAME_FONT, CENTER_LINE_SIZE)?,
        );

        // create the center line text object
        let center_line =
            TextEntity::new(center_line_text_style, Vec2::new(WINDOW_WIDTH / 2.0, 0.0));

        // common Text for the game-score
        let score_text_style = Text::new("0", Font::vector(ctx, PONG_GAME_FONT, SCORE_TEXT_SIZE)?);

        // create the winner msg style
        let winner_msg_style = Text::new(
            WIN_MESSAGE,
            Font::vector(ctx, MAIN_MENU_FONT, MAIN_MENU_HEADER_SIZE)?,
        );

        // create winner msg text object
        let winner_msg = TextEntity::new(winner_msg_style, Vec2::zero());

        // create score player 1
        let score_player_1 = TextEntity::with_score_value(
            score_text_style.clone(),
            0,
            Vec2::new((WINDOW_WIDTH / 2.0) - 43.0, 16.0),
        );

        // load the paddle sprite for player 1 from the resources folder
        let paddle_player_1 = Texture::new(ctx, "./resources/player1.png")?;
        // pos player 1
        let pos_player_1 = Vec2::new(
            16.0,
            (WINDOW_HEIGHT - paddle_player_1.height() as f32) / 2.0,
        );
        // create player 1 -> paddle has normal speed, if player choose game play mode 'HARD' it will be adjust
        let player_1 = SpriteEntity::with_velocity(
            paddle_player_1,
            pos_player_1,
            Vec2::new(0.0, PADDLE_SPEED_NORMAL),
        );

        // create score player 2
        let score_player_2 = TextEntity::with_score_value(
            score_text_style,
            0,
            Vec2::new(WINDOW_WIDTH / 2.0, 0.0) + SCORE_TEXT_OFFSET,
        );

        // pos player 2
        let paddle_player_2 = Texture::new(ctx, "./resources/player2.png")?;
        let pos_player_2 = Vec2::new(
            WINDOW_WIDTH - (paddle_player_2.width() as f32) - 16.0,
            (WINDOW_HEIGHT - paddle_player_2.height() as f32) / 2.0,
        );

        // create player 2
        let player_2 = SpriteEntity::with_velocity(
            paddle_player_2,
            pos_player_2,
            Vec2::new(0.0, PADDLE_SPEED_NORMAL),
        );

        // create the ball sprite
        let ball_sprite = Texture::new(ctx, "./resources/ball.png")?;

        // ball will be centered on screen
        let ball_pos = Vec2::new(
            (WINDOW_WIDTH - ball_sprite.width() as f32) / 2.0,
            (WINDOW_HEIGHT - ball_sprite.height() as f32) / 2.0,
        );

        // create the ball sprite -> with zero ball speed
        // the ball speed will be set later, dependent on the user input (Normal or Hard)
        let ball: SpriteEntity = SpriteEntity::with_velocity(ball_sprite, ball_pos, Vec2::zero());

        Ok(GameState {
            player_1,
            player_2,
            score_player_1,
            score_player_2,
            ball,
            center_line,
            main_menu_header,
            main_menu_usage,
            winner_msg,
            is_paused,
            is_main_menu_showing,
            is_to_end,
        })
    }

    // Method to check if the ball goes out of the screen and if this the case,
    // increment the score counter for the right player and repositioning the ball.
    fn score_checker(&mut self) {
        // we increase the actual window width, to achieve a small delay until the ball comes into play
        if self.ball.position.x > (WINDOW_WIDTH + 100.0) {
            // Player 1 (left hand side) gets one point
            self.score_player_1.value += 1;
            self.score_player_1
                .text_to_display
                .set_content(self.score_player_1.value.to_string());

            // repositioning of the ball
            self.ball.position = Vec2::new(
                (WINDOW_WIDTH - self.ball.width()) / 2.0,
                (WINDOW_HEIGHT - self.ball.height()) / 2.0,
            );

            // move the ball randomly to player one or player two
            match random() {
                // make sign negative and move the ball to the left hand side
                true => self.ball.speed = Vec2::new(-self.ball.speed.x, 0.0),
                // reverse the sign and move the ball to the right hand side
                false => self.ball.speed = Vec2::new(-(-self.ball.speed.x), 0.0),
            }
        } else if self.ball.position.x < -100.0 {
            // Player 2 (right hand side) gets one point
            self.score_player_2.value += 1;
            self.score_player_2
                .text_to_display
                .set_content(self.score_player_2.value.to_string());

            // repositioning of the ball
            self.ball.position = Vec2::new(
                (WINDOW_WIDTH - self.ball.width()) / 2.0,
                (WINDOW_HEIGHT - self.ball.height()) / 2.0,
            );

            match random() {
                true => self.ball.speed = Vec2::new(-self.ball.speed.x, 0.0),
                false => self.ball.speed = Vec2::new(-(-self.ball.speed.x), 0.0),
            }
        }

        // check score limit -> if it's reached -> game ends
        if self.score_player_1.value == SCORE_LIMIT || self.score_player_2.value == SCORE_LIMIT {
            self.is_to_end = true;
        }
    }
}

// we override the default error type from tetra crate with the given generic type annotation
// and we will use anyhow::Error to get a better error description
impl State<anyhow::Error> for GameState {
    // All implement methods in the trait 'State' will be automatically called from the game loop within the tetra::Context
    // by passing the GameState struct into the Context.run() method within the main function
    fn draw(&mut self, ctx: &mut tetra::Context) -> anyhow::Result<()> {
        // we can implements here our own logic to draw the game screen or canvas
        // if an error occurs, we will be able to use the anyhow::Error variant to
        // make the error more readable

        // First: clear always the screen and fill them with baby blue color
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        // On game start, or if player pressed 'Backspace', show the main menu
        if self.is_main_menu_showing {
            self.main_menu_header
                .text_to_display
                .draw(ctx, self.main_menu_header.position);
            self.main_menu_usage
                .text_to_display
                .draw(ctx, self.main_menu_usage.position);

            // check if one player wins
        } else if self.is_to_end {

            // draw the dashed center line
            self.center_line
                .text_to_display
                .draw(ctx, self.center_line.position);

            // draw the endpoint status
            self.score_player_1
                .text_to_display
                .draw(ctx, self.score_player_1.position);
            self.score_player_2
                .text_to_display
                .draw(ctx, self.score_player_2.position);

            // draw the winner message for the right player
            if self.score_player_1.value == SCORE_LIMIT {
                self.winner_msg
                    .text_to_display
                    .draw(ctx, Vec2::new((WINDOW_WIDTH / 2.0) - 300.0, 100.0));
            } else if self.score_player_2.value == SCORE_LIMIT {
                self.winner_msg
                    .text_to_display
                    .draw(ctx, Vec2::new((WINDOW_WIDTH / 2.0) + 25.0, 100.0));
            }
        } else {
            // Otherwise start the game and draw the whole game context
            self.ball.sprite.draw(ctx, self.ball.position);

            // draw the dashed center line
            self.center_line
                .text_to_display
                .draw(ctx, self.center_line.position);

            // draw all required objects on screen
            self.score_player_1
                .text_to_display
                .draw(ctx, self.score_player_1.position);
            self.score_player_2
                .text_to_display
                .draw(ctx, self.score_player_2.position);
            self.player_1.sprite.draw(ctx, self.player_1.position);
            self.player_2.sprite.draw(ctx, self.player_2.position);
        }

        Ok(())
    }

    // Method is automatically called 60 times per second
    // and updates the game objects on the screen
    fn update(&mut self, ctx: &mut Context) -> Result<(), anyhow::Error> {
        // Update game objects only if the game currently running
        // that means no main menu is shown and no player win the game
        if !self.is_main_menu_showing && !self.is_to_end {
            // define game border (top and bottom)
            let upper_game_limit: f32 = 10.0;
            let lower_game_limit: f32 = (WINDOW_HEIGHT - self.player_1.sprite.height() as f32) - 10.0;

            //Movement player 1
            if input::is_key_down(ctx, Key::W) {
                // move paddle up -> if the paddle is in the right range
                if self.player_1.position.y > upper_game_limit {
                    self.player_1.position.y -= self.player_1.speed.y;
                }
            }

            if input::is_key_down(ctx, Key::S) {
                // move paddle down
                if self.player_1.position.y < lower_game_limit {
                    self.player_1.position.y += self.player_1.speed.y;
                }
            }

            // Movement player 2
            if input::is_key_down(ctx, Key::Up) {
                // move paddle up
                if self.player_2.position.y > upper_game_limit {
                    self.player_2.position.y -= self.player_2.speed.y;
                }
            }

            if input::is_key_down(ctx, Key::Down) {
                // move paddle down
                if self.player_2.position.y < lower_game_limit {
                    self.player_2.position.y += self.player_2.speed.y;
                }
            }

            // move the ball on the screen with a certain speed
            self.ball.position += self.ball.speed;

            // get bounds of the game objects
            let player1_bounds = self.player_1.bounds();
            let player2_bounds = self.player_2.bounds();
            let ball_bounds = self.ball.bounds();

            // use the build-in method 'intersects' to check if the ball rectangle collides with
            // one of the paddle rectangle
            let paddle_hit = if ball_bounds.intersects(&player1_bounds) {
                Some(&self.player_1)
            } else if ball_bounds.intersects(&player2_bounds) {
                Some(&self.player_2)
            } else {
                None
            };

            // when a collision occurred
            if let Some(paddle) = paddle_hit {
                // Increase the ball's velocity, then flip it.
                self.ball.speed.x = -(self.ball.speed.x + (BALL_ACC * self.ball.speed.x.signum()));

                // Calculate the offset between the paddle and the ball, as a number between
                // -1.0 and 1.0.
                let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();

                // Apply the spin to the ball.
                self.ball.speed.y += PADDLE_SPIN * -offset;
            }

            // check if the ball hit the top or the bottom of the screen
            if self.ball.position.y <= 0.0
                || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT
            {
                // if this is the case -> revert the ball direction
                self.ball.speed.y = -self.ball.speed.y;
            }

            // check if the ball goes out of the screen and calculate the score
            self.score_checker();
        }

        Ok(())
    }

    // Called when a window or input event occurs
    // Handle the keyboard events
    fn event(&mut self, ctx: &mut Context, event: tetra::Event) -> Result<(), anyhow::Error> {
        match event {
            Event::KeyPressed { key: Key::P } => {
                // it is only possible to pause the game, when the the main menu is not shown
                if !self.is_main_menu_showing {
                    // check if the game is already paused
                    if !self.is_paused {
                        // When user pressed 'P' reduce the update rate of the game from the game so that it freezes
                        // --> default update rate is 60.0_f64
                        time::set_timestep(ctx, time::Timestep::Fixed(FREEZE_UPDATE_RATE));
                        self.is_paused = true;
                    } else {
                        // run the game loop again, set the default game update rate -> 60.0
                        time::set_timestep(ctx, time::Timestep::Fixed(DEFAULT_UPDATE_RATE));
                        self.is_paused = false;
                    }
                }
            }
            Event::KeyPressed { key: Key::N } => {
                // it is only possible to set the game, when the main menu is shown
                if self.is_main_menu_showing {
                    // use method random() from rand crate to create a random bool to move the ball randomly to one of the two players
                    match random() {
                        // move the ball to the left hand side
                        true => self.ball.speed = Vec2::new(-BALL_SPEED_NORMAL, 0.0),
                        // otherwise move the ball to the right hand side
                        false => self.ball.speed = Vec2::new(BALL_SPEED_NORMAL, 0.0),
                    }
                }
                // Game starts -> main menu will be hide
                self.is_main_menu_showing = false;
                self.is_to_end = false;
            }
            Event::KeyPressed { key: Key::H } => {
                if self.is_main_menu_showing {
                    // adjust the paddle speed for both players, because the ball speed goes up
                    self.player_1.speed = Vec2::new(0.0, PADDLE_SPEED_HIGH);
                    self.player_2.speed = Vec2::new(0.0, PADDLE_SPEED_HIGH);

                    match random() {
                        true => self.ball.speed = Vec2::new(-BALL_SPEED_HARD, 0.0),
                        false => self.ball.speed = Vec2::new(BALL_SPEED_HARD, 0.0),
                    }
                }
                self.is_main_menu_showing = false;
                self.is_to_end = false;
            }
            Event::KeyPressed {
                key: Key::Backspace,
            } => {
                if !self.is_main_menu_showing {
                    self.is_main_menu_showing = true;

                    // reset the score value
                    self.score_player_1.value = 0;
                    self.score_player_2.value = 0;

                    // reset the visibility score text
                    self.score_player_1
                        .text_to_display
                        .set_content(self.score_player_1.value.to_string());
                    self.score_player_2
                        .text_to_display
                        .set_content(self.score_player_2.value.to_string());

                    // draw the cleaned score text again
                    self.score_player_1
                        .text_to_display
                        .draw(ctx, self.score_player_1.position);
                    self.score_player_2
                        .text_to_display
                        .draw(ctx, self.score_player_2.position);

                    // reset the paddle position for both players
                    self.player_1.position =
                        Vec2::new(16.0, (WINDOW_HEIGHT - self.player_1.height()) / 2.0);

                    self.player_2.position = Vec2::new(
                        WINDOW_WIDTH - (self.player_2.width()) - 16.0,
                        (WINDOW_HEIGHT - self.player_2.height()) / 2.0,
                    );

                    // draw the paddles again
                    self.player_1.sprite.draw(ctx, self.player_1.position);
                    self.player_2.sprite.draw(ctx, self.player_2.position);

                    // reset the ball
                    self.ball.position = Vec2::new(
                        (WINDOW_WIDTH - self.ball.width()) / 2.0,
                        (WINDOW_HEIGHT - self.ball.height()) / 2.0,
                    );

                    // draw the ball again
                    self.ball.sprite.draw(ctx, self.ball.position);
                }
            }
            _ => {}
        }

        Ok(())
    }
}
