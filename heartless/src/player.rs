//! Players related definitions.

use crate::card::*;
use std::ops::Range;

pub const ME       : usize = 0;
pub const LEFT     : usize = 1;
pub const OPPOSITE : usize = 2;
pub const RIGHT    : usize = 3;
pub const NOBODY   : usize = 4;

pub const PLAYERS  : Range<usize> = ME..NOBODY;
pub const OTHERS   : Range<usize> = LEFT..NOBODY;

/// The game player's definition
#[derive( Clone, Copy, Default, Debug )]
pub struct Player {
    /// Cards in hand
    pub hand       : Cards,
    /// The card which the player is discarding
    pub discarding : Cards,
    /// The score in this deal (up to 26)
    pub hand_score : u8,
    /// The score in this game (up to 99 or the game is over)
    pub game_score : u8,
    /// Observing which suit this player's hand does not hold
    pub suit_bits  : u8,
    /// Times of winning the game
    pub awards     : u32,
}

impl Player {
    /// Reset everything but keep the times of winning
    pub fn reset( &mut self ) {
        self.hand       = NO_CARD;
        self.discarding = NO_CARD;
        self.hand_score = 0;
        self.game_score = 0;
        self.suit_bits  = NO_SUIT;
    }
    /// Returns true if this player's hand may hold cards of the given suit,
    /// otherwise returns false.
    pub fn may_have( &self, suit: u8 ) -> bool {
        self.suit_bits & ( 1 << (suit>>4) ) != 0
    }
    /// Prints this player's suit_bits to human friendly string.
    pub fn suit_chars( &self ) -> &'static str {
        match self.suit_bits {
            0b0000 => "",
            0b0001 => "c",
            0b0010 => "d",
            0b0011 => "dc",
            0b0100 => "s",
            0b0101 => "sc",
            0b0110 => "sd",
            0b0111 => "sdc",
            0b1000 => "h",
            0b1001 => "hc",
            0b1010 => "hd",
            0b1011 => "hdc",
            0b1100 => "hs",
            0b1101 => "hsc",
            0b1110 => "hsd",
            0b1111 => "hsdc",
            _ => unreachable!(),
        }
    }
}
