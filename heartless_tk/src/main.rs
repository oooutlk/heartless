#![windows_subsystem = "windows"]

use ::bind::bind;

use heartless::*;

use indexmap::IndexMap;

use std::{
    cell::{Cell, RefCell},
    collections::{HashSet, VecDeque},
    fmt::{self, Display},
    rc::Rc,
    thread,
};

use tcl::*;
use tk::*;
use tk::cmd::*;
type InterpResult<T> = Result<T, tcl::error::InterpError>;

mod and_extra;
use and_extra::*;

const FOR_TK: u32 = ASCII_SUIT | LOWERCASE_RANK | NO_SHARED_SUIT;
const SCORING_CARDS: [&'static str; 14] = ["qs","2h","3h","4h","5h","6h","7h","8h","9h","0h","jh","qh","kh","ah"];
const DISCARD_ORDER: [usize; 13] = [0,12,1,11,2,10,3,9,4,8,5,7,6];

#[derive( Clone, Copy, PartialEq, Eq )]
enum State {
    Welcome,
    Dealing,
    Dealed,
    Passing,
    Received,
    ToAccept,
    Discarding,
    Obey,
    Discarded,
    CheckWinner,
    ShowWinner,
    WaitForStart,
    WaitForPassing,
    WaitForDiscarding,
    Exit,
}

struct RelCoord{ x: f64, y: f64 }

impl RelCoord {
    fn from_btn( tk: Tk<()>, btn: TkButton<()> ) -> InterpResult<Option<RelCoord>> {
        let place = btn.place_info()?;
        let x = match place.get( "-relx" ) {
            Some( x ) => tk.double( x.clone() )?,
            None => return Ok( None ),
        };
        let y = match place.get( "-rely" ) {
            Some( y ) => tk.double( y.clone() )?,
            None => return Ok( None ),
        };
        Ok( Some( RelCoord{ x, y }))
    }
}

enum Animation {
    Move( Move ),
    Hide( Hide ),
}

struct Move {
    card        : String,
    src         : RelCoord,
    dest        : RelCoord,
    percent     : u32,
    done_state  : Option<State>, // set State on completion
    parallel    : bool, // true: run this and the next one in parallel
}

struct Hide {
    card        : String,
    percent     : u32,
    parallel    : bool, // true: run this and the next one in parallel
}

impl Animation {
    fn is_completed( &self ) -> bool {
        match self {
            Animation::Move( mov  ) =>  mov.percent >= 100,
            Animation::Hide( hide ) => hide.percent >= 100,
        }
    }
    fn in_parallel( &self ) -> bool {
        match self {
            Animation::Move( mov  ) =>  mov.parallel,
            Animation::Hide( hide ) => hide.parallel,
        }
    }
}

#[derive( Clone )]
struct Animations( Rc<RefCell<AnimationsInner>> );

struct AnimationsInner {
    queue         : VecDeque<Animation>,
    the_suits     : Rc<Cell<Cards>>,
    under_the_gun : Rc<Cell<usize>>,
}

impl Animations {
    fn new( the_suits: Rc<Cell<Cards>>, under_the_gun: Rc<Cell<usize>> ) -> Self {
        Animations( Rc::new( RefCell::new( AnimationsInner {
            queue: VecDeque::new(),
            the_suits,
            under_the_gun,
        })))
    }
    fn discard( &self, who: usize, card: &str, to_discard: TkButton<()>, tk: Tk<()> ) -> InterpResult<()> {
        let src = match RelCoord::from_btn( tk, to_discard )? {
            Some( rel_coord ) => rel_coord,
            None => return Ok(()),
        };

        let x_of = [0.48, 0.34, 0.48, 0.62];
        let y_of = [0.60, 0.41, 0.21, 0.41];

        self.0.borrow_mut().queue.push_back( Animation::Move( Move {
            card       : card.to_owned(),
            src        ,
            dest       : RelCoord{ x: x_of[who], y: y_of[who] },
            percent    : 0,
            done_state : (who == ME).then_some( State::Discarded ),
            parallel   : false,
        }));

        if who == ME {
            Ok(())
        } else {
            let gun = self.0.borrow().under_the_gun.get();
            if gun != NOBODY {
                if who == (gun+3) % 4 {
                    self.0.borrow().the_suits.set( NO_CARD );
                }
            }

            to_discard.place_forget()
        }
    }
    fn hide( &self, card: &str, parallel: bool ) {
        self.0.borrow_mut().queue.push_back( Animation::Hide( Hide {
            card        : card.to_owned(),
            percent     : 0,
            parallel    ,
        }));
    }
    fn is_empty( &self ) -> bool { self.0.borrow().queue.is_empty() }
}

fn set_card_line<'card, 'line:'card>(
        who : usize,
        line: &'line str,
        card: &mut &'card str,
        gun: &mut usize,
        win: &mut usize
) {
    *card = line.trim_start_matches(':').trim_end_matches('.');
    if line.starts_with(':') { *gun = who; }
    if line.ends_with('.') { *win = who; }
}

fn set_discarding_cards<'card, 'line, Lines>(
    lines : &mut Lines,
    cards : &mut [&'card str; 4],
      gun : &mut usize,
      win : &mut usize
)
where 'line : 'card
    , Lines : Iterator<Item=&'line str>
{
    let opposite_line = lines.next().unwrap().trim();
    set_card_line( OPPOSITE, opposite_line, &mut cards[OPPOSITE], gun, win );

    let left_right_line = lines.next().unwrap();
    let num_of_starting_spaces = left_right_line.chars().take_while( |&c| c == ' ' ).fold( 0, |n,_| n+1 );
    let left_right_line = left_right_line.trim();
    let mut left_and_right = left_right_line.split_whitespace();
    if num_of_starting_spaces <= 4 {
        let left_line = left_and_right.next().unwrap();
        set_card_line( LEFT, left_line, &mut cards[LEFT], gun, win );
    }
    if let Some( right_line ) = left_and_right.next() {
        set_card_line( RIGHT, right_line, &mut cards[RIGHT], gun, win );
    }

    let my_line = lines.next().unwrap().trim();
    set_card_line( ME, my_line, &mut cards[ME], gun, win );
}

#[derive( Clone )]
struct KbInput( Rc<RefCell<String>> );

impl KbInput {
    fn new() -> Self { KbInput( Rc::new( RefCell::new( String::new() )))}
    fn clear( &self ) { self.0.borrow_mut().clear(); }
    fn is_empty( &self ) -> bool { self.0.borrow().is_empty() }
    fn push( &self, ch: char ) { self.0.borrow_mut().push( ch ); }
    fn pop( &self ) { self.0.borrow_mut().pop(); }
    fn repeat_cnt( &self ) -> usize {
        let input = self.0.borrow();
        let mut rchars = input.chars().rev();
        let last = rchars.next().unwrap();
        rchars.take_while( |&ch| ch == last ).fold( 0, |cnt,_| cnt+1 )
    }
    fn trim_start( &self ) -> bool {
        let repeat = self.repeat_cnt();
        let mut s = self.0.borrow_mut();
        if repeat > 0 {
            let len = s.len();
            let new_value = s[ (len-repeat-1).. ].to_owned();
            if *s == new_value  {
                false
            } else {
                *s = new_value;
                true
            }
        } else {
            if s.len() == 1 {
                false
            } else {
                let ch = s.chars().last();
                ch.map( |ch| *s = ch.into() );
                true
            }
        }
    }
    fn trim_end( &self ) {
        let cnt = self.repeat_cnt();
        let mut s = self.0.borrow_mut();
        (0..cnt).for_each( |_| { s.pop(); });
    }
    fn trim_end_to_string( &self ) -> String {
        let cnt = self.repeat_cnt();
        let mut s = self.0.borrow().to_owned();
        (0..cnt).for_each( |_| { s.pop(); });
        s
    }
}

impl Display for KbInput {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "{}", self.0.borrow().as_str() )
    }
}

type Deck = Rc<IndexMap<&'static str,TkButton<()>>>;

fn restart_game( deck: &Deck, start_btn: TkButton<()> ) -> InterpResult<()> {
    deck.values().try_for_each( |btn| btn.place_forget() )?;
    start_btn.invoke()?;
    Ok(())
}

fn select_card( deck: &Deck, card: &str ) -> InterpResult<()> {
    if let Some( btn ) = deck.get( card ) {
        if btn.winfo_viewable()? {
            btn.place( -rely(0.77) )?;
        }
    }
    Ok(())
}

fn unselect_card( deck: &Deck, card: &str ) -> InterpResult<()> {
    if let Some( btn ) = deck.get( card ) {
        if btn.winfo_viewable()? {
            btn.place( -rely(0.80) )?;
        }
    }
    Ok(())
}

#[derive( Clone )]
struct MyPasses{
    cards    : Rc<RefCell<HashSet<String>>>,
    pass_btn : TkButton<()>,
    deck     : Deck,
}

impl MyPasses {
    fn new( pass_btn: TkButton<()>, deck: Deck ) -> Self {
        MyPasses {
            cards   : Rc::new( RefCell::new( HashSet::new() )),
            pass_btn,
            deck    ,
        }
    }
    fn remove( &self, card: &str ) -> InterpResult<bool> {
        let once_existed = self.cards.borrow_mut().remove( card );
        self.set_pass_btn_state()?;
        self.deck[ card ].place( -rely(0.80) )?;
        Ok( once_existed )
    }
    fn len( &self ) -> usize { self.cards.borrow().len() }
    fn insert( &self, card: String ) -> InterpResult<()> {
        select_card( &self.deck, &card )?;
        self.cards.borrow_mut().insert( card );
        self.set_pass_btn_state()
    }
    fn clear( &self ) -> InterpResult<()> {
        self.cards.borrow().iter().try_for_each( |card| { self.deck[ &**card ].place( -rely(0.8) )?; Ok(()) })?;
        self.cards.borrow_mut().clear();
        self.set_pass_btn_state()
    }
    fn forget( &self ) -> InterpResult<()> {
        self.cards.borrow().iter().try_for_each( |card| self.deck[ &**card ].place_forget() )?;
        self.cards.borrow_mut().clear();
        Ok(())
    }
    fn set_pass_btn_state( &self ) -> InterpResult<()> {
        let new_state = if self.len() == 3 {"normal"} else {"disabled"};
        self.pass_btn.configure( -state(new_state) )
    }
}

impl Display for MyPasses {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "{}", self.cards.borrow().iter().cloned().collect::<String>() )
    }
}

#[derive( Clone )]
struct MyDiscard {
    card : Rc<Cell<&'static str>>, // stores card name
    deck : Deck,
}

impl MyDiscard {
    fn new( deck: Deck ) -> Self {
        MyDiscard {
            card : Rc::new( Cell::new( "" )),
            deck ,
        }
    }
    fn is( &self, card: &str ) -> bool { self.card.get() == card }
    fn is_empty( &self ) -> bool { self.card.get().is_empty() }
    fn clear( &self ) { self.card.set(""); }
    fn unselect( &self ) -> InterpResult<()> {
        unselect_card( &self.deck, self.card.get() )?;
        self.clear();
        Ok(())
    }
    fn select( &self, card: &str ) -> InterpResult<()> {
        self.unselect()?;
        select_card( &self.deck, &card )?;
        self.card.set( heartless::LNAMES.iter().find( |&&name| name == card ).unwrap() );
        Ok(())
    }
}

impl Display for MyDiscard {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "{}", self.card.get() )
    }
}

fn hearts_in_relative_coordinates() -> Vec<(f64,f64)> {
    let x_scale  = 0.8;
    let x_offset = 0.56; // move right
    let y_offset = 0.1; // raise up
    let y_bottom = 0.8 - y_offset; // bottom point
    (0..100).fold( vec![], |mut acc, val| {
        let t = -1.0 + (val as f64) / 50.0;
        let x = t.sin() * t.cos() * t.abs().ln();
        let x = (x+x_offset) * x_scale;
        let y = t.abs().powf(0.3) * t.cos().sqrt();
        let y = 1.0 - y - y_offset;
        if x.is_nan() {
            acc.push(( x_offset * x_scale, y_bottom ));
        } else {
            acc.push(( x, y ));
        }
        acc
    })
}

fn main() -> TkResult<()> {
    let mut config = heartless::cli::Config::from_args( std::env::args_os() );

    // for split_whitespace() and tk widgets whose paths are all ascii chars in lowercase.
    config.ascii_suit     = true;
    config.lowercase_rank = true;
    config.no_spaces      = false;
    config.no_shared_suit = true;

    config.impatient = Some( true );

    let automatic = config.automatic;
    config.automatic = false;

    let mut game = Game::with_config( config );
    let game_io = game.altio.clone();
    thread::spawn( move || game.main_loop() );

    let tk = Tk::new(())?;
    let root = tk.root();

    let sw = root.winfo_screenwidth()?;
    let sh = root.winfo_screenheight()?;
    let w = std::cmp::min( sw, 800 );
    let h = std::cmp::min( sh, 600 );
    let x = (sw-w)/2;
    let y = (sh-h)/2;
    root.set_wm_geometry( TkGeometry{ w, h, x, y })?;
    root.set_wm_title( "heartless" )?;

    let card_back_png = tk.image_create_photo( "back" -data(heartless_assets::CARD_BACK) )?;
    let mut card_back_btns = vec![];
    for _ in 0..39 {
        card_back_btns.push( root.add_button( -image(card_back_png.name()) )? );
    }

    let deck = Rc::new(
        heartless::LNAMES.iter()
        .zip( heartless_assets::BASE64S.iter() )
        .try_fold( IndexMap::new(), |mut deck, (lname, base64)| {
            let png = tk.image_create_photo( *lname -data(*base64) )?;
            let btn = root.add_button( *lname -image(png.name()) )?;
            deck.entry( *lname ).or_insert( btn );
            InterpResult::Ok( deck )
        })?
    );

    let the_state     = Rc::new( Cell::new( State::Welcome ));
    let quit_msg      = Rc::new( RefCell::new( None ));

    let under_the_gun = Rc::new( Cell::new( NOBODY )); // for the_suits
    let round_cnt     = Rc::new( Cell::new( 0_usize ));
    let round_winner  = Rc::new( Cell::new( 0_usize )); // for the_suits

    let my_recvs    = Rc::new( RefCell::new( String::new() )); // 3 cards to receive
    let my_discard  = MyDiscard::new( deck.clone() ); // for cancellable and multiple selection

    let kb_input    = KbInput::new();
    let my_hand     = Rc::new( Cell::new( NO_CARD ));
    let the_suits   = Rc::new( Cell::new( NO_CARD ));

    let hint_cards  = Rc::new( Cell::new( NO_CARD ));

    let animations = Animations::new( the_suits.clone(), under_the_gun.clone() );

    let welcome_lb = root.add_label( "welcome-lb" )?;

    let start_btn = root.add_button( "start-btn" -text("Start") )?;
    start_btn.configure( -command( tclosure!( tk, bind:(game_io,deck,the_state), || {
        the_state.set( State::Dealing );
        game_io.send_line( "" );
        SCORING_CARDS.iter().try_for_each( |card| {
            deck[ card ].place_forget()?;
            Ok(())
        })?;
        welcome_lb.place_forget()?;
        start_btn.place_forget()
    })))?;

    let score_lbs = vec![
        root.add_label( "my-score-lb"       -text("0/0") )?,
        root.add_label( "left-score-lb"     -text("0/0") )?,
        root.add_label( "opposite-score-lb" -text("0/0") )?,
        root.add_label( "right-score-lb"    -text("0/0") )?,
    ];

    let pass_btn = root.add_button( "pass-btn" )?;
    let my_passes = MyPasses::new( pass_btn, deck.clone() ); // 3 cards to pass
    pass_btn.configure( -command( tclosure!( tk, bind:(game_io,the_state,my_passes), || {
        the_state.set( State::Passing );
        game_io.send_line( &my_passes.to_string() );
        pass_btn.configure( -state("disabled") )?;
        pass_btn.place_forget()
    })))?;

    let acpt_btn = root.add_button( "acpt-btn" -text("Accept") )?;
    acpt_btn.configure( -command( tclosure!( tk, bind:(deck,my_recvs,round_cnt,the_state,the_suits), || {
        the_state.set( State::Obey );
        the_suits.set( CLUBS );
        round_cnt.set( 1 );
        deck.iter().try_for_each( |(name, btn)| {
            if my_recvs.borrow().contains( name ) {
                btn.place( -rely(0.80) )?;
            }
            btn.configure( -state("normal") )
        })?;
        my_recvs.borrow_mut().clear();
        acpt_btn.place_forget()
    })))?;

    deck.iter().try_for_each( bind!((quit_msg) |(card, btn)| {
        btn.configure( -command(
            tclosure!( tk, bind:(animations,btn,card,deck,my_discard,my_hand,my_passes,round_cnt,the_state), || {
                if animations.is_empty() {
                    if the_state.get() == State::ShowWinner {
                        if quit_msg.borrow().is_some() {
                            the_state.set( State::Exit );
                        } else {
                            restart_game( &deck, start_btn )?;
                        }
                    } else if round_cnt.get() == 0 { // passing cards
                        if pass_btn.winfo_viewable()? {
                            if !my_passes.remove( card )? {
                                if my_passes.len() < 3 {
                                    my_passes.insert( card.to_owned() )?;
                                }
                            }
                        } else {
                            the_state.set( State::Welcome );
                            deck.values().try_for_each( |btn| btn.place_forget() )?;
                        }
                    } else { // discarding card
                        if my_discard.is( card ) {
                            animations.discard( ME, card, btn, tk )?;
                        } else if my_hand.get().contains( card.parse().unwrap_or( NO_CARD )) {
                            my_discard.select( card )?;
                        }
                    }
                }
                Ok(())
            })
        ))
    }))?;

    let animate = bind!((animations,deck,the_state) move || {
        let one = &extra(1);
        let queue = &mut animations.0.borrow_mut().queue;
        queue
            .iter_mut()
            .take_while( |animation| (*animation).in_parallel().and_extra(one) )
            .try_for_each( |animation| Ok( match animation {
                Animation::Move( mov ) => {
                    let (x, y);
                    if mov.percent < 100 {
                        mov.percent += if automatic {100} else {20};
                        x = ( mov.src.x*100.0 + (mov.dest.x-mov.src.x)* (mov.percent as f64) )/100.0;
                        y = ( mov.src.y*100.0 + (mov.dest.y-mov.src.y)* (mov.percent as f64) )/100.0;
                    } else {
                        x = mov.dest.x;
                        y = mov.dest.y;
                    }
                    deck[ &*mov.card ].place( -relx(x) -rely(y) )?;
                }
                Animation::Hide( hide ) => {
                    if hide.percent < 100 {
                        hide.percent += if automatic {100} else {20};
                    }
                }
            }))?;
        while queue.front().map( |front| front.is_completed() ).unwrap_or( false ) {
            match queue.pop_front().unwrap() {
                Animation::Move( mov ) => if let Some( done_state ) = mov.done_state {
                    if the_state.get() == State::WaitForDiscarding {
                        the_state.set( done_state );
                    }
                },
                Animation::Hide( hide ) => deck[ &*hide.card ].place_forget()?,
            }
        }
        Ok(())
    });

    let hearts_curve = hearts_in_relative_coordinates();
    let hc_len = hearts_curve.len();
    let idx_in_hearts_curve = Rc::new( Cell::new( 0_usize ));
    let show_winner_animation = bind!((deck,idx_in_hearts_curve) move || {
        let idx = idx_in_hearts_curve.get();
        idx_in_hearts_curve.set( (idx+1) % hc_len );
        if automatic && idx+1 == hc_len {
            deck.values().try_for_each( |btn| btn.place_forget() )?;
            deck["ah"].invoke()?;
            InterpResult::Ok(())
        } else {
            deck.values().enumerate().try_for_each( |(nth, btn)| {
                let idx = (idx+nth) % hc_len;
                let (x, y) = hearts_curve[ idx ];
                btn.place( -relx(x) -rely(y) )?;
                InterpResult::Ok(())
            })
        }
    });

    let kb_input_lb = root.add_label( "kb-input" )?;

    root.bind( event::any_key_press(), tclosure!( tk,
        bind:(animations,deck,my_discard,my_hand,my_passes,quit_msg,round_cnt,round_winner,the_state,the_suits),
        |evt_key| -> TkResult<()> {
            if the_state.get() == State::ShowWinner {
                if quit_msg.borrow().is_some() {
                    the_state.set( State::Exit );
                } else {
                    restart_game( &deck, start_btn )?;
                }
            } else {
                if animations.is_empty() {
                    let mut submit = false;
                    match evt_key {
                        TkKey::_2 | TkKey::_3 | TkKey::_4 | TkKey::_5 | TkKey::_6 |
                        TkKey::_7 | TkKey::_8 | TkKey::_9 | TkKey::_0 |
                        TkKey::j  | TkKey::q  | TkKey::k  | TkKey::a  |
                        TkKey::c  | TkKey::d  | TkKey::s  | TkKey::h  =>
                            kb_input.push( evt_key.as_char() ),
                        TkKey::BackSpace => kb_input.pop(),
                        TkKey::Escape    => kb_input.clear(),
                        TkKey::Return    => submit = true,
                        _ => {}
                    }

                    let input = kb_input.to_string();
                    let hand = my_hand.get();

                    let suit = if round_winner.get() == ME && round_cnt.get() != 1 {
                        NO_CARD
                    } else {
                        the_suits.get()
                    };

                    if submit {
                        if start_btn.winfo_viewable()? {
                            start_btn.invoke()?;
                        } else if pass_btn.winfo_viewable()? {
                            pass_btn.invoke()?;
                        } else if acpt_btn.winfo_viewable()? {
                            acpt_btn.invoke()?;
                        } else if let Ok( card ) = Cards::parse_in_hand( my_discard.card.get(), hand, suit ) {
                            if hand.contains( card ) {
                                let name = card.text( FOR_TK );
                                if let Some( btn ) = deck.get( &*name ) {
                                    if btn.winfo_viewable()? {
                                        btn.invoke()?;
                                    }
                                }
                            }
                        }
                        kb_input.clear();
                    } else {
                        hand.text( FOR_TK ).split(' ').try_for_each( |card| unselect_card( &deck, card ))?;
                        if round_cnt.get() == 0 {
                            match Cards::parse_in_hand( &input, hand, suit ) {
                                Ok( cards ) | Err( ParseError::Ambiguous{ cards, ok:_, err:_ }) => {
                                    my_passes.clear()?;
                                    cards.iter().try_for_each( |card| {
                                        if hand.contains( card ) {
                                            my_passes.insert( card.text( FOR_TK ))?;
                                        }
                                        InterpResult::Ok(())
                                    })?;
                                }
                                _ => kb_input.clear(),
                            }
                        } else {
                            while !kb_input.is_empty() {
                                let repeat = kb_input.repeat_cnt();
                                let input = kb_input.trim_end_to_string();
                                match Cards::parse_in_hand( &input, hand, suit ) {
                                    Ok( cards ) => {
                                        if cards.count() == 1 {
                                            let card = cards;
                                            if hand.contains( card ) {
                                                my_discard.select( &card.text( FOR_TK ))?;
                                                if repeat > 0 { kb_input.trim_end(); }
                                                break;
                                            }
                                        } else {
                                            kb_input.trim_start();
                                            continue;
                                        }
                                    }
                                    Err( ParseError::Ambiguous{ cards, ok:_, err:_ }) => {
                                        let card = cards.iter().cycle().skip( repeat ).next().unwrap();
                                        if hand.contains( card ) {
                                            my_discard.select( &card.text( FOR_TK ))?;
                                            if kb_input.trim_start() {
                                                continue;
                                            } else {
                                                break;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                kb_input.clear();
                                break;
                            }
                        }
                    }
                }
                let input = kb_input.0.borrow();
                if input.is_empty() {
                    kb_input_lb.place_forget()?;
                } else {
                    kb_input_lb.place( -relx(0.5) -rely(0.72) )?.configure( -text(input.as_str()) )?;
                }
            }
            Ok(())
        })
    )?;

    let get_hint = bind!((hint_cards) move |line: &str| {
        if let Some( pos ) = line.find( "e.g." ) {
            hint_cards.set( line[ pos+4.. ].trim().parse().unwrap() );
        }
    });

    tk.run( tclosure!( tk, cmd:"poll_received", bind:(game_io,quit_msg,round_winner,the_state), || {
        match the_state.get() {
            State::Welcome => if let Some( welcome ) = game_io.try_recv_err() {
                SCORING_CARDS.iter().enumerate().try_for_each( |(nth, card)| {
                    deck[ card ].place( -relx(0.15) -rely(0.01+0.05*(nth as f64)) )?;
                    Ok(())
                })?;
                welcome_lb.place( -relx(0.27) -rely(0.00) )?
                    .configure( -text(welcome) -justify("left") )?;

                start_btn.place( -relx(0.48) -rely(0.75) )?;
                game_io.recv_line(); // Press enter to start...
                the_state.set( State::WaitForStart );
            }
            State::Dealing => if animations.is_empty() {
                for i in 0..13 {
                    card_back_btns[i+ 0].place( -relx(0.14) -rely(0.20+0.03*(i as f64)) )?; // LEFT
                    card_back_btns[i+13].place( -relx(0.34+0.02*(i as f64)) -rely(0.01) )?; // OPPOSITE
                    card_back_btns[i+26].place( -relx(0.79) -rely(0.20+0.03*(i as f64)) )?; // RIGHT
                }

                score_lbs[ ME       ].place( -relx(0.51) -rely(0.51) )?;
                score_lbs[ LEFT     ].place( -relx(0.47) -rely(0.46) )?;
                score_lbs[ OPPOSITE ].place( -relx(0.51) -rely(0.41) )?;
                score_lbs[ RIGHT    ].place( -relx(0.55) -rely(0.46) )?;

                the_state.set( State::Dealed );
            }
            State::Dealed => if let Some( hand_line ) = game_io.try_recv_line() {
                round_cnt.set( 0 );

                if hand_line.starts_with('=') {
                    my_hand.set( hand_line[1..].trim().parse().unwrap() );
                    hand_line[1..].trim().split_whitespace().enumerate()
                        .try_for_each( |(nth, card)| {
                            deck[ card ].place( -relx(0.34+0.02*(nth as f64)) -rely(0.80) )?;
                            Ok(())
                        })?;
                } else {
                    panic!("The line \"{hand_line}\" should starts with '='.");
                }

                let pass_hint_line = game_io.recv_line();
                if pass_hint_line.starts_with("Pass 3 cards to ") {
                    pass_btn.place( -relx(0.44) -rely(0.65) )?
                        .configure( -state("disabled") )?;
                    if let Some( pos ) = pass_hint_line.find( ',' ) {
                        pass_btn.configure( -text(&pass_hint_line[..pos]) )?;
                    }
                    get_hint( &pass_hint_line );
                    the_state.set( State::WaitForPassing );
                } else {
                    the_suits.set( CLUBS );
                    round_cnt.set( 1 );
                    if pass_hint_line.starts_with("Do not need to pass cards. Discard 1 card") {
                        get_hint( &pass_hint_line );
                        the_state.set( State::WaitForDiscarding );
                    } else if pass_hint_line.starts_with("Do not need to pass cards") {
                        the_state.set( State::Discarding );
                    }
                }
            }
            State::Passing => if let Some( line ) = game_io.try_recv_line() {
                if line.starts_with( "Received " ) {
                    *my_recvs.borrow_mut() = line[9..].trim().to_owned();
                    the_state.set( State::Received );
                }
            }
            State::Received => if let Some( line ) = game_io.peek_line() {
                if line.starts_with( '=' ) {
                    my_passes.forget()?;
                    line[1..].trim().split_whitespace().enumerate()
                        .try_for_each( |(nth, card)| {
                            let x = 0.34+0.02*(nth as f64);
                            let y = if my_recvs.borrow().contains( card ) {0.77} else {0.80};
                            deck[ &*card ].place( -relx(x) -rely(y) )?.configure( -state("disabled") )?;
                            Ok(())
                        })?;
                    my_hand.set( line[1..].trim().parse().unwrap() );
                    the_state.set( State::ToAccept );
                }
            }
            State::ToAccept => {
                acpt_btn.place( -relx(0.48) -rely(0.65) )?;
                if automatic {
                    acpt_btn.invoke()?;
                }
            }
            State::Discarding => {
                if let Some( received ) = game_io.try_recv_err() {
                    if received.chars().next() != Some('\n') && received != "Press enter to start...\n" {
                        tk.message_box( -type_("ok") -message(received) )?;
                    } // skip ASCII hearts
                }
                if let Some( received ) = game_io.try_recv_lines(8) {
                    let mut lines = received.lines();

                    let mut discarding_cards = ["","","",""];
                    let mut win = NOBODY;
                    let mut gun = NOBODY;

                    let splitter = lines.next().unwrap();
                    let (round_is_pending, round);

                    match splitter.find('.') {
                        Some( offset ) => {
                            round_is_pending = false;
                            round = offset / 2 + 1;
                        },
                        None => {
                            round_is_pending = true;
                            round = splitter.find("  -").unwrap_or(24) / 2 + 1;
                        },
                    }
                    round_cnt.set( round );

                    if round_is_pending {
                        for _ in 0..4 { lines.next(); } // skip 3 score lines and 1 empty line
                    } else {
                        let opposite_scores = lines.next().unwrap().trim();
                        let mut left_right_scores = lines.next().unwrap().trim().split_whitespace();
                        let left_scores = left_right_scores.next().unwrap();
                        let right_scores = left_right_scores.next().unwrap();
                        let my_scores = lines.next().unwrap().trim();

                        score_lbs[ ME       ].configure( -text(my_scores)       )?;
                        score_lbs[ LEFT     ].configure( -text(left_scores)     )?;
                        score_lbs[ OPPOSITE ].configure( -text(opposite_scores) )?;
                        score_lbs[ RIGHT    ].configure( -text(right_scores)    )?;

                        lines.next(); // skip the empty line between scores and discarding cards
                    }

                    set_discarding_cards( &mut lines, &mut discarding_cards, &mut gun, &mut win );
                    under_the_gun.set( gun );
                    round_winner.set( win );
                    if win == ME {
                        the_suits.set( NO_CARD );
                    }

                    for i in (gun..gun+4).rev() {
                        let who = i%4;
                        if who != ME {
                            if !discarding_cards[who].is_empty() {
                                let to_discard = card_back_btns[ 13*(who-1) + DISCARD_ORDER[ round-1 ]];
                                animations.discard( who, discarding_cards[who], to_discard, tk )?;
                                break;
                            }
                        }
                    }

                    if round_is_pending {
                        the_suits.set( discarding_cards[gun].parse::<Cards>().unwrap().suit_mask() );
                    } else {
                        discarding_cards.rotate_left( gun );
                        discarding_cards.iter().enumerate()
                            .for_each( |(nth, card)| animations.hide( card, /*None,*/ nth != 3 ));
                    }
                    the_state.set( State::Obey );
                }
            }
            State::Obey => if let Some( received ) = game_io.peek_lines(2) {
                let mut lines = received.lines();

                let hand_line = lines.next().unwrap();
                if hand_line.starts_with('=') && hand_line != "= " {
                    my_hand.set( hand_line[1..].trim().parse().unwrap() );
                }

                let cmd = lines.next().unwrap();
                let discarding = cmd.starts_with("discarding...");

                if discarding {
                    let round_cnt = if discarding {round_cnt.get()} else {0};
                    hand_line[1..].trim().split_whitespace().enumerate().try_for_each( |(nth, card)| {
                        let x = 0.34+0.02*( ( nth + (round_cnt+1)/2 ) as f64 );
                        let y = 0.80;
                        deck[ &*card ].place( -relx(x) -rely(y) )?.configure( -state("normal") )?;
                        Ok(())
                    })?;

                    game_io.recv_lines(2);
                    the_state.set( State::Discarding );
                } else if cmd.starts_with("Discard 1 card") {
                    game_io.recv_lines(2);
                    get_hint( cmd );
                    the_state.set( State::WaitForDiscarding );
                } else if cmd.starts_with("Pass 3 cards to ") || cmd.starts_with("Do not need to pass cards") {
                    the_state.set( State::Dealing );
                } else if cmd.starts_with("Press enter to start...") {
                    the_state.set( State::CheckWinner );
                } else if cmd.starts_with("Statistics:") {
                    the_state.set( State::CheckWinner );
                    let pos = cmd.find(':').unwrap();
                    *quit_msg.borrow_mut() = Some( cmd[pos+1..].to_owned() );
                } else {
                    panic!( "Don't know how to obey `{cmd}`" );
                }
            }
            State::Discarded => {
                if !my_discard.is_empty() {
                    let card = my_discard.to_string();
                    game_io.send_line( &card );
                    my_discard.clear();
                }
                the_state.set( State::Discarding );
            }
            State::CheckWinner => {
                let received = game_io.recv_lines(2);
                let mut lines = received.lines();
                let first_line = lines.next().unwrap();
                let _ = lines.next().unwrap();
                let the_winner_is = "The winner is ";
                assert!( first_line.starts_with( the_winner_is ));

                let winner = match first_line[ the_winner_is.len().. ].trim_end_matches('!') {
                    "me"                   => ME,
                    "the left player"      => LEFT,
                    "the opposite player"  => OPPOSITE,
                    "the right player"     => RIGHT,
                    winner                 => panic!("{winner}"),
                };

                const MSG: [&'static str; 4] = [
                    "You won the game!",
                    "The left player won the game!",
                    "The opposite player won the game!",
                    "The right player won the game!",
                ];

                if !automatic {
                    tk.message_box( -type_("ok") -message(MSG[winner]) )?;
                }
                if winner == ME {
                    the_state.set( State::ShowWinner );
                } else if quit_msg.borrow().is_some() {
                    the_state.set( State::Exit );
                } else {
                    game_io.send_line("");
                    the_state.set( State::Dealing );
                }
            }
            State::ShowWinner => {
                score_lbs[ ME       ].place_forget()?;
                score_lbs[ LEFT     ].place_forget()?;
                score_lbs[ OPPOSITE ].place_forget()?;
                score_lbs[ RIGHT    ].place_forget()?;
                show_winner_animation()?;
            }
            State::WaitForStart => if automatic && animations.is_empty() {
                start_btn.invoke()?;
            }
            State::WaitForPassing => if automatic && animations.is_empty() {
                hint_cards.get().text( FOR_TK ).split(' ').try_for_each( |card| my_passes.insert( card.to_owned() ))?;
                pass_btn.invoke()?;
            }
            State::WaitForDiscarding => if automatic && animations.is_empty() {
                let card = deck[ &*hint_cards.get().text( FOR_TK )];
                card.invoke()?; card.invoke()?;
            }
            State::Exit => {
                if !automatic {
                    tk.message_box( -type_("ok") -message(quit_msg.borrow().as_ref().unwrap().as_str()) )?;
                }
                return tk.destroy(( ".", ));
            }
        }

        animate()?;
        tk.after( 100, ("poll_received",) )?; // 10 fps ought to be enough for anybody
        Ok(())
    }))?;

    Ok( main_loop() )
}
