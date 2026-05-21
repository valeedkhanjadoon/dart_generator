//! Document your crate :-)
//! To run the fuzzer: RUSTFLAGS=-Znext-solver cargo +nightly test dart_can_generate_and_execute -- --no-capture

#![no_std]

extern crate alloc;

pub use defs::*;
pub use fuzzer::*;
mod defs {
	use core::convert::Infallible;
	use fandango::Fandango;

	/// Base for the grammar
	#[derive(Fandango)]
	#[fandango(grammar = "grammars/dart2.fan", parse = false)]
	pub struct LanguageName(Infallible);

}

#[cfg(test)]
mod test {
	// Import any non-terminals you like from crate. Crate refers to your local crate.
	use crate::{nonterminal_start, gen_and_execute_one};
	use fandango::typing::Structured;
	use fandango::generation::Generated;

	// These are the fandango-rs imports. A later lesson will go over them in more detail.
	use fandango::tuple_list::tuple_list;
	use fandango::visitor::write::WriteVisitor;
	use fandango::visitor::Visitor;
	use fandango_runtime::operators::{DepthLimiter};
	use alloc::string::String;
	use alloc::vec::Vec;
	use rand::SeedableRng;
	use rand::rngs::StdRng;

	#[test]
	fn dart_lang_can_generate() {
		// Because we want access to the standard library in this test:
		extern crate std;
		// Set up the various RNGs used for generation:
		let mut rng = StdRng::seed_from_u64(0);
		// tuple_list! is a macro that expands the supplied arguments into a tuple list. this is just the creation pattern for generators.
		// This indicates we want to limit the depth of the created derivation trees w.r.t. the start symbol, to avoid things getting too crazy
		let mut generators =
			tuple_list!(DepthLimiter::new(nonterminal_start::ROOT.inner(), 50));
		// Just generate 10 to see that it works
		for _ in 0..10 {
			// Generate an input based on the start symbol.
			let generated =
				nonterminal_start::generate(&mut rng, &mut generators, 0);
			// Print the input.
			// Yup, everything's a visitor!
			std::println!(
				"{}",
				String::from_utf8(
					WriteVisitor::new(Vec::new())
						.visit(&generated, 0)
						.unwrap()
						.continue_value()
						.unwrap()
						.output()
				).unwrap()
			);
		}
	}

	#[test]
	fn dart_can_generate_and_execute() {
		extern crate std;

		// Calculate a unique timestamped file path for logging
		let timestamp = std::time::SystemTime::now()
			.duration_since(std::time::UNIX_EPOCH)
			.unwrap()
			.as_secs();
		let log_dir = std::path::Path::new("fuzzer_runs");
		let _ = std::fs::create_dir_all(log_dir);
		let log_path = log_dir.join(std::format!("run_{}.txt", timestamp));

		let num_tests = 100;
		let mut executed = 0;
		for _ in 0..num_tests {
			std::println!("============================");
			match gen_and_execute_one(Some(&log_path)) {
				Ok(true) => executed += 1,
				Ok(false) => (),
				Err(_) => {
					std::println!("Issues setting up Dart.")
				}
			}
		}

		std::println!("Of {}, {} executed.", num_tests, executed);
		std::println!("Fuzzer run log saved to: {}", log_path.display());

		// Append final fuzzer execution summary to the log file
		if let Ok(mut file) = std::fs::OpenOptions::new()
			.create(true)
			.append(true)
			.open(&log_path)
		{
			use std::io::Write;
			let success_rate = (executed as f64 / num_tests as f64) * 100.0;
			let _ = writeln!(file, "=========================================");
			let _ = writeln!(file, "SUMMARY:");
			let _ = writeln!(file, "Total programs generated: {}", num_tests);
			let _ = writeln!(file, "Successfully compiled/executed: {} / {}", executed, num_tests);
			let _ = writeln!(file, "Success rate: {:.2}%", success_rate);
			let _ = writeln!(file, "=========================================");
		}
	}
}

pub mod fuzzer {
	extern crate std;

	use alloc::string::String;
	use alloc::vec::Vec;
	use anyhow::Error;
	use fandango::tuple_list::tuple_list;
	use fandango_core::generation::Generated;
	use fandango_runtime::operators::DepthLimiter;
	use rand::SeedableRng;
	use rand::rngs::StdRng;
	use std::process::{Command, Stdio};
	use fandango_core::visitor::Visitor;
	use fandango_core::visitor::write::WriteVisitor;
	use crate::defs::*;

	struct TempFile {
		path: std::path::PathBuf,
	}

	impl Drop for TempFile {
		fn drop(&mut self) {
			let _ = std::fs::remove_file(&self.path);
		}
	}

	pub fn gen_and_execute_one(log_path: Option<&std::path::Path>) -> Result<bool, Error> {
		// First, generate a program.
		let generator = DepthLimiter::new(STRUCTURE.inner(), 10);
		let mut generators = tuple_list!(generator);
		let mut rng = StdRng::from_os_rng(); // So you get a different program each time.

		let generated =
			nonterminal_start::generate(&mut rng, &mut generators, 0);

		let generated_as_str = String::from_utf8(
			WriteVisitor::new(Vec::new())
				.visit(&generated, 0)?
				.continue_value()
				.unwrap()
				.output()
		)?;

		// Get standard temporary directory path and generate a filename.
		let temp_dir = std::env::temp_dir();
		let file_path = temp_dir.join("fuzzed_program.dart");

		// Write fuzzed program to temporary file
		std::fs::write(&file_path, &generated_as_str)?;

		// Ensure automatic cleanup using RAII
		let _cleanup = TempFile { path: file_path.clone() };

		// Compile and run the temporary file using Dart
		let process_or_not = Command::new("dart")
			.arg("run")
			.arg(&file_path)
			.stdout(Stdio::piped())
			.stderr(Stdio::piped())
			.spawn();

		let process = match process_or_not {
			Ok(p) => p,
			Err(e) => {
				std::println!("Failed to spawn Dart process: {e}");
				return Err(e.into());
			}
		};

		let output = process
			.wait_with_output()
			.expect("Failed to read Dart output");

		let is_success = output.status.success();

		if let Some(path) = log_path {
			use std::io::Write;
			let mut file = std::fs::OpenOptions::new()
				.create(true)
				.append(true)
				.open(path)?;

			writeln!(file, "=========================================")?;
			writeln!(file, "Generated Program:")?;
			writeln!(file, "------------------")?;
			writeln!(file, "{}", generated_as_str)?;
			writeln!(file, "------------------")?;
			if is_success {
				writeln!(file, "Result: SUCCESS")?;
			} else {
				writeln!(file, "Result: FAILED")?;
				writeln!(file, "Exit Code: {}", output.status)?;
				writeln!(file, "Stdout:\n{}", String::from_utf8_lossy(&output.stdout))?;
				writeln!(file, "Stderr:\n{}", String::from_utf8_lossy(&output.stderr))?;
			}
			writeln!(file, "=========================================\n")?;
		}

		if is_success {
			std::println!("Successfully executed:");
			std::println!(
				"{}", generated_as_str
			);

			Ok(true)
		} else {
			std::println!("Failed to execute:");
			std::println!("Dart exit code: {}", output.status);
			std::println!("Dart stdout: {}", String::from_utf8_lossy(&output.stdout));
			std::println!("Dart stderr: {}", String::from_utf8_lossy(&output.stderr));
			std::println!(
				"{}", generated_as_str
			);

			Ok(false)
		}
	}
}