// end goal: a website with a UI so you can put in the guesses and get the next one
#![feature(unchecked_math)]
#![feature(const_inherent_unchecked_arith)]
#![feature(rustc_attrs)]
#![feature(const_trait_impl)]

use wordle_bot_fast_nolife::{
	wordle::{self, WordleEntry, WORDLE_NUM_GUESSES},
	constants::{NUM_WORDLE_WORDS, WordleWord, WordleClue, NUM_WORDLE_ANSWERS, WordleAnswer},
	best_word, sumoverpartitions,
	total_guesses::GuessTotal,
};

// TODO: use some more ideas from https://www.youtube.com/watch?v=doFowk4xj7Q


fn main() {
	let mut state = wordle::WordleState::new();
	
	'main: loop {
		if state.current_entry > 0 {
			println!("{}", state.share_text());
		}
		
		match {
			let mut option = String::new();
			print!("enter option: ");
			std::io::Write::flush(&mut std::io::stdout()).unwrap();
			std::io::stdin().read_line(&mut option).unwrap();
			option
		}.as_str().trim() {
			"quit" => break 'main,
			"hard" => {
				if state.current_entry == 0 {
					state.hard_mode = true;
				} else {
					println!("can't change hard mode after the first guess!");
				}
			},
			"easy" => state.hard_mode = false,
			"next" => {
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
			},
			"best" => {
				println!("Best guess: {:?}", best_word(&state));
			}
			"back" => {
				state.pop_entry();
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
				
				print!("evaluating...\r");
				std::io::Write::flush(&mut std::io::stdout()).unwrap();
				
				let eval = if state.hard_mode {
					sumoverpartitions::<true>(
						guessable_words.as_ref(),
						possible_answers.as_ref(), 
						remaining_guesses-1,
						guess, 
						GuessTotal::Infinity
					)
				} else {
					sumoverpartitions::<false>(
						guessable_words.as_ref(),
						possible_answers.as_ref(), 
						remaining_guesses-1,
						guess, 
						GuessTotal::Infinity
					)
				};
				
				println!("Eval of {guess:?}: {eval}");
			},
			"help" => {
				println!("quit: quit the program");
				println!("hard: enable hard mode");
				println!("easy: disable hard mode");
				println!("next: add your next guess and clue");
				println!("best: show the best guess");
				println!("back: undo the last guess");
				println!("eval: evaluate a guess");
				println!("help: show this help message");
			},
			option => {
				println!("invalid option: '{}', use 'help' for help", option);
			}
		}
	}
}