
use std::collections::BTreeSet;
use rand::prelude::*;
use rand::seq::SliceRandom;
use unicode_segmentation::UnicodeSegmentation;

pub fn cal_perms() {
	let s = std::env::args().nth(1).expect("String must be provided");
	let mut group = UnicodeSegmentation::graphemes(s.as_str(), true).collect::<Vec<&str>>();
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
