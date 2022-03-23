The Little Prover
=================

When reading [The Little Prover](https://mitpress.mit.edu/books/little-prover), 
I wanted to follow along on my own terms with a convenient application. 
[Try it in the browser](https://mbillingr.github.io/the-little-prover/).
(At least it's convenient for me. I don't really expect anyone else to find it useable in its current state.)

This is what I produced in the process:
- `jbob-scm/` contains the J-Bob sources in the Lisp-like J-Bob language.
- `jbob-py/` contains a J-Bob language interpreter and a J-Bob -> Rust transpiler, both written in Python.
- `jbob-py/build_jbob_rust.py` compiles the J-Bob proof assistant into a Rust module.
- `jbob-rust/jbob-rs` is a Rust crate containing the transpiled J-Bob assistant and supporting runtime infrastructure.
- `jbob-rust/jbob_eframe` is a GUI frontend for the J-Bob assistant. 
  Thanks to [eframe](https://github.com/emilk/egui/tree/master/eframe), 
  it runs natively or [in the browser](https://mbillingr.github.io/the-little-prover/).
