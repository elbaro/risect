use clap::Clap;
use clap::{App, AppSettings, Arg, ArgGroup};
use std::io::Read;
// use indoc::indoc;

#[derive(clap::Clap)]
#[clap(version = clap::crate_version!(), author = "d <f@gmail.com>")]
pub struct Opts {
	#[clap(long)]
	pub stdin: bool,
	#[clap(long)]
	pub custom: Option<String>,
	#[clap(long)]
	pub git: bool,
	#[clap(long)]
	pub date: bool,
	#[clap(long)]
	pub from: Option<String>,
	#[clap(long)]
	pub from_date: Option<String>,
	#[clap(long)]
	pub to: Option<String>,
	#[clap(long)]
	pub to_date: Option<String>,
	#[clap(long)]
	pub path: Option<String>,
	#[clap(long)]
	pub checkout: Option<bool>,
	#[clap(long, default_value = "1")]
	pub parallel: usize,
	#[clap(long)]
	pub verbose: bool,
	#[clap()]
	test_cmd: Vec<String>,
}

pub fn parse() -> Result<(Vec<String>, crate::command::Command), anyhow::Error> {
	let opts: Opts = Opts::parse();
	let mut candidates = Vec::<String>::new();

	// parse candidates

	if opts.git {
		assert!(opts.from.is_some() || opts.from_date.is_some());
		assert!(opts.to.is_some() || opts.to_date.is_some());
	} else if opts.date {
	} else if opts.custom.is_some() {
	} else {
		// --stdin
		let mut s = String::new();
		std::io::stdin().read_to_string(&mut s)?;
		candidates = s.lines().map(|s| s.to_string()).collect();
	}

	// parse command
	let cmd = crate::command::Command {
		cmds: opts.test_cmd.clone(),
	};
	return Ok((candidates, cmd));
}

/*
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
