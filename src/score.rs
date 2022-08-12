#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WordleScore {
	// TODO: should there be a depth marker in this?
	// it would have to be incremented whenever it is returned
	pub turn_num: u8,
	pub num_remaining_words: u16,
	// pub avg_clue_score: f32
}

impl WordleScore {
	pub const MIN: Self = Self {
		turn_num: u8::MAX,
		num_remaining_words: u16::MAX, 
	};
	pub const MAX: Self = Self {
		turn_num: 0,
		num_remaining_words: u16::MIN, 
	}; 
}

impl PartialOrd for WordleScore {
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		// TODO: this looks cancerous
		match self.num_remaining_words.cmp(&other.num_remaining_words) {
			std::cmp::Ordering::Equal => {
				match self.turn_num.cmp(&other.turn_num) {
					std::cmp::Ordering::Equal => Some(std::cmp::Ordering::Equal),
					std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Less),
					std::cmp::Ordering::Less => Some(std::cmp::Ordering::Greater)
				}
			},
			std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Less),
			std::cmp::Ordering::Less => Some(std::cmp::Ordering::Greater),
		}
	}
}