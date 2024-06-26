use crate::card::Card;
use crate::deck::Deck;

pub fn deal_hands(deck: &mut Deck, num_players: usize) -> (Vec<Vec<Card>>, Vec<Card>) {
    let mut hands = Vec::with_capacity(num_players);
    for _ in 0..num_players {
        hands.push(deck.deal(2));
    }
    let community_cards = deck.deal(5);
    (hands, community_cards)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::deck::Deck;

    use super::*;

    #[test]
    fn deal_hands_should_deal_correct_hands_number_matching_players() {
        let mut deck = Deck::new();
        deck.shuffle();
        let num_players = 4;
        let (hands, _) = deal_hands(&mut deck, num_players);

        assert_eq!(hands.len(), num_players);
        for hand in hands {
            assert_eq!(hand.len(), 2);
        }
    }

    #[test]
    fn deal_hands_should_deal_correct_number_of_community_cards() {
        let mut deck = Deck::new();
        deck.shuffle();
        let num_players = 4;
        let (_, community_cards) = deal_hands(&mut deck, num_players);

        assert_eq!(community_cards.len(), 5);
    }

    #[test]
    fn deal_hands_should_change_deck_size_after_deal_hands() {
        let mut deck = Deck::new();
        deck.shuffle();
        let num_players = 4;
        let total_dealt_cards = num_players * 2 + 5; // 2 cards per player + 5 community cards
        let initial_deck_size = deck.cards.len();
        deal_hands(&mut deck, num_players);
        let remaining_deck_size = deck.cards.len();

        assert_eq!(initial_deck_size - total_dealt_cards, remaining_deck_size);
    }

    #[test]
    fn test_dealt_cards_are_unique() {
        let mut deck = Deck::new();
        deck.shuffle();
        let num_players = 4;
        let (hands, community_cards) = deal_hands(&mut deck, num_players);

        let mut dealt_cards = HashSet::new();
        for hand in hands {
            for card in hand {
                assert!(dealt_cards.insert(card));
            }
        }
        for card in community_cards {
            assert!(dealt_cards.insert(card));
        }
    }
}
