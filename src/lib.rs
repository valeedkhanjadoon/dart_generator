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

		let num_tests = 100;
		let mut executed = 0;
		for _ in 0..num_tests {
			std::println!("============================");
			match gen_and_execute_one() {
				Ok(true) => executed += 1,
				Ok(false) => (),
				Err(_) => {
					std::println!("Issues setting up Dart.")
				}
			}
		}

		std::println!("Of {}, {} executed.", num_tests, executed);
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

	pub fn gen_and_execute_one() -> Result<bool, Error> {
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

		if output.status.success() {
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