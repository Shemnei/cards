use std::fmt;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rank {
	Ace,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
}

impl fmt::Display for Rank {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Rank::Ace => f.write_str("A"),
			Rank::Two => f.write_str("2"),
			Rank::Three => f.write_str("3"),
			Rank::Four => f.write_str("4"),
			Rank::Five => f.write_str("5"),
			Rank::Six => f.write_str("6"),
			Rank::Seven => f.write_str("7"),
			Rank::Eight => f.write_str("8"),
			Rank::Nine => f.write_str("9"),
			Rank::Ten => f.write_str("10"),
			Rank::Jack => f.write_str("J"),
			Rank::Queen => f.write_str("Q"),
			Rank::King => f.write_str("K"),
		}
	}
}
