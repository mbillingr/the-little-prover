from jbob_parser import parse
from jbob_interpreter import analyze, evaluate, global_env


def undefine(name):
    del global_env[name]


#  J-Bob
# =======
with open("j-bob.scm") as fd:
    ast = parse(fd.read())
program = analyze(ast)
evaluate(program)


if __name__ == "__main__":
    import jbob_rustcompiler as jbrc

    rust_program = jbrc.analyze_program(ast)
    with open("jbob-rust/src/j_bob.rs", "w") as fd:
        fd.write("\n".join(rust_program))
