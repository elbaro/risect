use clap::{App, AppSettings, Arg, ArgGroup};
use std::io::Read;
// use indoc::indoc;

pub fn parse() -> Result<(Vec<String>, crate::command::Command), anyhow::Error> {
	let app = build_cli();
	let matches = app.get_matches();
	let mut candidates = Vec::<String>::new();

	if matches.is_present("--git") {
	} else if matches.is_present("--date") {
	} else if matches.is_present("--custom") {
	} else {
		// --stdin
		let mut s = String::new();
		std::io::stdin().read_to_string(&mut s)?;
		candidates = s.lines().map(|s| s.to_string()).collect();
	}

	let cmd_vec: Vec<String> = matches
		.values_of("test_cmd")
		.unwrap()
		.map(|s| s.to_string())
		.collect();
	let cmd = crate::command::Command { cmds: cmd_vec };
	return Ok((candidates, cmd));
}

fn build_cli() -> App<'static, 'static> {
	/**
	* git-rev aaaa...bbbb | ./cmd -- test-cmd arg1 arg2  (defaults stdin)
	* ./cmd --custom "git-rev aaaa...bbbb" -- test-cmd arg1 arg2
	* ./cmd --git aaaa bbbb -- test-cmd arg1 arg2
	*
	*
	* ./cmd --git aaaaaa bbbbbb
	* ./cmd --git master~5 master
	* ./cmd --git --from aaaaaa --to bbbbbb
	* ./cmd --git --from-date 2020-03-21 --to bbbbbbx

	* ./cmd --git ../dir1 aaaaaa bbbbbb
	* ./cmd --git https://.../...git aaaaaa bbbbbb
	* ./cmd --git

	* ./cmd --date 2020-04-05 2020-06-20
	* ./cmd --date
	* ./cmd --int

	* ./cmd --custom "git-rev aaaaaa...bbbbbb"
	* ./cmd --custom "seq 0.1 0.1 0.4"
	*/
	App::new("risect")
		.version(clap::crate_version!())
		.author("github.com/elbaro/risect")
		.about("???")
		.arg(Arg::with_name("stdin").long("stdin"))
		.arg(Arg::with_name("custom").long("custom").takes_value(true))
		.arg(Arg::with_name("git").long("git"))
		.arg(Arg::with_name("date").long("date"))
		.arg(
			Arg::with_name("parallel")
				.long("parallel")
				.takes_value(true),
		)
		.arg(Arg::with_name("verbose").long("verbose"))
		.arg(Arg::with_name("test_cmd").required(true).multiple(true))
}
