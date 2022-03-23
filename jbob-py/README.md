The Little Prover
=================

Python Stuff
------------

- `build_jbob_rust.py` compiles the J-Bob proof assistant into a Rust module.
- `jbob.py` loads the proof assistance (../jbob-scm/j-bob.scm) into the J-Bob interpreter's environment.
- `jbob_interpreter.py` interpreter for the J-Bob language.
- `jbob_parser.py` a small S-expression parser, sufficient for J-Bob
- `jbob_runtime.py` basically an implementation of cons-lists.
- `jbob_rustcompiler.py` similar to the interpreter, but produces Rust code.
- `jbob_testing.py` tools for testing
- `test_jbob.py` test the proof assistant, based on Chapters 2, 3, and Recess.

Implementation Quirks
---------------------
- Python does not eliminate tail calls, so running sufficiently long proofs might
cause stack overflows. (The same might be true for the Rust transpilation.)
- J-Bob does not have side effects, so I tried to compile `(defun)`s to memoized
  Python functions... resulting in amazing speed gains. This might also mitigate
  the recursion issue.