use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::{Card, Rank, Suit};

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::with_capacity(52);
        for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for &rank in &[
                Rank::Two, Rank::Three, Rank::Four, Rank::Five,
                Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine,
                Rank::Ten, Rank::Jack, Rank::Queen, Rank::King,
                Rank::Ace
            ] {
                cards.push(Card { suit, rank });
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.as_mut_slice().shuffle(&mut rng);
    }

    pub fn deal(&mut self, num_cards: usize) -> Vec<Card> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::card::{Rank, Suit};

    use super::*;

    #[test]
    fn deck_should_have_52_unique_cards() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);

        //check if all are unique
        let mut unique_cards = HashSet::new();
        for card in deck.cards {
            unique_cards.insert(card);
        }
        assert_eq!(unique_cards.len(), 52);
    }

    #[test]
    fn deck_should_contain_all_suits_and_ranks() {
        let deck = Deck::new();

        let mut suit_rank_set = HashSet::new();
        for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for &rank in &[
                Rank::Two, Rank::Three, Rank::Four, Rank::Five,
                Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine,
                Rank::Ten, Rank::Jack, Rank::Queen, Rank::King,
                Rank::Ace
            ] {
                suit_rank_set.insert((suit, rank));
            }
        }

        for card in deck.cards {
            assert!(suit_rank_set.contains(&(card.suit, card.rank)));
        }
    }

    #[test]
    fn shuffle_should_change_order_of_cards() {
        let mut deck = Deck::new();
        let original_deck = deck.cards.clone();

        deck.shuffle();
        assert_ne!(deck.cards, original_deck);
    }

    #[test]
    fn shuffle_should_preserve_all_cards() {
        let mut deck = Deck::new();
        let original_deck = deck.cards.clone();

        deck.shuffle();

        // Check if shuffle preserves all cards
        let mut original_sorted = original_deck.clone();
        original_sorted.sort();
        let mut shuffled_sorted = deck.cards.clone();
        shuffled_sorted.sort();
        assert_eq!(original_sorted, shuffled_sorted);
    }

    #[test]
    fn deal_hand_length_should_match() {
        let mut deck = Deck::new();
        let hand = deck.deal(5);
        assert_eq!(hand.len(), 5);
    }

    #[test]
    fn deck_size_should_change_after_deal() {
        let mut deck = Deck::new();
        deck.deal(5);
        assert_eq!(deck.cards.len(), 47);
    }

    #[test]
    fn dealt_cards_should_be_removed_from_deck() {
        let mut deck = Deck::new();
        let hand = deck.deal(5);
        let remaining_cards = deck.cards.clone();
        for card in hand {
            assert!(!remaining_cards.contains(&card));
        }
    }
}
