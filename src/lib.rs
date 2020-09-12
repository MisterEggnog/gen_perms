
use std::collections::{HashSet, HashMap};
use rand::prelude::*;
use rand::seq::SliceRandom;
use unicode_segmentation::UnicodeSegmentation;

pub fn cal_perms() {
	if let Some(s) = std::env::args().nth(1) {
		if s == "--help" {
			println!("Pass string to get all permutations.");
			println!("You may want to check if these permutations would be a reasonable size,\ncurrently these produce n! strings, where n is the length of the string.");
			println!("Note, currently all characters must be distinct.");
			return;
		}
		calculate(&s);
	} else {
		println!("Usage `{} str`", std::env::args().next().unwrap());
		println!("Pass --help for more info");
	}
}

fn calculate(s: &str) {
	let mut group = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();
	let mut rand = thread_rng();

	let perm_num = multinomial(&group);
	let mut permutations = HashSet::new();

	while permutations.len() != perm_num {
		group.shuffle(&mut rand);
		let permut = group.iter().copied().collect::<String>();

		if !permutations.contains(&permut) {
			println!("{}", permut);
			permutations.insert(permut);
		}
	}
}

fn factorial(n: usize) -> usize {
	(1..=n).product()
}

fn multinomial(strs: &[&str]) -> usize {
	let top = factorial(strs.len());
	let bottom: usize = {
		let mut counts = HashMap::new();
		for grph in strs {
			*counts.entry(grph).or_insert(0) += 1;
		}
		counts.iter().map(|c| factorial(*c.1)).product()
	};
	top / bottom
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn multinomial_is_correct() {
		let word = UnicodeSegmentation::graphemes("mississippi", true).collect::<Vec<&str>>();
		assert_eq!(multinomial(&word), 34650);
	}
}
