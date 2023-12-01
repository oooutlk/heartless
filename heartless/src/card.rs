//! Cards definition of storing, comparing, passing, discarding, printing.

use std::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Add, AddAssign, BitAnd, Not, Range, Sub},
    str::FromStr,
};

pub const BIT_CLUB      : u8 = 0b0001;
pub const BIT_DIAMOND   : u8 = 0b0010;
pub const BIT_SPADE     : u8 = 0b0100;
pub const BIT_HEART     : u8 = 0b1000;

pub const CLUB          : u8 = 0x00;
pub const DIAMOND       : u8 = 0x10;
pub const SPADE         : u8 = 0x20;
pub const HEART         : u8 = 0x30;
pub const NO_SUIT       : u8 = 0x40;

pub const NO_RANK       : u8 = 0x0;
pub const TWO           : u8 = 0x2;
pub const THREE         : u8 = 0x3;
pub const FOUR          : u8 = 0x4;
pub const FIVE          : u8 = 0x5;
pub const SIX           : u8 = 0x6;
pub const SEVEN         : u8 = 0x7;
pub const EIGHT         : u8 = 0x8;
pub const NINE          : u8 = 0x9;
pub const TEN           : u8 = 0xa;
pub const JACK          : u8 = 0xb;
pub const QUEEN         : u8 = 0xc;
pub const KING          : u8 = 0xd;
pub const ACE           : u8 = 0xe;

pub const RANKS         : Range<u8> = TWO..(ACE+1);

pub const   TWO_OF_CLUBS    : Cards = Cards::of(   TWO, CLUB );
pub const THREE_OF_CLUBS    : Cards = Cards::of( THREE, CLUB );
pub const  FOUR_OF_CLUBS    : Cards = Cards::of(  FOUR, CLUB );
pub const  FIVE_OF_CLUBS    : Cards = Cards::of(  FIVE, CLUB );
pub const   SIX_OF_CLUBS    : Cards = Cards::of(   SIX, CLUB );
pub const SEVEN_OF_CLUBS    : Cards = Cards::of( SEVEN, CLUB );
pub const EIGHT_OF_CLUBS    : Cards = Cards::of( EIGHT, CLUB );
pub const  NINE_OF_CLUBS    : Cards = Cards::of(  NINE, CLUB );
pub const   TEN_OF_CLUBS    : Cards = Cards::of(   TEN, CLUB );
pub const  JACK_OF_CLUBS    : Cards = Cards::of(  JACK, CLUB );
pub const QUEEN_OF_CLUBS    : Cards = Cards::of( QUEEN, CLUB );
pub const  KING_OF_CLUBS    : Cards = Cards::of(  KING, CLUB );
pub const   ACE_OF_CLUBS    : Cards = Cards::of(   ACE, CLUB );

pub const   TWO_OF_DIAMONDS : Cards = Cards::of(   TWO, DIAMOND );
pub const THREE_OF_DIAMONDS : Cards = Cards::of( THREE, DIAMOND );
pub const  FOUR_OF_DIAMONDS : Cards = Cards::of(  FOUR, DIAMOND );
pub const  FIVE_OF_DIAMONDS : Cards = Cards::of(  FIVE, DIAMOND );
pub const   SIX_OF_DIAMONDS : Cards = Cards::of(   SIX, DIAMOND );
pub const SEVEN_OF_DIAMONDS : Cards = Cards::of( SEVEN, DIAMOND );
pub const EIGHT_OF_DIAMONDS : Cards = Cards::of( EIGHT, DIAMOND );
pub const  NINE_OF_DIAMONDS : Cards = Cards::of(  NINE, DIAMOND );
pub const   TEN_OF_DIAMONDS : Cards = Cards::of(   TEN, DIAMOND );
pub const  JACK_OF_DIAMONDS : Cards = Cards::of(  JACK, DIAMOND );
pub const QUEEN_OF_DIAMONDS : Cards = Cards::of( QUEEN, DIAMOND );
pub const  KING_OF_DIAMONDS : Cards = Cards::of(  KING, DIAMOND );
pub const   ACE_OF_DIAMONDS : Cards = Cards::of(   ACE, DIAMOND );

pub const   TWO_OF_SPADES   : Cards = Cards::of(   TWO, SPADE );
pub const THREE_OF_SPADES   : Cards = Cards::of( THREE, SPADE );
pub const  FOUR_OF_SPADES   : Cards = Cards::of(  FOUR, SPADE );
pub const  FIVE_OF_SPADES   : Cards = Cards::of(  FIVE, SPADE );
pub const   SIX_OF_SPADES   : Cards = Cards::of(   SIX, SPADE );
pub const SEVEN_OF_SPADES   : Cards = Cards::of( SEVEN, SPADE );
pub const EIGHT_OF_SPADES   : Cards = Cards::of( EIGHT, SPADE );
pub const  NINE_OF_SPADES   : Cards = Cards::of(  NINE, SPADE );
pub const   TEN_OF_SPADES   : Cards = Cards::of(   TEN, SPADE );
pub const  JACK_OF_SPADES   : Cards = Cards::of(  JACK, SPADE );
pub const QUEEN_OF_SPADES   : Cards = Cards::of( QUEEN, SPADE );
pub const  KING_OF_SPADES   : Cards = Cards::of(  KING, SPADE );
pub const   ACE_OF_SPADES   : Cards = Cards::of(   ACE, SPADE );

pub const   TWO_OF_HEARTS   : Cards = Cards::of(   TWO, HEART );
pub const THREE_OF_HEARTS   : Cards = Cards::of( THREE, HEART );
pub const  FOUR_OF_HEARTS   : Cards = Cards::of(  FOUR, HEART );
pub const  FIVE_OF_HEARTS   : Cards = Cards::of(  FIVE, HEART );
pub const   SIX_OF_HEARTS   : Cards = Cards::of(   SIX, HEART );
pub const SEVEN_OF_HEARTS   : Cards = Cards::of( SEVEN, HEART );
pub const EIGHT_OF_HEARTS   : Cards = Cards::of( EIGHT, HEART );
pub const  NINE_OF_HEARTS   : Cards = Cards::of(  NINE, HEART );
pub const   TEN_OF_HEARTS   : Cards = Cards::of(   TEN, HEART );
pub const  JACK_OF_HEARTS   : Cards = Cards::of(  JACK, HEART );
pub const QUEEN_OF_HEARTS   : Cards = Cards::of( QUEEN, HEART );
pub const  KING_OF_HEARTS   : Cards = Cards::of(  KING, HEART );
pub const   ACE_OF_HEARTS   : Cards = Cards::of(   ACE, HEART );

pub const NO_CARD: Cards = Cards(0);

const fn rank_mask() -> u64 {
    1 << TWO   | 1 << THREE | 1 << FOUR  | 1 << FIVE  |
    1 << SIX   | 1 << SEVEN | 1 << EIGHT | 1 << NINE  |
    1 << TEN   | 1 << JACK  | 1 << QUEEN | 1 << KING  |
    1 << ACE
}

pub const HEARTS    : Cards = Cards( rank_mask() << HEART     );
pub const SPADES    : Cards = Cards( rank_mask() << SPADE     );
pub const DIAMONDS  : Cards = Cards( rank_mask() << DIAMOND   );
pub const CLUBS     : Cards = Cards( rank_mask() << CLUB      );

pub const SUIT_CHARS: [[char; 4]; 2] = [
    ['♣','♦','♠','♥'],
    ['c','d','s','h'],
];

pub const RANK_CHARS: [[char;13]; 2] = [
    ['2','3','4','5','6','7','8','9','0','J','Q','K','A'],
    ['2','3','4','5','6','7','8','9','0','j','q','k','a'],
];

/// A deck of cards composed of 4 suits of cards: clubs, diamonds, spades, hearts,
/// each of which has 13 ranks 2,3,4,5,6,7,8,9,10,J,Q,K,A.
///
/// # The order
///
/// The order of suits and the order of ranks are listed above.
///
/// Cards are sorted by comparing suits then comparing ranks.
/// For instance, 2 of clubs is the first card and Ace of hearts is the last one.
///
/// The order of suits is not used to decide the highest card in a round,
/// but used in displaying cards in hand.
///
/// # Binary representation
///
/// Since 4 * 13 == 52, a `u64` ought to be enough for every card to store as 1 bit.
///
/// 4 suits occupy 2 bits, 13 ranks occupy 4 bits, a `u8` can represent suits and ranks.
///
/// A single card has no representation on its own, and considered as `Cards`, the `count()` of which is 1.
///
/// # String representation
///
/// Cards are printed in the form of rank char + suit char.
/// For example, `2♣` stands for 2 of clubs, and `QKA♠" stands for Queen, King, Ace of spades.
///
/// There are 4 flags to control the string representation:
///
/// 1. ASCII_SUIT
///
/// If true, clubs, diamonds, spades, hearts will be printed as 'c','d','s','h'.
/// Otherwise ,they are printed as '♣', '♦', '♠', '♥', which is the default.
///
/// The `std::fmt::Formatter` flag is '#'.
///
/// 2. LOWERCASE_RANK
///
/// By default, Jack, Queen, King, Ace are printed as 'J',"Q","K","A".
///
/// Set this flag to be true, to print them as 'j','q','k','a'.
///
/// The `std::fmt::Formatter` flag is '<'.
///
/// 3. NO_SHARED_SUIT
///
/// By default, cards in `Cards`'s string representation are grouped in suits, concatenating their rank chars together,
/// with a trailing suit char, to increase human readability.
///
/// Set this flag to be true, to give each card its own trailing suit char.
///
/// The `std::fmt::Formatter` flag is '?'. Yes the `Debug` format.
///
/// 4. NO_SPACES
///
/// In `Cards`'s string representation, whitespaces are inserted in between cards by default, to increase human redability.
///
/// Users can supress these whitespaces by set this flag to be true.
///
/// The `std::fmt::Formatter` flag is '-'.
#[derive( Copy, Clone, Default, PartialEq, Eq, PartialOrd )]
pub struct Cards( pub u64 );

#[derive( Copy, Clone, Debug )]
pub enum ParseError {
    Invalid{ invalid: char, offset: usize },
    Ambiguous{ cards: Cards, ok: Cards, err: Cards }, // cards = ok + err
    NoCard,
}

#[derive( Clone, Copy )]
pub struct CardIter {
    card    : Cards,
    bit_pos : usize,
}

impl Iterator for CardIter {
    type Item = Cards;
    fn next( &mut self ) -> Option<Cards> {
        while self.bit_pos < 64 {
            let cnt = self.bit_pos;
            self.bit_pos += 1;
            if (self.card.0 >> cnt) & 1 == 1 {
                return Some( Cards( 1 << cnt ));
            }
        }
        None
    }
}

impl DoubleEndedIterator for CardIter {
    fn next_back( &mut self ) -> Option<Cards> {
        while self.bit_pos < 64 {
            let cnt = self.bit_pos;
            self.bit_pos += 1;
            if (self.card.0 << cnt) & 0x8000000000000000 == 0x8000000000000000 {
                return Some( Cards( 1 << (63-cnt) ));
            }
        }
        None
    }
}

impl Cards {
    #[inline]
    pub const fn of( rank: u8, suit: u8 ) -> Self {
        Cards( 1 << ( rank + suit ))
    }
    #[inline]
    pub fn contains( self, cards: Cards ) -> bool {
        cards != NO_CARD && self & cards == cards
    }
    #[inline]
    pub fn count( self ) -> u8 {
        self.0.count_ones() as u8
    }
    #[inline]
    pub fn count_suit( self, suit_mask: Cards ) -> u8 {
        (self & suit_mask).count()
    }
    /// Parses cards from the string represention.
    /// These cards must be held in `hand`, following the `suits` whenever possible.
    ///
    /// ```rust
    /// use heartless::*;
    /// let hand = "467QAc 480d 9Ks 6QKh".parse::<Cards>().unwrap();
    /// assert_eq!( Cards::parse_in_hand( "4ch", hand, NO_CARD ).unwrap(), FOUR_OF_CLUBS );
    /// assert_eq!( Cards::parse_in_hand( "Q", hand, CLUBS     ).unwrap(), QUEEN_OF_CLUBS );
    /// match Cards::parse_in_hand( "0", hand, CLUBS ) {
    ///     Err( ParseError::NoCard ) => (),
    ///     result => panic!( "{result:?} should be `ParseError::NoCard`" ),
    /// }
    /// ```
    pub fn parse_in_hand( input: &str, hand: Cards, suits: Cards ) -> Result<Cards, ParseError> {
        let mut cards = NO_CARD;
        let mut chars = input.chars().rev().enumerate();
        let mut suit = NO_SUIT;
        let mut rank = NO_RANK;

        let suits = {
            let suit = suits.mask_to_suit();
            if hand.contains_suit( suits ) {
                suit
            } else {
                NO_SUIT
            }
        };

        let len = input.len();
        let mut ambiguous_cards = NO_CARD;

        while let Some(( rev_nth, ch )) = chars.next() {
            match ch {
                'c'|'C'|'♣' => suit = CLUB,
                'd'|'D'|'♦' => suit = DIAMOND,
                's'|'S'|'♠' => suit = SPADE,
                'h'|'H'|'♥' => suit = HEART,
                '2' => rank = 0x2, '3' => rank = 0x3, '4' => rank = 0x4, '5' => rank = 0x5, '6' => rank = 0x6,
                '7' => rank = 0x7, '8' => rank = 0x8, '9' => rank = 0x9, '0' => rank = 0xa,
                'j'|'J' => rank = 0xb, 'q'|'Q' => rank = 0xc, 'k'|'K' => rank = 0xd, 'a'|'A' => rank = 0xe,
                ' ' => continue,
                invalid => return Err( ParseError::Invalid{ invalid, offset: len-rev_nth-1 }),
            }
            if rank != NO_RANK {
                let suit = if suit == NO_SUIT {suits} else {suit};
                if suit == NO_SUIT {
                    let mut guess = NO_CARD;
                    let mut ambiguous = false;
                    for suit in [CLUB, DIAMOND, SPADE, HEART] {
                        let card = Cards::of( rank, suit );
                        if hand == NO_CARD || hand.contains( card ) {
                            if guess == NO_CARD {
                                guess = card;
                            } else {
                                ambiguous = true;
                                guess += card;
                            }
                        }
                    }
                    if ambiguous {
                        ambiguous_cards += guess;
                    } else {
                        cards += guess;
                    }
                } else {
                    let card = Cards::of( rank, suit );
                    if hand == NO_CARD || hand.contains( card ) {
                        cards += card;
                    }
                }
                rank = NO_RANK;
            }
        }
        if ambiguous_cards != NO_CARD {
            Err( ParseError::Ambiguous{ cards: cards+ambiguous_cards, ok: cards, err: ambiguous_cards })
        } else if cards == NO_CARD {
            Err( ParseError::NoCard )
        } else {
            Ok( cards )
        }
    }
    /// Shuffles a deck and deals cards to 4 players.
    pub fn deal() -> [Self; 4] {
        let mut deck = CARDS;
        fastrand::shuffle( &mut deck );
        [
            deck[ 0..13].iter().fold( NO_CARD, |cards, &card| cards + card ),
            deck[13..26].iter().fold( NO_CARD, |cards, &card| cards + card ),
            deck[26..39].iter().fold( NO_CARD, |cards, &card| cards + card ),
            deck[39..52].iter().fold( NO_CARD, |cards, &card| cards + card ),
        ]
    }
    #[inline]
    /// Computates the result of passing cards / discarding card.
    pub fn transfer( self, cards: Cards ) -> Option<Transfer> {
        (Transfer{ src: self, dest: NO_CARD }).transfer( cards )
    }
    #[inline]
    /// Checks if this card's suit is hearts.
    pub fn is_heart( self ) -> bool {
        self & HEARTS != NO_CARD
    }
    #[inline]
    /// Checks if this card is the Queen of Spades.
    pub fn is_queen_of_spade( self ) -> bool {
        self == QUEEN_OF_SPADES
    }
    /// Checks if these cards contains the given suit.
    #[inline]
    pub fn contains_suit( self, suit_mask: Cards ) -> bool {
        self & suit_mask != NO_CARD
    }
    /// Checks if these cards are all of the given suit.
    #[inline]
    pub fn are_all_of( self, suit_mask: Cards ) -> bool {
        self & (!suit_mask) == NO_CARD
    }
    /// Computes the result of picks three cards for passing.
    ///
    /// Currently the stategy of this is naive, regardless of hitting the moon.
    pub fn pick_three_cards( self ) -> Transfer {
        let mut src  = self;
        let mut dest = NO_CARD;
        let mut passed = 0;

        if let Some( transfered ) = self.transfer( QUEEN_OF_SPADES ) {
            src  = transfered.src;
            dest = transfered.dest;
            passed += 1;
        }

        for rank in RANKS.rev() {
            for suit in [HEART, SPADE, DIAMOND, CLUB] {
                if let Some( transfered ) = (Transfer{ src, dest }).transfer( Cards::of( rank, suit )) {
                    src  = transfered.src;
                    dest = transfered.dest;
                    passed += 1;
                    if passed == 3 {
                        return Transfer{ src, dest };
                    }
                }
            }
        }

        Transfer{ src, dest } //unreachable!()
    }
    /// Computes this card's suit.
    pub fn suit_mask( self ) -> Cards {
        if self < TWO_OF_DIAMONDS {
            CLUBS
        } else if self < TWO_OF_SPADES {
            DIAMONDS
        } else if self < TWO_OF_HEARTS {
            SPADES
        } else {
            HEARTS
        }
    }
    /// Converts the suit representation from u64 to u8.
    ///
    /// These cards are not really set of cards but stands for some suit.
    pub fn mask_to_suit( self ) -> u8 {
        match self {
            CLUBS    => CLUB   ,
            DIAMONDS => DIAMOND,
            SPADES   => SPADE  ,
            HEARTS   => HEART  ,
            _        => NO_SUIT,
        }
    }
    /// Computes this card's suit, represented in u8.
    pub fn suit( self ) -> u8 {
        if self <= ACE_OF_CLUBS {
            CLUB
        } else if self <= ACE_OF_DIAMONDS {
            DIAMOND
        } else if self <= ACE_OF_SPADES {
            SPADE
        } else {
            HEART
        }
    }
    /// Computes this card's suit and rank, represented in u8.
    pub fn suit_and_rank( self ) -> (u8,u8) {
        fn rank( bits: u64 ) -> u8 {
            let mut cnt = 0;
            while (bits >> cnt) & 1 == 0 {
                cnt += 1;
            }
            cnt as u8
        }
        if self <= ACE_OF_CLUBS {
            (CLUB   , rank( self.0 >> CLUB ))
        } else if self <= ACE_OF_DIAMONDS {
            (DIAMOND, rank( self.0 >> DIAMOND ))
        } else if self <= ACE_OF_SPADES {
            (SPADE  , rank( self.0 >> SPADE ))
        } else {
            (HEART  , rank( self.0 >> HEART ))
        }
    }
    /// Returns an iterator that iterating all the cards in this hand.
    pub fn iter( self ) -> CardIter {
        CardIter{ card: self, bit_pos: 0 }
    }
    /// Returns an iterator that iterating all the cards in this hand, with the given suit.
    pub fn iter_of_suit( self, suit_mask: Cards ) -> CardIter {
        let bit_pos = suit_mask.suit() as usize;
        let card = Cards( (self & suit_mask).0 >> bit_pos );
        CardIter{ card, bit_pos }
    }

    /// Computes the card's order in the deck.
    ///
    /// # Example
    ///
    /// ```
    /// use heartless::*;
    /// assert_eq!(   TWO_OF_CLUBS   .deck_index(),  0 );
    /// assert_eq!( THREE_OF_CLUBS   .deck_index(),  1 );
    /// assert_eq!(  FOUR_OF_CLUBS   .deck_index(),  2 );
    /// assert_eq!(  FIVE_OF_CLUBS   .deck_index(),  3 );
    /// assert_eq!(   SIX_OF_CLUBS   .deck_index(),  4 );
    /// assert_eq!( SEVEN_OF_CLUBS   .deck_index(),  5 );
    /// assert_eq!( EIGHT_OF_CLUBS   .deck_index(),  6 );
    /// assert_eq!(  NINE_OF_CLUBS   .deck_index(),  7 );
    /// assert_eq!(   TEN_OF_CLUBS   .deck_index(),  8 );
    /// assert_eq!(  JACK_OF_CLUBS   .deck_index(),  9 );
    /// assert_eq!( QUEEN_OF_CLUBS   .deck_index(), 10 );
    /// assert_eq!(  KING_OF_CLUBS   .deck_index(), 11 );
    /// assert_eq!(   ACE_OF_CLUBS   .deck_index(), 12 );
    /// assert_eq!(   TWO_OF_DIAMONDS.deck_index(), 13 );
    /// assert_eq!( THREE_OF_DIAMONDS.deck_index(), 14 );
    /// assert_eq!(  FOUR_OF_DIAMONDS.deck_index(), 15 );
    /// assert_eq!(  FIVE_OF_DIAMONDS.deck_index(), 16 );
    /// assert_eq!(   SIX_OF_DIAMONDS.deck_index(), 17 );
    /// assert_eq!( SEVEN_OF_DIAMONDS.deck_index(), 18 );
    /// assert_eq!( EIGHT_OF_DIAMONDS.deck_index(), 19 );
    /// assert_eq!(  NINE_OF_DIAMONDS.deck_index(), 20 );
    /// assert_eq!(   TEN_OF_DIAMONDS.deck_index(), 21 );
    /// assert_eq!(  JACK_OF_DIAMONDS.deck_index(), 22 );
    /// assert_eq!( QUEEN_OF_DIAMONDS.deck_index(), 23 );
    /// assert_eq!(  KING_OF_DIAMONDS.deck_index(), 24 );
    /// assert_eq!(   ACE_OF_DIAMONDS.deck_index(), 25 );
    /// assert_eq!(   TWO_OF_SPADES  .deck_index(), 26 );
    /// assert_eq!( THREE_OF_SPADES  .deck_index(), 27 );
    /// assert_eq!(  FOUR_OF_SPADES  .deck_index(), 28 );
    /// assert_eq!(  FIVE_OF_SPADES  .deck_index(), 29 );
    /// assert_eq!(   SIX_OF_SPADES  .deck_index(), 30 );
    /// assert_eq!( SEVEN_OF_SPADES  .deck_index(), 31 );
    /// assert_eq!( EIGHT_OF_SPADES  .deck_index(), 32 );
    /// assert_eq!(  NINE_OF_SPADES  .deck_index(), 33 );
    /// assert_eq!(   TEN_OF_SPADES  .deck_index(), 34 );
    /// assert_eq!(  JACK_OF_SPADES  .deck_index(), 35 );
    /// assert_eq!( QUEEN_OF_SPADES  .deck_index(), 36 );
    /// assert_eq!(  KING_OF_SPADES  .deck_index(), 37 );
    /// assert_eq!(   ACE_OF_SPADES  .deck_index(), 38 );
    /// assert_eq!(   TWO_OF_HEARTS  .deck_index(), 39 );
    /// assert_eq!( THREE_OF_HEARTS  .deck_index(), 40 );
    /// assert_eq!(  FOUR_OF_HEARTS  .deck_index(), 41 );
    /// assert_eq!(  FIVE_OF_HEARTS  .deck_index(), 42 );
    /// assert_eq!(   SIX_OF_HEARTS  .deck_index(), 43 );
    /// assert_eq!( SEVEN_OF_HEARTS  .deck_index(), 44 );
    /// assert_eq!( EIGHT_OF_HEARTS  .deck_index(), 45 );
    /// assert_eq!(  NINE_OF_HEARTS  .deck_index(), 46 );
    /// assert_eq!(   TEN_OF_HEARTS  .deck_index(), 47 );
    /// assert_eq!(  JACK_OF_HEARTS  .deck_index(), 48 );
    /// assert_eq!( QUEEN_OF_HEARTS  .deck_index(), 49 );
    /// assert_eq!(  KING_OF_HEARTS  .deck_index(), 50 );
    /// assert_eq!(   ACE_OF_HEARTS  .deck_index(), 51 );
    /// ```
    pub fn deck_index( self ) -> usize {
        let (suit, rank) = self.suit_and_rank();
        ( 13 * (suit>>4) + rank - 2 ) as usize
    }
}

impl FromStr for Cards {
    type Err = ParseError;
    fn from_str( s: &str ) -> Result<Self,Self::Err> {
        Cards::parse_in_hand( s, NO_CARD, NO_CARD )
    }
}

impl Not for Cards {
    type Output = Self;

    #[inline]
    fn not( self ) -> Self {
        Cards( !self.0 )
    }
}

impl BitAnd<Self> for Cards {
    type Output = Self;

    #[inline]
    fn bitand( self, other: Cards ) -> Self {
        Cards( self.0 & other.0 )
    }
}

impl Add<Self> for Cards {
    type Output = Self;

    #[inline]
    fn add( self, other: Cards ) -> Self {
        Cards( self.0 | other.0 )
    }
}

impl AddAssign<Self> for Cards {
    #[inline]
    fn add_assign( &mut self, other: Cards ) {
        self.0 |= other.0;
    }
}

impl Sub<Self> for Cards {
    type Output = Self;

    #[inline]
    fn sub( self, other: Cards ) -> Self {
        self & !other
    }
}

/// Prints clubs, diamonds, spades, hearts as 'c','d','s','h'.
pub const ASCII_SUIT        : u32 = 0b1;

/// Prints Jack, Queen, King, Ace as 'j','q','k','a'.
pub const LOWERCASE_RANK    : u32 = 0b10;

/// Prints each card in hand with its own trailing suit char.
pub const NO_SHARED_SUIT    : u32 = 0b100;

/// Prints cards without whitespaces.
pub const NO_SPACES         : u32 = 0b1000;

#[derive( Copy, Clone )]
struct Mask( u32 );

impl Mask {
    fn matches( self, flags: u32 ) -> bool {
        self.0 & flags == flags
    }
}

impl Cards {
    /// The `Cards` string representation.
    ///
    /// # Example
    ///
    /// ```
    /// use heartless::*;
    /// let cards = "467QAc 480d 9Ks 6QKh".parse::<Cards>().unwrap();
    /// assert_eq!( cards.text(0), "467QA♣ 480♦ 9K♠ 6QK♥");
    /// assert_eq!( cards.text(ASCII_SUIT), "467QAc 480d 9Ks 6QKh" );
    /// assert_eq!( cards.text(NO_SPACES), "467QA♣480♦9K♠6QK♥");
    /// assert_eq!( cards.text(NO_SHARED_SUIT), "4♣ 6♣ 7♣ Q♣ A♣ 4♦ 8♦ 0♦ 9♠ K♠ 6♥ Q♥ K♥");
    /// assert_eq!( cards.text(LOWERCASE_RANK), "467qa♣ 480♦ 9k♠ 6qk♥" );
    /// ```
    pub fn text( &self, flags: u32 ) -> String {
        let mut result = String::new();

        let mask = Mask( flags );
        let use_ascii_suits = mask.matches( ASCII_SUIT );
        let suit_chars = SUIT_CHARS[ use_ascii_suits as usize ];
        let rank_chars = RANK_CHARS[ mask.matches( LOWERCASE_RANK ) as usize ];
        let mut last_suit = NO_SUIT;
        for suit in [CLUB, DIAMOND, SPADE, HEART] {
            for rank in TWO..(TWO+13) {
                if ( self.0 >> (suit+rank) ) & 1 == 1 {
                    if !mask.matches( NO_SHARED_SUIT ) && last_suit != suit  {
                        if last_suit != NO_SUIT {
                            result.push( suit_chars[ (last_suit>>4) as usize ]);
                            if !mask.matches( NO_SPACES ) { result.push( ' ' )}
                        }
                        last_suit = suit;
                    }
                    result.push( rank_chars[ (rank-TWO) as usize ]);
                    if mask.matches( NO_SHARED_SUIT ) {
                        result.push( suit_chars[ (suit>>4) as usize ]);
                        if !mask.matches( NO_SPACES ) { result.push( ' ' )}
                    }
                }
            }
        }
        if last_suit != NO_SUIT {
            result.push( suit_chars[ (last_suit>>4) as usize ]);
        }

        if result.ends_with(' ') {
            result.pop();
        }

        result
    }
}

fn fmt_to_mask( f: &Formatter ) -> u32 {
    let mut mask = 0;
    if f.align() == Some( fmt::Alignment::Left ) {
        mask |= LOWERCASE_RANK;
    }
    if !f.alternate() {
        mask |= ASCII_SUIT;
    }
    if f.sign_minus() {
        mask |= NO_SPACES;
    }
    mask
}

impl Debug for Cards {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        write!( f, "{}", self.text( fmt_to_mask(f) | NO_SHARED_SUIT ))
    }
}

impl Display for Cards {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        write!( f, "{}", self.text( fmt_to_mask(f) ))
    }
}

/// Computates the result of passing cards / discarding card.
#[derive( Debug, Default )]
pub struct Transfer {
    /// the hand of cards to pass/discard or passed/discarded card(s).
    pub src  : Cards,
    /// the hand of cards to receive/be discarded or received/discarded.
    pub dest : Cards,
}

impl Transfer {
    #[inline]
    /// Computates the result of passing cards / discarding card.
    pub fn transfer( self, cards: Cards ) -> Option<Self> {
        if self.src.contains( cards ) {
            Some( Transfer{ src: self.src-cards, dest: self.dest+cards })
        } else {
            None
        }
    }
}

/// A deck of cards.
pub const CARDS: [Cards; 52] = [
      TWO_OF_CLUBS   ,
    THREE_OF_CLUBS   ,
     FOUR_OF_CLUBS   ,
     FIVE_OF_CLUBS   ,
      SIX_OF_CLUBS   ,
    SEVEN_OF_CLUBS   ,
    EIGHT_OF_CLUBS   ,
     NINE_OF_CLUBS   ,
      TEN_OF_CLUBS   ,
     JACK_OF_CLUBS   ,
    QUEEN_OF_CLUBS   ,
     KING_OF_CLUBS   ,
      ACE_OF_CLUBS   ,
      TWO_OF_DIAMONDS,
    THREE_OF_DIAMONDS,
     FOUR_OF_DIAMONDS,
     FIVE_OF_DIAMONDS,
      SIX_OF_DIAMONDS,
    SEVEN_OF_DIAMONDS,
    EIGHT_OF_DIAMONDS,
     NINE_OF_DIAMONDS,
      TEN_OF_DIAMONDS,
     JACK_OF_DIAMONDS,
    QUEEN_OF_DIAMONDS,
     KING_OF_DIAMONDS,
      ACE_OF_DIAMONDS,
      TWO_OF_SPADES  ,
    THREE_OF_SPADES  ,
     FOUR_OF_SPADES  ,
     FIVE_OF_SPADES  ,
      SIX_OF_SPADES  ,
    SEVEN_OF_SPADES  ,
    EIGHT_OF_SPADES  ,
     NINE_OF_SPADES  ,
      TEN_OF_SPADES  ,
     JACK_OF_SPADES  ,
    QUEEN_OF_SPADES  ,
     KING_OF_SPADES  ,
      ACE_OF_SPADES  ,
      TWO_OF_HEARTS  ,
    THREE_OF_HEARTS  ,
     FOUR_OF_HEARTS  ,
     FIVE_OF_HEARTS  ,
      SIX_OF_HEARTS  ,
    SEVEN_OF_HEARTS  ,
    EIGHT_OF_HEARTS  ,
     NINE_OF_HEARTS  ,
      TEN_OF_HEARTS  ,
     JACK_OF_HEARTS  ,
    QUEEN_OF_HEARTS  ,
     KING_OF_HEARTS  ,
      ACE_OF_HEARTS  ,
];

/// Lowercase names of cards. Both the ranks and suits are lowercases.
pub const LNAMES: [&'static str; 52] = [
    "2c", "3c", "4c", "5c", "6c", "7c", "8c", "9c", "0c", "jc", "qc", "kc", "ac",
    "2d", "3d", "4d", "5d", "6d", "7d", "8d", "9d", "0d", "jd", "qd", "kd", "ad",
    "2s", "3s", "4s", "5s", "6s", "7s", "8s", "9s", "0s", "js", "qs", "ks", "as",
    "2h", "3h", "4h", "5h", "6h", "7h", "8h", "9h", "0h", "jh", "qh", "kh", "ah",
];

/// Lowercase names of cards. Both the ranks of Jack, Queen, King, Ace are uppercases.
pub const UNAMES: [&'static str; 52] = [
    "2c", "3c", "4c", "5c", "6c", "7c", "8c", "9c", "0c", "Jc", "Qc", "Kc", "Ac",
    "2d", "3d", "4d", "5d", "6d", "7d", "8d", "9d", "0d", "Jd", "Qd", "Kd", "Ad",
    "2s", "3s", "4s", "5s", "6s", "7s", "8s", "9s", "0s", "Js", "Qs", "Ks", "As",
    "2h", "3h", "4h", "5h", "6h", "7h", "8h", "9h", "0h", "Jh", "Qh", "Kh", "Ah",
];

/// Graphic names of cards. The suits are "graphic chars".
pub const GNAMES: [&'static str; 52] = [
    "2♣'", "3♣'", "4♣'", "5♣'", "6♣'", "7♣'", "8♣'", "9♣'", "0♣'", "J♣'", "Q♣'", "K♣'", "A♣'",
    "2♦'", "3♦'", "4♦'", "5♦'", "6♦'", "7♦'", "8♦'", "9♦'", "0♦'", "J♦'", "Q♦'", "K♦'", "A♦'",
    "2♠'", "3♠'", "4♠'", "5♠'", "6♠'", "7♠'", "8♠'", "9♠'", "0♠'", "J♠'", "Q♠'", "K♠'", "A♠'",
    "2♥'", "3♥'", "4♥'", "5♥'", "6♥'", "7♥'", "8♥'", "9♥'", "0♥'", "J♥'", "Q♥'", "K♥'", "A♥'",
];
