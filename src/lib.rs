
use std::collections::BTreeSet;
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
	}
	else {
		println!("Usage `{} str`", std::env::args().nth(0).unwrap());
		println!("Pass --help for more info");
		return;
	}
}

fn calculate(s: &str) {
	let mut group = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();
	let mut rand = thread_rng();

	let perm_num = (1..=group.len()).product();
	let mut permutations = BTreeSet::new();

	while permutations.len() != perm_num {
		group.shuffle(&mut rand);
		permutations.insert(group.iter().map(|s| *s).collect::<String>());
	}
	
	for permutation in permutations {
		println!("{}", permutation);
	}
}
