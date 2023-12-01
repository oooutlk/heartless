use std::{
    env,
    fmt::{self, Debug, Display, Formatter},
    ops::{Add, AddAssign, BitAnd, Not, Range, Sub},
};

const CLUB      : u8 = 0x00;
const DIAMOND   : u8 = 0x10;
const SPADE     : u8 = 0x20;
const HEART     : u8 = 0x30;
const NO_SUIT   : u8 = 0x40;

const NO_RANK   : u8 = 0x0;
const TWO       : u8 = 0x2;
const THREE     : u8 = 0x3;
const FOUR      : u8 = 0x4;
const FIVE      : u8 = 0x5;
const SIX       : u8 = 0x6;
const SEVEN     : u8 = 0x7;
const EIGHT     : u8 = 0x8;
const NINE      : u8 = 0x9;
const TEN       : u8 = 0xa;
const JACK      : u8 = 0xb;
const QUEEN     : u8 = 0xc;
const KING      : u8 = 0xd;
const ACE       : u8 = 0xe;

const RANKS     : Range<u8> = TWO..(ACE+1);

const   TWO_OF_HEARTS   : Cards = Cards::from(   TWO, HEART );
const THREE_OF_HEARTS   : Cards = Cards::from( THREE, HEART );
const  FOUR_OF_HEARTS   : Cards = Cards::from(  FOUR, HEART );
const  FIVE_OF_HEARTS   : Cards = Cards::from(  FIVE, HEART );
const   SIX_OF_HEARTS   : Cards = Cards::from(   SIX, HEART );
const SEVEN_OF_HEARTS   : Cards = Cards::from( SEVEN, HEART );
const EIGHT_OF_HEARTS   : Cards = Cards::from( EIGHT, HEART );
const  NINE_OF_HEARTS   : Cards = Cards::from(  NINE, HEART );
const   TEN_OF_HEARTS   : Cards = Cards::from(   TEN, HEART );
const  JACK_OF_HEARTS   : Cards = Cards::from(  JACK, HEART );
const QUEEN_OF_HEARTS   : Cards = Cards::from( QUEEN, HEART );
const  KING_OF_HEARTS   : Cards = Cards::from(  KING, HEART );
const   ACE_OF_HEARTS   : Cards = Cards::from(   ACE, HEART );

const   TWO_OF_SPADES   : Cards = Cards::from(   TWO, SPADE );
const THREE_OF_SPADES   : Cards = Cards::from( THREE, SPADE );
const  FOUR_OF_SPADES   : Cards = Cards::from(  FOUR, SPADE );
const  FIVE_OF_SPADES   : Cards = Cards::from(  FIVE, SPADE );
const   SIX_OF_SPADES   : Cards = Cards::from(   SIX, SPADE );
const SEVEN_OF_SPADES   : Cards = Cards::from( SEVEN, SPADE );
const EIGHT_OF_SPADES   : Cards = Cards::from( EIGHT, SPADE );
const  NINE_OF_SPADES   : Cards = Cards::from(  NINE, SPADE );
const   TEN_OF_SPADES   : Cards = Cards::from(   TEN, SPADE );
const  JACK_OF_SPADES   : Cards = Cards::from(  JACK, SPADE );
const QUEEN_OF_SPADES   : Cards = Cards::from( QUEEN, SPADE );
const  KING_OF_SPADES   : Cards = Cards::from(  KING, SPADE );
const   ACE_OF_SPADES   : Cards = Cards::from(   ACE, SPADE );

const   TWO_OF_DIAMONDS : Cards = Cards::from(   TWO, DIAMOND );
const THREE_OF_DIAMONDS : Cards = Cards::from( THREE, DIAMOND );
const  FOUR_OF_DIAMONDS : Cards = Cards::from(  FOUR, DIAMOND );
const  FIVE_OF_DIAMONDS : Cards = Cards::from(  FIVE, DIAMOND );
const   SIX_OF_DIAMONDS : Cards = Cards::from(   SIX, DIAMOND );
const SEVEN_OF_DIAMONDS : Cards = Cards::from( SEVEN, DIAMOND );
const EIGHT_OF_DIAMONDS : Cards = Cards::from( EIGHT, DIAMOND );
const  NINE_OF_DIAMONDS : Cards = Cards::from(  NINE, DIAMOND );
const   TEN_OF_DIAMONDS : Cards = Cards::from(   TEN, DIAMOND );
const  JACK_OF_DIAMONDS : Cards = Cards::from(  JACK, DIAMOND );
const QUEEN_OF_DIAMONDS : Cards = Cards::from( QUEEN, DIAMOND );
const  KING_OF_DIAMONDS : Cards = Cards::from(  KING, DIAMOND );
const   ACE_OF_DIAMONDS : Cards = Cards::from(   ACE, DIAMOND );

const   TWO_OF_CLUBS    : Cards = Cards::from(   TWO, CLUB );
const THREE_OF_CLUBS    : Cards = Cards::from( THREE, CLUB );
const  FOUR_OF_CLUBS    : Cards = Cards::from(  FOUR, CLUB );
const  FIVE_OF_CLUBS    : Cards = Cards::from(  FIVE, CLUB );
const   SIX_OF_CLUBS    : Cards = Cards::from(   SIX, CLUB );
const SEVEN_OF_CLUBS    : Cards = Cards::from( SEVEN, CLUB );
const EIGHT_OF_CLUBS    : Cards = Cards::from( EIGHT, CLUB );
const  NINE_OF_CLUBS    : Cards = Cards::from(  NINE, CLUB );
const   TEN_OF_CLUBS    : Cards = Cards::from(   TEN, CLUB );
const  JACK_OF_CLUBS    : Cards = Cards::from(  JACK, CLUB );
const QUEEN_OF_CLUBS    : Cards = Cards::from( QUEEN, CLUB );
const  KING_OF_CLUBS    : Cards = Cards::from(  KING, CLUB );
const   ACE_OF_CLUBS    : Cards = Cards::from(   ACE, CLUB );

const NO_CARD: Cards = Cards(0);

const fn rank_mask() -> u64 {
    1 << TWO   | 1 << THREE | 1 << FOUR  | 1 << FIVE  |
    1 << SIX   | 1 << SEVEN | 1 << EIGHT | 1 << NINE  |
    1 << TEN   | 1 << JACK  | 1 << QUEEN | 1 << KING  |
    1 << ACE
}

const HEARTS    : Cards = Cards( rank_mask() << HEART     );
const SPADES    : Cards = Cards( rank_mask() << SPADE     );
const DIAMONDS  : Cards = Cards( rank_mask() << DIAMOND   );
const CLUBS     : Cards = Cards( rank_mask() << CLUB      );

const SUIT_CHARS: [[char;4]; 2] = [['c','d','s','h'], ['♣','♦','♠','♥']];
const RANK_CHARS: [char;13] = ['2','3','4','5','6','7','8','9','0','J','Q','K','A'];

const ME       : usize = 0;
const LEFT     : usize = 1;
const OPPOSITE : usize = 2;
const RIGHT    : usize = 3;
const NOBODY   : usize = 4;

const PLAYERS  : Range<usize> = ME..NOBODY;
const OTHERS   : Range<usize> = LEFT..NOBODY;

#[derive( Copy, Clone, Default, PartialEq, Eq, PartialOrd )]
struct Cards( u64 );

impl Cards {
    #[inline]
    const fn from( rank: u8, suit: u8 ) -> Self {
        Cards( 1 << ( rank + suit ))
    }
    #[inline]
    fn contains( self, cards: Cards ) -> bool {
        cards != NO_CARD && self & cards == cards
    }
    #[inline]
    fn count( self ) -> u8 {
        self.0.count_ones() as u8
    }
    #[inline]
    fn count_suit( self, suit_mask: Cards ) -> u8 {
        (self & suit_mask).count()
    }
    fn deal() -> [Self; 4] {
        let mut deck = [
              TWO_OF_HEARTS,   TWO_OF_SPADES,   TWO_OF_DIAMONDS,   TWO_OF_CLUBS,
            THREE_OF_HEARTS, THREE_OF_SPADES, THREE_OF_DIAMONDS, THREE_OF_CLUBS,
             FOUR_OF_HEARTS,  FOUR_OF_SPADES,  FOUR_OF_DIAMONDS,  FOUR_OF_CLUBS,
             FIVE_OF_HEARTS,  FIVE_OF_SPADES,  FIVE_OF_DIAMONDS,  FIVE_OF_CLUBS,
              SIX_OF_HEARTS,   SIX_OF_SPADES,   SIX_OF_DIAMONDS,   SIX_OF_CLUBS,
            SEVEN_OF_HEARTS, SEVEN_OF_SPADES, SEVEN_OF_DIAMONDS, SEVEN_OF_CLUBS,
            EIGHT_OF_HEARTS, EIGHT_OF_SPADES, EIGHT_OF_DIAMONDS, EIGHT_OF_CLUBS,
             NINE_OF_HEARTS,  NINE_OF_SPADES,  NINE_OF_DIAMONDS,  NINE_OF_CLUBS,
              TEN_OF_HEARTS,   TEN_OF_SPADES,   TEN_OF_DIAMONDS,   TEN_OF_CLUBS,
             JACK_OF_HEARTS,  JACK_OF_SPADES,  JACK_OF_DIAMONDS,  JACK_OF_CLUBS,
            QUEEN_OF_HEARTS, QUEEN_OF_SPADES, QUEEN_OF_DIAMONDS, QUEEN_OF_CLUBS,
             KING_OF_HEARTS,  KING_OF_SPADES,  KING_OF_DIAMONDS,  KING_OF_CLUBS,
              ACE_OF_HEARTS,   ACE_OF_SPADES,   ACE_OF_DIAMONDS,   ACE_OF_CLUBS,
        ];
        fastrand::shuffle( &mut deck );
        [
            deck[ 0..13].iter().fold( NO_CARD, |cards, card| cards + *card ),
            deck[13..26].iter().fold( NO_CARD, |cards, card| cards + *card ),
            deck[26..39].iter().fold( NO_CARD, |cards, card| cards + *card ),
            deck[39..52].iter().fold( NO_CARD, |cards, card| cards + *card ),
        ]
    }
    #[inline]
    fn transfer( self, cards: Cards ) -> Option<Transfer> {
        (Transfer{ src: self, dest: NO_CARD }).transfer( cards )
    }
    #[inline]
    fn is_heart( self ) -> bool {
        self & HEARTS != NO_CARD
    }
    #[inline]
    fn is_queen_of_spade( self ) -> bool {
        self == QUEEN_OF_SPADES
    }
    #[inline]
    fn contains_suit( self, suit_mask: Cards ) -> bool {
        self & suit_mask != NO_CARD
    }
    #[inline]
    fn are_all_of( self, suit_mask: Cards ) -> bool {
        self & (!suit_mask) == NO_CARD
    }
    fn pick_three_cards( self ) -> Transfer {
        let mut src  = self;
        let mut dest = NO_CARD;
        let mut passed = 0;

        if let Some( transfered ) = self.transfer( QUEEN_OF_SPADES ) {
            src  = transfered.src;
            dest = transfered.dest;
            passed += 1;
        }

        'polling:
        for rank in RANKS.rev() {
            for suit in [HEART, SPADE, DIAMOND, CLUB] {
                if let Some( transfered ) = (Transfer{ src, dest }).transfer( Cards::from( rank, suit )) {
                    src  = transfered.src;
                    dest = transfered.dest;
                    passed += 1;
                    if passed == 3 {
                        break 'polling;
                    }
                }
            }
        }

        Transfer{ src, dest }
    }
    fn suit_mask( self ) -> Cards {
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
    fn show( self ) {
        println!( "{:#}", self );
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

impl Display for Cards {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        let which = {
            if f.alternate() && env::args().map( |s| s == "ascii" ).nth(1) != Some( true ) {
                1
            } else {
                0
            }
        };
        let suit_chars = SUIT_CHARS[ which ];
        let mut last_suit = NO_SUIT;
        for suit in [CLUB, DIAMOND, SPADE, HEART] {
            for rank in TWO..(TWO+13) {
                if ( self.0 >> (suit+rank) ) & 1 == 1 {
                    if last_suit != suit  {
                        if last_suit != NO_SUIT {
                            write!( f, "{} ", suit_chars[ (last_suit>>4) as usize ] )?;
                        }
                        last_suit = suit;
                    }
                    write!( f, "{}", RANK_CHARS[ (rank-2) as usize ] )?;
                }
            }
        }
        if last_suit != NO_SUIT {
            write!( f, "{} ", suit_chars[ (last_suit>>4) as usize ] )
        } else {
            Ok(())
        }
    }
}

impl Debug for Cards {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        <Self as Display>::fmt( self, f )
    }
}

#[derive( Default )]
struct Transfer {
    src  : Cards,
    dest : Cards,
}

impl Transfer {
    #[inline]
    fn transfer( self, cards: Cards ) -> Option<Self> {
        if self.src.contains( cards ) {
            Some( Transfer{ src: self.src-cards, dest: self.dest+cards })
        } else {
            None
        }
    }
}

#[derive( Copy, Clone, Debug, PartialEq )]
enum Input {
    Deal,
    Discard( Cards ),
    Exit,
    Invalid,
    Pass( Cards ),
}

fn parse_cards( s: &str, hand: Cards, suits: Cards ) -> Cards {
    let mut cards = NO_CARD;
    let mut chars = s.chars().rev();
    let mut suit = NO_SUIT;
    let mut rank = NO_RANK;

    while let Some( ch ) = chars.next() {
        match ch {
            'c'|'C' => suit = CLUB, 'd'|'D' => suit = DIAMOND, 's'|'S' => suit = SPADE, 'h'|'H' => suit = HEART,
            '2' => rank = 0x2, '3' => rank = 0x3, '4' => rank = 0x4, '5' => rank = 0x5, '6' => rank = 0x6,
            '7' => rank = 0x7, '8' => rank = 0x8, '9' => rank = 0x9, '0' => rank = 0xa,
            'j'|'J' => rank = 0xb, 'q'|'Q' => rank = 0xc, 'k'|'K' => rank = 0xd, 'a'|'A' => rank = 0xe,
            ' ' => continue,
            _ => return NO_CARD,
        }
        if rank != NO_RANK {
            if suit == NO_SUIT {
                let mut guess = match suits {
                    CLUBS    => Cards::from( rank, CLUB    ),
                    DIAMONDS => Cards::from( rank, DIAMOND ),
                    SPADES   => Cards::from( rank, SPADE   ),
                    HEARTS   => Cards::from( rank, HEART   ),
                    _        => NO_CARD,
                };
                if hand.contains( guess ) {
                    cards += guess;
                } else {
                    guess = NO_CARD;
                    for suit in [HEART, SPADE, DIAMOND, CLUB] {
                        let card = Cards::from( rank, suit );
                        if hand.contains( card ) {
                            if guess == NO_CARD {
                                guess = card;
                            } else {
                                hint_multiple_cards( card + guess );
                                return NO_CARD;
                            }
                        }
                    }
                    cards += guess;
                }
            } else {
                cards += Cards::from( rank, suit );
            }
            rank = NO_RANK;
        }
    }
    cards
}

fn hint_welcome() {
    println!( "Welcome to play heartless!" );
    hint_deal();
}
fn hint_deal() {
    println!( "Press enter to start..." );
}
fn hint_not_your_card( card: Cards ) {
    println!( "discarding card is not in hand: {card:#}" );
}
fn hint_not_your_cards( cards: Cards ) {
    println!( "Passing cards are not all in hand: {cards:#}" );
}
fn hint_two_of_clubs() {
    println!( "Two of clubs should be dropped first!" );
}
fn hint_follow_suit() {
    println!( "You should follow the suit!" );
}
fn hint_no_heartbreak() {
    println!( "Heart not allowed to be early hand if no hearts has been discarded before." );
}
fn hint_heart_in_first_round() {
    println!( "Heart not allowed in first round." );
}
fn hint_queen_in_first_round() {
    println!( "Queen of Spades not allowed in first round." );
}
fn hint_multiple_cards( cards: Cards ) {
    println!( "Which card to discard? {:#}", cards );
}
fn hint_discard_one( cards: Cards ) {
    println!( "Not allowed to discard multiple cards in one round: {cards:#}" );
}
fn hint_pass_three( cards: Cards ) {
    println!( "These are not 3 cards to pass: {cards:#}" );
}

#[derive( Default, Debug )]
struct Player {
    hand        : Cards,
    discarding  : Cards,
    deal_score  : u8,
    game_score  : u8,
    no_club     : bool,
    no_diamond  : bool,
    no_spade    : bool,
    no_heart    : bool,
}

impl Player {
    fn follows( &self, suit: u8 ) -> bool {
        match suit {
            CLUB    => !self.no_club,
            DIAMOND => !self.no_diamond,
            SPADE   => !self.no_spade,
            HEART   => !self.no_heart,
            _ => unreachable!(),
        }
    }
}

#[derive( Debug )]
struct Game {
    players         : [Player;4],
    winner          : usize,
    deal            : usize,
    discarded       : Cards,
    suit_mask       : Cards,
    round           : usize,
    under_the_gun   : usize,
    round_winner    : usize,
    high_card       : Cards,
}

impl Default for Game {
    fn default() -> Self {
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
        }
    }
}

impl Game {
    fn input( &self ) -> Input {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line( &mut buffer )
            .expect("to get an input from the player");
        let buffer = buffer.trim();

        let mut input = Input::Invalid;
        let hand = self.players[ME].hand;
        match buffer {
            "" => input = Input::Deal,
            "exit" => input = Input::Exit,
            s if self.round > 0 => {
                let card = parse_cards( &s, hand, self.suit_mask );
                if card != NO_CARD {
                    if card.count() == 1 {
                        input = Input::Discard( card );
                    } else {
                        hint_discard_one( card );
                    }
                }
            },
            s if self.round == 0 => {
                let cards = parse_cards( &s, hand, self.suit_mask );
                if cards != NO_CARD {
                    if cards.count() == 3 {
                        input = Input::Pass( cards );
                    } else {
                        hint_pass_three( cards );
                    }
                }
            },
            _ => (),
        }
        input
    }
    fn find_the_winner( &mut self ) -> usize {
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
            self.players[i].deal_score  = 0;
            self.players[i].no_club     = false;
            self.players[i].no_diamond  = false;
            self.players[i].no_spade    = false;
            self.players[i].no_heart    = false;
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
                transfers[i] = self.players[i].hand.pick_three_cards();
            }
            let offset = [0,3,1,2][ self.deal%4 ];
            for i in PLAYERS {
                let j = ( i + offset ) % 4;
                self.players[i].hand = transfers[i].src + transfers[j].dest;
            }
            true
        } else {
            hint_not_your_cards( my_three_cards );
            false
        }

    }
    fn break_the_rule( &self, who: usize, card: Cards ) -> Rule {
        let hand = self.players[who].hand;

        if {
            hand.contains( TWO_OF_CLUBS ) &&
            card != TWO_OF_CLUBS
        } {
            return Rule::TwoOfClubs;
        } else if {
            self.suit_mask != NO_CARD &&
            hand.contains_suit( self.suit_mask ) &&
            !card.contains_suit( self.suit_mask )
        } {
            return Rule::FollowSuit;
        } else if card.is_heart() && !hand.are_all_of( HEARTS ) {
            if self.under_the_gun == who {
                if !self.heart_broken() {
                    return Rule::Heartbreak;
                }
            } else if self.round == 1 {
                return Rule::HeartInFirstRound;
            }
        } else if card.is_queen_of_spade() {
            if self.round == 1 && self.under_the_gun != who {
                return Rule::QueenInFirstRound;
            }
        }
        Rule::None
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
                    let card = Cards::from( rank, suit );
                    if let Some( transfered ) = self.transfer_card( who, card ) {
                        return transfered;
                    }
                }
            }
        } else if hand & self.suit_mask != NO_CARD { // follow suit
            let suit = match self.suit_mask {
                CLUBS    => CLUB   ,
                DIAMONDS => DIAMOND,
                SPADES   => SPADE  ,
                HEARTS   => HEART  ,
                _ => unreachable!(),
            };

            let mut to_discard = Cards::from( ACE, suit );
            let mut high_card_to_discard = NO_CARD;
            for rank in RANKS.rev() {
                let card = Cards::from( rank, suit );
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
                        if high_card_to_discard < card {
                            high_card_to_discard = card;
                        }
                    }
                }
            }
            let mut late_hand_follows = false;
            for i in 1..3 {
                let i = (who+i) %4;
                if self.players[i].follows( suit ) {
                    late_hand_follows = true;
                }
            }
            return self.transfer_card(
                who,
                if late_hand_follows { to_discard } else { high_card_to_discard }
            ).unwrap();
        } else { // don't follow suit
            if hand.contains( QUEEN_OF_SPADES ) && self.round != 1 { // try to discard Queen of Spades
                return self.transfer_card( who, QUEEN_OF_SPADES ).unwrap();
            }
            if hand.contains_suit( HEARTS ) && self.round != 1 { // try to discard hearts
                for rank in RANKS.rev() {
                    if let Some( transfered ) = self.transfer_card( who, Cards::from( rank, HEART )) {
                        return transfered;
                    }
                }
            }
            for rank in RANKS.rev() { // try to discard high card
                for suit in [SPADE, DIAMOND, CLUB, HEART] {
                    if let Some( transfered ) = self.transfer_card( who, Cards::from( rank, suit )) {
                        return transfered;
                    }
                }
            }
        }
        unreachable!();
    }
    fn transfer_card( &self, who: usize, card: Cards ) -> Option<Transfer> {
        self.players[who].hand.transfer( card )
    }
    fn who_holds_two_of_clubs( &mut self ) -> usize {
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
            CLUBS    => if !discarding.contains_suit( CLUBS    ) { self.players[who].no_club    = true; },
            DIAMONDS => if !discarding.contains_suit( DIAMONDS ) { self.players[who].no_diamond = true; },
            SPADES   => if !discarding.contains_suit( SPADES   ) { self.players[who].no_spade   = true; },
            HEARTS   => if !discarding.contains_suit( HEARTS   ) { self.players[who].no_heart   = true; },
            _ => (),
        }

        self.discarded += transfered.dest;
        if self.suit_mask == NO_CARD {
            self.suit_mask = transfered.dest.suit_mask();
        }
        let card = self.players[who].discarding & self.suit_mask;
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
            self.players[ self.round_winner ].deal_score += score;
            if self.players[ self.round_winner ].deal_score == 26 {
                self.players[ self.round_winner ].deal_score = 0;
                for i in PLAYERS {
                    if i != self.round_winner {
                        self.players[i].deal_score = 26;
                    }
                }
            }
        }
        if self.round == 13 {
            for i in PLAYERS {
                self.players[i].game_score += self.players[i].deal_score;
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
    fn show_my_hand( &self ) {
        self.players[ME].hand.show()
    }
    fn show_winner( &self ) {
        if self.winner == ME {
            println!( "Win!" );
        } else {
            println!( "Lose!" );
        }
    }
    fn hint_pass( &self ) {
        let receiver = [ "nobody", "left", "right", "opposite" ][ self.deal%4 ];
        let three_cards = self.players[ME].hand.pick_three_cards().dest;
        println!( "Pass 3 cards to {receiver}. For example: {three_cards}" );
    }
    fn hint_discard( &self ) {
        let transfered = self.pick_card( ME );
        let card = transfered.dest;
        println!( "Discard 1 card. For example: {card}" );
    }
    fn show_scores_and_discarding( &self ) {
        println!( "- - - - - - - - - - -" );
        let players = &self.players;
        println!( "       {:>7}",
            format!( "{}/{}", players[OPPOSITE].deal_score, players[OPPOSITE].game_score ));
        println!( "{:>7}       {:>7}",
            format!( "{}/{}", players[LEFT] .deal_score, players[LEFT] .game_score ),
            format!( "{}/{}", players[RIGHT].deal_score, players[RIGHT].game_score )
        );
        println!( "       {:>7}",
            format!( "{}/{}", players[ME].deal_score, players[ME].game_score ));
        println!( "" );

        let mut gun = [' ', ' ', ' ', ' '];
        gun[ self.under_the_gun ] = '.';
        let n = self.players[ OPPOSITE ].discarding;
        let w = self.players[ LEFT     ].discarding;
        let e = self.players[ RIGHT    ].discarding;
        let s = self.players[ ME       ].discarding;
        println!( "          {}{:#}",        gun[OPPOSITE] , n );
          print!( "   {}{:#} ",              gun[LEFT]     , w );
        if w == NO_CARD {
            print!(     "   " );
        }
        println!(         "         {}{:#}", gun[RIGHT]    , e );
        println!( "          {}{:#}",        gun[ME]       , s );
    }
}

enum Rule {
    None,
    TwoOfClubs,
    FollowSuit,
    Heartbreak,
    HeartInFirstRound,
    QueenInFirstRound,
}

fn main() {
    fastrand::seed( 2023 );
    hint_welcome();
    let mut game = Game::default();
    'game: loop {
        match game.input() {
            Input::Deal => {
                while game.find_the_winner() == NOBODY {
                    game.deal();
                    if game.needs_pass_cards() {
                        game.show_my_hand();
                        'pass: loop {
                            game.hint_pass();
                            match game.input() {
                                Input::Pass( cards ) => if game.pass_cards( cards ) {
                                    break 'pass;
                                },
                                Input::Exit => break 'game,
                                _ => (),
                            }
                        }
                    }
                    while game.next_round() {
                        let start = game.under_the_gun;
                        for i in start..start+4 {
                            let i = i % 4;
                            if i == ME {
                                if start != ME {
                                    game.show_scores_and_discarding();
                                }
                                'discard: loop {
                                    game.show_my_hand();
                                    game.hint_discard();
                                    match game.input() {
                                        Input::Discard( card ) => match game.break_the_rule( ME, card ) {
                                            Rule::TwoOfClubs => hint_two_of_clubs(),
                                            Rule::FollowSuit => hint_follow_suit(),
                                            Rule::Heartbreak => hint_no_heartbreak(),
                                            Rule::HeartInFirstRound=> hint_heart_in_first_round(),
                                            Rule::QueenInFirstRound=> hint_queen_in_first_round(),
                                            Rule::None => {
                                                if let Some( transfered ) = game.players[ME].hand.transfer( card ) {
                                                    game.discard( ME, transfered );
                                                    break 'discard;
                                                } else {
                                                    hint_not_your_card( card );
                                                }
                                                continue 'discard;
                                            },
                                        },
                                        Input::Exit => break 'game,
                                        _ => (),
                                    }
                                }
                            } else {
                                game.discard( i, game.pick_card(i) );
                            }
                        }
                        game.keep_score();
                        game.show_scores_and_discarding();
                    }
                }
                game.show_winner();
                game.deal = 0;
            },
            Input::Exit => break 'game,
            _ => hint_deal(),
        }
    }
}
