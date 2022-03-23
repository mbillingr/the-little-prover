from pathlib import Path

from jbob_parser import parse
from jbob_interpreter import analyze, evaluate, global_env


JBOB_SOURCE_DIR = (Path(__file__).parent / "../jbob-scm").resolve()


def undefine(name):
    del global_env[name]


#  J-Bob
# =======
with open(JBOB_SOURCE_DIR / "j-bob.scm") as fd:
    ast = parse(fd.read())
program = analyze(ast)
evaluate(program)
