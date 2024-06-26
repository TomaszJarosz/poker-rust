use deck::Deck;
use game::deal_hands;

mod card;
mod deck;
mod game;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    let num_players = 4;

    let (hands, community_cards) = deal_hands(&mut deck, num_players);

    for (i, hand) in hands.iter().enumerate() {
        println!("Player {}: {:?}", i + 1, hand);
    }

    println!("Community Cards: {:?}", community_cards);
}
