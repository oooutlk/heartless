//! Implementation of game logic: input, output, rule checker, AI strategies.

use altio::{Altio, impl_altio_output, echo};

use crate::*;
use crate::cli::Config;

use std::{
    cell::Cell,
    ffi::OsString,
    io::Read,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

#[cfg( not( feature = "altio" ))]
use std::io::Write;

use wait_timeout::ChildExt;

#[derive( Copy, Clone, Debug, PartialEq )]
enum Input {
    Start,
    Discard( Cards ),
    Exit,
    Invalid,
    Pass( Cards ),
}

/// The game rules of heartless which are checked.
pub enum Rule {
    /// The player can only pass the cards that they hold in hand.
    PassInHand,
    /// The players should pass excactly 3 cards, no more, no less.
    PassThreeCards,
    /// The player can only discard the cards that they hold in hand.
    DiscardInHand,
    /// The early hand in round 1 should be Two of Clubs.
    TwoOfClubs,
    /// The late hands should follow the early hand's suit whenever possible.
    FollowSuit,
    /// The players are not allowed to discard any hearts as early hand, since no hearts have been discarded before.
    Heartbreak,
    /// Not allowed to discard hearts in round 1, unless the player holds 13 hearts in hand which is almost impossible.
    HeartInFirstRound,
    /// Not allowed to discard Queen of Spades in round 1.
    QueenInFirstRound,
}

#[derive( Clone, Copy, PartialEq )]
enum RoundStatus { Pending = 0, Complete = 1 }

/// The splitters for splitting scores and cards in different rounds.
pub const SPLITTER: [[&'static str; 13]; 2] = [
    [
        "  - - - - - - - - - - - -",
        "-   - - - - - - - - - - -",
        "- -   - - - - - - - - - -",
        "- - -   - - - - - - - - -",
        "- - - -   - - - - - - - -",
        "- - - - -   - - - - - - -",
        "- - - - - -   - - - - - -",
        "- - - - - - -   - - - - -",
        "- - - - - - - -   - - - -",
        "- - - - - - - - -   - - -",
        "- - - - - - - - - -   - -",
        "- - - - - - - - - - -   -",
        "- - - - - - - - - - - -  ",
    ],
    [
        ". - - - - - - - - - - - -",
        "- . - - - - - - - - - - -",
        "- - . - - - - - - - - - -",
        "- - - . - - - - - - - - -",
        "- - - - . - - - - - - - -",
        "- - - - - . - - - - - - -",
        "- - - - - - . - - - - - -",
        "- - - - - - - . - - - - -",
        "- - - - - - - - . - - - -",
        "- - - - - - - - - . - - -",
        "- - - - - - - - - - . - -",
        "- - - - - - - - - - - . -",
        "- - - - - - - - - - - - .",
    ],
];

/// The pointer type of functions that provide strategies for passing/discarding cards.
pub type Strategy = fn(&Game,usize)->Cards;

fn input_or_hint( game: &Game, _who: usize ) -> Cards {
    game.pick_cards( ME ) // for hint only
}

/// All players' strategies for passing/discarding cards.
///
/// Note: `me` is invoked by `game` only in automatic mode, e.g. `game.config.me.is_some()`.
#[derive( Debug )]
pub struct Strategies {
    of : [Strategy; 4],
}

impl Strategies {
    /// Decides to run scripts or using the built-in strategy, depending on `config`.
    pub fn from_config( config: &Config ) -> Self {
        let me       = if config.me      .is_some() { Game::run_script_to_pick_cards } else { input_or_hint  };
        let left     = if config.left    .is_some() { Game::run_script_to_pick_cards } else { Game::pick_cards };
        let opposite = if config.opposite.is_some() { Game::run_script_to_pick_cards } else { Game::pick_cards };
        let right    = if config.right   .is_some() { Game::run_script_to_pick_cards } else { Game::pick_cards };
        Strategies{ of: [ me, left, opposite, right ]}
    }
    /// Sets `strategy` for all players.
    pub fn set_for_all( &mut self, strategy: Strategy ) {
        for whom in PLAYERS {
            self.of[ whom ] = strategy;
        }
    }
    /// Sets `strategy` for other players (not for me).
    pub fn set_for_others( &mut self, strategy: Strategy ) {
        for whom in OTHERS {
            self.of[ whom ] = strategy;
        }
    }
}

/// The game engine of heartless.
///
/// # Example of writing customizable AI strategies
///
/// ```rust,no_run
/// use heartless::*;
/// fn your_great_ai( game: &Game, who: usize ) -> Cards { todo!() }
///
/// let mut game = Game::default();
/// game.strategies.set_for_others( your_great_ai );
/// game.main_loop();
/// ```
#[derive( Debug )]
pub struct Game {
        players         : [Player;4],
        winner          : usize,
        deal            : usize,
    pub discarded       : Cards,
    pub suit_mask       : Cards,
    pub round           : usize,
    pub under_the_gun   : usize,
    pub round_winner    : usize,
    pub high_card       : Cards,
        started         : bool,
        config          : Config,
    pub hand            : Cell<Cards>,
    pub strategies      : Strategies,
    pub altio           : Altio,
}

impl_altio_output!( Game );

impl Default for Game {
    fn default() -> Self {
        Game::with_env_args( std::env::args_os() )
    }
}

impl Game {
    /// Constructs with given environment arguments.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let mut game = heartless::Game::with_env_args( std::env::args_os() );
    /// ```
    pub fn with_env_args<I,T>( iter: I ) -> Self
        where I: IntoIterator<Item=T>
            , T: Into<OsString> + Clone
    {
        let config = Config::from_args::<I,T>( iter );
        let strategies = Strategies::from_config( &config );

        Game {
            players         : <[Player;4]>::default(),
            winner          : NOBODY,
            deal            : 0_usize,
            discarded       : NO_CARD,
            suit_mask       : NO_CARD,
            round           : 0_usize,
            under_the_gun   : NOBODY,
            round_winner    : NOBODY,
            high_card       : NO_CARD,
            started         : false,
            config          ,
            hand            : Cell::new( NO_CARD ),
            strategies      ,
            altio           : Altio::default(),
        }
    }
    /// Constructs with given configurations.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let config = heartless::cli::Config::from_args( std::env::args_os() );
    /// let mut game = heartless::Game::with_config( config );
    /// ```
    pub fn with_config( config: Config ) -> Self {
        let strategies = Strategies::from_config( &config );

        Game {
            players         : <[Player;4]>::default(),
            winner          : NOBODY,
            deal            : 0_usize,
            discarded       : NO_CARD,
            suit_mask       : NO_CARD,
            round           : 0_usize,
            under_the_gun   : NOBODY,
            round_winner    : NOBODY,
            high_card       : NO_CARD,
            started         : false,
            config          ,
            hand            : Cell::new( NO_CARD ),
            strategies      ,
            altio           : Altio::default(),
        }
    }
    /// Returns true if this player's hand may hold cards of the given suit,
    /// otherwise returns false.
    pub fn player_may_have( &self, who: usize, suit: u8 ) -> bool {
        self.players[who].may_have( suit )
    }
    fn reset( &mut self ) {
        self.players.iter_mut().for_each( |player| player.reset() );
        self.winner         = NOBODY;
        self.deal           = 0_usize;
        self.discarded      = NO_CARD;
        self.suit_mask      = NO_CARD;
        self.round          = 0_usize;
        self.under_the_gun  = NOBODY;
        self.round_winner   = NOBODY;
        self.high_card      = NO_CARD;
        self.started        = false;
    }
    fn start( &mut self ) {
        self.started = true;
    }
    fn input( &mut self ) -> Input {
        if self.config.automatic {
            if self.started {
                let cards = self.pick_cards( ME );
                if self.round == 0 {
                    Input::Pass( cards )
                } else {
                    Input::Discard( cards )
                }
            } else {
                Input::Start
            }
        } else {
            self.input_from_user()
        }
    }
    fn handle_card_parse_error( &self, input: &str, error: card::ParseError ) {
        match error {
            card::ParseError::Invalid  { invalid, offset:_  } => self.hint_valid_chars_for_cards( invalid, input ),
            card::ParseError::Ambiguous{ cards:_, ok:_, err } => self.hint_ambiguous_cards( err ),
            card::ParseError::NoCard                          => self.hint_no_card_parsed( input ),
        }
    }
    fn input_from_user( &self ) -> Input {
        let mut buffer = String::new();
        self.altio.input().read_line( &mut buffer ).unwrap();
        let buffer = buffer.trim();

        let mut input = Input::Invalid;
        let hand = self.players[ME].hand;
        match buffer {
            "" => input = Input::Start,
            "exit" => input = Input::Exit,
            s if self.round > 0 => {
                match Cards::parse_in_hand( &s, hand, self.suit_mask ) {
                    Ok( card ) => {
                        if card.count() == 1 {
                            input = Input::Discard( card );
                        } else {
                            self.hint_discard_one( card );
                        }
                    },
                    Err( err ) => self.handle_card_parse_error( s, err ),
                }
            },
            s if self.round == 0 => {
                match Cards::parse_in_hand( &s, hand, self.suit_mask ) {
                    Ok( cards ) => {
                        if cards.count() == 3 {
                            input = Input::Pass( cards );
                        } else {
                            self.hint_pass_three( cards );
                        }
                    },
                    Err( err  ) => self.handle_card_parse_error( s, err ),
                }
            },
            _ => (),
        }
        input
    }
    fn get_winner( &mut self ) -> usize {
        let mut game_over = false;
        let mut winner = NOBODY;
        let mut min_score = 100;
        for i in PLAYERS {
            let s = self.players[i].game_score;
            if s >= 100 {
                game_over = true;
            }
            if min_score > s {
                min_score = s;
                winner = i;
            }
        }
        if game_over {
            self.winner = winner;
            self.players[ winner ].awards += 1;
            winner
        } else {
            NOBODY
        }
    }
    fn deal( &mut self ) {
        self.discarded = NO_CARD;
        self.deal += 1;
        self.round = 0;
        let hands = Cards::deal();
        for i in PLAYERS {
            self.players[i].hand        = hands[i];
            self.players[i].hand_score  = 0;
            self.players[i].suit_bits   = BIT_CLUB | BIT_DIAMOND | BIT_SPADE | BIT_HEART;
        }
    }
    fn needs_pass_cards( &self ) -> bool {
        self.deal % 4 != 0
    }
    fn pass_cards( &mut self, my_three_cards: Cards ) -> bool {
        let mut transfers = <[Transfer; 4]>::default();
        if let Some( transfered ) = self.players[ME].hand.transfer( my_three_cards ) {
            transfers[ME] = transfered;
            for i in OTHERS {
                transfers[i] = self.pick_cards_for(i);
            }
            let offset = [0,3,1,2][ self.deal%4 ];
            for i in PLAYERS {
                let j = ( i + offset ) % 4;
                self.players[i].hand = transfers[i].src + transfers[j].dest;
                if i == ME {
                    self.hint_received_cards( transfers[j].dest );
                }
            }
            true
        } else {
            self.hint_pass_cards_in_hands( my_three_cards );
            false
        }

    }
    pub fn break_the_rule( &self, who: usize, card: Cards ) -> Option<Rule> {
        let hand = self.players[who].hand;

        if self.round == 0 { // passing
            if !hand.contains( card ) {
                Some( Rule::PassInHand )
            } else if card.count() != 3 {
                Some( Rule::PassThreeCards )
            } else {
                None
            }
        } else { // discarding
            if !hand.contains( card ) {
                return Some( Rule::DiscardInHand );
            }

            if hand.contains( TWO_OF_CLUBS ) && card != TWO_OF_CLUBS {
                return Some( Rule::TwoOfClubs );
            } else if {
                self.suit_mask != NO_CARD &&
                hand.contains_suit( self.suit_mask ) &&
                !card.contains_suit( self.suit_mask )
            } {
                return Some( Rule::FollowSuit );
            } else if card.is_heart() && !hand.are_all_of( HEARTS ) {
                if self.under_the_gun == who {
                    if !self.heart_broken() {
                        return Some( Rule::Heartbreak );
                    }
                } else if self.round == 1 {
                    return Some( Rule::HeartInFirstRound );
                }
            } else if card.is_queen_of_spade() {
                if self.round == 1 {
                    return Some( Rule::QueenInFirstRound );
                }
            }
            None
        }
    }
    fn heart_broken( &self ) -> bool {
        self.discarded.contains_suit( HEARTS )
    }
    fn pick_card( &self, who: usize ) -> Transfer {
        let hand = self.players[who].hand;

        if self.under_the_gun == who { // early hand
            if self.round == 1 {
                return self.transfer_card( who, TWO_OF_CLUBS ).unwrap();
            }

            let mut suits_cnts = [
                ( CLUB   , hand.count_suit( CLUBS )),
                ( DIAMOND, hand.count_suit( DIAMONDS )),
                ( SPADE  , if hand & SPADES >= QUEEN_OF_SPADES {14} else {1} ),
                ( HEART  , if self.heart_broken() { hand.count_suit(HEARTS) } else {15} ),
            ];
            suits_cnts.sort_by_key( |suit_cnt| suit_cnt.1 ); // prefer discarding short suits than long suits

            for (suit,_) in suits_cnts {
                for rank in RANKS {
                    let card = Cards::of( rank, suit );
                    if let Some( transfered ) = self.transfer_card( who, card ) {
                        return transfered;
                    }
                }
            }
        } else if hand & self.suit_mask != NO_CARD { // follow suit
            let suit = self.suit();
            let mut to_discard = Cards::of( ACE, suit );
            let mut high_card_to_discard = NO_CARD;
            for rank in RANKS.rev() {
                let card = Cards::of( rank, suit );
                if hand.contains( card ) {
                    if card < self.high_card {
                        if {
                            self.suit_mask == SPADES &&
                            hand.contains( QUEEN_OF_SPADES ) &&
                            QUEEN_OF_SPADES < self.high_card
                        } {
                            return self.transfer_card( who, QUEEN_OF_SPADES ).unwrap();
                        }
                        return self.transfer_card( who, card ).unwrap();
                    }
                    if to_discard >= card {
                        to_discard = card;
                        if high_card_to_discard < card && suit != HEART && card != QUEEN_OF_SPADES {
                            high_card_to_discard = card;
                        }
                    }
                }
            }
            let mut late_hand_follows = false;
            for i in 1..3 {
                let i = (who+i) %4;
                if i == self.under_the_gun {
                    break;
                }
                if self.players[i].may_have( suit ) {
                    late_hand_follows = true;
                }
            }
            return self.transfer_card(
                who,
                if late_hand_follows || high_card_to_discard == NO_CARD { to_discard } else { high_card_to_discard }
            ).unwrap();
        } else { // don't follow suit
            if hand.contains( QUEEN_OF_SPADES ) && self.round != 1 { // try to discard Queen of Spades
                return self.transfer_card( who, QUEEN_OF_SPADES ).unwrap();
            }
            if hand.contains_suit( HEARTS ) && self.round != 1 { // try to discard hearts
                for rank in RANKS.rev() {
                    if let Some( transfered ) = self.transfer_card( who, Cards::of( rank, HEART )) {
                        return transfered;
                    }
                }
            }
            for rank in RANKS.rev() { // try to discard high card
                for suit in [SPADE, DIAMOND, CLUB] {
                    let card = Cards::of( rank, suit );
                    if card == QUEEN_OF_SPADES && self.round == 1 {
                        continue;
                    }
                    if let Some( transfered ) = self.transfer_card( who, card ) {
                        return transfered;
                    }
                }
            }
            for rank in RANKS.rev() { // all hearts
                if let Some( transfered ) = self.transfer_card( who, Cards::of( rank, HEART )) {
                    return transfered;
                }
            }
        }
        unreachable!();
    }
    fn transfer_card( &self, who: usize, card: Cards ) -> Option<Transfer> {
        self.players[who].hand.transfer( card )
    }
    fn who_holds_two_of_clubs( &self ) -> usize {
        for i in PLAYERS {
            if self.players[i].hand.contains( TWO_OF_CLUBS ) {
                return i;
            }
        }
        NOBODY
    }
    fn discard( &mut self, who: usize, transfered: Transfer ) {
        self.players[who].hand = transfered.src;
        let discarding = transfered.dest;
        self.players[who].discarding = discarding;
        match self.suit_mask {
            CLUBS    if !discarding.contains_suit( CLUBS    ) => self.players[who].suit_bits &= !BIT_CLUB   ,
            DIAMONDS if !discarding.contains_suit( DIAMONDS ) => self.players[who].suit_bits &= !BIT_DIAMOND,
            SPADES   if !discarding.contains_suit( SPADES   ) => self.players[who].suit_bits &= !BIT_SPADE  ,
            HEARTS   if !discarding.contains_suit( HEARTS   ) => self.players[who].suit_bits &= !BIT_HEART  ,
            _ => (),
        }

        self.discarded += discarding;
        if self.suit_mask == NO_CARD {
            self.suit_mask = transfered.dest.suit_mask();
        }
        let card = discarding & self.suit_mask;
        if self.high_card < card {
            self.high_card = card;
            self.round_winner = who;
        }
    }
    fn keep_score( &mut self ) {
        let mut score = 0_u8;
        for i in PLAYERS {
            let discarding = self.players[i].discarding;
            if discarding.is_heart() {
                score += 1;
            } else if discarding.is_queen_of_spade() {
                score += 13;
            }
        }
        if score != 0 {
            self.players[ self.round_winner ].hand_score += score;
            if self.players[ self.round_winner ].hand_score == 26 { // hit the moon
                self.players[ self.round_winner ].hand_score = 0;
                for i in PLAYERS {
                    if i != self.round_winner {
                        self.players[i].hand_score = 26;
                    }
                }
            }
        }
        if self.round == 13 {
            for i in PLAYERS {
                self.players[i].game_score += self.players[i].hand_score;
            }
        }
    }
    fn next_round( &mut self ) -> bool {
        self.round += 1;
        for i in PLAYERS {
            self.players[i].discarding = NO_CARD;
        }
        if self.round == 1 {
            self.under_the_gun = self.who_holds_two_of_clubs();
            self.suit_mask = CLUBS;
        } else {
            self.under_the_gun = self.round_winner;
            self.suit_mask = NO_CARD;
        };
        self.round_winner = NOBODY;
        self.high_card = NO_CARD;
        self.round <= 13
    }
    fn suit( &self ) -> u8 {
        self.suit_mask.mask_to_suit()
    }
    fn text_of( &self, cards: Cards ) -> String {
        let mut mask = 0;
        if self.config.lowercase_rank { mask |= LOWERCASE_RANK }
        if self.config.ascii_suit     { mask |= ASCII_SUIT     }
        if self.config.no_shared_suit { mask |= NO_SHARED_SUIT }
        if self.config.no_spaces      { mask |= NO_SPACES      }
        cards.text( mask )
    }
    fn show_my_hand( &self ) {
        echo!( self.out(), "= {}", self.text_of( self.players[ME].hand ));
    }
    fn show_winner( &self ) {
        if self.winner == ME {
            echo!( self.out(), "The winner is me!" );
            echo!( self.err(),
r#"
 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
< :::::::::::::::::::::::::::::::::::::::::::::::::::::::::::: >
< ::::::::::::            ::::::::::::            :::::::::::: >
< :::::::::                   ::::                   ::::::::: >
< :::::::         ********     ::     ********         ::::::: >
< :::::      ****          ****  ****         ****       ::::: >
< ::::     ****               ****               ****     :::: >
< :::     ****                 **                 ****     ::: >
< :::     ****                                    ****     ::: >
< ::::     ****           YOU ARE THE            *****    :::: >
< :::::     ****                                ****     ::::: >
< :::::::     ****           WINNER!          ****     ::::::: >
< :::::::::     ****                        ****     ::::::::: >
< :::::::::::     ****                    ****     ::::::::::: >
< ::::::::::::::      ****            ****      :::::::::::::: >
< :::::::::::::::::       ****    ****       ::::::::::::::::: >
< ::::::::::::::::::::        ****        :::::::::::::::::::: >
< :::::::::::::::::::::::      **      ::::::::::::::::::::::: >
< :::::::::::::::::::::::::          ::::::::::::::::::::::::: >
< :::::::::::::::::::::::::::      ::::::::::::::::::::::::::: >
< :::::::::::::::::::::::::::::  ::::::::::::::::::::::::::::: >
< :::::::::::::::::::::::::::::::::::::::::::::::::::::::::::: >
 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++"#
            );
        } else {
            echo!( self.out(), "The winner is the {} player!",
                ["me", "left", "opposite", "right"][ self.winner ]);
        }
    }
    fn sleep_in_interactive_mode( &self ) {
        match self.config.impatient {
            Some( true ) => {}
            Some( false ) => thread::sleep( Duration::from_secs(1) ),
            None => if !self.config.automatic {
                thread::sleep( Duration::from_secs(1) );
            }
        }
    }
    fn show_scores_and_discarding( &self, round_status: RoundStatus, who: usize ) {
        if who != ME {
            self.sleep_in_interactive_mode();
        }

        let p = &self.players;

        let mut out = self.out();

        echo!( out, "{}", SPLITTER[ round_status as usize ][ self.round-1 ]);
        echo!( out, "       {:>7}",
            format!( "{}/{}", p[OPPOSITE].hand_score, p[OPPOSITE].game_score ));
        echo!( out, "{:>7}       {:>7}",
            format!( "{}/{}", p[LEFT] .hand_score, p[LEFT] .game_score ),
            format!( "{}/{}", p[RIGHT].hand_score, p[RIGHT].game_score )
        );
        echo!( out, "       {:>7}",
            format!( "{}/{}", p[ME].hand_score, p[ME].game_score ));
        echo!( out, "" );

        let mut gun = [' ', ' ', ' ', ' '];
        gun[ self.under_the_gun ] = ':';
        let [gm, gl, go, gr] = gun;

        let mut win = [' ', ' ', ' ', ' '];
        win[ self.round_winner ] = '.';
        let [wm, wl, wo, wr] = win;

        let co = p[ OPPOSITE ].discarding;
        let cl = p[ LEFT     ].discarding;
        let cr = p[ RIGHT    ].discarding;
        let cm = p[ ME       ].discarding;

        echo!( out, "          {go}{}{wo}"   , self.text_of(co) );
        echo!( -n,
               out, "   {gl}{}{wl}"          , self.text_of(cl) );
        if cl == NO_CARD {
            echo!( -n, out, "  " );
        }
        echo!( out,              "          {gr}{}{wr}"
                                             , self.text_of(cr) );
        echo!( out, "          {gm}{}{wm}"   , self.text_of(cm) );

        drop( out );

        match round_status {
            RoundStatus::Pending => if (who+1) % 4 != ME {
                self.show_my_hand();
                echo!( self.out(), "discarding..." );
            }
            RoundStatus::Complete => if self.round_winner != ME && self.round != 13 {
                self.show_my_hand();
                echo!( self.out(), "discarding..." );
                self.sleep_in_interactive_mode();
            }
        }
    }
    fn run_script_to_pick_cards( &self, who: usize ) -> Cards {
        let script_path = match who {
            ME       => if let Some( path ) = self.config.me      .as_ref() { path.clone() } else { return NO_CARD },
            LEFT     => if let Some( path ) = self.config.left    .as_ref() { path.clone() } else { return NO_CARD },
            OPPOSITE => if let Some( path ) = self.config.opposite.as_ref() { path.clone() } else { return NO_CARD },
            RIGHT    => if let Some( path ) = self.config.right   .as_ref() { path.clone() } else { return NO_CARD },
            _ => return NO_CARD,
        };
        let hand                = self.players[who].hand;
        let discarded           = self.discarded;
        let suit_to_follow      = ["club", "diamond", "spade", "heart", ""][ (self.suit()>>4) as usize ];
        let round               = self.round;
        let under_the_gun       = self.under_the_gun;
        let round_winner        = self.round_winner;
        let high_card           = self.high_card;

        let my_discarding       = self.players[ME].discarding;
        let my_hand_score       = self.players[ME].hand_score;
        let my_game_score       = self.players[ME].game_score;
        let my_suit_chars       = self.players[ME].suit_chars();

        let left_discarding     = self.players[LEFT].discarding;
        let left_hand_score     = self.players[LEFT].hand_score;
        let left_game_score     = self.players[LEFT].game_score;
        let left_suit_chars     = self.players[LEFT].suit_chars();

        let opposite_discarding = self.players[OPPOSITE].discarding;
        let opposite_hand_score = self.players[OPPOSITE].hand_score;
        let opposite_game_score = self.players[OPPOSITE].game_score;
        let opposite_suit_chars = self.players[OPPOSITE].suit_chars();

        let right_discarding    = self.players[RIGHT].discarding;
        let right_hand_score    = self.players[RIGHT].hand_score;
        let right_game_score    = self.players[RIGHT].game_score;
        let right_suit_chars    = self.players[RIGHT].suit_chars();

        let mut cmd = Command::new( script_path );
        cmd.args([
                &format!( "--who={who}" ),
                &format!( "--hand={hand:-?}" ),
                &format!( "--discarded={discarded:-?}" ),
                &format!( "--suit_to_follow={suit_to_follow}" ),
                &format!( "--round={round}" ),
                &format!( "--under_the_gun={under_the_gun}" ),
                &format!( "--round_winner={round_winner}" ),
                &format!( "--high_card={high_card}" ),
                &format!( "--my_discarding={my_discarding}" ),
                &format!( "--my_hand_score={my_hand_score}" ),
                &format!( "--my_game_score={my_game_score}" ),
                &format!( "--my_suit_chars={my_suit_chars}" ),
                &format!( "--left_discarding={left_discarding}" ),
                &format!( "--left_hand_score={left_hand_score}" ),
                &format!( "--left_game_score={left_game_score}" ),
                &format!( "--left_suit_chars={left_suit_chars}" ),
                &format!( "--opposite_discarding={opposite_discarding}" ),
                &format!( "--opposite_hand_score={opposite_hand_score}" ),
                &format!( "--opposite_game_score={opposite_game_score}" ),
                &format!( "--opposite_suit_chars={opposite_suit_chars}" ),
                &format!( "--right_discarding={right_discarding}" ),
                &format!( "--right_hand_score={right_hand_score}" ),
                &format!( "--right_game_score={right_game_score}" ),
                &format!( "--right_suit_chars={right_suit_chars}" ),
            ]);
        let output = match self.config.timeout {
            Some( timeout ) => {
                let mut spawned = match cmd.stdout( Stdio::piped() ).stderr( Stdio::piped() ).spawn() {
                    Ok( spawned ) => spawned,
                    Err( e ) => {
                        echo!( self.err(), "{e:#?}" );
                        return NO_CARD;
                    },
                };
                let timeout = Duration::from_millis( timeout );
                match spawned.wait_timeout( timeout ) {
                    Ok(Some(_)) => {
                        spawned.stderr.and_then( |mut stderr| {
                            let mut buf = String::new();
                            stderr.read_to_string( &mut buf ).ok();
                            Some( echo!( self.err(), "{buf}" ))
                        });
                        spawned.stdout.map( |mut stdout| {
                            let mut buf = String::new();
                            stdout.read_to_string( &mut buf ).ok();
                            buf
                        }).unwrap_or_default()
                    },
                    Ok(None) => {
                        echo!( self.err(), "Running out of time: `{cmd:?}` after {timeout:?} milliseconds." );
                        spawned.kill().unwrap();
                        return NO_CARD;
                    },
                    Err( e ) => {
                        echo!( self.err(), "{}", format!( "{e:#?}" ).replace( "\n", "\r" ));
                        return NO_CARD;
                    },
                }
            }
            None => match cmd.output() {
                Ok( output ) => {
                    if !output.stderr.is_empty() {
                        echo!( self.err(), "{:?}", output.stderr );
                    }
                    String::from_utf8_lossy( &output.stdout ).to_string()
                },
                Err( e ) => {
                    echo!( self.err(), "{e:#?}" );
                    return NO_CARD;
                },
            }
        };
        let output = output.trim();
        match Cards::parse_in_hand( output, hand, self.suit_mask ) {
            Ok( cards ) => match self.break_the_rule( who, cards ) {
                Some( rule ) => self.hint_break_the_rule( cards, rule ),
                None => return cards,
            },
            Err( err ) => self.handle_card_parse_error( output, err ),
        }
        NO_CARD
    }
}

impl Game {
    fn hint_rules( &self ) {
        echo!( self.err(), r#"Welcome to play heartless!

To be the player with the lowest score at the end of the game. When one player
hits 100 score or higher, the game ends; and the player with the lowest score
wins.

At the end of each hand, players count the number of hearts they have taken as
well as the queen of spades, if applicable. Hearts count as one point each and
the queen counts 13 points.

When a player takes all 13 hearts and the queen of spades in one hand, instead
of losing 26 points, that player scores zero and each of his opponents score an
additional 26 points.

The player holding the 2 of clubs after the pass makes the opening lead. Each
player must follow suit if possible. If a player is void of the suit led, a card
of any other suit may be discarded. However, if a player has no clubs when the
first trick is led, a heart or the queen of spades cannot be discarded. The
highest card of the suit led wins a trick and the winner of that trick leads
next. Hearts may not be led until a heart has been discarded.
"#
        );
    }
    fn hint_break_the_rule( &self, cards: Cards, rule: Rule ) {
        match rule {
            Rule::PassInHand        => self.hint_pass_cards_in_hands( cards ),
            Rule::PassThreeCards    => self.hint_pass_three_cards( cards ),
            Rule::DiscardInHand     => self.hint_discard_card_in_hand( cards ),
            Rule::TwoOfClubs        => self.hint_two_of_clubs(),
            Rule::FollowSuit        => self.hint_follow_suit(),
            Rule::Heartbreak        => self.hint_no_heartbreak(),
            Rule::HeartInFirstRound => self.hint_heart_in_first_round(),
            Rule::QueenInFirstRound => self.hint_queen_in_first_round(),
        }
    }
    fn hint_deal( &self ) {
        echo!( self.out(), "Press enter to start..." );
    }
    fn hint_discard_card_in_hand( &self, card: Cards ) {
        echo!( self.err(), "Discarding card is not in hand: {}", self.text_of(card) );
    }
    fn hint_pass_cards_in_hands( &self, cards: Cards ) {
        echo!( self.err(), "Passing cards are not all in hand: {}", self.text_of(cards) );
    }
    fn hint_pass_three_cards( &self, cards: Cards ) {
        echo!( self.err(), "These are not three cards: {}", self.text_of(cards) );
    }
    fn hint_two_of_clubs( &self ) {
        echo!( self.err(), "Two of clubs should be dropped first!" );
    }
    fn hint_follow_suit( &self ) {
        echo!( self.err(), "You must follow the suit!" );
    }
    fn hint_no_heartbreak( &self ) {
        echo!( self.err(), "Heart not allowed to be early hand if no hearts has been discarded before." );
    }
    fn hint_heart_in_first_round( &self ) {
        echo!( self.err(), "Heart not allowed in first round." );
    }
    fn hint_queen_in_first_round( &self ) {
        echo!( self.err(), "Queen of Spades not allowed in first round." );
    }
    fn hint_ambiguous_cards( &self, cards: Cards ) {
        echo!( self.err(), "Which card to discard? {}", self.text_of(cards) );
    }
    fn hint_discard_one( &self, cards: Cards ) {
        echo!( self.err(), "Not allowed to discard multiple cards in one round: {}", self.text_of(cards) );
    }
    fn hint_pass_three( &self, cards: Cards ) {
        echo!( self.err(), "These are not 3 cards to pass: {}", self.text_of(cards) );
    }
    fn hint_valid_chars_for_cards( &self, invalid: char, input: &str ) {
        echo!( self.err(), r#"This string \"{input}\" contains an invalid char '{invalid}'.
Please use 2,3,4,5,6,7,8,9,0,J,Q,K,A for ranks and c,d,s,h for suits.
Ranks followed by a suit representing cards with these ranks and in the suit.
For example, Qs means Queen of Spades.
Another example: 0Jh means Ten of Hearts and Jack of Hearts.\n"# );
    }
    fn hint_no_card_parsed( &self, input: &str ) {
        echo!( self.err(), "This string does not represent any card: \"{input}\"" );
    }
    fn hint_pass( &self ) {
        let receiver = ["nobody", "left", "right", "opposite"][ self.deal%4 ];
        let three_cards = self.pick_cards_for(ME).dest;
        echo!( self.out(), "Pass 3 cards to {receiver}, e.g. {three_cards}" );
    }
    fn hint_discard( &self ) {
        let card = self.pick_cards_for(ME).dest;
        echo!( self.out(), "Discard 1 card, e.g. {card}" );
    }
    fn hint_no_pass_discard( &self ) {
        let card = self.pick_cards_for(ME).dest;
        echo!( self.out(), "Do not need to pass cards. Discard 1 card, e.g. {card}" );
    }
    fn hint_received_cards( &self, received: Cards ) {
        echo!( self.out(), "Received {}", self.text_of( received ));
        if self.who_holds_two_of_clubs() != ME {
            self.show_my_hand();
        }
    }
}

impl Game {
    fn pick_cards( &self, who: usize ) -> Cards {
        if self.round == 0 {
            self.players[who].hand.pick_three_cards()
        } else {
            self.pick_card( who )
        }.dest
    }
    fn pick_cards_for( &self, who: usize ) -> Transfer {
        self.hand.set( self.players[who].hand );
        let mut cards = self.strategies.of[who]( self, who );

        #[cfg( feature = "test-replica" )]
        {
            let expected = self.pick_cards(who);
            if cards != expected {
                panic!( "expect {expected}, got {cards}" );
            }
        }

        if let Some( rule ) = self.break_the_rule( who, cards ) {
            self.hint_break_the_rule( cards, rule );
            cards = self.pick_cards( who );
        }
        Transfer{ src: self.hand.get()-cards, dest: cards }
    }
    /// Enters the game main loop.
    pub fn main_loop( &mut self ) {
        self.config.seed.map( |seed| fastrand::seed( seed ));
        self.hint_rules();

        'game: while self.config.count.map( |n| n > 0 ).unwrap_or( true ) {
            self.hint_deal();
            match self.input() {
                Input::Start => {
                    self.start();
                    while self.get_winner() == NOBODY {
                        self.deal();
                        let mut hint_no_pass = false;
                        let mut hint_discarding = false;
                        if self.needs_pass_cards() {
                            self.show_my_hand();
                            'pass: loop {
                                self.hint_pass();
                                match self.input() {
                                    Input::Pass( cards ) => if self.pass_cards( cards ) {
                                        break 'pass;
                                    },
                                    Input::Exit => break 'game,
                                    _ => (),
                                }
                            }
                            if self.who_holds_two_of_clubs() != ME {
                                hint_discarding = true;
                            }
                        } else if self.who_holds_two_of_clubs() == ME {
                            hint_no_pass = true;
                        } else {
                            self.show_my_hand();
                            echo!( self.out(), "Do not need to pass cards." );
                        }
                        while self.next_round() {
                            let start = self.under_the_gun;
                            for i in start..start+4 {
                                let who = i % 4;
                                if who == ME {
                                    'discard: loop {
                                        self.show_my_hand();
                                        if hint_no_pass {
                                            hint_no_pass = false;
                                            self.hint_no_pass_discard();
                                        } else {
                                            self.hint_discard();
                                        }
                                        match self.input() {
                                            Input::Discard( card ) => match self.break_the_rule( ME, card ) {
                                                Some( rule ) => self.hint_break_the_rule( card, rule ),
                                                None => {
                                                    let transfered = self.players[ME].hand.transfer( card ).unwrap();
                                                    self.discard( ME, transfered );
                                                    if i != start+3 {
                                                        self.show_scores_and_discarding( RoundStatus::Pending, ME );
                                                    }
                                                    break 'discard;
                                                },
                                            },
                                            Input::Exit => break 'game,
                                            _ => (),
                                        }
                                    }
                                } else {
                                    if hint_discarding {
                                        hint_discarding = false;
                                        echo!( self.out(), "discarding..." );
                                    }
                                    let cards = self.pick_cards_for(who);
                                    self.discard( who, cards );
                                    if i != start+3 {
                                        self.show_scores_and_discarding( RoundStatus::Pending, who );
                                    }
                                }
                            }
                            self.keep_score();
                            self.show_scores_and_discarding( RoundStatus::Complete, (start+3)%4 );
                        }
                    }
                    self.show_winner();
                    self.reset();
                    self.config.count.as_mut().map( |n| *n -= 1 );
                    continue 'game;
                },
                Input::Exit => break 'game,
                _ => self.hint_deal(),
            }
        }
        let p = &self.players;
        echo!( self.out(), "Statistics: YOU won {}, LEFT won {}, OPPOSITE won {}, RIGHT won {}.",
            p[ME].awards, p[LEFT].awards, p[OPPOSITE].awards, p[RIGHT].awards );
    }
}
