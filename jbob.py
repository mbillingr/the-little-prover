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


from jbob_compiler import compile, evaluate, global_functions, optimize

program = compile(ast)
program = optimize(program)
print(program)
print(evaluate(program))

print(evaluate("(list1 '42)"))

print(evaluate("(list2? '(1 2))"))


def undefine(name):
    del global_functions[name]
