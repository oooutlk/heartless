The `heartless_tk` program is a card game similar with mshearts.

It is a GUI frontend of [heartless](../heartless/README.md), using the
[Tk](https://crates.io/crates/tk) as the UI client.

# Rule

The same as [heartless's Rule](../heartless/README.md#rule).

# User input

You can select and discard cards via mouse clicking or keyboard striking.

For mouse clicking, the first click selects the card and the second click
discards it.

Keyboard input is almost the same as
[heartless's Input](../heartless/README.md#input). You can strike 2,3,4,5,6,7,
8,9,0,j,q,k,a, and Esc, Backspace, Enter for keyboard input. Note that jqka are
lowercases.

To alternate selecting between different cards of the same rank, just type the
same rank repeatly. For instance, typing "jj" will pick Jack of Diamonds out of
"236Jc 50Jd JsQs", while typing "jjj" will pick Jack of Spades.

# Customizable AI strategies

The same as
[heartless's Customizable AI strategies](../heartless/README.md#customizable-ai-strategies).

## Automatic mode

Given "--automatic", the game will run in automatic mode and the user has no
interaction with the game at all. You can enable this mode to test your AI.
With "--count 100" you can run 100 times to see how many times your AI won.

Note that the `heartless` server is not in automatic mode, it gets input from
the `heartless_tk` client which is automated.

# License

Under Apache License 2.0 or MIT License, at your will.
