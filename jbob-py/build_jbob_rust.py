from pathlib import Path

from jbob_parser import parse
import jbob_rustcompiler as jbrc

JBOB_SOURCE_DIR = (Path(__file__).parent / "../jbob-scm").resolve()


if __name__ == "__main__":
    with open(JBOB_SOURCE_DIR / "j-bob.scm") as fd:
        ast = parse(fd.read())

    header = ["// This source file is automatically generated. Changes may get lost!"]

    rust_program = jbrc.analyze_program(ast)

    rust_source = header + rust_program

    with open("../rust/jbob/src/j_bob.rs", "w") as fd:
        fd.write("\n".join(rust_source))
