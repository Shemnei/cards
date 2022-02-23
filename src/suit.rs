use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Suit {
	Club,
	Diamond,
	Heart,
	Spade,
}

impl Suit {
	pub fn ascii_display(self) -> AsciiDisplay {
		AsciiDisplay(self)
	}

	pub fn unicode_display(self) -> UnicodeDisplay {
		UnicodeDisplay(self)
	}
}

pub struct AsciiDisplay(Suit);

impl fmt::Display for AsciiDisplay {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.0 {
			Suit::Club => f.write_str("C"),
			Suit::Diamond => f.write_str("D"),
			Suit::Heart => f.write_str("H"),
			Suit::Spade => f.write_str("S"),
		}
	}
}

pub struct UnicodeDisplay(Suit);

impl fmt::Display for UnicodeDisplay {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.0 {
			Suit::Club => f.write_str("♣"),
			Suit::Diamond => f.write_str("♦"),
			Suit::Heart => f.write_str("♥"),
			Suit::Spade => f.write_str("♠"),
		}
	}
}
