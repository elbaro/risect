pub struct Command {
	// fn new_shell(cmd: String) -> Command{  // cmd: "echo 3"
	// }
	pub cmds: Vec<String>,
}

impl Command {
	pub fn render(
		&self,
		params: &std::collections::HashMap<String, String>,
	) -> std::process::Command {
		let cmds: Vec<String> = self
			.cmds
			.iter()
			.map(|s| {
				let mut s = std::borrow::Cow::from(s);
				for (from, to) in params {
					s = s.replace(from, to).into();
				}
				s.into()
			})
			.collect();
		let mut c = std::process::Command::new(&cmds[0]);
		c.args(&cmds[1..]);
		c
	}
}
