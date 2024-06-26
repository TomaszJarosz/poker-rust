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
    use super::*;

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
