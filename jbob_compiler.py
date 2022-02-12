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

    global_constants, c_mapping = Code.merge_constants(global_constants, expr.constants)
    expr = expr.map_constants(c_mapping)

    global_constants += expr.constants
    global_functions |= expr.functions

    return run_vm(expr.instructions)


def compile(expr, lexical_env=(), tail=False):
    match expr:
        case Pair("quote", Pair(x, ())):
            return compile_quotation(x, lexical_env)
        case Pair("defun", Pair(name, Pair(args, Pair(body, ())))):
            return compile_definition(name, args, body, lexical_env)
        case Pair("if", Pair(q, Pair(a, Pair(e, ())))):
            return compile_if(q, a, e, lexical_env, tail)
        case Pair("quote" | "defun" | "if", _):
            line, col = src_pos(expr)
            raise SyntaxError(f"{to_string(expr)} (line {line}, column {col})")
        case Pair(f, args):
            return compile_application(f, args, lexical_env, tail)
        case str(symbol):
            return compile_reference(symbol, lexical_env)
        case list([*x]):
            return compile_sequence(x, lexical_env, tail)
        case _:
            line, col = src_pos(expr)
            raise NotImplementedError(f"{to_string(expr)} (line {line}, column {col})")


def compile_quotation(value, _lexical_env):
    return Code.constant(value)


def compile_sequence(exprs, lexical_env, tail):
    code = Code()
    for x in exprs[:-1]:
        code = code.append(compile(x, lexical_env, tail=False))
    return code.append(compile(exprs[-1], lexical_env, tail))


def compile_application(fname, args, lexical_env, tail):
    compiled_args = list(compile_args(args, lexical_env))

    code = Code()
    for ca in compiled_args:
        code = code.append(Code.set_arg(ca))

    if fname in UNARY_BUILTINS:
        return code.append(Code.unop(fname))
    elif fname in BINARY_BUILTINS:
        return code.append(Code.binop(fname))
    elif tail:
        return code.append(Code.tailcall(fname))
    else:
        return code.append(Code.call(fname))


def compile_args(args, lexical_env):
    if is_null(args):
        return args
    return cons(
        compile(car(args), lexical_env, tail=False),
        compile_args(cdr(args), lexical_env),
    )


def compile_reference(name, lexical_env):
    idx = lexical_env.index(name)
    return Code.reference(idx)


def compile_if(q, a, e, lexical_env, tail):
    qc = compile(q, lexical_env, tail=False)
    ac = compile(a, lexical_env, tail)
    ec = compile(e, lexical_env, tail)

    code = qc
    code = code.append(Code.jump_not(1 + ac.length()))
    code = code.append(ac)
    code = code.append(Code.jump(ec.length()))
    code = code.append(ec)

    return code


def compile_definition(name, params, body, lexical_env):
    assert not lexical_env, "Nested function definitions are not allowed"

    header = Code.init_args(params)

    lexical_env = tuple(params)

    compiled_body = header.append(compile(body, lexical_env, tail=True)).append(
        Code.return_()
    )

    return Code.define(name, compiled_body)


class Code:
    def __init__(self, code=None, constants=None, functions=None):
        self.constants = constants or []
        self.functions = functions or {}
        self.instructions = code or []

    def length(self):
        return len(self.instructions)

    @staticmethod
    def constant(value):
        return Code([("CONSTANT", 0)], [value])

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
    def unop(name):
        return Code([("UNARY-OP", name)])

    @staticmethod
    def binop(name):
        return Code([("BINARY-OP", name)])

    @staticmethod
    def return_():
        return Code([("RETURN",)])

    @staticmethod
    def call(name):
        return Code(
            [
                ("SAVE", "return_point"),
                ("SAVE", "env"),
                ("CALL", name),
                ("RESTORE", "env"),
                ("RESTORE", "return_point"),
            ]
        )

    @staticmethod
    def tailcall(name):
        return Code([("TAIL-CALL", name)])

    @staticmethod
    def set_arg(arg_code):
        code = Code([("SAVE", "args")])
        code = code.append(arg_code)
        code = code.append(Code([("RESTORE", "args"), ("PUSH-ARG",)]))
        return code

    @staticmethod
    def init_args(params):
        return Code()

    @staticmethod
    def define(name, body):
        assert not body.functions, "Nested functions are not allowed"
        constants = body.constants
        body = Code(body.instructions)
        return Code(constants=constants, functions={name: body})

    def append(self, other):
        constants, c_mapping = self.merge_constants(self.constants, other.constants)
        other = other.map_constants(c_mapping)
        return Code(
            self.instructions + other.instructions,
            constants,
            self.functions | other.functions,
        )

    @staticmethod
    def merge_constants(existing, other):
        offset = len(existing)
        new_constants = []
        mapping = []
        for c in other:
            try:
                idx = existing.index(c)
            except ValueError:
                idx = offset + len(new_constants)
                new_constants.append(c)
            mapping.append(idx)
        return existing + new_constants, mapping

    def map_constants(self, mapping):
        code = [
            ("CONSTANT", mapping[op[1]]) if op[0] == "CONSTANT" else op
            for op in self.instructions
        ]
        functions = {
            name: body.map_constants(mapping) for name, body in self.functions.items()
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
        code = "\n".join(map(str, self.instructions))

        out = ""
        if constants:
            out += "CONSTANTS:\n" + constants + "\n"

        if functions:
            out += functions

        out += code

        return out


UNARY_BUILTINS = {
    "num": num,
    "atom": atom,
    "car": car,
    "cdr": cdr,
    "natp": lambda x: "t" if isinstance(x, int) and x >= 0 else "nil",
}

BINARY_BUILTINS = {
    "cons": cons,
    "equal": lambda x, y: "t" if x == y else "nil",
    "+": lambda x, y: num(x) + num(y),
    "<": lambda x, y: "t" if num(x) < num(y) else "nil",
}


def run_vm(code):
    stack = []
    ip = 0
    val = None
    args = []
    env = []
    return_point = None
    while ip < len(code):
        op = code[ip]
        match op:
            case ("CONSTANT", idx):
                val = global_constants[idx]
            case ("REF", idx):
                val = env[idx]
            case ("JUMP", offset):
                ip += offset
            case ("JUMP-FALSE", offset):
                if val == "nil":
                    ip += offset
            case ("PUSH-ARG",):
                args.append(val)
            case ("SAVE", "args"):
                stack.append(args)
                args = []
            case ("SAVE", "env"):
                stack.append(env)
            case ("SAVE", "return_point"):
                stack.append(return_point)
            case ("RESTORE", "args"):
                args = stack.pop()
            case ("RESTORE", "env"):
                env = stack.pop()
            case ("RESTORE", "return_point"):
                return_point = stack.pop()
            case ("UNARY-OP", name):
                val = UNARY_BUILTINS[name](args.pop())
            case ("BINARY-OP", name):
                b = args.pop()
                a = args.pop()
                val = BINARY_BUILTINS[name](a, b)
            case ("TAIL-CALL", func):
                env, args = args, []
                code = global_functions[func].instructions
                ip = -1
            case ("CALL", func):
                return_point = ip, code

                env, args = args, []
                code = global_functions[func].instructions
                ip = -1
            case ("RETURN",):
                ip, code = return_point
            case _op:
                raise NotImplementedError(_op)
        ip += 1
    return val
