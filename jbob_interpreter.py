from functools import lru_cache
import keyword

from jbob_parser import parse, src_pos
from jbob_runtime import atom, car, cdr, cons, is_null, num, Pair


global_env = {}  # runtime store of language-defined functions


#  Initial Environment
# =====================
global_env["num"] = num
global_env["atom"] = atom
global_env["cons"] = cons
global_env["car"] = car
global_env["cdr"] = cdr
global_env["equal"] = lambda x, y: "t" if x == y else "nil"
global_env["natp"] = lambda x: "t" if isinstance(x, int) and x >= 0 else "nil"
global_env["+"] = lambda x, y: num(x) + num(y)
global_env["<"] = lambda x, y: "t" if num(x) < num(y) else "nil"


def evaluate(expr, env=None):
    env = env or {}
    if isinstance(expr, str):
        expr = analyze(parse(expr))
    return expr(env)


def analyze(expr):
    match expr:
        case Pair("quote", Pair(x, ())):
            return analyze_quotation(x)
        case Pair("defun", Pair(name, Pair(args, Pair(body, ())))):
            return analyze_definition(name, args, body)
        case Pair("if", Pair(q, Pair(a, Pair(e, ())))):
            return analyze_if(q, a, e)
        case Pair("quote" | "defun" | "if", _):
            line, col = src_pos(expr)
            raise SyntaxError(f"{to_string(expr)} (line {line}, column {col})")
        case Pair(f, args):
            return analyze_application(f, args)
        case str(symbol):
            return analyze_reference(symbol)
        case list([*x]):
            return analyze_sequence(x)
        case _:
            line, col = src_pos(expr)
            raise NotImplementedError(f"{to_string(expr)} (line {line}, column {col})")


class AddErrorContext:
    def __init__(self, expr):
        self.line, self.col = src_pos(expr)

    def __call__(self, func):
        def wrapped(*args, **kwargs):
            try:
                return func(*args, **kwargs)
            except Traceback as e:
                original_exception = e.args[0]
                existing_context = e.args[1]
                new_context = existing_context + self.context_str()
                raise Traceback(original_exception, new_context) from None
            except Exception as e:
                raise Traceback(e, self.context_str()) from None

        return wrapped

    def context_str(self):
        return f"(line {self.line}, column {self.col})"


class Traceback(RuntimeError):
    pass


def analyze_quotation(value):
    return lambda _: value


def analyze_application(fexpr, args):
    arg_execs = analyze_args(args)

    @AddErrorContext(fexpr)
    def the_application(env):
        f = global_env[fexpr]
        args = map(lambda a: a(env), iter(arg_execs))
        return f(*args)

    return the_application


def analyze_args(args):
    if is_null(args):
        return args
    return cons(analyze(car(args)), analyze_args(cdr(args)))


def analyze_reference(name):
    def the_reference(env):
        try:
            return env[name]
        except KeyError:
            line, col = src_pos(name)
            raise NameError(f"{name} (line {line}, column {col})")

    return the_reference


def analyze_sequence(exprs):
    execs = [analyze(x) for x in exprs]

    def the_sequence(env):
        result = None
        for x in execs:
            result = x(env)
        return result

    return the_sequence


def analyze_if(q, a, e):
    q_exec = analyze(q)
    a_exec = analyze(a)
    e_exec = analyze(e)
    return lambda env: e_exec(env) if q_exec(env) == "nil" else a_exec(env)


def analyze_definition(name, params, body):
    body_exec = analyze(body)
    param_strs = list(map(pythonize, iter(params)))
    python_name = pythonize(name)

    local_env = ", ".join(f"'{p}': {pyp}" for p, pyp in zip(params, param_strs))
    local_env = "local_env = env | {" + local_env + "}"

    # define functions as actual python functions with name and arguments
    # caching the results seems feasible, because as far I know at the moment, there are no side-effects in J-Bob
    fndef = f"""
@cache 
def {python_name}({", ".join(param_strs)}):
    {local_env}
    return body_exec(local_env)
"""

    def the_definition(env):
        if name in global_env:
            first_name = next(filter(lambda n: n == name, global_env.keys()))
            line1, col1 = src_pos(first_name)
            line2, col2 = src_pos(name)
            raise NameError(
                f"{name} multiply defined (line {line1}, column {col1}) and (line {line2}, column {col2})"
            )
        glob = {}
        exec(fndef, {"body_exec": body_exec, "env": env, "cache": lru_cache}, glob)
        global_env[name] = glob[python_name]

    return the_definition


def pythonize(name: str) -> str:
    pyname = name.replace("_", "__")
    if pyname.endswith("?"):
        pyname = "is_" + pyname[:-1]
    pyname = (
        pyname.replace("-", "_")
        .replace("?", "_p_")
        .replace("/", "_slash_")
        .replace("+", "_plus_")
        .replace("*", "_star_")
        .replace(".", "_dot_")
        .replace("<", "_lt_")
        .replace("=", "_eq_")
        .replace(">", "_gt_")
    )
    if len(pyname) > 1 and pyname[-1] == "_" and pyname[-2] != "_":
        pyname = pyname[:-1]
    if keyword.iskeyword(pyname):
        pyname = pyname + "_"
    return pyname
