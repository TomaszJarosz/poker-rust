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
