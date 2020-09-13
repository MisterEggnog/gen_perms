
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
