include!("new_constants.rs");

use std::lazy::SyncOnceCell;

pub const fn get_clues_uncached(guess: &[u8], answer: &[u8]) -> WordleClueEnum {
	debug_assert!(guess.len() == 5);
    debug_assert!(answer.len() == 5);
	
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
	
	WordleClueEnum::from(g + y) // since these have disjoint digits in base 3, we can add them together
}


// TODO: should this just be `thread_local!` instead of `SyncOnceCell`?
type ClueCache = [[WordleClueEnum; NUMBER_OF_ANSWERS]; NUMBER_OF_WORDS];
pub static CLUE_CACHE: SyncOnceCell<Box<ClueCache>> = SyncOnceCell::new();

fn initialize_clue_cache() -> Box<ClueCache> {
    // ClueCache is literally too big to put on the stack.
    // I know becaus`exit code: 0xc00000fd, STATUS_STACK_OVERFLOW`
    
    // TODO: just use `#![feature(new_uninit)]` and `new_zeroed()`
    
    // SAFETY: this is basically just a calloc call, so its safe (probably)
    let memory: *mut ClueCache = unsafe {
        std::alloc::alloc_zeroed(
            std::alloc::Layout::from_size_align(
                std::mem::size_of::<ClueCache>(),
                std::mem::align_of::<ClueCache>(),
            )
            .unwrap(),
        // SAFETY: this pointer is allocated with the size/align of *ClueCache so you can cast it to *ClueCache
        ) as *mut ClueCache
    };
    
    // SAFETY: this is pretty obviously fine
    let mut cache: Box<ClueCache> = unsafe {Box::from_raw(memory)};
    
    for i in 0..NUMBER_OF_WORDS {
        for j in 0..NUMBER_OF_ANSWERS {
            cache[i][j] = get_clues_uncached(WordleWordEnum::from(i).as_str().as_bytes(), WordleAnswerEnum::from(j).as_str().as_bytes());
        }
    }
    
    cache
}

pub fn get_clues(word: WordleWordEnum, answer: WordleAnswerEnum) -> WordleClueEnum {
    CLUE_CACHE.get_or_init(|| initialize_clue_cache())[word as usize][answer as usize]
}
