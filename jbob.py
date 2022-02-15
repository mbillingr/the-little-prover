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
