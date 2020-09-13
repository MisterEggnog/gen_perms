/*
See end of file for license details.
*/
use std::io::{self, Write, Result};
use std::collections::{HashSet, HashMap};
use rand::prelude::*;
use rand::seq::SliceRandom;
use unicode_segmentation::UnicodeSegmentation;

pub fn cal_perms() {
	if let Some(s) = std::env::args().nth(1) {
		if s == "--help" {
			println!("Pass string to get all permutations.");
			println!("This will generate n! / m1!m2!...mn! permutations");
			println!("Where n is the length of the string & mx is the number of times that number repeats.");
			println!("For example, the word `tree` would have 4!/2!, (simplifying the 1! operations)");
			println!("This can get large quickly.");
			return;
		}
		if let Err(_e) = calculate(&s, &mut io::stdout()) {
			// It is not an error if stdout is closed early.
		}
	} else {
		println!("Usage `{} str`", std::env::args().next().unwrap());
		println!("Pass --help for more info");
	}
}

pub fn calculate<W: Write>(s: &str, mut ofs: &mut W) -> Result<()> {
	let mut group = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();
	let mut rand = thread_rng();

	let perm_num = multinomial(&group);
	let mut permutations = HashSet::new();

	while permutations.len() != perm_num {
		group.shuffle(&mut rand);
		let permut = group.iter().copied().collect::<String>();

		if !permutations.contains(&permut) {
			writeln!(&mut ofs, "{}", permut)?;
			permutations.insert(permut);
		}
	}

	Ok(())
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
/*
	gen_perms - Generates all variations of a string.
    Copyright (C) 2020  Baldwin, Josiah

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
