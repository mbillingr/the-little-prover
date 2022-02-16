from jbob_runtime import Pair


RUST_VALUE_TYPE = "S<'a>"

RUST_KEYWORDS = {
    "const",
    "enum",
    "fn",
    "impl",
    "let",
    "mut",
    "pub",
    "ref",
    "return",
    "struct",
    "use",
}


constants = {"C_NIL": None, "C_T": None}  # predefined in the runtime
pair_constants = []


def analyze_program(stmts):
    statements = sum(
        (analyze_statement(s) for s in stmts), start=[]
    )  # sum appends nested lists

    consts = [
        f"const {name}: S<'static> = {value};"
        for name, value in constants.items()
        if value is not None
    ]

    for i, (car, cdr) in enumerate(pair_constants):
        consts.append(f"const C_PAIR_{i}: S<'static> = S::Pair(&({car}, {cdr}));")

    return [
        "#![allow(non_snake_case)]",
        "use crate::jbob_runtime::*;",
        "",
        *consts,
        "",
        *statements,
    ]


def analyze_statement(stmt):
    match stmt:
        case Pair("defun", Pair(name, Pair(args, Pair(body, ())))):
            return analyze_definition(name, args, body)
        case _:
            raise NotImplementedError(stmt)


def analyze_definition(name, params, body):
    typed_formals = [
        "context: &'a Context<'a>",
        *(f"{rustify(p)}: {RUST_VALUE_TYPE}" for p in params),
    ]
    typed_formals = ", ".join(typed_formals)

    rust_body = analyze_expr(body)

    return [
        f"pub fn {rustify(name)}<'a>({typed_formals}) -> {RUST_VALUE_TYPE} {{",
        *rust_body,
        "}",
        "",
    ]


def analyze_expr(expr):
    match expr:
        case Pair("quote", Pair(x, ())):
            return analyze_quotation(x)
        case Pair("if", Pair(q, Pair(a, Pair(e, ())))):
            return analyze_if(q, a, e)
        case Pair(f, args):
            return analyze_application(f, args)
        case str(symbol):
            return analyze_reference(symbol)
        case _:
            raise NotImplementedError(expr)


def analyze_quotation(value):
    match value:
        case ():
            return ["S::Empty"]
        case int(x):
            return [f"S::Num({x})"]
        case str(name):
            rname = "C_" + rustify(name).upper()
            if rname not in constants:
                constants[rname] = f'S::Symbol("{name}")'
            return [rname]
        case Pair(a, b):
            b_ = ' '.join(analyze_quotation(b))
            a_ = ' '.join(analyze_quotation(a))
            p_ = (a_, b_)
            for i, c in enumerate(pair_constants):
                if c == p_:
                    return [f"C_PAIR_{i}"]
            i = len(pair_constants)
            pair_constants.append(p_)
            return [f"C_PAIR_{i}"]
        case _:
            raise NotImplementedError(value)


def analyze_reference(name):
    return [rustify(name)]


def analyze_application(fname, args):
    arguments = ["context"]
    for a in args:
        arguments.append(",")
        arguments.extend(analyze_expr(a))
    return [f"{rustify(fname)}(", *arguments, ")"]


def analyze_if(q, a, e):
    q_exec = analyze_expr(q)
    a_exec = analyze_expr(a)
    e_exec = analyze_expr(e)
    return ["if", *q_exec, "!=", *analyze_quotation("nil"), "{", *a_exec, "} else {", *e_exec, "}"]


def rustify(name: str) -> str:
    rsname = name.replace("_", "__")
    rsname = rsname.lower()
    if rsname.endswith("?"):
        rsname = "is_" + rsname[:-1]
    rsname = (
        rsname.replace("-", "_")
        .replace("?", "_p_")
        .replace("/", "_slash_")
        .replace("+", "_plus_")
        .replace("*", "_star_")
        .replace(".", "_dot_")
        .replace("<", "_lt_")
        .replace("=", "_eq_")
        .replace(">", "_gt_")
    )
    if len(rsname) > 1 and rsname[-1] == "_" and rsname[-2] != "_":
        rsname = rsname[:-1]
    if rsname in RUST_KEYWORDS:
        rsname = rsname + "_"
    return rsname
