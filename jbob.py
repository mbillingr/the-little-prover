from dataclasses import dataclass
import keyword
from typing import Any

from lark import Lark, Transformer, v_args


global_env = {}


#  The Evaluator
# ===============


def evaluate(expr, env=None):
    if env is None:
        env = global_env
    if isinstance(expr, str):
        expr = analyze(parse(expr))
    return expr(env)


def analyze(expr):
    match expr:
        case Pair("quote", Pair(x, ())):
            return lambda _: x
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


def analyze_application(f, args):
    f_exec = analyze(f)
    arg_execs = analyze_args(args)

    def the_application(env):
        f = f_exec(env)
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
    fndef = f"""def {python_name}({", ".join(param_strs)}):
                    {local_env}
                    return body_exec(local_env)"""

    def the_definition(env):
        if name in global_env:
            first_name = next(filter(lambda n: n == name, global_env.keys()))
            line1, col1 = src_pos(first_name)
            line2, col2 = src_pos(name)
            raise NameError(
                f"{name} multiply defined (line {line1}, column {col1}) and (line {line2}, column {col2})"
            )
        glob = {}
        exec(fndef, {"body_exec": body_exec, "env": env}, glob)
        global_env[name] = glob[python_name]

    return the_definition


def pythonize(name: str) -> str:
    pyname = name.replace("_", "__")
    if pyname.endswith("?"):
        pyname = "is_" + pyname[:-1]
    pyname = (
        pyname.replace("-", "_")
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


#  The S-Expression Parser
# =========================


def parse(src):
    tree = sexpr_parser.parse(src)
    return TreeToSexpr().transform(tree)


sexpr_parser = Lark(
    r"""
    program: sexpr*
    
    ?sexpr: SIGNED_NUMBER   -> number
         | SYMBOL           -> symbol
         | list
         | quote         
         
    quote: "'" sexpr
    list: "(" sexpr* ")"    
         
    SYMBOL: ("A".."Z" | "a".."z" | "0".."9" | "-" | "+" | "/" | "_" | "." | "!" | "?" | "<" | "=" | ">")+
    
    COMMENT: ";" /[^\n]*/ NEWLINE
    %ignore COMMENT
    
    %import common.SIGNED_NUMBER
    %import common.NEWLINE
    %import common.WS
    %ignore WS
""",
    start="program",
    propagate_positions=True,
)

SRCMAP = {}


def src_pos(obj):
    return SRCMAP[id(obj)][0]


class TreeToSexpr(Transformer):
    @v_args(meta=True)
    def quote(self, q, meta):
        (q,) = q
        return self.list(["quote", q], meta)

    @v_args(meta=True)
    def symbol(self, s, meta):
        (s,) = s
        name = s[:]
        SRCMAP[id(name)] = ((meta.line, meta.column), (meta.end_line, meta.end_column))
        return name

    def number(self, n):
        (n,) = n
        return int(n)

    @v_args(meta=True)
    def list(self, l, meta):
        out = ()
        for x in l[::-1]:
            out = cons(x, out)
            SRCMAP[id(out)] = (
                (meta.line, meta.column),
                (meta.end_line, meta.end_column),
            )
        return out

    def program(self, prog):
        return prog


#  The Basics
# ============


def atom(x):
    return "nil" if is_pair(x) else "t"


@dataclass
class Pair:
    car: Any
    cdr: Any

    def __iter__(self):
        pair = self
        while is_pair(pair):
            yield pair.car
            pair = pair.cdr


assert Pair(1, 2) == Pair(1, 2)


def num(x):
    if isinstance(x, int):
        return x
    else:
        return 0


def is_null(obj):
    return obj == ()


def is_pair(obj):
    return isinstance(obj, Pair)


def cons(a, d):
    return Pair(a, d)


def car(p):
    return p.car if is_pair(p) else ()


def cdr(p):
    return p.cdr if is_pair(p) else ()


def cadr(p):
    return car(cdr(p))


def is_list(obj, len=None):
    match len:
        case None:
            return is_null(obj) or (is_pair(obj) and is_list(cdr(obj)))
        case 0:
            return is_null(obj)
        case _:
            return is_pair(obj) and is_list(cdr(obj), len - 1)


def to_string(obj):
    if is_list(obj, len=2) and car(obj) == "quote":
        return f"'{to_string(cadr(obj))}"
    elif is_pair(obj):
        out = ["(", to_string(car(obj))]
        obj = cdr(obj)
        while is_pair(obj):
            out.append(" ")
            out.append(to_string(car(obj)))
            obj = cdr(obj)
        if not is_null(obj):
            out.append(" . ")
            out.append(to_string(obj))
        out.append(")")
        return "".join(out)
    else:
        return str(obj)


#  Environment
# =============

global_env["num"] = num
global_env["atom"] = atom
global_env["cons"] = cons
global_env["car"] = car
global_env["cdr"] = cdr
global_env["equal"] = lambda x, y: "t" if x == y else "nil"
global_env["natp"] = lambda x: "t" if isinstance(x, int) and x >= 0 else "nil"
global_env["+"] = lambda x, y: num(x) + num(y)
global_env["<"] = lambda x, y: "t" if num(x) < num(y) else "nil"


#  J-Bob
# =======


with open("j-bob.scm") as fd:
    ast = parse(fd.read())
program = analyze(ast)
evaluate(program)

evaluate("(J-Bob/step (prelude) '(car (cons 'ham '(cheese))) '())")
