from jbob_interpreter import analyze
from jbob_runtime import Pair


def evaluate_testsuite(expr):
    match expr:
        case list(_):
            for testcase in expr:
                evaluate_testcase(testcase)
        case _:
            raise SyntaxError("Expected list of test cases")


def evaluate_testcase(expr):
    match expr:
        case Pair("testcase", Pair(testname, Pair(assertion, ()))):
            evaluate_assertion(assertion)
            print(testname, "... OK")
        case _: raise NotImplementedError(expr)


def evaluate_assertion(expr):
    match expr:
        case Pair("assert-equal", Pair(a, Pair(b, ()))):
            val_a = analyze(a)({})
            val_b = analyze(b)({})
            assert val_a == val_b, f"\n    Actual: {val_a}\n  Expected: {val_b}"
        case _: raise NotImplementedError(expr)
