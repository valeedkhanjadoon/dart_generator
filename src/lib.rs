//! Document your crate :-)

#![no_std]

extern crate alloc;

pub use defs::*;
pub use fuzzer::*;
mod defs {
	use core::convert::Infallible;
	use fandango::Fandango;
	use core::ops::ControlFlow;
	use alloc::collections::VecDeque;
	use alloc::vec::Vec;
	use alloc::string::String;
	use num_rational::Ratio;

	// Explicit traits required for the visitor pattern implementations
	use fandango_core::visitor::{Visitor, VisitableChildren, VisitResult};
	use fandango_core::visitor::write::WriteVisitor;
	use fandango_core::typing::{AsNodeRef, ChildAccessor, Downcast, Node, Nth, Opaque};
	use fandango_runtime::operators::{Checker};
	use fandango_runtime::measurement::{Violations};

	/// Base for the grammar stored in scriptsizec.fan.
	#[derive(Fandango)]
	#[fandango(grammar = "grammars/scriptsizec.fan", parse = false)]
	pub struct LanguageName(Infallible);

	#[derive(Default)]
	pub struct HypotheticalWhileLoopConstraint {
		path: VecDeque<usize>, // "breadcrumb trail" of the path taken to the current node
		violations: Vec<VecDeque<usize>>, // List of paths to violations
		checked: usize // Number of nodes visited; how many times the constraint has been evaluated.
	}

    // Arbitrary Letter as Variable Name
	#[derive(Default)]
	pub struct SingleVariableConstraint {
		path: VecDeque<usize>,
		violations: Vec<VecDeque<usize>>,
		checked: usize,
		// Store the first variable name we see in expected_variable (e.g. "a", "b")
		expected_variable: Option<String>,
	}

	impl SingleVariableConstraint {
		fn check_id(&mut self, id_node: &nonterminal_id, path_extension: impl IntoIterator<Item = usize>) {
			self.checked += 1;

			// Extract the literal string value of the ID node
			let id_str = String::from_utf8(
				WriteVisitor::new(Vec::new())
					.visit(id_node, 0)
					.unwrap()
					.continue_value()
					.unwrap()
					.output()
			).unwrap();

			match &self.expected_variable {
				Some(expected) => {
					if expected != &id_str {
						// Mismatch! Record the exact path to this violating node
						let mut violation_path = self.path.clone();
						violation_path.extend(path_extension);
						self.violations.push(violation_path);
					}
				}
				None => {
					// This is the very first variable we have seen, set the standard!
					self.expected_variable = Some(id_str);
				}
			}
		}
	}

	impl<T> Visitor<T> for SingleVariableConstraint
	where
		T: VisitableChildren<T>
	+ AsNodeRef<nonterminal_declaration>
	+ AsNodeRef<nonterminal_expr>
	+ AsNodeRef<nonterminal_term>
	+ AsNodeRef<nonterminal_id> // We still need this to pass it to our helper
	{
		type Continue = Self;
		type Break = Infallible;
		type Error = Infallible;

		fn visit<'program, N>(mut self, node: &'program N, idx: usize) -> VisitResult<Self, T>
		where
			N: Node<Type<'program> = T>,
			T: From<&'program N> + AsNodeRef<N>,
		{
			self.path.push_back(idx);
			let visited = node.opaque();

			// 1. Check Declarations
			if let Some(tree) = visited.downcast::<nonterminal_declaration>() {
				match tree.nth::<0>() {
					nonterminal_declaration_0::variant_0(inner) => {
						// | "int " <id> "=" <expr> ";"
						let (_, id_part, _, _, _) = inner.children();
						// Path: 0 (go inside), 0 (1st variant), 1 (2nd child is <id>)
						self.check_id(id_part, [0, 0, 1]);
					}
					nonterminal_declaration_0::variant_1(inner) => {
						// | "int " <id> ";"
						let (_, id_part, _) = inner.children();
						// Path: 0 (go inside), 1 (2nd variant), 1 (2nd child is <id>)
						self.check_id(id_part, [0, 1, 1]);
					}
					_ => {}
				}
			}

			// 2. Check Expressions
			if let Some(tree) = visited.downcast::<nonterminal_expr>() {
				if let nonterminal_expr_0::variant_0(inner) = tree.nth::<0>() {
					// | <id> "=" <expr>
					let (id_part, _, _) = inner.children();
					// Path: 0 (go inside), 0 (1st variant), 0 (1st child is <id>)
					self.check_id(id_part, [0, 0, 0]);
				}
			}

			// 3. Check Terms
			if let Some(tree) = visited.downcast::<nonterminal_term>() {
				if let nonterminal_term_0::variant_1(inner) = tree.nth::<0>() {
					// | <id>
					// Because this variant only has exactly one child, we use .child() !! This did not work !!
					// Because this variant is just a single non-terminal.
					// 'inner' IS the nonterminal_id! !! IMPORTANT !! No need to unwrap it further.
					let id_part = inner;
					// Path: 0 (go inside), 1 (2nd variant), 0 (1st child is <id>)
					self.check_id(id_part, [0, 1, 0]);
				}
			}

			let result = visited.visit_each(self);
			let Ok(ControlFlow::Continue(mut visitor)) = result;
			visitor.path.pop_back();
			Ok(ControlFlow::Continue(visitor))
		}
	}

	// Don't forget to implement the Checker trait so the Evolver knows how to grade it.
	impl Checker for SingleVariableConstraint {
		fn violations(self) -> Violations {
			Violations::new(
				if self.checked  != 0 {
					Ratio::new(self.checked - self.violations.len(), self.checked)
				} else {
					Ratio::default()
				},
				self.violations,
			)
		}
	}

	impl HypotheticalWhileLoopConstraint { // "Checking" magic or logic that is specific to this constraint.
		fn check_paren_expr_for_validity(p_expr : &nonterminal_paren_expr) -> bool {
			// Placeholder, for illustrative purposes.
			return true;
		}
	}

	impl<T> Visitor<T> for HypotheticalWhileLoopConstraint
	where
		T: VisitableChildren<T> // Basically always needed
		+ AsNodeRef<nonterminal_statement> // Need one for each nonterminal_ you reference
		+ AsNodeRef<nonterminal_paren_expr> // If you remove this, what happens?
	{
		type Continue = Self;
		type Break = Infallible;
		type Error = Infallible;

		fn visit<'program, N>(mut self, node: &'program N, idx: usize) -> VisitResult<Self, T>
		where
			N: Node<Type<'program> = T>,
			T: From<&'program N> + AsNodeRef<N>,
		{
			self.path.push_back(idx);
			let visited = node.opaque();

			// relevant part of the grammar, for reference
			// <statement> ::= <block>
			//               | "if " <paren_expr> <statement> " else " <statement>
			//               | "if " <paren_expr> <statement>
			//               | "while " <paren_expr> <statement>
			//               | "do " <statement> " while " <paren_expr> ";"
			//               | <expr> ";"
			//               | ";" ;
			if let Some(tree) = visited.downcast::<nonterminal_statement>() {
				match tree.nth::<0>() {
					nonterminal_statement_0::variant_3(inner) => {
						// | "while " <paren_expr> <statement>
						let (_, paren_part, _) = inner.children();
						self.checked += 1;
						if Self::check_paren_expr_for_validity(paren_part) {
							// Valid, nothing to do.
						} else {
							// Violation. Record the best "path" as a violation; these are used
							// as the basis for mutation and crossover.
							let mut path_to_violation = self.path.clone();
							// 0: go into statement
							// 3: third variant
							// 1: first part of variant -- the offending paren_expr
							path_to_violation.extend([0, 3, 1]);
							self.violations.push(path_to_violation);
						}
					}
					nonterminal_statement_0::variant_4(inner) => {
						// | "do " <statement> " while " <paren_expr> ";"
						let (_, _, _, paren_part, _) = inner.children();
						self.checked += 1;
						if Self::check_paren_expr_for_validity(paren_part) {
							// Valid, nothing to do.
						} else {
							// Violation. Record the best "path" as a violation; these are used
							// as the basis for mutation and crossover.
							let mut path_to_violation = self.path.clone();
							// 0: go into statement
							// 4: fourth variant
							// 3: third part -- the offending paren_expr
							path_to_violation.extend([0, 4, 3]);
							self.violations.push(path_to_violation);
						}
					}
					_ => {}
				}
			}

			let result = visited.visit_each(self);
			let Ok(ControlFlow::Continue(mut visitor)) = result;
			visitor.path.pop_back();
			Ok(ControlFlow::Continue(visitor))
		}
	}

	impl Checker for HypotheticalWhileLoopConstraint {
		fn violations(self) -> Violations {
			Violations::new(
				if self.checked != 0 {
					Ratio::new(self.checked - self.violations.len(), self.checked)
				} else {
					Ratio::default()
				},
				self.violations,
			)
		}
	}
}

#[cfg(test)]
mod test {
	// Import any non-terminals you like from crate. Crate refers to your local crate.
	use crate::{gen_and_compile_one, nonterminal_start};

	// These are the fandango-rs imports. A later lesson will go over them in more detail.
	use fandango::generation::Generated;
	use fandango::tuple_list::tuple_list;
	use fandango::typing::Structured;
	use fandango::visitor::write::WriteVisitor;
	use fandango::visitor::Visitor;
	use fandango_runtime::population::Individual;
	use fandango_runtime::operators::{Checker, DepthLimiter};
	use fandango_runtime::evolvers::Evolver;
	use fandango_runtime::evolvers::basic::BasicEvolver;
	use fandango_runtime::measurement::{HasFitness, ViolationFitness, HasMeasurement};
	use alloc::string::String;
	use alloc::vec::Vec;
	use num_rational::Ratio;
	use rand::SeedableRng;
	use rand::rngs::StdRng;

	#[test]
	fn c_lang_can_generate() {
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
	fn c_can_generate_and_compile() {
		extern crate std;

		let num_tests = 10;
		let mut compiled = 0;
		for _ in 0..num_tests {
			std::println!("============================");
			match gen_and_compile_one() {
				Ok(true) => compiled += 1,
				Ok(false) => (),
				Err(_) => {
					std::println!("Issues setting up gcc.")
				}
			}
		}

		std::println!("Of {}, {} compiled.", num_tests, compiled);
	}

	#[test]
	fn c_hypothetical_while_loop_constraint_in_evolver() {
		extern crate std;

		// Get the checker ready for fitness computation.
		let fitness = ViolationFitness::<crate::HypotheticalWhileLoopConstraint>::new();
		let fixer = (); // No fixers in this example.
		// Initialize the BasicEvolver (for only single constraints/fixers)
		let mut runtime = BasicEvolver::new::<nonterminal_start>(
			fitness,
			fixer,
			100,
			10,
			1000,
			Ratio::new(50, 100),
		)
			.expect("Should be valid.");

		// Set up stuff for the generator.
		let generator = DepthLimiter::new(crate::STRUCTURE.inner(), 10);
		let mut generators = tuple_list!(generator);
		let mut sampler = StdRng::from_os_rng();

		// Remember the way the evolutionary algorithm works: first create an initial pop, then solve constraints.
		let mut population = runtime.initial(&mut generators, &mut sampler).unwrap();

		// Allow 25 generations
		for i in 0..25 {
			let fitness = population
				.iter()
				.map(|i| i.measurement().fitness())
				.fold(0.0f64, |v, r| v + *r.numer() as f64 / *r.denom() as f64)
				/ population.len() as f64;
			if fitness == 1.0 {
				std::println!("saturated fitness at generation {i}");
				break;
			}
			std::println!("average fitness at generation {i}: {fitness}");
			// This is what tries to mutate/crossover for constraint satisfaction.
			population = runtime.step(&mut generators, &mut sampler, population).unwrap();
		}

		// Sort and dedupe.
		population.sort_by(|i1, i2| i1.node().cmp(i2.node()));
		population.dedup_by(|i1, i2| i1.node() == i2.node());

		// Print final population.
		std::println!("Population:");
		for (i, candidate) in population.into_iter().enumerate() {
			std::println!(
				"{i}: {}",
				String::from_utf8(
					WriteVisitor::new(Vec::new())
						.visit(candidate.node(), 0)
						.unwrap()
						.continue_value()
						.unwrap()
						.output()
				).unwrap()
			);
		}
	}

	#[test]
	fn c_hypothetical_while_loop_constraint_alone() {
		// Because we want access to the standard library in this test:
		extern crate std;
		// Set up the various RNGs used for generation:
		let mut rng = StdRng::seed_from_u64(0);
		let mut generators =
			tuple_list!(DepthLimiter::new(nonterminal_start::ROOT.inner(), 50));

		for _ in 0..10 {
			// Generate an input based on the start symbol.
			let generated =
				nonterminal_start::generate(&mut rng, &mut generators, 0);

			// Initialize the constraint.
			// hint: #[derive(Default)] is what allows us to do this.
			let while_loop_constraint = crate::HypotheticalWhileLoopConstraint::default();
			// Visit the generated program.
			let visited = while_loop_constraint.visit(&generated, 0).unwrap().continue_value().unwrap();
			let pass_rate = visited.violations().pass_rate();

			std::println!("pass rate: {pass_rate}");

			// Print the input.
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
	fn c_single_variable_constraint_in_evolver() {
		extern crate std;

		// Hook up our new constraint
		let fitness = ViolationFitness::<crate::SingleVariableConstraint>::new();
		let fixer = (); // We aren't testing fixers yet

		let mut runtime = BasicEvolver::new::<nonterminal_start>(
			fitness,
			fixer,
			100, // Population size
			10, // Offspring
			1000, // Max nodes
			Ratio::new(50, 100),
		).expect("Should be valid.");

		let generator = DepthLimiter::new(crate::STRUCTURE.inner(), 10);
		let mut generators = tuple_list!(generator);
		let mut sampler = StdRng::seed_from_u64(42); // Seeded for reproducibility

		let mut population = runtime.initial(&mut generators, &mut sampler).unwrap();

		for i in 0..25 {
			let fitness = population
				.iter()
				.map(|i| i.measurement().fitness())
				.fold(0.0f64, |v, r| v + *r.numer() as f64 / *r.denom() as f64)
				/ population.len() as f64;

			if fitness == 1.0 {
				std::println!("Saturated fitness at generation {i}! Constraint satisfied.");
				break;
			}
			std::println!("Average fitness at generation {i}: {fitness}");
			population = runtime.step(&mut generators, &mut sampler, population).unwrap();
		}

		population.sort_by(|i1, i2| i1.node().cmp(i2.node()));
		population.dedup_by(|i1, i2| i1.node() == i2.node());

		std::println!("Final Compliant Population:");
		for (i, candidate) in population.into_iter().enumerate() {
			std::println!(
				"Program {i}:\n{}",
				String::from_utf8(
					WriteVisitor::new(Vec::new())
						.visit(candidate.node(), 0)
						.unwrap()
						.continue_value()
						.unwrap()
						.output()
				).unwrap()
			);
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

	pub fn gen_and_compile_one() -> Result<bool, Error> {
		// First, generate a program.
		let generator = DepthLimiter::new(STRUCTURE.inner(), 10);
		let mut generators = tuple_list!(generator);
		let mut rng = StdRng::from_os_rng(); // So you get a different program each time.

		let generated =
			nonterminal_start::generate(&mut rng, &mut generators, 0);

		// We also want the string later, grab it now.
		let generated_as_str = String::from_utf8(
			WriteVisitor::new(Vec::new())
				.visit(&generated, 0)?
				.continue_value()
				.unwrap()
				.output()
		)?;

		// Compile it.
		// Configure this based on whichever C compiler you have access to.
		// This is just a sample configuration for invoking gcc like `gcc -x c -o /dev/ull - <stdin>`
		let process_or_not = Command::new("gcc")
			.arg("-x")
			.arg("c")
			.arg("-o")
			.arg("/dev/null")
			.arg("-")
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.stderr(Stdio::piped())
			.spawn();

		let mut process = match process_or_not {
			Ok(p) => p,
			Err(e) => {
				std::println!("Failed to spawn gcc process: {e}");
				return Err(e.into());
			}
		};

		let stdin = process.stdin.as_mut().expect("Failed to open stdin");
		use std::io::Write;
		// Add stdio manually so things can compile.
		writeln!(stdin, "#include <stdio.h>")?;
		writeln!(stdin)?;
		// Wrap this in a main function.
		stdin
			.write_all(
				&WriteVisitor::new(Vec::new())
					.visit(&generated, 0)?
					.continue_value()
					.unwrap()
					.output(),
			)?;
		// Also add a main function that returns 0 to make it a valid C program.
		writeln!(stdin)?;
		writeln!(stdin, "int main() {{ return 0; }}")?;

		let output = process
			.wait_with_output()
			.expect("Failed to read gcc output");

		if output.status.success() {
			std::println!("Successfully compiled:");
			std::println!(
				"{}", generated_as_str
			);

			Ok(true)
		} else {
			std::println!("Failed to compile:");
			std::println!("GCC exit code: {}", output.status);
			std::println!("GCC stdout: {}", String::from_utf8_lossy(&output.stdout));
			std::println!("GCC stderr: {}", String::from_utf8_lossy(&output.stderr));
			std::println!(
				"{}", generated_as_str
			);

			Ok(false)
		}
	}
}
