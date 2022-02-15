from code_transform import inline_all
from jbob_parser import parse
from jbob_interpreter import analyze, evaluate, global_env


def undefine(name):
    del global_env[name]


#  J-Bob
# =======
with open("j-bob.scm") as fd:
    ast = parse(fd.read())
#ast = inline_all(ast)

program = analyze(ast)
evaluate(program)


from jbob_compiler import compile, evaluate, global_functions, optimize

program = compile(ast)
# evaluate(program)  # evaluate once, to populate the function environment
program = optimize(program)
# evaluate(program)  # evaluate once, to populate the function environment
# program = optimize(program)
#print(program)
with open("j-bob.json", "w") as fd:
    fd.write(program.export())
evaluate(program)

print(evaluate("(member? '1 '(1 2))"))

print(evaluate("(list1 '42)"))

print(evaluate("(list2? '(1 2))"))


def undefine(name):
    del global_functions[name]
