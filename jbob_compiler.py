from jbob_parser import parse, src_pos
from jbob_runtime import atom, car, cdr, cons, is_null, num, Pair, to_string


global_constants = []
global_functions = {}


def evaluate(expr):
    global global_functions, global_constants

    if isinstance(expr, str):
        from jbob_compiler import compile
        from jbob_parser import parse

        expr = compile(parse(expr))

    constant_offset = len(global_constants)
    expr = expr.add_constant_offset(constant_offset)

    global_constants += expr.constants
    global_functions |= expr.functions

    return run_vm(expr.code)


def compile(expr, lexical_env=()):
    match expr:
        case Pair("quote", Pair(x, ())):
            return compile_quotation(x, lexical_env)
        case Pair("defun", Pair(name, Pair(args, Pair(body, ())))):
            return compile_definition(name, args, body, lexical_env)
        case Pair("if", Pair(q, Pair(a, Pair(e, ())))):
            return compile_if(q, a, e, lexical_env)
        case Pair("quote" | "defun" | "if", _):
            line, col = src_pos(expr)
            raise SyntaxError(f"{to_string(expr)} (line {line}, column {col})")
        case Pair(f, args):
            return compile_application(f, args, lexical_env)
        case str(symbol):
            return compile_reference(symbol, lexical_env)
        case list([*x]):
            return compile_sequence(x, lexical_env)
        case _:
            line, col = src_pos(expr)
            raise NotImplementedError(f"{to_string(expr)} (line {line}, column {col})")


def compile_quotation(value, _lexical_env):
    return Code.constant(value)


def compile_sequence(exprs, lexical_env):
    code = Code()
    for x in exprs:
        code = code.append(compile(x, lexical_env))
    return code


def compile_application(fname, args, lexical_env):
    compiled_args = compile_args(args, lexical_env)

    code = Code()
    for i, ca in enumerate(compiled_args):
        code = code.append(ca)
        code = code.append(Code.set_arg(i))

    return code.append(Code.call(fname))


def compile_args(args, lexical_env):
    if is_null(args):
        return args
    return cons(compile(car(args), lexical_env), compile_args(cdr(args), lexical_env))


def compile_reference(name, lexical_env):
    idx = lexical_env[::-1].index(name)
    return Code.reference(idx)


def compile_if(q, a, e, lexical_env):
    qc = compile(q, lexical_env)
    ac = compile(a, lexical_env)
    ec = compile(e, lexical_env)

    code = qc
    code = code.append(Code.jump_not(1 + ac.length()))
    code = code.append(ac)
    code = code.append(Code.jump(ec.length()))
    code = code.append(ec)

    return code


def compile_definition(name, params, body, lexical_env):
    assert not lexical_env, "Nested function definitions are not allowed"
    header = Code()
    for p in params:
        header = header.append(Code.init_arg(p))

    lexical_env = lexical_env + tuple(params)

    compiled_body = header.append(compile(body, lexical_env)).append(Code.return_())

    return Code.define(name, compiled_body)


class Code:
    def __init__(self, code=None, constants=None, functions=None):
        self.constants = constants or []
        self.functions = functions or {}
        self.code = code or []

    def length(self):
        return len(self.code)

    @staticmethod
    def constant(value):
        return Code([("CONSTANT", 0)], [value])

    @staticmethod
    def return_():
        return Code([("RETURN",)])

    @staticmethod
    def jump(offset):
        return Code([("JUMP", offset)])

    @staticmethod
    def jump_not(offset):
        return Code([("JUMP-FALSE", offset)])

    @staticmethod
    def reference(name):
        return Code([("REF", name)])

    @staticmethod
    def call(name):
        return Code([("CALL", name)])

    @staticmethod
    def set_arg(i):
        return Code([("PUSH-ARG",)])

    @staticmethod
    def init_arg(name):
        return Code()

    @staticmethod
    def define(name, body):
        assert not body.functions, "Nested functions are not allowed"
        constants = body.constants
        body = Code(body.code)
        return Code(constants=constants, functions={name: body})

    def append(self, other):
        const_offset = len(self.constants)
        other = other.add_constant_offset(const_offset)
        return Code(
            self.code + other.code,
            self.constants + other.constants,
            self.functions | other.functions,
        )

    def add_constant_offset(self, offset):
        code = [
            ("CONSTANT", op[1] + offset) if op[0] == "CONSTANT" else op
            for op in self.code
        ]
        functions = {
            name: body.add_constant_offset(offset)
            for name, body in self.functions.items()
        }
        return Code(code=code, constants=self.constants, functions=functions)

    def __str__(self):
        constants = "\n".join(
            map(lambda c: f"    {c[0]}: {c[1]}", enumerate(self.constants))
        )
        functions = []
        for name, body in self.functions.items():
            functions.append(f"\n{name}:\n{body}\n")
        functions = "".join(functions)
        code = "\n".join(map(str, self.code))

        out = ""
        if constants:
            out += "CONSTANTS:\n" + constants + "\n"

        if functions:
            out += functions

        out += code

        return out


def run_vm(code):
    ip = 0
    stack = []
    val = None
    args = []
    while ip < len(code):
        match code[ip]:
            case ("CONSTANT", idx):
                val = global_constants[idx]
            case ("REF", idx):
                val = args[-1 - idx]
            case ("JUMP", offset):
                ip += offset
            case ("JUMP-FALSE", offset):
                if val == "nil":
                    ip += offset
            case ("PUSH-ARG",):
                args.append(val)
            case ("CALL", "atom"):
                val = atom(args.pop())
            case ("CALL", "car"):
                val = car(args.pop())
            case ("CALL", "cdr"):
                val = cdr(args.pop())
            case ("CALL", "cons"):
                second = args.pop()
                val = cons(args.pop(), second)
            case ("CALL", "equal"):
                val = "t" if args.pop() == args.pop() else "nil"
            case ("CALL", func):
                stack.append(("STATE", code, ip, len(args)))
                code = global_functions[func].code
                ip = -1
            case ("RETURN",):
                _, code, ip, nargs = stack.pop()
                args = args[:nargs]
            case op:
                raise NotImplementedError(op)
        ip += 1
    return val
