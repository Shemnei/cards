use std::fmt;
use std::fmt::Write as _;

use crate::rank::Rank;
use crate::suit::Suit;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card(Suit, Rank);

impl Card {
	pub const CLUB_ACE: Self = Self(Suit::Club, Rank::Ace);
	pub const CLUB_EIGHT: Self = Self(Suit::Club, Rank::Eight);
	pub const CLUB_FIVE: Self = Self(Suit::Club, Rank::Five);
	pub const CLUB_FOUR: Self = Self(Suit::Club, Rank::Four);
	pub const CLUB_JACK: Self = Self(Suit::Club, Rank::Jack);
	pub const CLUB_KING: Self = Self(Suit::Club, Rank::King);
	pub const CLUB_NINE: Self = Self(Suit::Club, Rank::Nine);
	pub const CLUB_QUEEN: Self = Self(Suit::Club, Rank::Queen);
	pub const CLUB_SEVEN: Self = Self(Suit::Club, Rank::Seven);
	pub const CLUB_SIX: Self = Self(Suit::Club, Rank::Six);
	pub const CLUB_TEN: Self = Self(Suit::Club, Rank::Ten);
	pub const CLUB_THREE: Self = Self(Suit::Club, Rank::Three);
	pub const CLUB_TWO: Self = Self(Suit::Club, Rank::Two);
	pub const DIAMOND_ACE: Self = Self(Suit::Diamond, Rank::Ace);
	pub const DIAMOND_EIGHT: Self = Self(Suit::Diamond, Rank::Eight);
	pub const DIAMOND_FIVE: Self = Self(Suit::Diamond, Rank::Five);
	pub const DIAMOND_FOUR: Self = Self(Suit::Diamond, Rank::Four);
	pub const DIAMOND_JACK: Self = Self(Suit::Diamond, Rank::Jack);
	pub const DIAMOND_KING: Self = Self(Suit::Diamond, Rank::King);
	pub const DIAMOND_NINE: Self = Self(Suit::Diamond, Rank::Nine);
	pub const DIAMOND_QUEEN: Self = Self(Suit::Diamond, Rank::Queen);
	pub const DIAMOND_SEVEN: Self = Self(Suit::Diamond, Rank::Seven);
	pub const DIAMOND_SIX: Self = Self(Suit::Diamond, Rank::Six);
	pub const DIAMOND_TEN: Self = Self(Suit::Diamond, Rank::Ten);
	pub const DIAMOND_THREE: Self = Self(Suit::Diamond, Rank::Three);
	pub const DIAMOND_TWO: Self = Self(Suit::Diamond, Rank::Two);
	pub const HEART_ACE: Self = Self(Suit::Heart, Rank::Ace);
	pub const HEART_EIGHT: Self = Self(Suit::Heart, Rank::Eight);
	pub const HEART_FIVE: Self = Self(Suit::Heart, Rank::Five);
	pub const HEART_FOUR: Self = Self(Suit::Heart, Rank::Four);
	pub const HEART_JACK: Self = Self(Suit::Heart, Rank::Jack);
	pub const HEART_KING: Self = Self(Suit::Heart, Rank::King);
	pub const HEART_NINE: Self = Self(Suit::Heart, Rank::Nine);
	pub const HEART_QUEEN: Self = Self(Suit::Heart, Rank::Queen);
	pub const HEART_SEVEN: Self = Self(Suit::Heart, Rank::Seven);
	pub const HEART_SIX: Self = Self(Suit::Heart, Rank::Six);
	pub const HEART_TEN: Self = Self(Suit::Heart, Rank::Ten);
	pub const HEART_THREE: Self = Self(Suit::Heart, Rank::Three);
	pub const HEART_TWO: Self = Self(Suit::Heart, Rank::Two);
	pub const SPADE_ACE: Self = Self(Suit::Spade, Rank::Ace);
	pub const SPADE_EIGHT: Self = Self(Suit::Spade, Rank::Eight);
	pub const SPADE_FIVE: Self = Self(Suit::Spade, Rank::Five);
	pub const SPADE_FOUR: Self = Self(Suit::Spade, Rank::Four);
	pub const SPADE_JACK: Self = Self(Suit::Spade, Rank::Jack);
	pub const SPADE_KING: Self = Self(Suit::Spade, Rank::King);
	pub const SPADE_NINE: Self = Self(Suit::Spade, Rank::Nine);
	pub const SPADE_QUEEN: Self = Self(Suit::Spade, Rank::Queen);
	pub const SPADE_SEVEN: Self = Self(Suit::Spade, Rank::Seven);
	pub const SPADE_SIX: Self = Self(Suit::Spade, Rank::Six);
	pub const SPADE_TEN: Self = Self(Suit::Spade, Rank::Ten);
	pub const SPADE_THREE: Self = Self(Suit::Spade, Rank::Three);
	pub const SPADE_TWO: Self = Self(Suit::Spade, Rank::Two);

	pub const fn suit(&self) -> &Suit {
		&self.0
	}

	pub const fn rank(&self) -> &Rank {
		&self.1
	}

	pub fn ascii_display(&self) -> AsciiDisplay<'_> {
		AsciiDisplay(self)
	}

	pub fn unicode_display(&self) -> UnicodeDisplay<'_> {
		UnicodeDisplay(self)
	}

	pub fn unicode_card_display(&self) -> UnicodeCardDisplay<'_> {
		UnicodeCardDisplay(self)
	}
}

pub struct AsciiDisplay<'a>(&'a Card);

impl fmt::Display for AsciiDisplay<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}{}", self.0.rank(), self.0.suit().ascii_display())
	}
}

pub struct UnicodeDisplay<'a>(&'a Card);

impl fmt::Display for UnicodeDisplay<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}{}", self.0.rank(), self.0.suit().unicode_display())
	}
}

pub struct UnicodeCardDisplay<'a>(&'a Card);

impl fmt::Display for UnicodeCardDisplay<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let base = match self.0.suit() {
			Suit::Club => '\u{1F0A1}',
			Suit::Diamond => '\u{1F0B1}',
			Suit::Heart => '\u{1F0C1}',
			Suit::Spade => '\u{1F0D1}',
		};

		let mut add = *self.0.rank() as u8 as u32;
		if self.0.rank() > &Rank::Jack {
			add += 1;
		}

		f.write_char(char::from_u32(base as u32 + add).unwrap())
	}
}
