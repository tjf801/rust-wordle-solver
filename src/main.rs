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
	
	state.hard_mode = false;
	'main: loop {
		// read guess from stdin
		let guess = {
			let mut guess = String::new();
			print!("Guess: ");
			std::io::Write::flush(&mut std::io::stdout()).unwrap();
			std::io::stdin().read_line(&mut guess).unwrap();
			(0..NUM_WORDLE_WORDS).map(WordleWord::from).find(|word| word.as_str() == guess.trim()).unwrap()
		};
		
		// read clue (as int) from stdin
		let clue = WordleClue::from({
			let mut clue = String::new();
			print!("Clue: ");
			std::io::Write::flush(&mut std::io::stdout()).unwrap();
			std::io::stdin().read_line(&mut clue).unwrap();
			clue.trim().parse::<u16>().unwrap() as usize
		});
		
		state.push_entry(WordleEntry {guess: guess, clue: clue});
		
		println!("{:?}", best_word(&state));
		
		state.pop_entry();
		
		'options: loop {
			println!("{}", state.share_text());
			
			match {
				let mut option = String::new();
				print!("quit, next, or back: ");
				std::io::Write::flush(&mut std::io::stdout()).unwrap();
				std::io::stdin().read_line(&mut option).unwrap();
				option
			}.as_str().trim() {
				"quit" => break 'main,
				"next" => continue 'main,
				"back" => {
					state.pop_entry();
					continue 'options;
				},
				option => panic!("invalid option {}", option),
			}
		}
	}
}