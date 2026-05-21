//! Document your crate :-)

#![no_std]

extern crate alloc;

pub use defs::*;
// pub use fuzzer::*;
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
	use crate::{nonterminal_start};
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

	// #[test]
	// fn c_can_generate_and_execute() {
	// 	extern crate std;
	//
	// 	let num_tests = 10;
	// 	let mut compiled = 0;
	// 	for _ in 0..num_tests {
	// 		std::println!("============================");
	// 		match gen_and_compile_one() {
	// 			Ok(true) => compiled += 1,
	// 			Ok(false) => (),
	// 			Err(_) => {
	// 				std::println!("Issues setting up gcc.")
	// 			}
	// 		}
	// 	}
	//
	// 	std::println!("Of {}, {} compiled.", num_tests, compiled);
	// }
}

// pub mod fuzzer {
// 	extern crate std;
//
// 	use alloc::string::String;
// 	use alloc::vec::Vec;
// 	use anyhow::Error;
// 	use fandango::tuple_list::tuple_list;
// 	use fandango_core::generation::Generated;
// 	use fandango_runtime::operators::DepthLimiter;
// 	use rand::SeedableRng;
// 	use rand::rngs::StdRng;
// 	use std::process::{Command, Stdio};
// 	use fandango_core::visitor::Visitor;
// 	use fandango_core::visitor::write::WriteVisitor;
// 	use crate::defs::*;
//
// 	pub fn gen_and_compile_one() -> Result<bool, Error> {
// 		// First, generate a program.
// 		let generator = DepthLimiter::new(STRUCTURE.inner(), 10);
// 		let mut generators = tuple_list!(generator);
// 		let mut rng = StdRng::from_os_rng(); // So you get a different program each time.
//
// 		let generated =
// 			nonterminal_start::generate(&mut rng, &mut generators, 0);
//
// 		// We also want the string later, grab it now.
// 		let generated_as_str = String::from_utf8(
// 			WriteVisitor::new(Vec::new())
// 				.visit(&generated, 0)?
// 				.continue_value()
// 				.unwrap()
// 				.output()
// 		)?;
//
// 		// Compile it.
// 		// Configure this based on whichever C compiler you have access to.
// 		// This is just a sample configuration for invoking gcc like `gcc -x c -o /dev/ull - <stdin>`
// 		let process_or_not = Command::new("gcc")
// 			.arg("-x")
// 			.arg("c")
// 			.arg("-o")
// 			.arg("/dev/null")
// 			.arg("-")
// 			.stdin(Stdio::piped())
// 			.stdout(Stdio::piped())
// 			.stderr(Stdio::piped())
// 			.spawn();
//
// 		let mut process = match process_or_not {
// 			Ok(p) => p,
// 			Err(e) => {
// 				std::println!("Failed to spawn gcc process: {e}");
// 				return Err(e.into());
// 			}
// 		};
//
// 		let stdin = process.stdin.as_mut().expect("Failed to open stdin");
// 		use std::io::Write;
// 		// Add stdio manually so things can compile.
// 		writeln!(stdin, "#include <stdio.h>")?;
// 		writeln!(stdin)?;
// 		// Wrap this in a main function.
// 		stdin
// 			.write_all(
// 				&WriteVisitor::new(Vec::new())
// 					.visit(&generated, 0)?
// 					.continue_value()
// 					.unwrap()
// 					.output(),
// 			)?;
// 		// Also add a main function that returns 0 to make it a valid C program.
// 		writeln!(stdin)?;
// 		writeln!(stdin, "int main() {{ return 0; }}")?;
//
// 		let output = process
// 			.wait_with_output()
// 			.expect("Failed to read gcc output");
//
// 		if output.status.success() {
// 			std::println!("Successfully compiled:");
// 			std::println!(
// 				"{}", generated_as_str
// 			);
//
// 			Ok(true)
// 		} else {
// 			std::println!("Failed to compile:");
// 			std::println!("GCC exit code: {}", output.status);
// 			std::println!("GCC stdout: {}", String::from_utf8_lossy(&output.stdout));
// 			std::println!("GCC stderr: {}", String::from_utf8_lossy(&output.stderr));
// 			std::println!(
// 				"{}", generated_as_str
// 			);
//
// 			Ok(false)
// 		}
// 	}
// }
