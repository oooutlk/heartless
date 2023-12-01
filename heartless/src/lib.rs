//! The heartless game library.
//!
//! This is a game similar with mshearts but running in console.
//!
//! See README.md for the rule of the game, the commandline interface and the input.
//!
//! If the library users want to run this game in the background,
//! and doing interaction with the player in another UI frontend, just enables the altio feature.
//!
//! # Example
//!
//! ```toml
//! [dependencies]
//! heartless = { version = "0.1", features = ["altio"] }
//! ```
//!
//! ```rust,no_run
//! #[cfg( feature="altio" )] {
//!     let config = heartless::cli::Config::from_args( std::env::args_os() );
//!     let mut game = heartless::Game::with_config( config );
//!     let game_io = game.altio.clone();
//!     std::thread::spawn( move || game.main_loop() );
//!     // use `game_io.send_line()`/`game_io.recv()` to communicate with `game`.
//!     // See https://docs.rs/altio for more.
//! }
//! ```

pub mod card;
pub use card::*;

pub mod cli;

pub mod game;
pub use game::*;

pub mod player;
pub use player::*;

#[cfg( all( test, feature="altio" ))]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let mut game = Game::with_config( cli::Config {
            ascii_suit      : true,
            lowercase_rank  : false,
            no_shared_suit  : false,
            no_spaces       : false,
            me              : None,
            left            : None,
            opposite        : None,
            right           : None,
            seed            : Some( 2024 ),
            count           : Some( 1 ),
            timeout         : None,
            impatient       : Some( true ),
            automatic       : true,
        });

        let io = game.altio.clone();
        game.main_loop();

        assert_eq!( io.recv(),     include_str!( "../output/out-seed_2024" ));
        assert_eq!( io.recv_err(), include_str!( "../output/err-seed_2024" ));
    }
}
