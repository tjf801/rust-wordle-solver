// i rewrote this in rust and now its 🦀 BLAZING FAST!!! 🚀🚀🚀
// (just kidding, its only like 30% faster than my original C impl, 
// but there have been way fewer segfaults and annoying stuff like that)
// also because i want to get better at rust cuz i lowkey suck at it :/

#![allow(mixed_script_confusables)]

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
include!("wordle.rs");


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


pub struct Partitions {
	pub partitions: [Option<(WordleClue, Box<[WordleAnswer]>)>; NUM_WORDLE_CLUES],
	num_partitions: usize,
	is_sorted: bool
}
impl Partitions {
	pub fn new(trial_word: WordleWord, prev_possible_answers: &[WordleAnswer]) -> Self {
		let partitions = {
			// ah yes, rust
			const INIT: Option<(WordleClue, Vec<WordleAnswer>)> = None::<(WordleClue, Vec<WordleAnswer>)>;
			let mut arr = [INIT; NUM_WORDLE_CLUES];
			
			for &answer in prev_possible_answers {
				let clue = get_clues(trial_word, answer);
				match &mut arr[usize::from(clue)] {
					Some(v) => v.1.push(answer),
					None => arr[usize::from(clue)] = Some((clue, vec![answer])),
				}
			}
			
			arr.map(
				|v| v.map(
					|(c, v)| (c, v.into_boxed_slice())
				)
			)
		};
		
		let size = partitions.iter().filter(|&x| x.is_some()).count();
		
		Self {
			partitions: partitions,
			num_partitions: size,
			is_sorted: false
		}
	}
	
	pub fn num(&self) -> usize {
		self.num_partitions
	}
	
	pub fn sort(&mut self) {
		if self.is_sorted {return;}
		
		// sort partitions by size
		self.partitions.sort_by_key(
			|x| 
			x.as_ref()
			.map(|x| x.1.len())
			.unwrap_or(usize::MAX) // put all the `None`s at the end
		);
		
		self.is_sorted = true;
	}
}

pub struct PartitionIterator<'a> {
	partitions: &'a Partitions,
	current_index: usize,
}
impl<'a> Iterator for PartitionIterator<'a> {
	type Item = (WordleClue, &'a [WordleAnswer]);
	
	fn next(&mut self) -> Option<Self::Item> {
		if self.partitions.is_sorted {
			if self.current_index < self.partitions.num_partitions {
				let (clue, answers) = &self.partitions.partitions[self.current_index].as_ref().unwrap();
				
				self.current_index += 1;
				
				return Some((*clue, answers));
			} else {
				return None
			}
		}
		
		for i in self.current_index..243 {
			if let Some((clue, answers)) = &self.partitions.partitions[i] {
				self.current_index = i + 1;
				return Some((*clue, answers));
			}
		}
		
		return None;
	}
}
impl<'a> IntoIterator for &'a Partitions {
	type Item = (WordleClue, &'a [WordleAnswer]);
	type IntoIter = PartitionIterator<'a>;
	
	fn into_iter(self) -> Self::IntoIter {
		PartitionIterator {
			partitions: self,
			current_index: 0,
		}
	}
}


include!("total_guesses.rs");

const WORDLE_GUESSES: usize = 6;


pub fn minoverwords<const HARD_MODE: bool>(
	guessable_words: &[WordleWord], 
	possible_answers: &[WordleAnswer], 
	remaining_guesses: u8, 
	mut β: GuessTotal
) -> GuessTotal {
	if remaining_guesses == 0 {return GuessTotal::Infinity;}
	else if possible_answers.is_empty() {panic!("no possible answers");}
	else if guessable_words.is_empty() {panic!("no guessable words");}
	else if possible_answers.len() == 1 {
		// if there is only one possible answer, we can just guess it
		return 1.into();
	}
	else if GuessTotal::Number(2*possible_answers.len()as u16-1) >= β {return β;}
	else if possible_answers.len() == 2 {
		// this is because in the two possibilities, the best you can do is just guess one of them
		// making one possible answer have a score of 1 and the other have a score of 2
		if remaining_guesses > 1 {
			return 3.into();
		} else {
			return GuessTotal::Infinity; // we can't guess both
		}
	}
	else if possible_answers.len() == 3 {
		// if there are three possibilities, there are different cases
		// 1. num_guesses >= 2 ∃word∈possible_answers s.t. the clues for each answer are all different
		//        in this case, the splitting answer has a score of 1, and the others have 2, making the score 5
		// 2. num_guesses >= 3:
		//        just guess all the answers in order, making the three possibilities have scores 1, 2, and 3
		//        making the score 6, which is the same as case 3, but without a full loop!
		// 3. num_guesses >= 2 and ∃word∈guessable_words s.t the clues for each answer are all different
		//	      in this case, all the answers have a score of 2, making the score 6
		
		// check for case 1
		// NOTE: we know that the first answer will be GGGGG when we guess it (which MUST be different
		//       from the other answers), so we don't need to check for that
		if get_clues(possible_answers[0].into(), possible_answers[1]) != get_clues(possible_answers[0].into(), possible_answers[2])
		|| get_clues(possible_answers[1].into(), possible_answers[0]) != get_clues(possible_answers[1].into(), possible_answers[2])
		|| get_clues(possible_answers[2].into(), possible_answers[0]) != get_clues(possible_answers[2].into(), possible_answers[1]) {
			return 5.into();
		}
		
		// check for case 2
		// if we have enough guesses to just guess all the answers, just do that
		if remaining_guesses >= 3 {return 6.into();}
		
		// if we are able to split the answers and get them all in 2, then we can just return 6
		for &word in guessable_words {
			let (c1, c2, c3) = (
				get_clues(word, possible_answers[0]),
				get_clues(word, possible_answers[1]),
				get_clues(word, possible_answers[2]),
			);
			
			if c1 != c2 && c1 != c3 && c2 != c3 {
				return 6.into();
			}
		}
		
		// otherwise, we can't guess all the answers without running out of guesses
		return GuessTotal::Infinity;
	}
	
	for &guess in guessable_words {
		β = sumoverpartitions::<HARD_MODE>(
				guessable_words,
				possible_answers, 
				remaining_guesses-1,
				guess, 
				β
			);
	}
	
	β
}

pub fn minoverwords_fast_bound(
	possible_answers: &[WordleAnswer], 
	remaining_guesses: u8, 
	β: GuessTotal
) -> Option<GuessTotal> {
	if remaining_guesses == 0 {return Some(GuessTotal::Infinity);}
	else if possible_answers.is_empty() {panic!("no possible answers");}
	else if possible_answers.len() == 1 {
		// if there is only one possible answer, we can just guess it
		return Some(1.into());
	}
	else if GuessTotal::Number(2*possible_answers.len()as u16-1) >= β {return Some(β);}
	else if possible_answers.len() == 2 {
		// this is because in the two possibilities, the best you can do is just guess one of them
		// making one possible answer have a score of 1 and the other have a score of 2
		if remaining_guesses > 1 {
			return Some(3.into());
		} else {
			return Some(GuessTotal::Infinity); // we can't guess both
		}
	}
	else if possible_answers.len() == 3 {
		// if there are three possibilities, there are different cases
		// 1. num_guesses >= 2 ∃word∈possible_answers s.t. the clues for each answer are all different
		//        in this case, the splitting answer has a score of 1, and the others have 2, making the score 5
		// 2. num_guesses >= 3:
		//        just guess all the answers in order, making the three possibilities have scores 1, 2, and 3
		//        making the score 6, which is the same as case 3, but without a full loop!
		
		// check for case 1
		// NOTE: we know that the first answer will be GGGGG when we guess it (which MUST be different
		//       from the other answers), so we don't need to check for that
		if get_clues(possible_answers[0].into(), possible_answers[1]) != get_clues(possible_answers[0].into(), possible_answers[2])
		|| get_clues(possible_answers[1].into(), possible_answers[0]) != get_clues(possible_answers[1].into(), possible_answers[2])
		|| get_clues(possible_answers[2].into(), possible_answers[0]) != get_clues(possible_answers[2].into(), possible_answers[1]) {
			return Some(5.into());
		}
		
		// check for case 2
		// if we have enough guesses to just guess all the answers, just do that
		if remaining_guesses >= 3 {return Some(6.into());}
		
		return None;
	}
	
	return None;
}

pub fn sumoverpartitions<const HARD_MODE: bool>(
	guessable_words: &[WordleWord], 
	possible_answers: &[WordleAnswer],
	remaining_guesses: u8,
	guess: WordleWord,
	β: GuessTotal
) -> GuessTotal {
	let mut partitions = Partitions::new(guess, possible_answers);
	
	let mut total_lower_bound: GuessTotal = GuessTotal::Number(0);
	
	for (_clue, partition) in &partitions {
		if partition.is_empty() {continue;}
		if _clue == WordleClue::GGGGG {total_lower_bound+=GuessTotal::Number(1); continue;}
		
		match minoverwords_fast_bound(partition, remaining_guesses, β - total_lower_bound - partition.len() as u16) {
			Some(x) => {
				total_lower_bound += x + partition.len() as u16;
			},
			None => {
				// minoverwords(possible_answers=H) ≥ 2|H|-1
				// TODO: justify this (better) lower bound
				total_lower_bound += (3*partition.len() - 1) as u16;
			},
		}
		
		if total_lower_bound > β {return β;}
	}
	
	if remaining_guesses>=4 {println!("{guess:?} lower bound = {total_lower_bound}");}
	
	let new_guessable_words = if HARD_MODE {
		todo!("implement hard mode")
	} else {
		guessable_words
	};
	
	let mut total: GuessTotal = 0.into();
	
	partitions.sort();
	
	for (_clue, partition) in &partitions {
		if _clue == WordleClue::GGGGG {continue;}
		if remaining_guesses>=4 {print!("{:?}: ...\r", _clue);std::io::Write::flush(&mut std::io::stdout()).unwrap();}
		
		let x = minoverwords::<HARD_MODE>(
			new_guessable_words,
			partition.as_ref(), 
			remaining_guesses, 
			β
		);
		if remaining_guesses>=4 {println!("  {_clue:?} {x} {}", 3*partition.len()-1);}
		
		total += x;
		
		if total >= β {return β;}
	}
	
	if remaining_guesses>=4 {println!("{guess:?} actual = {}", total + possible_answers.len() as u16);}
	
	// W = Guessable words, H = possible answers
	// f(H) = |H| + min_{g∈W} ∑_{c∈C|c≠GGGGG} f(H.partition(g,c))
	total + possible_answers.len() as u16
}


pub fn best_word(state: &WordleState) -> (WordleWord, GuessTotal) {
	if state.current_entry == 0 {
		if state.hard_mode {
			(WordleWord::SALET, 8122.into())
		} else {
			(WordleWord::SALET, 7920.into())
		}
	} else {
		let possible_answers = (0..NUM_WORDLE_ANSWERS)
			.map(|i| WordleAnswer::from(i))
			.filter(|&a| state.is_possible_answer(a))
			.collect::<Box<[WordleAnswer]>>();
		if possible_answers.is_empty() {panic!("no possible answers");}
		
		if state.hard_mode {panic!("todo")}
		let guessable_words = (0..NUM_WORDLE_WORDS)
			.map(|i| WordleWord::from(i))
			.collect::<Box<[WordleWord]>>();
		if guessable_words.is_empty() {panic!("no guessable words")}
		
		let mut β = GuessTotal::Infinity;
		let mut best_word = WordleWord::from(0);
		
		let remaining_guesses = (WORDLE_GUESSES - state.current_entry) as u8;
		
		for &guess in guessable_words.as_ref() {
			print!("{:?}: {}\r", guess, β);
			std::io::Write::flush(&mut std::io::stdout()).unwrap();
			
			let b = sumoverpartitions::<false>(
					guessable_words.as_ref(),
					possible_answers.as_ref(), 
					remaining_guesses-1,
					guess, 
					β
				);
			
			if b < β {
				β = b;
				best_word = guess;
				println!("{:?}: {}", guess, β);
			}
		}
		
		(best_word, β)
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