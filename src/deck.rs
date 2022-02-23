use rand::prelude::*;

use crate::card::Card;

pub trait Deck {
	#[cfg(feature = "shuffle")]
	fn shuffle(&mut self);

	fn draw(&mut self) -> Option<Card>;

	fn len(&self) -> usize;

	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

pub type Deck52 = ArrayDeck<52>;
#[rustfmt::skip]
pub const DECK_52: Deck52 = Deck52::new([
    Card::CLUB_ACE, Card::CLUB_TWO, Card::CLUB_THREE, Card::CLUB_FOUR,
    Card::CLUB_FIVE, Card::CLUB_SIX, Card::CLUB_SEVEN, Card::CLUB_EIGHT,
    Card::CLUB_NINE, Card::CLUB_TEN, Card::CLUB_JACK, Card::CLUB_QUEEN,
    Card::CLUB_KING,
    Card::DIAMOND_ACE, Card::DIAMOND_TWO, Card::DIAMOND_THREE,
    Card::DIAMOND_FOUR, Card::DIAMOND_FIVE, Card::DIAMOND_SIX,
    Card::DIAMOND_SEVEN, Card::DIAMOND_EIGHT, Card::DIAMOND_NINE,
    Card::DIAMOND_TEN, Card::DIAMOND_JACK, Card::DIAMOND_QUEEN,
    Card::DIAMOND_KING,
    Card::HEART_ACE, Card::HEART_TWO, Card::HEART_THREE, Card::HEART_FOUR,
    Card::HEART_FIVE, Card::HEART_SIX, Card::HEART_SEVEN, Card::HEART_EIGHT,
    Card::HEART_NINE, Card::HEART_TEN, Card::HEART_JACK, Card::HEART_QUEEN,
    Card::HEART_KING,
    Card::SPADE_ACE, Card::SPADE_TWO, Card::SPADE_THREE, Card::SPADE_FOUR,
    Card::SPADE_FIVE, Card::SPADE_SIX, Card::SPADE_SEVEN, Card::SPADE_EIGHT,
    Card::SPADE_NINE, Card::SPADE_TEN, Card::SPADE_JACK, Card::SPADE_QUEEN,
    Card::SPADE_KING,
]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArrayDeck<const SIZE: usize> {
	// index on which the last valid card resides. All cards 0..valid_idx should
	// be valid.
	valid_idx: usize,
	cards: [Card; SIZE],
}

impl<const SIZE: usize> ArrayDeck<SIZE> {
	pub const fn new(cards: [Card; SIZE]) -> Self {
		Self { valid_idx: SIZE, cards }
	}

	pub fn cards(&self) -> Iter<'_> {
		Iter::new(&self.cards[..self.valid_idx])
	}
}

impl<const SIZE: usize> Deck for ArrayDeck<SIZE> {
	#[cfg(feature = "shuffle")]
	fn shuffle(&mut self) {
		self.cards[..self.valid_idx].shuffle(&mut thread_rng())
	}

	fn draw(&mut self) -> Option<Card> {
		if self.is_empty() {
			None
		} else {
			self.valid_idx -= 1;
			Some(self.cards[self.valid_idx])
		}
	}

	fn len(&self) -> usize {
		self.valid_idx
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VecDeck(Vec<Card>);

impl VecDeck {
	pub const fn new(cards: Vec<Card>) -> Self {
		Self(cards)
	}
}

impl Deck for VecDeck {
	#[cfg(feature = "shuffle")]
	fn shuffle(&mut self) {
		self.0.shuffle(&mut thread_rng());
	}

	fn draw(&mut self) -> Option<Card> {
		self.0.pop()
	}

	fn len(&self) -> usize {
		self.0.len()
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Iter<'a> {
	idx: usize,
	cards: &'a [Card],
}

impl<'a> Iter<'a> {
	pub const fn new(cards: &'a [Card]) -> Self {
		Self { idx: 0, cards }
	}
}

impl<'a> Iterator for Iter<'a> {
	type Item = &'a Card;

	fn next(&mut self) -> Option<Self::Item> {
		if self.idx == self.cards.len() {
			None
		} else {
			let card = &self.cards[self.idx];
			self.idx += 1;
			Some(card)
		}
	}
}
