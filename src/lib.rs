// i rewrote this in rust and now its 🦀 BLAZING FAST!!! 🚀🚀🚀
// (just kidding, its only like 30% faster than my original C impl, 
// but there have been way fewer segfaults and annoying stuff like that)
// also because i want to get better at rust cuz i lowkey suck at it :/

#![feature(const_option)] // why does this need to be here if it gives me a warning??? 🤔

// TODO: i have no clue how to do this correctly
include!("constants.rs");


// i hate myself for spending like an hour in compiler explorer optimizing this
// TODO: make this able to work with SIMD instructions
// UPDATE: my cpu doesnt have avx 512 so this idea dont rlly work (at least easily)
// update 2: it seems compacting the result into a single byte by 
//           using base 3 doesnt really have any effect on speed
pub const fn get_clues(guess: WordleWord, answer: WordleWord) -> WordleClue {
	debug_assert!(guess[0]!=0&&guess[1]!=0&&guess[2]!=0&&guess[3]!=0&&guess[4]!=0);
    debug_assert!(answer[0]!=0&&answer[1]!=0&&answer[2]!=0&&answer[3]!=0&&answer[4]!=0);
	
    let mut guess = guess;
    let mut answer = answer;
    
    let mut result: WordleClue = 0;
	
	//  81 27  9  3  1
	// 162 54 18  6  2
	
	if guess[0] == answer[0] {
		result += 162;
		guess[0] = 0;
		answer[0] = 0;
	}
	if guess[1] == answer[1] {
		result += 54;
		guess[1] = 0;
		answer[1] = 0;
	}
	if guess[2] == answer[2] {
		result += 18;
		guess[2] = 0;
		answer[2] = 0;
	}
	if guess[3] == answer[3] {
		result += 6;
		guess[3] = 0;
		answer[3] = 0;
	}
	if guess[4] == answer[4] {
		result += 2;
		guess[4] = 0;
		answer[4] = 0;
	}
    
    if guess[0] != 0 {
        let mut i = 5;
		
		// NOTE: these all get optimized into single cmov instructions :D
        if guess[0]==answer[4] {i = 4}
        if guess[0]==answer[3] {i = 3}
        if guess[0]==answer[2] {i = 2}
        if guess[0]==answer[1] {i = 1}
        
		if i != 5 {
			result += 81;
			answer[i] = 0;
		}
    }
    if guess[1] != 0 {
        let mut i = 5;

        if guess[1]==answer[4] {i = 4}
        if guess[1]==answer[3] {i = 3}
        if guess[1]==answer[2] {i = 2}
        if guess[1]==answer[0] {i = 0}
        
        if i != 5 {
			result += 27;
			answer[i] = 0;
		}
    }
    if guess[2] != 0 {
        let mut i = 5;

        if guess[2]==answer[4] {i = 4}
        if guess[2]==answer[3] {i = 3}
        if guess[2]==answer[1] {i = 1}
        if guess[2]==answer[0] {i = 0}
        
        if i != 5 {
			result += 9;
			answer[i] = 0;
		}
    }
    if guess[3] != 0 {
        let mut i = 5;

        if guess[3]==answer[4] {i = 4}
        if guess[3]==answer[2] {i = 2}
        if guess[3]==answer[1] {i = 1}
        if guess[3]==answer[0] {i = 0}
        
        if i != 5 {
			result += 3;
			answer[i] = 0;
		}
    }
    if guess[4] != 0 {
        let mut i = 5;

        if guess[4]==answer[3] {i = 3}
        if guess[4]==answer[2] {i = 2}
        if guess[4]==answer[1] {i = 1}
        if guess[4]==answer[0] {i = 0}
        
		if i != 5 {
			result += 1;
			// answer[i] = 0;
		}
    }
	
	result
}


#[derive(Default, Clone)]
pub struct WordleState {
	current_entry: usize, // "size"
	pub entries: [Option<WordleEntry>; WORDLE_NUM_GUESSES]
}

impl WordleState {
	pub fn new() -> WordleState {
		WordleState {
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
	
	pub const fn is_solved(&self) -> bool {
		if self.current_entry == 0 {
			false
		} else {
			self.get_entry(self.current_entry-1).unwrap().clue == 242
		}
	}
	
	pub const fn is_lost(&self) -> bool {
		self.current_entry >= WORDLE_NUM_GUESSES
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
		if self.current_entry == 0 {return true}
		for entry in self {
			if get_clues(entry.guess, possible_answer) != entry.clue {
				return false;
			}
		}
		true
	}
	
	pub fn is_possible_hardmode_word(&self, possible_word: WordleWord) -> bool {
		todo!("is_possible_hardmode_word({})", String::from_utf8_lossy(&possible_word));
	}
	
	pub fn emoji_grid(&self) -> String {
		todo!("emoji_grid()")
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
	pub fn get_avg_entropy(&self, word: WordleWord) -> f32 {
		let mut dist: [u16; NUM_WORDLE_CLUES] = [0; NUM_WORDLE_CLUES];
		let mut dist_total: u16 = 0;
		
		// build up the distribution
		// ______ | 182
		// y_____ | 147
		// _____y | 119
		// [...]
		for answer in WORDLE_ANSWERS {
			if self.is_possible_answer(answer) {
				dist[get_clues(word, answer) as usize] += 1;
				dist_total += 1;
			}
		}
		
		// H = -sum(p*log(p))
		//  => H = log(t) - sum(f*log(f))/t
		
		// TODO: theres a more idiomatic way to do this using iterators and
		//       fold() but im pretty much just copying c code rn so idrc
		let mut sum = 0.0;
		for px in dist {
			if px == 0 {continue;}
			sum += px as f32 * (px as f32).log2();
		}
		
		(dist_total as f32).log2() - sum / (dist_total as f32)
	}
	
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
						dist[get_clues(word, answer) as usize] += 1;
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
				let entropy = self.get_avg_entropy(word);
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
		
		let h_0 = self.get_avg_entropy(word);
		if depth == 1 {return h_0;}
		
		let mut dist: [WordleClue; NUM_WORDLE_CLUES] = [0; NUM_WORDLE_CLUES];
		let mut dist_total: u16 = 0;
		
		for answer in WORDLE_ANSWERS {
			if self.is_possible_answer(answer) {
				dist[get_clues(word, answer) as usize] += 1;
				dist_total += 1;
			}
		}
		
		let mut sum: f32 = 0.0;
		
		let mut s = self.clone();
		
		for (pattern, &px) in dist.iter().enumerate() {
			if px == 0 {continue;}
			
			s.push_entry(WordleEntry {guess: word, clue: pattern as WordleClue});
			
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


// better way to guess wordle words
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
	
	pub fn get_min_worst_entropy_word(&self) -> (u16, WordleWord) {
		let mut min_worst_case: u16 = u16::MAX;
		let mut best_guess: WordleWord = WordleWord::default();
		
		// use a lookup table to avoid recomputing the same thing over and over again
		let possible_answers: [bool; NUM_WORDLE_ANSWERS] = WORDLE_ANSWERS.map(|w: WordleWord| -> bool {self.is_possible_answer(w)});
		let num_possible_answers = possible_answers.iter().filter(|&&b| b).count();
		
		if num_possible_answers == 1 {
			return (0, WORDLE_ANSWERS[possible_answers.iter().position(|&b| b).unwrap()]);
		}
		
		// arbitrary cutoff to make it always guess a possible answer when close to the end (idk if this helps or hurts average performance, i havent tested it tbqh)
		if num_possible_answers > 16 {
			'outer:
			for (i, &word) in WORDLE_ANSWERS.iter().enumerate() {
				if !possible_answers[i] {continue;}
				
				let max_value = {
					let mut distribution: [u16; NUM_WORDLE_CLUES] = [0; NUM_WORDLE_CLUES];
					for (i, &answer) in WORDLE_ANSWERS.iter().enumerate() {
						if possible_answers[i] {
							let clue = get_clues(word, answer) as usize;
							
							distribution[clue] += 1;
							if distribution[clue] > min_worst_case {continue 'outer;}
						}
					}
					distribution.iter().fold(0, |a, &b| if a > b {a} else {b})
				};
				
				if max_value < min_worst_case {
					best_guess = word;
					min_worst_case = max_value;
				} /* else if max_value == min_worst_case {
					// TODO: you have already calculated the distribution for BOTH of these, so dont recompute it by using get_avg_entropy()
					// find some way to quickly compare entropies of distributions without going into float arithmetic
					// maybe sm like comparing the sum of the abs differences between the distributions and mean rather than entropies
					// or just do actual math on the -Σ[P(x)*log2(p(x))] formulas to find sm quicker to compute
					
					// cuz tbh computing the actual entropy isnt important, just comparing them
					// i mean you can still compute it when needed (like when displaying to the user) 
					// but in these intense calculations its not rlly worth it
					
					// ALSO i dont know if this whole thing is strictly necessary on high enough depth :/
					
					println!("{} {} {}", min_worst_case, String::from_utf8_lossy(&word), self.get_avg_entropy(word));
				} // */
			}
		} else {
			'outer:
			for word in WORDLE_VALID_WORDS {
				let max_value = {
					let mut distribution: [u16; NUM_WORDLE_CLUES] = [0; NUM_WORDLE_CLUES];
					for (j, &answer) in WORDLE_ANSWERS.iter().enumerate() {
						if possible_answers[j] {
							let clue = get_clues(word, answer) as usize;
							
							distribution[clue] += 1;
							if distribution[clue] > min_worst_case {continue 'outer;}
						}
					}
					distribution.iter().fold(0, |a, &b| if a > b {a} else {b})
				};
				
				if max_value < min_worst_case {
					best_guess = word;
					min_worst_case = max_value;
				}
			}
		}
		
		(min_worst_case as u16, best_guess)
	}
	
	pub fn get_min_worst_entropy_word_depth(&mut self, depth: u8, alpha: u16, beta: u16) -> Option<(u16, WordleWord)> {
		debug_assert!(0 < depth && depth < WORDLE_NUM_GUESSES as u8); // i like python chained comparisons :/
		
		// TODO: do these ending states actually work for the winning/losing states?
		if self.current_entry >= WORDLE_NUM_GUESSES {return None}
		else if self.current_entry > 0 && self[self.current_entry-1].clue == 242 {return None}
		
		else if depth == 1 {
			let mut beta = beta;
			let mut min_worst_case: u16 = u16::MAX;
			let mut best_guess: Option<WordleWord> = None;
			
			// use a lookup table to avoid recomputing the same thing over and over again
			let possible_answers: [bool; NUM_WORDLE_ANSWERS] = WORDLE_ANSWERS.map(|w: WordleWord| -> bool {self.is_possible_answer(w)});
			let num_possible_answers = possible_answers.iter().filter(|&&b| b).count();
			
			// this is important bc if you dont explicitly check this, the bot goes "oh well no matter
			// what, theres always just 1 possible answer, so ill just guess sm random instead of the
			// only remaining option! i am very smart :sunglasses:"
			// TODO: should i have this outside of depth==1??? (probably)
			if num_possible_answers == 1 {
				return Some((0, WORDLE_ANSWERS[possible_answers.iter().position(|&b| b).unwrap()]));
			}
			
			// TODO: optimize order of this
			for word in WORDLE_VALID_WORDS {
				let max_value = {
					let mut distribution: [u16; NUM_WORDLE_CLUES] = [0; NUM_WORDLE_CLUES];
					for (i, &answer) in WORDLE_ANSWERS.iter().enumerate() {
						if possible_answers[i] {
							let clue = get_clues(word, answer) as usize;
							
							distribution[clue] += 1;
							if distribution[clue] > beta {break;}
						}
					}
					distribution.iter().fold(0, |a, &b| if a > b {a} else {b})
				};
				
				if max_value <= min_worst_case {
					if max_value <= alpha {break;}
					
					if max_value < beta {beta = max_value;}
					
					if max_value == min_worst_case {
						// TODO: see above (long) comment in get_min_worst_entropy_word()
						// yeah i still dont know if this slows things down too much
						if self.get_avg_entropy(word) < self.get_avg_entropy(best_guess.unwrap()) {
							best_guess = Some(word);
						}
					} else {
						best_guess = Some(word);
						min_worst_case = max_value;
					}
				}
			}
			
			return best_guess.map(|w| (min_worst_case, w));
		}
		
		let mut beta = beta;
		
		let mut min_value = u16::MAX;
		let mut min_word: Option<WordleWord> = None;
		
		// pulled out of the loop for optimization
		let possible_answers: [bool; NUM_WORDLE_ANSWERS] = WORDLE_ANSWERS.map(|w: WordleWord| -> bool {self.is_possible_answer(w)});
		
		
		// TODO: iterate over this sorted by the results of a lower depth search
		for guess in WORDLE_VALID_WORDS {
			let possible_clues: [bool; NUM_WORDLE_CLUES] = {
				let mut clue_is_possible: [bool; NUM_WORDLE_CLUES] = [false; NUM_WORDLE_CLUES];
				for (i, &answer) in WORDLE_ANSWERS.iter().enumerate() {
					// TODO: optimize all this down
					if possible_answers[i] {
						clue_is_possible[get_clues(guess, answer) as usize] = true;
					}
				}
				clue_is_possible
			};
			
			let (max_value, _max_word) = {
				let mut alpha = alpha;
				let mut max_value = u16::MIN;
				let mut max_word: Option<WordleWord> = None;
				
				// for each child node
				for i in 0..NUM_WORDLE_CLUES {
					if possible_clues[i] {
						self.push_entry(WordleEntry {guess: guess, clue: i as u8});
						let result = self.get_min_worst_entropy_word_depth(depth-1, alpha, beta);
						self.pop_entry();
						
						if result.is_none() {break;}
						let (value, word) = result.unwrap();
						
						// println!("\t{} {} {} max:{} b:{} a:{}", radix_fmt::radix(i, 3), String::from_utf8_lossy(&word), value, max_value, beta, alpha);
						
						if value > max_value {
							max_value = value;
							max_word = Some(word);
							
							if max_value >= beta {
								break;
							}
							
							if alpha < max_value {
								alpha = max_value;
							}
						}
					}
				}
				
				(max_value, max_word)
			};
			
			// convenient for test purposes
			if self.current_entry==0 {print!("{} {} {}  {}", min_value, String::from_utf8_lossy(&guess), max_value, (if max_value <= min_value {'\n'} else {'\r'}));use std::io::Write;std::io::stdout().flush().unwrap();};
			
			if max_value < min_value {
				if min_value <= alpha {break;}
				
				if min_value < beta {beta = min_value;}
				
				min_value = max_value;
				min_word = Some(guess);
			}
		}
		
		min_word.map(|w| (min_value, w))
	}
}



#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_clue_repeated_letters() {
		// stolen from 3blue1brown's wordle correction video because theyre good test cases for this behavior
		// see first ~60 seconds of https://www.youtube.com/watch?v=fRed0Xmc2Wg
		assert_eq!(get_clues(*b"SPEED", *b"ABIDE"), 10); // 0t00101
		assert_eq!(get_clues(*b"SPEED", *b"ERASE"), 93); // 0t10110
		assert_eq!(get_clues(*b"SPEED", *b"STEAL"), 180); // 0t20200
		assert_eq!(get_clues(*b"SPEED", *b"CREPE"), 48); // 0t01210
	}
	
    #[test]
	fn test_clues() {
		let mut dist: [i32; 256] = [0; 256];
		
		for a in WORDLE_ANSWERS {
			for g in WORDLE_VALID_WORDS {
				dist[get_clues(g, a) as usize] += 1;
			}
		}
		
		// now THIS is what i call a test.
		assert_eq!(dist, [6694602,1816948,734049,1828604,471396,126405,721800,145608,128206,1854852,388353,132025,478211,87053,20893,139385,21436,13077,651495,142036,71960,139441,24709,9586,80925,11165,19390,2249147,512417,152649,506143,102328,22593,139363,23615,14112,614704,99892,27131,121224,15712,2882,25335,2854,1456,132945,23375,7543,24474,3176,994,8090,979,566,1097968,198355,111762,172723,30512,8652,102784,11675,19838,187575,27218,9868,28590,3567,694,12321,1197,1005,128703,16446,13427,12898,1482,603,15115,775,4594,1420187,337945,118788,337301,65318,17386,113273,18831,12440,304654,50919,16812,62212,7496,1958,15775,1646,915,95588,18144,7797,16559,2057,881,6837,850,545,437564,81728,22416,79707,10993,2390,18600,2497,1411,87202,10116,2721,12749,809,177,2173,148,51,16525,2393,613,2130,174,22,547,46,30,163341,22990,10901,20193,2292,691,11389,966,1000,22126,2031,905,2250,129,15,1093,46,62,11296,1031,685,963,34,44,725,41,0,616362,103814,62552,132770,21401,7080,59481,6297,10282,126729,16828,7062,24674,2710,739,7600,706,493,69445,7620,7276,9904,762,468,7946,401,1917,157029,20607,8555,27045,2952,866,7656,544,566,31691,2645,1097,4284,281,45,984,34,54,10527,825,401,1184,32,46,523,40,0,125767,15016,12442,14714,1935,486,11063,638,2301,14070,1334,437,1837,113,46,645,41,0,20647,1362,2142,1126,83,0,2682,0,2310,0,0,0,0,0,0,0,0,0,0,0,0,0]);
	}
	
	#[test]
	#[should_panic]
	fn test_wordlestate_appending() {
		let mut x = WordleState::default();
		
		x.push_entry(WordleEntry{guess: *b"AROSE", clue: 138}); // 0t12010
		assert_eq!(x[0], WordleEntry{guess: *b"AROSE", clue: 138});
		
		x.push_entry(WordleEntry{guess: *b"PATCH", clue: 36});  // 0t01100
		x.push_entry(WordleEntry{guess: *b"JELLY", clue: 0});   // 0t00000
		x.push_entry(WordleEntry{guess: *b"SOWLS", clue: 1});   // 0t00001
		x.push_entry(WordleEntry{guess: *b"BRISE", clue: 57});  // 0t02010
		x.push_entry(WordleEntry{guess: *b"FINAL", clue: 12});  // 0t00110
		x.push_entry(WordleEntry{guess: *b"PANIC", clue: 36});  // 0t01100
	}
	
	#[test]
	fn test_possible_answers() {
		let mut x = WordleState::default();
		
		x.push_entry(WordleEntry{guess: *b"AROSE", clue: 0});
		
		assert_eq!(WORDLE_ANSWERS.iter().filter(|&a| x.is_possible_answer(*a)).count(), 182);
	}
	
	#[test]
	fn test_avg_entropy() {
		let s = WordleState::default();
		let e = s.get_avg_entropy(*b"AROSE");
		
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
		
		assert_eq!(e, 167);
		// assert_eq!(w, *b"RAISE"); // TODO: RAISE has the lowest AVERAGE entropy, but random bs like AESIR has the same worst case entropy and since it comes first in the alphabet it gets picked - see comment in get_min_worst_entropy_word()
	}
	
	#[test]
	fn test_min_worst_depth() {
		// i have literally only tested this with a modified version that filters only 
		// possible answers and even THAT takes ~10 seconds, so slowing it down by a 
		// factor of 10^2 (and 1 / the % of filtered words) takes too long and i rlly 
		// cant be bothered to wait ~10-20 HOURS (idk exact time) for it to finish
		let mut s = WordleState::default();
		
		let (e, w) = s.get_min_worst_entropy_word_depth(2, u16::MIN, u16::MAX).unwrap();
		
		println!("{} {}", e, String::from_utf8_lossy(&w));
		
		// this is the result of the above test
		// so the real answer is <= this one
		// (im guessing that e is actually 7 or 8)
		assert_eq!(e, 9);
		assert!(w == *b"EARLY" || w == *b"NOSEY" || w == *b"RELAY");
	}
}