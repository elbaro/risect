mod cli;
mod command;

fn fancy_print(candidates: &Vec<String>, results: &Vec<Option<bool>>, center: usize) {
	if candidates.is_empty() {
		println!("nothing to test");
		return;
	}

	use colored::*;
	let first = (center as isize - 5).max(0) as usize;
	let last = (center as isize + 5).min(candidates.len() as isize - 1) as usize;

	for i in first..=last {
		let marker = if i == center { "  <-  found" } else { "" };
		match results[i] {
			None => {
				println!("[   ] {}{}", candidates[i], marker);
			}
			Some(true) => {
				println!("{}", format!("[ V ] {}{}", candidates[i], marker).green());
			}
			Some(false) => {
				println!("{}", format!("[ X ] {}{}", candidates[i], marker).red());
			}
		}
	}

	if center > last {
		println!("There is no failure");
	}
}

fn main() -> Result<(), anyhow::Error> {
	let (candidates, cmd) = cli::parse()?;
	let mut results = Vec::with_capacity(candidates.len());
	results.resize(candidates.len(), None);

	eprintln!("# of candidates: {}", candidates.len());
	eprintln!("   test command: {:?}", cmd.cmds);

	let mut left = 0;
	let mut right = candidates.len();
	while left < right {
		let mid = (left + right) / 2;

		eprintln!("================");
		eprintln!("candidate: {}", candidates[mid]);

		let mut params = std::collections::HashMap::new();
		params.insert("{}".to_string(), candidates[mid].clone());
		let mut c = cmd.render(&params);
		c.stdin(std::process::Stdio::null());
		c.stdout(std::process::Stdio::null());
		c.stderr(std::process::Stdio::null());
		eprintln!("  command: {:?}", c);
		let status = c.status()?;
		results[mid] = Some(status.success());

		eprintln!(" success?: {}", status.success());
		if status.success() {
			left = mid + 1;
		} else {
			right = mid;
		}
	}

	// fancy print
	fancy_print(&candidates, &results, left);

	Ok(())
}
