// end goal: a website with a UI so you can put in the guesses and get the next one
#![feature(unchecked_math)]
#![feature(const_inherent_unchecked_arith)]
#![feature(rustc_attrs)]
#![feature(const_trait_impl)]
#![feature(once_cell)]

mod lib;

use crate::lib::*;

// TODO: use some more ideas from https://www.youtube.com/watch?v=doFowk4xj7Q


fn main() {
	assert_eq!(unsafe{std::mem::transmute::<_, u16>(WordleAnswer::ABACK)}, 0);
	
	let mut state = WordleState::new();
	
	state.hard_mode = true;
	state.push_entry(WordleEntry {guess: WordleWord::LEARN, clue: WordleClue::YY___});
	// state.push_entry(WordleEntry {guess: WordleWord::FLEET, clue: WordleClue::_YG__});
	// state.push_entry(WordleEntry {guess: WordleWord::DIVER, clue: WordleClue::_G_GG});
	// state.push_entry(WordleEntry {guess: WordleWord::TIMER, clue: WordleClue::_G_GG});
	
	let w = state.get_best_word(6);
	
	// state.push_entry(WordleEntry {guess: WordleWord::AROSE, clue: WordleClue::BLANK});
	// let w = state.min_dist_possibility_minimax(
	// 	&possible_answers, WordleWord::AROSE,
	// 	3, WordleScore::MIN, WordleScore::MAX
	// );
	
	println!("\n{:?}", w); // */
}