
pub const WORDLE_NUM_GUESSES: usize = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WordleEntry {
	pub guess: WordleWord, 
	pub clue: WordleClue
}

impl WordleEntry {
	fn matches(&self, possible_answer: WordleAnswer) -> bool {
		// TODO: maybe optimize this?
		get_clues(self.guess, possible_answer) == self.clue
	}
}


#[derive(Default, Clone)]
pub struct WordleState {
	pub answer: Option<WordleAnswer>,
	pub hard_mode: bool,
	current_entry: usize, // "size"
	pub entries: [Option<WordleEntry>; WORDLE_NUM_GUESSES]
}

impl WordleState {
	pub const fn new() -> WordleState {
		WordleState {
			answer: None,
			hard_mode: false,
			current_entry: 0,
			entries: [None; WORDLE_NUM_GUESSES]
		}
	}
	
	pub const fn get_entry(&self, index: usize) -> Option<&WordleEntry> {
		assert!(index < WORDLE_NUM_GUESSES);
		
		self.entries[index].as_ref()
	}
	pub fn get_entry_mut(&mut self, index: usize) -> Option<&mut WordleEntry> {
		assert!(index < WORDLE_NUM_GUESSES);
		
		self.entries[index].as_mut()
	}
	
	pub fn is_solved(&self) -> bool {
		self.current_entry > 0 && self.get_entry(self.current_entry-1).unwrap().clue == WordleClue::SOLVED
	}
	
	pub fn is_lost(&self) -> bool {
		self.current_entry >= WORDLE_NUM_GUESSES && !self.is_solved()
	}
	
	pub fn push_entry(&mut self, entry: WordleEntry) {
		debug_assert!(self.current_entry < WORDLE_NUM_GUESSES);
		
		self.entries[self.current_entry] = Some(entry);
		
		self.current_entry += 1;
	}
	
	pub fn pop_entry(&mut self) -> Option<WordleEntry> {
		if self.current_entry == 0 {
			return None;
		}
		
		self.current_entry -= 1;
		
		self.entries[self.current_entry].take()
	}
	
	pub fn is_possible_answer(&self, possible_answer: WordleAnswer) -> bool {
		if self.current_entry == 0 {
			true
		}
		// a potential optimization that could dramatically improve performance is keeping 
		// track of which answers have been used in previous games before but it changes 
		// what guess you should use and invalidates any previous computational work, makes
		// the algorithm not work for the same answer twice, and it feels SUPER cheaty lol
		/* else if self.hard_mode {
			self.get_entry(self.current_entry-1).unwrap().matches(possible_answer)
		} */ else {
			self.into_iter().all(|entry| entry.matches(possible_answer))
		}
	}
	
	pub fn is_possible_hardmode_word(&self, possible_word: WordleWord) -> bool {
		true // TODO: make this actually correct
	}
	
	pub fn share_text(&self) -> String {
		let mut text = String::new();
		
		text += format!("Wordle {} {}/{}{}\n",
			self.answer.map_or(-1, |answer| answer.day_number() as i32),
			if self.is_lost() {"X".to_string()} else {self.current_entry.to_string()},
			WORDLE_NUM_GUESSES,
			if self.hard_mode {"*"} else {""}
		).as_str();
		
		for i in 0..self.current_entry {
			if let Some(entry) = self.entries[i] {
				let c = entry.clue as u8;
				
				let (q, c) = (c / 81, c % 81);
				text += if q == 0 {"⬜"} else if q == 1 {"🟨"} else {"🟩"};
				
				let (q, c) = (c / 27, c % 27);
				text += if q == 0 {"⬜"} else if q == 1 {"🟨"} else {"🟩"};
				
				let (q, c) = (c / 9, c % 9);
				text += if q == 0 {"⬜"} else if q == 1 {"🟨"} else {"🟩"};
				
				let (q, c) = (c / 3, c % 3);
				text += if q == 0 {"⬜"} else if q == 1 {"🟨"} else {"🟩"};
				text += if c == 0 {"⬜"} else if c == 1 {"🟨"} else {"🟩"};
				
				text += "\n";
			}
		}
		
		text
	}
}

impl std::ops::Index<usize> for WordleState {
	type Output = WordleEntry;
	
	fn index(&self, index: usize) -> &WordleEntry {
		assert!(index < self.current_entry);
		
		self.get_entry(index).unwrap()
	}
}

pub struct WordleStateIntoIterator<'a> {
	state: &'a WordleState,
	index: usize
}

impl<'a> Iterator for WordleStateIntoIterator<'a> {
	type Item = &'a WordleEntry;
	
	fn next(&mut self) -> Option<&'a WordleEntry> {
		if self.index >= WORDLE_NUM_GUESSES {
			return None;
		}
		
		self.index += 1;
		
		self.state.entries[self.index-1].as_ref()
	}
}

impl<'a> IntoIterator for &'a WordleState {
	type Item = &'a WordleEntry;
	type IntoIter = WordleStateIntoIterator<'a>;
	
	fn into_iter(self) -> Self::IntoIter {
		WordleStateIntoIterator {
			state: self,
			index: 0
		}
	}
}
