from jbob_parser import src_pos
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

    # put args in reverse order on the stack
    code = Code()
    for ca in compiled_args[::-1]:
        code = code.append(Code.set_arg(ca))

    if fname in UNARY_BUILTINS:
        return code.append(Code.unop(fname))
    elif fname in BINARY_BUILTINS:
        return code.append(Code.binop(fname))
    elif tail:
        return code.append(Code.tailcall(fname, len(compiled_args), len(lexical_env)))
    else:
        return code.append(Code.call(fname, len(compiled_args)))


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
        Code.return_(len(lexical_env))
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
    def return_(n_drop):
        return Code([("DROP", 1) for _ in range(n_drop)] + [("RETURN",)])

    @staticmethod
    def call(name, n_args):
        return Code([("CALL", name)])

    @staticmethod
    def tailcall(name, n_args, n_drop):
        return Code([("DROP", n_args) for _ in range(n_drop)] + [("GOTO", name)])

    @staticmethod
    def set_arg(arg_code):
        return arg_code

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

    def total_instructions(self):
        nops = sum(f.total_instructions() for f in self.functions.values())
        return nops + len(self.instructions)


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
    value_stack = [None]  # value stack initially contains the default result
    call_stack = []
    ip = 0
    fp = 0

    def push(x):
        value_stack.append(x)

    def pop(k=0):
        if k == 0:
            return value_stack.pop()
        else:
            x = value_stack[-1 - k]
            del value_stack[-1 - k]
            return x

    def ref(idx):
        value_stack.append(value_stack[fp - idx])

    while ip < len(code):
        op = code[ip]
        match op:
            case ("CONSTANT", idx):
                push(global_constants[idx])
            case ("REF", idx):
                ref(idx)
            case ("DROP",):
                pop()
            case ("DROP", k):
                pop(k)
            case ("SWAP",):
                value_stack[-2:] = value_stack[:-3:-1]
            case ("JUMP", offset):
                ip += offset
            case ("JUMP-FALSE", offset):
                if pop() == "nil":
                    ip += offset
            case ("UNARY-OP", name):
                push(UNARY_BUILTINS[name](pop()))
            case ("BINARY-OP", name):
                a = pop()
                b = pop()
                push(BINARY_BUILTINS[name](a, b))
            case ("GOTO", func):
                fp = len(value_stack) - 1
                code = global_functions[func].instructions
                ip = -1
            case ("CALL", func):
                call_stack.append((ip, fp, code))
                fp = len(value_stack) - 1
                code = global_functions[func].instructions
                ip = -1
            case ("RETURN",):
                ip, fp, code = call_stack.pop()
            case _op:
                raise NotImplementedError(_op)
        ip += 1
    return pop()


def optimize(code):
    code = insert_labels(code)
    # code = monitor(inline_functions, code)
    code = monitor(peephole, code)
    code = monitor(collapse_jump_cascades, code)
    code = monitor(mark_dead_code, code)
    code = resolve_labels(code)
    return code


def monitor(func, code):
    n_ops0 = code.total_instructions()
    code = func(code)

    n_ops = code.total_instructions()
    print(f"{func.__name__}: {n_ops0 - n_ops} instructions removed ({n_ops} remaining)")

    return code


def enter_functions(func):
    def wrapped(code):
        functions = {f: func(c) for f, c in code.functions.items()}
        code = func(code)
        return Code(code.instructions, code.constants, functions)

    wrapped.__name__ = func.__name__
    return wrapped


@enter_functions
def insert_labels(code):
    jump_targets = {}
    for i, op in enumerate(code.instructions, start=1):
        match op:
            case ("JUMP", ofs) | ("JUMP-FALSE", ofs):
                jump_targets[i + ofs] = f"label-{i+ofs}"

    instructions = []
    for i, op in enumerate(code.instructions):
        if i in jump_targets:
            instructions.append(jump_targets[i])
        match op:
            case ("JUMP", ofs):
                instructions.append(("JUMP", jump_targets[1 + i + ofs]))
            case ("JUMP-FALSE", ofs):
                instructions.append(("JUMP-FALSE", jump_targets[1 + i + ofs]))
            case _:
                instructions.append(op)

    return Code(instructions, code.constants, code.functions)


@enter_functions
def resolve_labels(code):
    labels = {}
    i = 0
    for op1 in code.instructions:
        match op1:
            case str(s1):
                labels[s1] = i
            case _:
                i += 1

    instructions = []
    for op in code.instructions:
        match op:
            case (("JUMP" | "JUMP-FALSE") as jmp, target):
                ofs = labels[target] - len(instructions) - 1
                instructions.append((jmp, ofs))
            case str(s):
                pass
            case _:
                instructions.append(op)

    return Code(instructions, code.constants, code.functions)


def find_label(code, label):
    return code.instructions.index(label)


@enter_functions
def peephole(code):

    instructions = []
    for op in code.instructions:
        instructions.append(op)
        while True:
            match instructions:
                case [
                    *_,
                    ("SAVE", r1),
                    ("CONSTANT" | "REF", _) as op,
                    ("RESTORE", r2),
                ] if r1 == r2:
                    instructions[-3:] = [op]

                case [*_, ("ARGS->ENV",), ("CONSTANT", _), ("RESTORE", "env")]:
                    del instructions[-3]

                case _:
                    break

    return Code(instructions, code.constants, code.functions)


@enter_functions
def collapse_jump_cascades(code):
    instructions = []
    for op in code.instructions:
        match op:
            case ("JUMP" | "JUMP-FALSE" as jmp, target):
                while True:
                    target_pos = find_label(code, target) + 1
                    match code.instructions[target_pos]:
                        case ("JUMP", t):
                            target = t
                        case _: break
                instructions.append((jmp, target))
            case _:
                instructions.append(op)

    return Code(instructions, code.constants, code.functions)


@enter_functions
def mark_dead_code(code):
    visited = [False] * len(code.instructions)

    def visit_code(pos):
        while True:
            if pos >= len(visited):
                break
            if visited[pos]:
                return
            visited[pos] = True
            match code.instructions[pos]:
                case ("JUMP", target):
                    pos = find_label(code, target)
                case ("JUMP-FALSE", target):
                    visit_code(find_label(code, target))
                    pos += 1
                case _: pos += 1

    visit_code(0)

    instructions = [op if v else ("NOP",) for v, op in zip(visited, code.instructions)]

    return Code(instructions, code.constants, code.functions)


@enter_functions
def inline_functions(code, max_len=10):
    instructions = []
    for op in code.instructions:
        match op:
            case ("CALL", name):
                body = global_functions[name]
                if len(body.instructions) < max_len:
                    instructions.extend(
                        filter(lambda op: op != ("RETURN",), body.instructions)
                    )
                else:
                    instructions.append(op)
            case _:
                instructions.append(op)

    return Code(instructions, code.constants, code.functions)
