//! Commandline interface and program configuration.

use std::{
    ffi::OsString,
    path::PathBuf
};

#[derive( clap::Parser )]
struct Cli {
    #[arg(long, help("Use c,d,s,h to display Club,Diamond,Spade,Heart"))]
    ascii_suit      : bool,
    #[arg(long, help("Use j,q,k,a to display Jade,Queen,King,Ace"))]
    lowercase_rank  : bool,
    #[arg(long, help("Each card is represented in the form of its rank followed by its suit"))]
    no_shared_suit  : bool,
    #[arg(long, help("No whitespaces in between cards"))]
    no_spaces       : bool,
    #[arg(long, help("Script for the left,opposite,right players, unless otherwise specified"))]
    others          : Option<PathBuf>,
    #[arg(long, help("Script for me to hint/decide which card(s) to pass/discard, see --automatic"))]
    me              : Option<PathBuf>,
    #[arg(long, help("Script for the left player"))]
    left            : Option<PathBuf>,
    #[arg(long, help("Script for the opposite player"))]
    opposite        : Option<PathBuf>,
    #[arg(long, help("Script for the right player"))]
    right           : Option<PathBuf>,
    #[arg(long, help("Script for all players including me, unless otherwise specified"))]
    all             : Option<PathBuf>,
    #[arg(long, help("Provide a random seed"))]
    seed            : Option<u64>,
    #[arg(long, help("For how many times playing the games"))]
    count           : Option<u32>,
    #[arg(long, help("Timout in milliseconds for scripting"))]
    timeout         : Option<u64>,
    #[arg(long, help("Don't delay output of each hand"))]
    impatient       : Option<bool>,
    #[arg(long, help("No user input, use script given by '--me'/'-all' or built-in strategy for me"))]
    pub automatic   : bool,
}

/// The configuration of heartless game.
#[derive( Clone, Debug )]
pub struct Config {
    /// Prints clubs, diamonds, spades, hearts as 'c','d','s','h'
    pub ascii_suit      : bool,
    /// Prints Jack, Queen, King, Ace as 'j','q','k','a'
    pub lowercase_rank  : bool,
    /// Prints each card in hand with its own trailing suit char
    pub no_shared_suit  : bool,
    /// Prints cards without whitespaces
    pub no_spaces       : bool,
    /// The path of AI script for me to hint/decide which card(s) to pass/discard
    pub me              : Option<PathBuf>,
    /// The path of AI script for the left player
    pub left            : Option<PathBuf>,
    /// The path of AI script for the opposite player
    pub opposite        : Option<PathBuf>,
    /// The path of AI script for the right player
    pub right           : Option<PathBuf>,
    /// The initial random seed to get determined dealing
    pub seed            : Option<u64>,
    /// The number of times the game will be played
    pub count           : Option<u32>,
    /// The timeout milliseconds for AI scripts
    pub timeout         : Option<u64>,
    /// Don't delay output of each hand
    pub impatient       : Option<bool>,
    /// No user input required
    pub automatic       : bool,
}

impl Config {
    pub fn from_args<I,T>( iter: I ) -> Self
        where I: IntoIterator<Item=T>
            , T: Into<OsString> + Clone
    {
        let cli = <Cli as clap::Parser>::parse_from( iter );

        Config {
            ascii_suit      : cli.ascii_suit,
            lowercase_rank  : cli.lowercase_rank,
            no_shared_suit  : cli.no_shared_suit,
            no_spaces       : cli.no_spaces,
            me              : cli.me                                       .or_else( || cli.all.clone() ),
            left            : cli.left    .or_else( || cli.others.clone() ).or_else( || cli.all.clone() ),
            opposite        : cli.opposite.or_else( || cli.others.clone() ).or_else( || cli.all.clone() ),
            right           : cli.right   .or_else( || cli.others.clone() ).or_else( || cli.all.clone() ),
            seed            : cli.seed,
            count           : cli.count,
            timeout         : cli.timeout,
            impatient       : cli.impatient,
            automatic       : cli.automatic,
        }
    }
}
