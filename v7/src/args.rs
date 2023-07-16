use crate::{
	about::{NAME, VERSION},
	errors::{InternalError, ArgumentError},
	id::Id
};
use std::{process::exit, path::PathBuf};

#[allow(unused)]
#[derive(Debug)]
pub struct ParsedArgs {
	pub exec_path: PathBuf,
	pub flags: Vec<Id>,
	pub main_path: PathBuf,
	pub args: Vec<Id>
}

pub fn parse_args() -> ParsedArgs {
	let mut raw_args = std::env::args_os().enumerate();

	let exec_path: PathBuf = match raw_args.next() {
		Some((_, os_string)) => PathBuf::from(os_string),
		None => InternalError!("raw_args is empty")
	};

	let mut flags: Vec<Id> = Vec::new();

	let mut main_path = PathBuf::new();

	for (i, raw_arg) in &mut raw_args {
		let flag: &str = match raw_arg.to_str() {
			Some(flag) => flag,
			None => ArgumentError!("argument {} has invalid unicode", i + 1)
		};

		if !flag.starts_with('-') {
			main_path.push(raw_arg);
			break;
		}

		match flag {
			"-v" | "--version" => {
				let arg_count = raw_args.len();
				if arg_count > 0 {
					println!("# unused {} extra arguments", arg_count);
				}
				println!("{NAME} {VERSION}");
				exit(0);
			}
			"--test" => flags.push(Id::from(flag)),
			_ => ArgumentError!("invalid flag {:?}", flag)
		}
	}

	if main_path.as_os_str().is_empty() {
		// TODO: interactive mode
		ArgumentError!("missing file path (insert REPL here)");
	}

	let mut args: Vec<Id> = Vec::new();

	for (i, arg) in &mut raw_args {
		args.push(match arg.to_str() {
			Some(arg) => Id::from(arg),
			None => ArgumentError!("argument {} has invalid unicode", i + 1)
		})
	}

	ParsedArgs {
		exec_path,
		flags,
		main_path,
		args
	}
}