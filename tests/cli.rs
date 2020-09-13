/*
See end of file for license details.
*/
extern crate gen_perms;
use std::io::Cursor;
use std::collections::HashSet;

#[test]
fn generates_all_permutations() -> Result<(), Box<dyn std::error::Error>> {
	let mut buff = Cursor::new(Vec::new());
	gen_perms::calculate("test", &mut buff)?;
	let perms_str = buff.into_inner().into_iter().map(|c| std::char::from_u32(c as u32).unwrap()).collect::<String>();
	let perms: HashSet<&str> = perms_str.split('\n').filter(|r| !r.is_empty()).collect();

	let hand_calculated_perms = {
		let mut perms = HashSet::with_capacity(24);

		perms.insert("test");
		perms.insert("tset");
		perms.insert("ttes");
		perms.insert("estt");
		perms.insert("sett");
		perms.insert("tets");
		perms.insert("tste");
		perms.insert("etst");
		perms.insert("etts");
		perms.insert("ttse");
		perms.insert("stet");
		perms.insert("stte");

		assert_eq!(12, perms.len());
		perms
	};

	assert_eq!(perms, hand_calculated_perms);

	Ok(())
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
