// i rewrote this in rust and now its 🦀 BLAZING FAST!!! 🚀🚀🚀
// (just kidding, its only like 30% faster than my original C impl, 
// but there have been way fewer segfaults and annoying stuff like that)
// also because i want to get better at rust cuz i lowkey suck at it :/

#![feature(const_option)] // why do these need to be here if they give me warnings??? 🤔
#![feature(unchecked_math, const_inherent_unchecked_arith)]
#![feature(rustc_attrs)]

include!("constants.rs"); // TODO: i have no clue how to do this correctly


// i hate myself for spending like an hour in compiler explorer optimizing this
// TODO: make this able to work with SIMD instructions
// UPDATE: my cpu doesnt have avx 512 so this idea dont rlly work (at least easily)
// update 2: it seems compacting the result into a single byte by 
//           using base 3 doesnt really have any effect on speed
// UPDATE 3: i made this faster but also it looks even shittier now lmao
pub const fn get_clues(guess: [u8;5], answer: [u8;5]) -> WordleClue {
	debug_assert!(guess[0]!=0&&guess[1]!=0&&guess[2]!=0&&guess[3]!=0&&guess[4]!=0);
    debug_assert!(answer[0]!=0&&answer[1]!=0&&answer[2]!=0&&answer[3]!=0&&answer[4]!=0);
	
    let mut g: u8 = 0;
	let mut y: u8 = 0;
	
	// this ingenius idea was shamelessly stolen from https://www.youtube.com/watch?v=doFowk4xj7Q
	// this reduces it from O(n^2) to O(n) lmao i guess leetcode actually does have some practical applications
	let mut misplaced_letters: [u8; 26] = [0u8; (b'Z' - b'A' + 1) as usize];
	
    // TODO: after writing this horrible monstrosity, i realized there is a 
    //       Wrapping trait - so uhh rewrite this to use that instead

	// Find all correct letters
    // i hate rust overflow checking
    // like dumb stupid rust compiler, this isnt gonna overflow
    // stop doing 10000 unnecessary branches every time i want to increment a value
    unsafe {
        // SAFETY: this is not ever gonna overflow, even if literally every 
        //         single number gets added, it sums to 242 and doesnt go higher after
        if guess[0] == answer[0] {
            g = g.unchecked_add(162);
        } else {
            let letter = answer[0].unchecked_sub(b'A') as usize;
            misplaced_letters[letter] = misplaced_letters[letter].unchecked_add(1);
        }
        if guess[1] == answer[1] {
            g = g.unchecked_add(54);
        } else {
            let letter = answer[1].unchecked_sub(b'A') as usize;
            misplaced_letters[letter] = misplaced_letters[letter].unchecked_add(1);
        }
        if guess[2] == answer[2] {
            g = g.unchecked_add(18);
        } else {
            let letter = answer[2].unchecked_sub(b'A') as usize;
            misplaced_letters[letter] = misplaced_letters[letter].unchecked_add(1);
        }
        if guess[3] == answer[3] {
            g = g.unchecked_add(6);
        } else {
            let letter = answer[3].unchecked_sub(b'A') as usize;
            misplaced_letters[letter] = misplaced_letters[letter].unchecked_add(1);
        }
        if guess[4] == answer[4] {
            g = g.unchecked_add(2);
        } else {
            let letter = answer[4].unchecked_sub(b'A') as usize;
            misplaced_letters[letter] = misplaced_letters[letter].unchecked_add(1);
        }
    }
	
	// Find all correct letters in the wrong place
    // SAFETY: see above
	unsafe {
        let mut g_temp = g;
        
        let i = guess[0].unchecked_sub(b'A') as usize;
        if g_temp < 162 && misplaced_letters[i] > 0 {
            misplaced_letters[i] = misplaced_letters[i].unchecked_sub(1);
            y = y.unchecked_add(81);
        }
        if g_temp >= 162 {g_temp = g_temp.unchecked_sub(162);}

        let i = guess[1].unchecked_sub(b'A') as usize;
        if g_temp < 54 && misplaced_letters[i] > 0 {
            misplaced_letters[i] = misplaced_letters[i].unchecked_sub(1);
            y = y.unchecked_add(27);
        }
        if g_temp >= 54 {
            g_temp = g_temp.unchecked_sub(54);
        }

        let i = guess[2].unchecked_sub(b'A') as usize;
        if g_temp < 18 && misplaced_letters[i] > 0 {
            misplaced_letters[i] = misplaced_letters[i].unchecked_sub(1);
            y = y.unchecked_add(9);
        }
        if g_temp >= 18 {
            g_temp = g_temp.unchecked_sub(18);
        }

        let i = guess[3].unchecked_sub(b'A') as usize;
        if g_temp < 6 && misplaced_letters[i] > 0 {
            misplaced_letters[i] = misplaced_letters[i].unchecked_sub(1);
            y = y.unchecked_add(3);
        }
        if g_temp >= 6 {
            g_temp = g_temp.unchecked_sub(6);
        }

        let i = guess[4].unchecked_sub(b'A') as usize;
        if g_temp < 2 && misplaced_letters[i] > 0 {
            misplaced_letters[i] = misplaced_letters[i].unchecked_sub(1);
            y = y.unchecked_add(1);
        }
    }
	
	WordleClue::new(g + y) // since these have disjoint digits in base 3, we can add them together
}



#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
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
		self.current_entry > 0 && self.get_entry(self.current_entry-1).unwrap().clue == clues!(G G G G G)
	}
	
	pub fn is_lost(&self) -> bool {
		self.current_entry >= WORDLE_NUM_GUESSES && !self.is_solved()
	}
	
	pub fn push_entry(&mut self, entry: WordleEntry) {
		assert!(self.current_entry < WORDLE_NUM_GUESSES);
		
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
		} else {
			self.into_iter().all(|entry| entry.matches(possible_answer))
		}
	}
	
	pub fn is_possible_hardmode_word(&self, possible_word: WordleWord) -> bool {
		self.is_possible_answer(possible_word) // TODO: make this actually correct
	}
	
	pub fn share_text(&self) -> String {
		let mut text = String::new();
		
		text += format!("Wordle {} {}/{}{}\n",
			match self.answer {
				Some(answer) => WORDLE_ANSWERS.binary_search(&answer).map(
					|x| WORDLE_ANSWER_NUMBERS[x]
				).unwrap_or(0),
				None => -1
			},
			if self.is_lost() {"X".to_string()} else {self.current_entry.to_string()},
			WORDLE_NUM_GUESSES,
			if self.hard_mode {"*"} else {""}
		).as_str();
		
		for i in 0..self.current_entry {
			if let Some(entry) = self.entries[i] {
				let c = entry.clue.0;
				
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



// entropy functions for WordleState
// (NOTE: this is not rlly used anymore because it is somehow
//        even slower than the absolute snail that is below this)
impl WordleState {
	pub fn get_max_entropy_word(&self) -> (f32, WordleWord) {
		let mut max_entropy = 0.0;
		let mut max_entropy_word = WordleWord::default();
		
		// TODO: see now THIS is the shit that could use some speeding up with SIMD and/or multi-threading
		//		 or sm like just limiting the search to possible words but thats much less epic
		for word in WORDLE_VALID_WORDS {
			let entropy = {
				let mut dist: [u16; NUM_WORDLE_CLUES] = [0; NUM_WORDLE_CLUES];
				let mut dist_total: u16 = 0;
				
				for answer in WORDLE_ANSWERS {
					if self.is_possible_answer(answer) {
						dist[get_clues(word, answer).as_usize()] += 1;
						dist_total += 1;
					}
				}
				
				let mut sum = 0.0;
				for px in dist {
					if px == 0 {continue;}
					sum += px as f32 * (px as f32).log2();
				}
				
				(dist_total as f32).log2() - sum / (dist_total as f32)
			};
			if entropy > max_entropy {
				max_entropy = entropy;
				max_entropy_word = word;
			}
		}
		
		(max_entropy, max_entropy_word)
	}
	
	pub fn get_max_entropy_possible_answer(&self) -> (f32, WordleAnswer) {
		let mut max_entropy = 0.0;
		let mut max_entropy_word = WordleWord::default();
		
		for word in WORDLE_ANSWERS {
			if self.is_possible_answer(word) {
				let entropy = self.avg_entropy(word);
				if entropy > max_entropy {
					max_entropy = entropy;
					max_entropy_word = word;
				}
			}
		}
		
		(max_entropy, max_entropy_word)
	}
	
	pub fn get_word_entropy_depth(&self, word: WordleWord, depth: u8) -> f32 {
		debug_assert!(depth > 0);
		debug_assert!(depth <= WORDLE_NUM_GUESSES as u8);
		
		let h_0 = self.avg_entropy(word);
		if depth == 1 {return h_0;}
		
		let mut dist: [u16; NUM_WORDLE_CLUES] = [0; NUM_WORDLE_CLUES];
		let mut dist_total: u16 = 0;
		
		for answer in WORDLE_ANSWERS {
			if self.is_possible_answer(answer) {
				dist[get_clues(word, answer).as_usize()] += 1;
				dist_total += 1;
			}
		}
		
		let mut sum: f32 = 0.0;
		
		let mut s = self.clone();
		
		for (pattern, &px) in dist.iter().enumerate() {
			if px == 0 {continue;}
			
			s.push_entry(WordleEntry {guess: word, clue: WordleClue::new(pattern as u8)});
			
			let mut max_entropy: f32 = 0.0;
			for word in WORDLE_VALID_WORDS {
				// if !s.is_possible_answer(word) {continue;}
				let entropy = s.get_word_entropy_depth(word, depth-1);
				if entropy > max_entropy {
					max_entropy = entropy;
				}
			}
			sum += (px as f32) * max_entropy;
			
			s.pop_entry();
		}
		
		// println!("{} {} {}", String::from_utf8(word.to_vec()).unwrap(), h_0, sum / (dist_total as f32));
		
		h_0 + sum/(dist_total as f32)
	}
}

// better way to guess wordle words, split into another part
impl WordleState {
	/*
	since calculating the entropy of a word is expensive, using this metric is much faster
	
	the idea is that instead of calculating the average entropy of a distribution,
	we can calculate the worst case entropy of a word, which will always be just 
	the log2 of the number of possible answers; and when you remove the logs, you 
	end up with being able to work with ints instead of floats, AND them being more 
	easily optimizable
	
	e.g: clue distribution for "AROSE" on first guess looks sm like this (kind of ordered)
	_____ | 182
	y____ | 147
	____y | 119
	__y__ | 104
	_y__y | 103
	____g | 79
	[...]
	_yygy | 1
	_gggg | 1
	ggggg | 1
	and so the heuristic for it would be 182
	
	and so, you can use the minimax algorithm on this - where one "player" is the user
	guessing words, and the other is an adversary selecting which set of clues to give
	back to the user. like if i were to start with the guess "AROSE", the hypothetical
	adversary (on a depth of 1) would select all grays as clues, because it maximizes 
	the amount of possible answers i still have to guess from (182). and so repeat 
	with greater depth for a better bot. (e.g: a depth of 2 minimizes the worst case 
	remaining guess amount after the first two guesses)
	
	also for some reason my idea for this came into my brain when i was listening to 
	doja cat (💙💜💖) and doodling a basic decision tree of a wordle game with the 
	original entropy optimizer and trying to force it to be able to be faster lmfao
	
	TODO: OPTIMIZE THIS BETTER!!!
			- add multithreading (?)
			- use SIMD
			- improve minimax effectiveness
			- all the random TODOs in here and unsolved problems
			- other optimizations i havent noticed/thought of yet
	*/
	
	// if there are less than this amount of possible answers, only choose from them
	// const SMALL_CUTOFF: usize = 10;
	
	
	pub fn avg_entropy(&self, word: WordleWord) -> f32 {
		let mut dist: [u16; NUM_WORDLE_CLUES] = [0; NUM_WORDLE_CLUES];
		let mut dist_total: u16 = 0;
		
		for answer in WORDLE_ANSWERS {
			if self.is_possible_answer(answer) {
				dist[get_clues(word, answer).as_usize()] += 1;
				dist_total += 1;
			}
		}
		
		// H = -sum(p*log(p))
		//  => H = log(t) - sum(f*log(f))/t
		//     (where t = total poss. answers and f = int freq of clue)
		let sum = dist.iter()
			.filter(|&&x| x != 0)
			.fold(0.0, |acc, &freq| acc + freq as f32 * (freq as f32).log2());
		
		(dist_total as f32).log2() - sum / (dist_total as f32)
	}
	
	pub fn get_min_worst_entropy_word(&self) -> (WordleWordScore, Option<WordleWord>) {
		self.get_min_worst_entropy_word_minimax(1, WordleWordScore::MIN, WordleWordScore::MAX)
	}
	
	pub fn get_min_worst_entropy_word_depth(&mut self, depth: u8) -> (WordleWordScore, Option<WordleWord>) {
		self.get_min_worst_entropy_word_minimax(depth, WordleWordScore::MIN, WordleWordScore::MAX)
	}
	
	
	fn get_min_worst_entropy_word_minimax(&self, depth: u8, mut alpha: WordleWordScore, beta: WordleWordScore) -> (WordleWordScore, Option<WordleWord>) {
		assert!(depth > 0);
		
		let mut best_score = WordleWordScore::MIN;
		let mut best_guess: Option<WordleWord> = None;
		
		let possible_answer_map: [bool; NUM_WORDLE_ANSWERS] = WORDLE_ANSWERS.map(|w: WordleWord| -> bool {self.is_possible_answer(w)});
		let possible_answers: Vec<&WordleWord> = WORDLE_ANSWERS.iter()
			.filter(|&a| self.is_possible_answer(*a))
			.collect();
		
		let num_possible_answers = possible_answers.len();
		
		if self.is_lost() {
			(WordleWordScore {max_worst_case: num_possible_answers as u16, clue_dist_sum: 0}, None)
		} else if self.is_solved() {
			(WordleWordScore::MAX, None)
		} else if possible_answers.len() == 0 {
			panic!("no possible answers");
		} else if possible_answers.len() == 1 {
			// if there is only one possible answer, just return it
			(
				WordleWordScore {
					max_worst_case: 1,
					clue_dist_sum: 10,
				},
				Some(WORDLE_ANSWERS[
					// SAFETY: we literally just checked above that there is
					//         a possible answer so unwrap_unchecked wont fail
					unsafe {
						possible_answer_map
							.iter()
							.position(|&b| b)
							.unwrap_unchecked()
					}
				])
			)
		}
		
		else if depth == 1 {
			/* if num_possible_answers < WordleState::SMALL_CUTOFF {
				// TODO: OPTIMIZE THIS
				// this helps in *most* cases, but when its remaining words like
				// BATCH, HATCH, LATCH, MATCH, PATCH, etc. it makes it MUCH worse .__.
				let filtered_words = WORDLE_ANSWERS.iter()
					.enumerate()
					.filter(|(i, _)| possible_answers[*i])
					.map(|(_, &w)| w)
					.collect::<Vec<_>>();
				
				'outer:
				for &word in &filtered_words {
					let max_value = {
						let mut distribution: [u16; NUM_WORDLE_CLUES] = [0; NUM_WORDLE_CLUES];
						for &answer in &filtered_words {
							let clue = get_clues(word, answer).as_usize();
							
							distribution[clue] += 1;
							if distribution[clue] > beta {continue 'outer;}
						}
						distribution.iter().fold(0, |a, &b| if a > b {a} else {b})
					};
					
					if max_value <= min_worst_case {
						if max_value <= alpha {break;}
						
						if max_value < beta {beta = max_value;}
						
						if max_value == min_worst_case {
							if let Some(bg) = best_guess {	
								if self.avg_entropy(word) > self.avg_entropy(bg) {
									best_guess = Some(word);
								}
							}
						} else {
							best_guess = Some(word);
							min_worst_case = max_value;
						}
					}
				}
				
				(min_worst_case, best_guess)
			} else */
			if self.hard_mode {
				todo!("hard mode depth 1");
			} else {
				'outer:
				for word in WORDLE_VALID_WORDS {
					let score: WordleWordScore = {
						let mut distribution: [u16; NUM_WORDLE_CLUES] = [0; NUM_WORDLE_CLUES];
						for &&answer in &possible_answers {
							let clue = get_clues(word, answer).as_usize();
							
							distribution[clue] += 1;
							if distribution[clue] > alpha.max_worst_case {continue 'outer;}
						}
						
						WordleWordScore {
							// SAFETY: its an initialized array -- of fucking course its gonna have at least one value
							max_worst_case: unsafe { *distribution.iter().max().unwrap_unchecked() },
							// SAFETY: 0 <= i < distribution.len() == NUM_WORDLE_CLUES == 243
							clue_dist_sum: distribution.iter().enumerate().fold(0, |a, (i, &x)| a + unsafe{WordleClue(i as u8)}.sum_of_base3_digits() as u16 * x )
						}
					};
					
					if score > best_score {
						if score >= beta {break;}
						
						if score > alpha {alpha = score;}
						
						best_guess = Some(word);
						best_score = score;
					} else if score == best_score {
						// still dont know how much performance this is worth or if it even works
						if let Some(bg) = best_guess {	
							if self.avg_entropy(word) > self.avg_entropy(bg) {
								best_guess = Some(word);
							}
						}
					}
				}
				
				(best_score, best_guess)
			}
		}
		
		else {
			if self.hard_mode {todo!("hard mode depth > 1");}
			
			let mut state = self.clone();
			// temporary entry that gets overwritten __A LOT__
			state.push_entry(WordleEntry {guess: *b"     ", clue: WordleClue::new(0)});
			
			// TODO: iterate over this sorted by the results of a lower depth search
			for guess in WORDLE_VALID_WORDS {
				print!("{}\r", String::from_utf8_lossy(&guess));
				state.get_entry_mut(state.current_entry-1).unwrap().guess = guess;
				
				// TODO: optimize this down somehow
				let possible_clues: [bool; NUM_WORDLE_CLUES] = {
					let mut clue_is_possible: [bool; NUM_WORDLE_CLUES] = [false; NUM_WORDLE_CLUES];
					for (i, &answer) in WORDLE_ANSWERS.iter().enumerate() {
						if possible_answer_map[i] {
							clue_is_possible[get_clues(guess, answer).as_usize()] = true;
						}
					}
					clue_is_possible
				};
				
				let score = {
					let mut beta = beta;
					let mut lowest_score = WordleWordScore::MAX;
					// let mut lowest_word: Option<WordleWord> = None;
					
					// for each child node
					for i in 0..NUM_WORDLE_CLUES {
						if possible_clues[i] {
							state.get_entry_mut(state.current_entry-1).unwrap().clue = WordleClue::new(i as u8);
							let (score, _word) = state.get_min_worst_entropy_word_minimax(depth-1, alpha, beta);
							
							// if _word.is_none() {break} // TODO: does this work??
							
							if score < lowest_score {
								lowest_score = score;
								
								if score <= alpha {break;}
								
								if score < beta {beta = score;}
							}
						}
					}
					
					lowest_score
				};
				
				// convenient for test purposes
				// print!("  {:?} {} {:?}  \r", min_value, String::from_utf8_lossy(&guess), max_value);
				
				
				if score > best_score {
					best_score = score;
					best_guess = Some(guess);
					
					if score >= beta {break;}
					
					if score > alpha {alpha = score;}
				} /* else if score == best_score { // TODO: ?????? does this work??? is it important???
					if let Some(bg) = best_guess {	
						if self.avg_entropy(guess) > self.avg_entropy(bg) {
							best_guess = Some(guess);
						}
					}
				} */
			}
			
			(best_score, best_guess)
		}
	}
}



#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_clue_repeated_letters() {
		// stolen from 3blue1brown's wordle correction video because theyre good test cases for this behavior
		// see first ~60 seconds of https://www.youtube.com/watch?v=fRed0Xmc2Wg
		assert_eq!(get_clues(*b"SPEED", *b"ABIDE"), clues!(_ _ Y _ Y));
		assert_eq!(get_clues(*b"SPEED", *b"ERASE"), clues!(Y _ Y Y _));
		assert_eq!(get_clues(*b"SPEED", *b"STEAL"), clues!(G _ G _ _));
		assert_eq!(get_clues(*b"SPEED", *b"CREPE"), clues!(_ Y G Y _));
	}
	
    #[test]
	fn test_clues() {
		let mut dist = [0; 243];
		
		for a in WORDLE_ANSWERS {
			for g in WORDLE_VALID_WORDS {
				dist[get_clues(g, a).as_usize()] += 1;
			}
		}
		
		// now THIS is what i call a test.
		assert_eq!(dist, [6693271, 1816409, 732917, 1828213, 471198, 126059, 721652, 145567, 128121, 1854177, 388163, 131605, 478006, 87000, 20805, 139368, 21433, 13059, 651309, 142000, 71787, 139332, 24679, 9521, 80878, 11155, 19355, 2248626, 512215, 152235, 506020, 102311, 22534, 139338, 23604, 14086, 614368, 99843, 26976, 121166, 15707, 2862, 25331, 2852, 1447, 132934, 23371, 7526, 24463, 3176, 989, 8089, 979, 566, 1097744, 198298, 111659, 172657, 30491, 8638, 102748, 11661, 19830, 187570, 27217, 9866, 28588, 3567, 694, 12321, 1197, 1005, 128627, 16430, 13381, 12880, 1480, 599, 15098, 771, 4590, 1419554, 337699, 118575, 337111, 65242, 17351, 113200, 18811, 12427, 304479, 50868, 16765, 62161, 7485, 1951, 15775, 1646, 914, 95491, 18128, 7780, 16527, 2048, 879, 6824, 847, 543, 437287, 81655, 22352, 79665, 10987, 2388, 18577, 2495, 1408, 87136, 10105, 2700, 12736, 809, 176, 2173, 148, 51, 16503, 2390, 613, 2120, 173, 22, 543, 46, 30, 163300, 22982, 10885, 20188, 2292, 690, 11387, 966, 1000, 22121, 2029, 903, 2249, 129, 15, 1093, 46, 62, 11296, 1031, 685, 963, 34, 44, 725, 41, 0, 616226, 103772, 62463, 132729, 21393, 7066, 59460, 6296, 10273, 126679, 16816, 7024, 24663, 2708, 735, 7598, 706, 491, 69431, 7619, 7264, 9898, 762, 463, 7939, 401, 1914, 156980, 20583, 8512, 27041, 2952, 865, 7652, 544, 565, 31665, 2643, 1088, 4282, 281, 45, 984, 34, 53, 10525, 825, 401, 1184, 32, 46, 523, 40, 0, 125725, 15012, 12420, 14702, 1935, 486, 11057, 637, 2299, 14070, 1334, 437, 1837, 113, 46, 645, 41, 0, 20632, 1361, 2135, 1125, 83, 0, 2679, 0, 2309]);
	}
	
	#[test]
	#[should_panic]
	fn test_wordlestate_appending() {
		let mut x = WordleState::default();
		
		x.push_entry(WordleEntry{guess: *b"AROSE", clue: clues!(Y G _ Y _)}); // 0t12010
		assert_eq!(x[0], WordleEntry{guess: *b"AROSE", clue: clues!(Y G _ Y _)});
		
		x.push_entry(WordleEntry{guess: *b"PATCH", clue: clues!(_ Y Y _ _)});  // 0t01100
		x.push_entry(WordleEntry{guess: *b"JELLY", clue: clues!(_ _ _ _ _)});  // 0t00000
		x.push_entry(WordleEntry{guess: *b"SOWLS", clue: clues!(_ _ _ _ Y)});  // 0t00001
		x.push_entry(WordleEntry{guess: *b"BRISE", clue: clues!(_ G _ Y _)});  // 0t02010
		x.push_entry(WordleEntry{guess: *b"FINAL", clue: clues!(_ _ Y Y _)});  // 0t00110
		x.push_entry(WordleEntry{guess: *b"PANIC", clue: clues!(_ Y Y _ _)});  // 0t01100
	}
	
	#[test]
	fn test_possible_answers() {
		let mut x = WordleState::default();
		
		x.push_entry(WordleEntry{guess: *b"AROSE", clue: clues!(_ _ _ _ _)});
		
		assert_eq!(WORDLE_ANSWERS.iter().filter(|&a| x.is_possible_answer(*a)).count(), 182);
	}
	
	#[test]
	fn test_avg_entropy() {
		let s = WordleState::default();
		let e = s.avg_entropy(*b"AROSE");
		
		assert!(5.76 < e && e < 5.78);
	}
	
	#[test]
	fn test_max_entropy() {
		let s = WordleState::default();
		let (e, w) = s.get_max_entropy_word();
		let (e2, a) = s.get_max_entropy_possible_answer();
		
		assert!(5.88 < e);
		assert_eq!(w, *b"SOARE");
		
		assert!(5.87 < e2 && e2 < e);
		assert_eq!(a, *b"RAISE");
	}
	
	#[test]
	fn test_avg_entropy_2() {
		let s = WordleState::default();
		let e = s.get_word_entropy_depth(*b"SLANE", 2);
		
		println!("{}", e);
		
		assert!(10.03 < e && e < 10.05);
	}
	
	#[test]
	fn test_max_min_entropy() {
		let s = WordleState::default();
		// s.push_entry(WordleEntry{word: *b"AROSE", clue: 0}); // _____
		// s.push_entry(WordleEntry{word: *b"CLIPT", clue: 9}); // __y__
		
		let (e, _w) = s.get_min_worst_entropy_word();
		
		// println!("{} {}", e, String::from_utf8_lossy(&w));
		
		assert_eq!(e, WordleWordScore {max_worst_case: 167, clue_dist_sum: 0});
		// assert_eq!(w, *b"RAISE"); // TODO: RAISE has the lowest AVERAGE entropy, but random bs like AESIR has the same worst case entropy and since it comes first in the alphabet it gets picked - see comment in get_min_worst_entropy_word()
	}
	
	#[test]
	fn test_min_worst_depth() {
		// i have literally only tested this with a modified version that filters only 
		// possible answers and even THAT takes ~10 seconds, so slowing it down by a 
		// factor of 10^2 (and 1 / the % of filtered words) takes too long and i rlly 
		// cant be bothered to wait ~24 HOURS (idk exact time) for it to finish
		// let mut s = WordleState::default();
		
		// let (e, Some(w)) = s.get_min_worst_entropy_word_depth(3).unwrap();
		
		// println!("{} {}", e, String::from_utf8_lossy(&w));
		
		// assert_eq!(e, 8);
		// assert!(w == *b"CERTY" || w == *b"RILEY");
	}
}