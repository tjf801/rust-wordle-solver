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
			std::io::stdin().read_line(&mut guess).expect("invalid guess");
			(0..NUM_WORDLE_WORDS).map(WordleWord::from).find(|word| word.as_str() == guess.trim()).unwrap()
		};
		
		// read clue (as int) from stdin
		let clue = {
			let mut clue = String::new();
			print!("Clue: ");
			std::io::Write::flush(&mut std::io::stdout()).unwrap();
			std::io::stdin().read_line(&mut clue).expect("invalid clue");
			(0..NUM_WORDLE_WORDS).map(WordleClue::from).find(|c| format!("{c:?}") == clue.trim()).unwrap()
		};
		
		state.push_entry(WordleEntry {guess: guess, clue: clue});
		
		println!("{:?}", best_word(&state));
		
		'options: loop {
			if state.current_entry > 0 {
				println!("{}", state.share_text());
			}
			
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
				"eval" => {
					let guess = {
						let mut guess = String::new();
						print!("Guess: ");
						std::io::Write::flush(&mut std::io::stdout()).unwrap();
						std::io::stdin().read_line(&mut guess).expect("invalid word");
						(0..NUM_WORDLE_WORDS).map(WordleWord::from).find(|word| word.as_str() == guess.trim()).unwrap()
					};
					
					let possible_answers = (0..NUM_WORDLE_ANSWERS)
						.map(|i| WordleAnswer::from(i))
						.filter(|&a| state.is_possible_answer(a))
						.collect::<Box<[WordleAnswer]>>();
					let guessable_words = (0..NUM_WORDLE_WORDS)
						.map(|i| WordleWord::from(i))
						.collect::<Box<[WordleWord]>>();
					let remaining_guesses = (WORDLE_NUM_GUESSES - state.current_entry) as u8;
					
					let eval = sumoverpartitions::<false>(
						guessable_words.as_ref(),
						possible_answers.as_ref(), 
						remaining_guesses-1,
						guess, 
						GuessTotal::Infinity
					);
					
					println!("Eval of {guess:?}: {eval}");
					
					continue 'options;
				},
				"help" => {
					println!("quit: quit the program");
					println!("next: add your next guess and clue");
					println!("back: undo the last guess");
					println!("eval: evaluate a guess");
					println!("help: show this help message");
					continue 'options;
				},
				option => {
					println!("invalid option: '{}'", option);
					continue 'options;
				}
			}
		}
	}
}