#!/usr/bin/env python3

import getopt
import os
import sys

long_opts = [
    'who=',
    'hand=',
    'discarded=',
    'suit_to_follow=',
    'round=',
    'under_the_gun=',
    'round_winner=',
    'high_card=',
    'my_discarding=',
    'my_hand_score=',
    'my_game_score=',
    'my_suit_chars=',
    'left_discarding=',
    'left_hand_score=',
    'left_game_score=',
    'left_suit_chars=',
    'opposite_discarding=',
    'opposite_hand_score=',
    'opposite_game_score=',
    'opposite_suit_chars=',
    'right_discarding=',
    'right_hand_score=',
    'right_game_score=',
    'right_suit_chars=',
]

opts, _ = getopt.getopt(sys.argv[1:], '', long_opts )

def extract_cards (packed):
    rev_packed = reversed(packed)
    extracted = []
    for c in rev_packed:
        if c == 'c':
            suit = 'club'
        elif c == 'd':
            suit = 'diamond'
        elif c == 's':
            suit = 'spade'
        elif c == 'h':
            suit = 'heart'
        elif c == '2':
            extracted.append(( 2,suit))
        elif c == '3':
            extracted.append(( 3,suit))
        elif c == '4':
            extracted.append(( 4,suit))
        elif c == '5':
            extracted.append(( 5,suit))
        elif c == '6':
            extracted.append(( 6,suit))
        elif c == '7':
            extracted.append(( 7,suit))
        elif c == '8':
            extracted.append(( 8,suit))
        elif c == '9':
            extracted.append(( 9,suit))
        elif c == '0':
            extracted.append((10,suit))
        elif c == 'J':
            extracted.append((11,suit))
        elif c == 'Q':
            extracted.append((12,suit))
        elif c == 'K':
            extracted.append((13,suit))
        elif c == 'A':
            extracted.append((14,suit))
        else:
            print(f'Invalid cards string "{packed}"', file=sys.stderr)
            exit()
    return extracted

def extract_suits (packed):
    extracted = []
    if 'c' in packed:
        extracted.append('club')
    if 'd' in packed:
        extracted.append('diamond')
    if 's' in packed:
        extracted.append('spade')
    if 'h' in packed:
        extracted.append('heart')
    return extracted

discardings     = [None,None,None,None]
hand_scores     = [None,None,None,None]
game_scores     = [None,None,None,None]
may_have_suits  = [None,None,None,None]

for opt,val in opts:
    if opt == '--who':
        who = int(val)
    elif opt == '--hand':
        hand = extract_cards(val)
    elif opt == '--discarded':
        discarded = extract_cards(val)
    elif opt == '--suit_to_follow':
        suit_to_follow = val
    elif opt == '--round':
        round_ = int(val)
    elif opt == '--under_the_gun':
        under_the_gun = int(val)
    elif opt == '--round_winner':
        round_winner = int(val)
    elif opt == '--high_card':
        high_card = extract_cards(val)
    elif opt == '--my_discarding':
        discardings[0] = extract_cards(val)
    elif opt == '--my_hand_score':
        hand_scores[0] = int(val)
    elif opt == '--my_game_score':
        game_scores[0] = int(val)
    elif opt == '--my_suit_chars':
        may_have_suits[0] = extract_suits(val)
    elif opt == '--left_discarding':
        discardings[1] = extract_cards(val)
    elif opt == '--left_hand_score':
        hand_scores[1] = int(val)
    elif opt == '--left_game_score':
        game_scores[1] = int(val)
    elif opt == '--left_suit_chars':
        may_have_suits[1] = extract_suits(val)
    elif opt == '--opposite_discarding':
        discardings[2] = extract_cards(val)
    elif opt == '--opposite_hand_score':
        hand_scores[2] = int(val)
    elif opt == '--opposite_game_score':
        game_scores[2] = int(val)
    elif opt == '--opposite_suit_chars':
        may_have_suits[2] = extract_suits(val)
    elif opt == '--right_discarding':
        discardings[3] = extract_cards(val)
    elif opt == '--right_hand_score':
        hand_scores[3] = int(val)
    elif opt == '--right_game_score':
        game_scores[3] = int(val)
    elif opt == '--right_suit_chars':
        may_have_suits[3] = extract_suits(val)

staggerer = os.getenv('HEARTLESS_STAGGERER')
if staggerer == str(who) or staggerer == '0':
    import time
    time.sleep(1)

rank_chars = ['2','3','4','5','6','7','8','9','0','J','Q','K','A']
suit_chars = ['c','d','s','h']
suit_names = ['club','diamond','spade','heart']

def card_chars(card):
    rank, suit = card
    if rank in range(2,15):
        rank_char = rank_chars[rank-2]
    else:
        print(f'Invalid rank {rank}', file=sys.stderr)
    if suit in suit_names:
        idx = suit_names.index(suit)
        suit_char = suit_chars[idx]
    else:
        print(f'Invalid suit name "{suit}"', file=sys.stderr)
    return rank_char + suit_char

def output_card(card):
    print(card_chars(card))

def output_cards(cards):
    output = ''
    for card in cards:
        output = output + card_chars(card)
    print(output)

def count_suit (hand,suit):
    return list(map(lambda rank_suit: rank_suit[1], hand)).count(suit)

def value_of_passing_card(card):
    if card == (12,'spade'):
        return -14*4-4 # Queen of Spades is the first card to pass
    else:
        rank, suit = card
        return -rank*4-suit_names.index(suit)

def value_of_discarding_card(card):
    rank, suit = card
    if round_ == 1 and rank == 12 and suit == 'spade':
        return 0 # don't discard Queen of Spades in round 1.
    if suit == 'heart':
        return -rank
    elif suit == 'club':
        return -rank*4-1-16
    elif suit == 'diamond':
        return -rank*4-2-16
    elif suit == 'spade':
        return -rank*4-3-16
    print(f'Invalid suit name "{suit}"', file=sys.stderr)

def is_qka_of_spades(card):
    rank, suit = card
    return suit == 'spade' and 12 <= rank and rank <=14

def heart_broken(discarded):
    return any(map(lambda rank_suit: rank_suit[1] == 'heart', discarded))

def cards_of_suit(hand,suit):
    return list(filter(lambda rank_suit: rank_suit[1] == suit, hand))

def contains_suit(hand,suit):
    return any(map(lambda rank_suit: rank_suit[1] == suit, hand))

def contains(hand,card):
    return any(map(lambda card_in_hand: card_in_hand == card, hand))

if round_ == 0: # passing three cards
    hand.sort(key=lambda card: value_of_passing_card(card))
    output_cards(hand[0:3])
else: # discard one card
    if under_the_gun == who: # early hand
        if round_ == 1:
            output_card((2,'club'))
            exit()
        suits_cnts = [
            ( 'club'   , count_suit(hand, 'club'   )),
            ( 'diamond', count_suit(hand, 'diamond')),
            ( 'spade'  , 14 if any(map(is_qka_of_spades, hand)) else 1),
            ( 'heart'  , count_suit(hand, 'heart') if heart_broken(discarded) else 15),
        ];
        suits_cnts.sort(key=lambda suit_cnt: suit_cnt[1]); # prefer discarding short suits than long suits

        for suit, cnts in suits_cnts:
            cards = cards_of_suit(hand,suit)
            cards.sort(key=lambda rank_suit: rank_suit[0])
            if len(cards) > 0:
                output_card(cards[0])
                exit()
    elif contains_suit(hand,suit_to_follow): # follow suit
        cards = cards_of_suit(hand,suit_to_follow)
        cards.sort(key=lambda rank_suit: -rank_suit[0])
        high_rank, _ = high_card[0] if high_card[0] != None else 0
        rank_to_discard = 14
        high_rank_to_discard = 0

        for card in cards:
            rank, _ = card
            if rank < high_rank:
                if suit_to_follow == 'spade' and contains(hand,(12,'spade')) and 12 < high_rank:
                    output_card((12,'spade'))
                    exit()
                output_card(card)
                exit()
            if rank_to_discard >= rank:
                rank_to_discard = rank
                if high_rank_to_discard < rank and suit_to_follow != 'heart' and card != (12, 'spade'):
                    high_rank_to_discard = rank

        late_hand_follows = False
        for i in range(1,3):
            j = (who+i)%4
            if j == under_the_gun:
                break
            if suit_to_follow in may_have_suits[j]:
                late_hand_follows = True

        output_card((rank_to_discard if late_hand_follows or high_rank_to_discard == 0 else high_rank_to_discard, suit_to_follow))
        exit()
    else: # don't follow suit
        if contains(hand,(12,'spade')) and round_ != 1: # try to discard Queen of Spades
            output_card((12,'spade'))
            exit()
        if contains_suit(hand,'heart') and round_ != 1: # try to discard hearts
            cards = cards_of_suit(hand,'heart')
            cards.sort(key=lambda rank_suit: -rank_suit[0])
            output_card(cards[0])
            exit()
        hand.sort(key=lambda card: value_of_discarding_card(card))
        output_card(hand[0])
