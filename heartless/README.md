Heartless, a card game inspired by mshearts.

It is running in console. To run in graphic mode, you can use a
[GUI frontend](../heartless_tk/README.md).

# Rule

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

## Pass phase

Before discarding cards, players have to pass along 3 cards to another player.

* In the 1st hand, each player passes cards to the opponent on the left.
* In the 2nd hand, each player passes cards to the opponent on the right.
* In the 3rd hand, each player passes cards to the opponent across the table.
* In the 4th hand, players do not pass cards.

And so forth.

# Card representation

A single card is in the form of rank + suit, e.g. "2♣".

Cards in hand are sorted from 2~A, grouped in firstly CLUBS then DIAMONDS then
SPADES finally HEARTS. Cards of the same suit will be put together, ends with
their suit, e.g. "467QA♣" means "4♣ 6♣ 7♣ Q♣ A♣".

Any number of whitespaces can be inserted anywhere and completely ignored.

If you'd like see all text in ASCII, you can use the "--ascii-suit" commandline
argument and the first letter of suits in lowercase c,d,s,h will be show
displayed as the suits. For example, "467QA♣" will be displayed as "467QAc",
which is "4c 6c 7c Qc Ac".

# User input

When you want to pick card(s) to pass/discard, just type the representation of
the card(s), then press "Enter" to submit. Use c,d,s,h to encode the suits.

The user input in heartless is designed to be user friendly. It's
case-insensitive, and the suit can be omitted in the following cases:

1. when following the suit, which has been already specified by early hand.

2. The suit in round 1 is always CLUBS.

3. when not following the suit, but the cards in hand contains only one card
    that is of the specified rank.

To quit the game, just type "exit".

# Customizable AI strategies

Sometimes the users may feel that the built-in AI strategies are too simple,
and dealing with hit-the-moon naively. If you got better idea, just implement it
in your favourite language. The "scripts/" folder provides an example of Python
script named "replica.py", which is the built-in AI replica.

With "--all", "--others", "--me", "--left", "--opposite", "--right" commandline
arguments provided, the heartless program will invoke your script repeatly, in
pass phase and each round. The following arguments will be sent:

* `--who`

Who the script is. If your script is invoked by "--left your.script", "who" will
be 1. And 0 for "--me", 2 for "--opposite", 3 for "--right". "--all" will
run the script 4 times, with different "who" of 0,1,2,3, and "--others" with
1,2,3.

* `--hand`

The cards in who's hand. It's a card representation with lowercase suit and
without any whitespaces.

* `--discarded`

All discarded cards so far. It's a card representation with lowercase suit and
without any whitespaces.

* `--suit_to_follow`

The suit to follow in current round.

* `--round`

Which round it is, 0 for pass phase and 1~13 for discard phase.

* `--under_the_gun`

Who discard the first card in this round.

* `--round_winner`

Whose card is the highest card in this round.

* `--high_card`

Which card is the highest card in this round.

* `--my_discarding`

Which card the player ME is discarding, with lowercase suit.

* `--my_hand_score`

How many points out of 26 the player ME scored.

* `--my_game_score`

How many points (up to 99) the player ME scored.

* `--my_suit_chars`

Which suits the player ME may have, e.g. a "cdh" means Me has no SPADES, may
have CLUBS, DIAMONDS and HEARTS.

* `--left_discarding`

Which card the left player is discarding, with lowercase suit.

* `--left_hand_score`

How many points out of 26 the left player scored.

* `--left_game_score`

How many points (up to 99) the left player scored.

* `--left_suit_chars`

Which suits the left player may have, e.g. a "cdh" means Me has no SPADES, may
have CLUBS, DIAMONDS and HEARTS.

* `--opposite_discarding`

Which card the opposite player is discarding, with lowercase suit.

* `--opposite_hand_score`

How many points out of 26 the opposite player scored.

* `--opposite_game_score`

How many points (up to 99) the opposite player scored.

* `--opposite_suit_chars`

Which suits the opposite player may have, e.g. a "cdh" means Me has no SPADES,
may have CLUBS, DIAMONDS and HEARTS.

* `--right_discarding`

Which card the right player is discarding, with lowercase suit.

* `--right_hand_score`

How many points out of 26 the right player scored.

* `--right_game_score`

How many points (up to 99) the right player scored.

* `--right_suit_chars`

Which suits the right player may have, e.g. a "cdh" means Me has no SPADES, may
have CLUBS, DIAMONDS and HEARTS.

## Write customizable AI strategies in Rust

The library users can customize the strategies by providing their functions
written in Rust.

```rust
use heartless::*;
fn your_ai( game: &Game, who: usize ) -> Cards { todo!() }

let mut game = Game::default();
game.strategies.set_for_others( your_ai );
game.main_loop();
```

## Automatic mode

Given "--automatic", the game will run in automatic mode and the user has no
interaction with the game at all. You can enable this mode to test your AI.
With "--count 100" you can run 100 times to see how many times your AI won.

# License

Under Apache License 2.0 or MIT License, at your will.
