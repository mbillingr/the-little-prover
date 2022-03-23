from jbob_interpreter import analyze
from jbob_runtime import Pair


def evaluate_testsuite(env, expr):
    match expr:
        case list(_):
            for testcase in expr:
                evaluate_testcase(env, testcase)
        case _:
            raise SyntaxError("Expected list of test cases")


def evaluate_testcase(env, expr):
    match expr:
        case Pair("testcase", Pair(testname, Pair(assertion, ()))):
            evaluate_assertion(env, assertion)
            print(testname, "... OK")
        case Pair("define", Pair(name, Pair(value, ()))):
            assert name not in env
            val = analyze(value)(env)
            env[name] = val
        case _:
            raise NotImplementedError(expr)


def evaluate_assertion(env, expr):
    match expr:
        case Pair("assert-equal", Pair(a, Pair(b, ()))):
            val_a = analyze(a)(env)
            val_b = analyze(b)(env)
            assert val_a == val_b, f"\n    Actual: {val_a}\n  Expected: {val_b}"
        case _:
            raise NotImplementedError(expr)
