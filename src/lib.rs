// i rewrote this in rust and now its 🦀 BLAZING FAST!!! 🚀🚀🚀
// (just kidding, its only like 30% faster than my original C impl, 
// but there have been way fewer segfaults and annoying stuff like that)
// also because i want to get better at rust cuz i lowkey suck at it :/

// why do these need to be here if they give me warnings??? 🤔
#![feature(unchecked_math, const_inherent_unchecked_arith)]
#![feature(rustc_attrs)]
#![feature(const_trait_impl)]
#![feature(once_cell)]

// TODO: WHY IS RUSTS IMPORT SYSTEM SO WEIRD?????
//       LITERALLY I AM DEFAULTING TO SHITTY C #INCLUDES I NEED JESUS
// NOTE: after further reflection i have come to the conclusion that 
//       i just do not understand rusts import system
include!("constants.rs");
include!("get_clues.rs");
include!("score.rs");

pub const WORDLE_NUM_GUESSES: usize = 10;

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
			self.answer.unwrap().day_number(),
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


impl WordleState {
	/*
	since calculating the entropy of a word is expensive, using this metric is much faster
	
	the idea is that instead of calculating the average entropy of a distribution,
	we can calculate the worst case entropy of a word, which will always be just 
	the log2 of the number of possible answers; and when you remove the logs, you 
	end up with being able to work with ints instead of floats, AND them being more 
	easily optimizable
	
	e.g: clue distribution for "AROSE" as the first guess looks sm like this (kind of ordered)
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
	
	NOTE: if you think abt it, the worst case is just 2 ^ the remaining entropy
	      so this is kind of just a very convoluted way to do entropy maximization
	      since r(H)=2^H is a monotonic function
	
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
	
	// TODO: cache the alpha beta scores for each state somehow
	
	
	pub fn avg_entropy(&self, word: WordleWord) -> f32 {
		let mut dist: [u16; NUM_WORDLE_CLUES] = [0; NUM_WORDLE_CLUES];
		let mut dist_total: u16 = 0;
		
		for answer in (0..NUM_WORDLE_ANSWERS)
			.map(|i| WordleAnswer::from(i))
			.filter(|&answer| self.is_possible_answer(answer)) // TODO: this computes the same thing twice
		{
			dist[get_clues(word, answer) as usize] += 1;
			dist_total += 1;
		}
		
		// H = -sum(p*log(p))
		//  => H = log(t) - sum(f*log(f))/t
		//     (where t = total poss. answers and f = int freq of clue)
		let sum = dist.iter()
			.filter(|&&x| x != 0)
			.fold(0.0, |acc, &freq| acc + freq as f32 * (freq as f32).log2());
		
		(dist_total as f32).log2() - sum / (dist_total as f32)
	}
	
	
	#[inline(always)]
	fn gen_possible_answer_map(&self) -> [bool; NUM_WORDLE_ANSWERS] {
		// TODO: collecting into a Vec is a bit slow (but not enough that its a bottleneck)
		
		// SAFETY: the resulting iterator will always have exactly NUM_WORDLE_ANSWERS 
		//         elements, so converting it into a [_; NUM_WORDLE_ANSWERS] will always work
		unsafe {(0..NUM_WORDLE_ANSWERS)
			.map(|i| -> bool {self.is_possible_answer(i.into())})
			.collect::<Vec<_>>()
			.try_into()
			.unwrap_unchecked() }
	}
	
	#[inline(always)]
	fn minimax_depth_1_heuristic(&mut self, possible_answer_map: &[bool; NUM_WORDLE_ANSWERS], mut alpha: WordleScore, beta: WordleScore) -> WordleScore {
		assert!(!self.hard_mode);
		
		'outer:
		for guess in (0..NUM_WORDLE_WORDS).map(|i| WordleWord::from(i)) {
			let score: WordleScore = {
				let mut distribution: [u16; NUM_WORDLE_CLUES] = [0; NUM_WORDLE_CLUES];
				
				// TODO: is this the fastest way to do this?
				for answer in (0..NUM_WORDLE_ANSWERS)
					.map(|i| (i, WordleAnswer::from(i)))
					.filter(|&(i, _)| possible_answer_map[i as usize])
					.map(|(_, answer)| answer)
				{
					// TODO: i dont think this is the most efficient but whatever
					let clue = get_clues(guess, answer) as usize;
					
					distribution[clue] += 1;
					if distribution[clue] > alpha.num_remaining_words {continue 'outer;}
				}
				
				WordleScore {
					turn_num: self.current_entry as u8 + 2,
					// SAFETY: its an initialized array -- of course its gonna have at least one value
					num_remaining_words: unsafe { *distribution.iter().max().unwrap_unchecked() },
					// avg_clue_score: distribution.iter().enumerate().fold(0, |a, (i, &x)| a + WordleClue::from(i).sum_of_base3_digits() as u16 * x ) as f32 / distribution.iter().sum::<u16>() as f32,
				}
			};
			
			if score > alpha {
				alpha = score;
				
				if score >= beta {break;}
			}
		}
		
		alpha
	}
	
	#[inline(always)]
	fn hardmode_depth_1_heuristic(&mut self, possible_answer_map: &[bool; NUM_WORDLE_ANSWERS], mut alpha: WordleScore, beta: WordleScore) -> WordleScore {
		assert!(self.hard_mode);
		
		'outer:
		for guess in (0..NUM_WORDLE_ANSWERS)
			// TODO: make this actually check hardmode words and not just possible answers
			.filter(|&i| possible_answer_map[i])
			.map(|i| WordleAnswer::from(i))
		{
			let score: WordleScore = {
				let mut distribution: [u16; NUM_WORDLE_CLUES] = [0; NUM_WORDLE_CLUES];
				
				// TODO: see analogous comments in `minimax_depth_1_heuristic`
				for answer in (0..NUM_WORDLE_ANSWERS)
					.map(|i| (i, WordleAnswer::from(i)))
					.filter(|&(i, _)| possible_answer_map[i as usize])
					.map(|(_, answer)| answer)
				{
					let clue = get_clues(guess.into(), answer) as usize;
					
					distribution[clue] += 1;
					if distribution[clue] > alpha.num_remaining_words {continue 'outer;}
				}
				
				WordleScore {
					turn_num: self.current_entry as u8 + 2,
					// SAFETY: see comment in minimax_depth_1_heuristic()
					num_remaining_words: unsafe { *distribution.iter().max().unwrap_unchecked() },
				}
			};
			
			if score > alpha {
				alpha = score;
				
				if score >= beta {break;}
			}
		}
		
		alpha
	}
	
	pub fn max_word_score_minimax(&mut self, depth: u8, mut alpha: WordleScore, beta: WordleScore) -> WordleScore {
		// TODO: there is some pretty bad code duplication between hard mode and normal mode --
		//          should i factor it into a generic parameter or something?
		assert!(depth > 0);
		
		let possible_answer_map = self.gen_possible_answer_map();
		// TODO: one could compute this along with the above function
		let num_possible_answers = possible_answer_map.iter().filter(|&&x| x).count();
		
		if self.is_solved() {
			WordleScore {
				turn_num: self.current_entry as u8 + 2,
				num_remaining_words: 0,
			}
		} else if num_possible_answers == 0 {
			panic!("no possible answers {:?}", self.entries);
		} else if num_possible_answers == 1 {
			// if there is only one possible answer, we can just return it
			WordleScore {
				turn_num: self.current_entry as u8 + 2,
				num_remaining_words: 0, 
			}
		} else if depth == 1 {
			if self.hard_mode {
				self.hardmode_depth_1_heuristic(&possible_answer_map, alpha, beta)
			} else {
				self.minimax_depth_1_heuristic(&possible_answer_map, alpha, beta)
			}
		} else if self.hard_mode {
			let guesses = (0..NUM_WORDLE_ANSWERS)
					.filter(|&i| possible_answer_map[i])
					.map(|i| WordleAnswer::from(i));
			
			self.push_entry(WordleEntry {guess: WordleWord::PENIS, clue: WordleClue::BLANK});
			
			// TODO: sort `guesses` by smaller depth search
			for guess in guesses {
				// SAFETY: see analagous comment below
				unsafe{self.get_entry_mut(self.current_entry-1).unwrap_unchecked()}.guess = guess.into();
				
				let score = self.min_dist_possibility_minimax(
					&possible_answer_map, guess.into(),
					depth, alpha, beta
				);
				
				if score > alpha {
					alpha = score;
					
					if score >= beta {break}
				}
			}
			
			self.pop_entry();
			
			alpha
		} else {
			// temporary entry that gets overwritten __A LOT__
			// its "penis" just cuz lmao
			self.push_entry(WordleEntry {guess: WordleWord::PENIS, clue: WordleClue::BLANK});
			
			// TODO: this can be sorted by the depth 1 heuristic
			for guess in (0..NUM_WORDLE_WORDS).map(|x| WordleWord::from(x)) {
				// SAFETY: we literally just appended a guess so there will always be at least one
				unsafe{self.get_entry_mut(self.current_entry-1).unwrap_unchecked()}.guess = guess;
				
				let score = self.min_dist_possibility_minimax(
					&possible_answer_map, guess,
					depth, WordleScore::MIN, WordleScore::MAX
				);
				
				if score > alpha {
					alpha = score;
					
					// TODO: check to see if this should be before the assignment
					if score >= beta {break}
				}
			}
			
			self.pop_entry();
			
			alpha
		}
	}
	
	#[inline(always)]
	fn min_dist_possibility_minimax(&mut self, 
		possible_answer_map: &[bool; NUM_WORDLE_ANSWERS], guess: WordleWord,
		depth: u8, alpha: WordleScore, mut beta: WordleScore
	) -> WordleScore 
	{
		assert!(depth > 0);
		
		// TODO: optimize this down somehow???
		// TODO: on depth 2 and lower, try not sorting by the heuristic and see if its faster
		let possible_clues: [Option<WordleClue>; NUM_WORDLE_CLUES] = {
			let mut dist: [u32; NUM_WORDLE_CLUES] = [0; NUM_WORDLE_CLUES];
			for answer in (0..NUM_WORDLE_ANSWERS)
					.map(|i| (i, WordleAnswer::from(i)))
					.filter(|&(i, _)| possible_answer_map[i as usize])
					.map(|(_, answer)| answer)
			{
				dist[get_clues(guess, answer) as usize] += 1;
			}
			
			let mut sorted_clues: [Option<WordleClue>; NUM_WORDLE_CLUES] = [None; NUM_WORDLE_CLUES];
			
			// selection "sort" into the other array
			// TODO: is this the best algorithm for this case???
			//       (it just needs to be FAST, and it has fixed size )
			for i in 0..NUM_WORDLE_CLUES {
				let mut max_index = 0;
				for j in 1..NUM_WORDLE_CLUES {
					if dist[j] > dist[max_index] {
						max_index = j;
					}
				}
				if dist[max_index] == 0 {break}
				dist[max_index] = 0;
				sorted_clues[i] = Some(WordleClue::from(max_index));
			}
			
			sorted_clues
		};
		
		// each node now is the resulting clue
		// this can be modeled as an imaginary adversary picking 
		// the clue that gives the least information back to us
		//   (this is the whole justification of being able to use minimax)
		
		// TODO: sort by lower depth search
		for clue in possible_clues {
			// TODO: is there a more idiomatic way to say "iterate until you see a none"?
			if let Some(clue) = clue {
				self.get_entry_mut(self.current_entry-1).unwrap().clue = clue;
				
				let score = self.max_word_score_minimax(
					depth-1, alpha, beta
				);
				
				// if depth == 6 {println!("  {clue:?} {score:?}")}
				
				if score < beta {
					beta = score;
					
					if score <= alpha {break}
				}
			} else {
				break
			}
		}
		
		beta
	}
	
	
	#[inline(always)]
	pub fn get_min_worst_case_word(&self) -> WordleWord {
		todo!()
	}
	
	pub fn get_best_word(&mut self, depth: u8) -> (WordleScore, WordleWord) {
		assert!(depth > 0);
		
		// TODO: if self.current_entry == 0, there are no existing guesses, so we 
		//          can have the results of each depth in an array and just return 
		//          sm like `self.first_turns[depth as usize]`
		// NOTE:    (there would have to be separate arrays for hard/easy modes)
		
		let possible_answer_map: [bool; NUM_WORDLE_ANSWERS] = self.gen_possible_answer_map();
		let num_possible_answers = possible_answer_map.iter().filter(|&&x| x).count();
		
		
		if num_possible_answers == 0 {
			panic!("no possible answers (impossible state)\n{:?}", self.entries)
		} else if num_possible_answers == 1 {
			(
				WordleScore {
					turn_num: self.current_entry as u8 + 2,
					num_remaining_words: 0,
				},
				// SAFETY: we just checked that there is a single possible
				//         answer, so position() will never return None
				unsafe{possible_answer_map.iter().position(|&x| x).unwrap_unchecked()}.into()
			)
		} else if depth == 1 {
			todo!("get_min_worst_entropy_word_depth depth 1")
		} else if self.hard_mode {
			let mut best_score = WordleScore::MIN;
			let mut best_word = None::<WordleWord>;
			
			let guesses = (0..NUM_WORDLE_ANSWERS)
					.filter(|&i| possible_answer_map[i])
					.map(|i| WordleAnswer::from(i));
			
			self.push_entry(WordleEntry {guess: WordleWord::PENIS, clue: WordleClue::BLANK});
			
			for guess in guesses {
				unsafe{self.get_entry_mut(self.current_entry-1).unwrap_unchecked()}.guess = guess.into();
				
				// println!("{guess:?}...");
				
				// TODO: explain why one needs to not use alpha and beta here
				//       (or alternatively just fix the algorithm so you can)
				let score = self.min_dist_possibility_minimax(
					&possible_answer_map, guess.into(),
					depth, WordleScore::MIN, WordleScore::MAX
				);
				
				// println!("{guess:?} {score:?}");
				
				if score > best_score {
					best_word = Some(guess.into());
					best_score = score;
				} else if score == best_score {
					let dist_info = |word: WordleWord| -> (u16, u16) {
						let mut t = 0;
						let mut dist: [u16; NUM_WORDLE_CLUES] = [0; NUM_WORDLE_CLUES];
						
						for a in possible_answer_map.iter().enumerate().filter(|(_, &v)| v).map(|(i, _)| WordleAnswer::from(i)) {
							let c = get_clues(word, a);
							t += c.sum_of_base3_digits() as u16;
							dist[usize::from(c)] += 1;
						}
						
						(*dist.iter().max().unwrap(), t)
					};
					
					// UNWRAP: best_word won't be None here because score can never be WordleScore::MIN
					let best_word_info = dist_info(best_word.unwrap());
					let (max_worst_case, dist_clue_sum) = dist_info(guess.into());
					
					if max_worst_case < best_word_info.0 {
						best_word = Some(guess.into());
					} else if max_worst_case == best_word_info.0 {
						if dist_clue_sum > best_word_info.1 {
							best_word = Some(guess.into());
						} else if dist_clue_sum == best_word_info.1 {
							// TODO: compare entropies
						}
					}
				}
			}
			
			self.pop_entry();
			
			(best_score, best_word.expect("no best word found - unreachable"))
		} else {
			todo!("get_min_worst_entropy_word_depth normal mode")
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
		assert_eq!(get_clues_uncached(b"SPEED", b"ABIDE"), WordleClue::BBYBY);
		assert_eq!(get_clues_uncached(b"SPEED", b"ERASE"), WordleClue::YBYYB);
		assert_eq!(get_clues_uncached(b"SPEED", b"STEAL"), WordleClue::GBGBB);
		assert_eq!(get_clues_uncached(b"SPEED", b"CREPE"), WordleClue::BYGYB);
	}
	
    #[test]
	fn test_clues() {
		let mut dist = [0; NUM_WORDLE_CLUES];
		
		for w in 0..NUM_WORDLE_WORDS {
			for a in 0..NUM_WORDLE_ANSWERS {
				dist[get_clues(w.into(), a.into()) as usize] += 1;
			}
		}
		
		assert_eq!(dist, [6693271, 1816409, 732917, 1828213, 471198, 126059, 721652, 145567, 128121, 1854177, 388163, 131605, 478006, 87000, 20805, 139368, 21433, 13059, 651309, 142000, 71787, 139332, 24679, 9521, 80878, 11155, 19355, 2248626, 512215, 152235, 506020, 102311, 22534, 139338, 23604, 14086, 614368, 99843, 26976, 121166, 15707, 2862, 25331, 2852, 1447, 132934, 23371, 7526, 24463, 3176, 989, 8089, 979, 566, 1097744, 198298, 111659, 172657, 30491, 8638, 102748, 11661, 19830, 187570, 27217, 9866, 28588, 3567, 694, 12321, 1197, 1005, 128627, 16430, 13381, 12880, 1480, 599, 15098, 771, 4590, 1419554, 337699, 118575, 337111, 65242, 17351, 113200, 18811, 12427, 304479, 50868, 16765, 62161, 7485, 1951, 15775, 1646, 914, 95491, 18128, 7780, 16527, 2048, 879, 6824, 847, 543, 437287, 81655, 22352, 79665, 10987, 2388, 18577, 2495, 1408, 87136, 10105, 2700, 12736, 809, 176, 2173, 148, 51, 16503, 2390, 613, 2120, 173, 22, 543, 46, 30, 163300, 22982, 10885, 20188, 2292, 690, 11387, 966, 1000, 22121, 2029, 903, 2249, 129, 15, 1093, 46, 62, 11296, 1031, 685, 963, 34, 44, 725, 41, 0, 616226, 103772, 62463, 132729, 21393, 7066, 59460, 6296, 10273, 126679, 16816, 7024, 24663, 2708, 735, 7598, 706, 491, 69431, 7619, 7264, 9898, 762, 463, 7939, 401, 1914, 156980, 20583, 8512, 27041, 2952, 865, 7652, 544, 565, 31665, 2643, 1088, 4282, 281, 45, 984, 34, 53, 10525, 825, 401, 1184, 32, 46, 523, 40, 0, 125725, 15012, 12420, 14702, 1935, 486, 11057, 637, 2299, 14070, 1334, 437, 1837, 113, 46, 645, 41, 0, 20632, 1361, 2135, 1125, 83, 0, 2679, 0, 2309]);
	}
	
	#[test]
	fn test_possible_answers() {
		let mut x = WordleState::default();
		
		x.push_entry(WordleEntry{guess: WordleWord::AROSE, clue: WordleClue::BBBBB});
		
		assert_eq!((0..NUM_WORDLE_ANSWERS).map(|i|WordleAnswer::from(i)).filter(|&a| x.is_possible_answer(a)).count(), 182);
	}
	
	#[test]
	fn test_avg_entropy() {
		let s = WordleState::default();
		let e = s.avg_entropy(WordleWord::AROSE);
		
		assert!(5.76 < e && e < 5.78);
	}
	
	#[test]
	fn test_max_min_entropy() {
		let s = WordleState::default();
		
		let w = s.get_min_worst_case_word();
		
		// println!("{} {}", e, String::from_utf8_lossy(&w));
		
		// assert_eq!(e, WordleScore {num_remaining_words: 167, avg_clue_score: 0});
		assert_eq!(w, WordleWord::RAISE);
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