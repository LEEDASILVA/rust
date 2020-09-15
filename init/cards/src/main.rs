// Represent cards from a desk
// A standard deck of cards has 52 cards: 4 suits and 13 cards per suit

// Start by creating the `Suit` enum and implement the associated
// function `random` which returns a random `Suit` (`Heart`,
// `Diamond`, `Spade` or `Club`)

// Then create the `Rank` enum that can have the value
// `Ace`, `King`, `Queen`, `Jack`, and `Number` associated to an `u8`
// value to represent the ranks 2 through 10
// After create an associated function to `Rank` called `Random` that
// returns a random `Rank`

// Finally create a structure name `Card` which has the fields `suit`
// and `rank`

// Write a program that takes that returns a random card in the deck
extern crate rand;
use rand::Rng;

#[derive(Debug, PartialEq)]
struct Card {
	suit: Suit,
	rank: Rank
}

#[derive(Debug, PartialEq)]
enum Suit {
	Heart,
	Diamond,
	Spade,
	Club,
}

#[derive(Debug, PartialEq)]
enum Rank {
	Ace,
	King,
	Queen,
	Jack,
	Number(u8),
}

impl Rank {
	fn random() -> Rank {
		let value: u8 = rand::thread_rng().gen_range(1, 14);
		println!("{}", value);
		Rank::translate(value)
	}
	fn translate(value: u8) -> Rank {
		match value {
			1 => Rank::Ace,
			// 2..=10 -> range from 2 to 10
			// for the cards with number
			// @ will save the value in n
			// n @ 2..=10 => Rank::Number(n)
			// this way is better?
			2..=10 => Rank::Number(value),
			11 => Rank::Jack,
			12 => Rank::Queen,
			_ => Rank::King,
		}
	}
}

impl Suit {
	fn random() -> Suit {
		let value = rand::thread_rng().gen_range(1, 5);
		Suit::translate(value)
	}
	fn translate(value: u8) -> Suit {
		match value {
			1 => Suit::Heart,
			2 => Suit::Diamond,
			3 => Suit::Spade,
			_ => Suit::Club,
		}
	}
}

fn winner_card(card: Card) -> bool {
	Card { suit: Suit::Spade, rank: Rank::Ace } == card
}

fn main() {
	for _ in 1..100 {
		let your_card = Card {
			rank: Rank::random(),
			suit: Suit::random(),
		};

		println!("You're card is a {:?}", your_card);

		if winner_card(your_card) {
			println!("You are the winner!");
		}
	}
}
