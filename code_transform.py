from jbob_parser import src_pos
from jbob_runtime import Pair, cons, map_list, to_string, is_null


def inline_all(expr):
    funcs = get_functions(expr)

    def recursive_inline(expr):
        match expr:
            case Pair("quote", _):
                return expr
            case Pair("defun", Pair(name, Pair(args, Pair(body, ())))):
                body = recursive_inline(body)
                #funcs[name] = (args, body)
                return cons(
                    "defun", cons(name, cons(args, cons(body, ())))
                )
            case Pair("if", Pair(q, Pair(a, Pair(e, ())))):
                return cons(
                    "if",
                    cons(
                        recursive_inline(q),
                        cons(recursive_inline(a), cons(recursive_inline(e), ())),
                    ),
                )
            case Pair("quote" | "defun" | "if", _):
                line, col = src_pos(expr)
                raise SyntaxError(f"{to_string(expr)} (line {line}, column {col})")
            case Pair(f, args):
                args = map_list(recursive_inline, args)
                return apply_inline(f, args)
            case list([*seq]):
                return list(map(recursive_inline, seq))
            case _:
                return expr

    def apply_inline(f, args):
        try:
            formals, body = funcs[f]
        except KeyError:
            return cons(f, args)

        formals = tuple(formals)
        args = tuple(args)

        assert len(formals) == len(args)

        if is_null(args):
            return body
        else:
            return substitute(body, formals, args)

    def substitute(expr, names, values):
        match expr:
            case Pair("quote", _):
                return expr
            case Pair("defun", _):
                raise RuntimeError("UNREACHABLE")
            case Pair("if", Pair(q, Pair(a, Pair(e, ())))):
                return cons(
                    "if",
                    cons(
                        substitute(q, names, values),
                        cons(
                            substitute(a, names, values),
                            cons(substitute(e, names, values), ()),
                        ),
                    ),
                )
            case Pair("quote" | "defun" | "if", _):
                line, col = src_pos(expr)
                raise SyntaxError(f"{to_string(expr)} (line {line}, column {col})")
            case Pair(f, args):
                args = map_list(lambda a: substitute(a, names, values), args)
                return cons(f, args)
            case str(symbol):
                return values[names.index(symbol)]
            case list([*seq]):
                return list(map(lambda x: substitute(x, names, values), seq))
            case _:
                line, col = src_pos(expr)
                raise NotImplementedError(
                    f"{to_string(expr)} (line {line}, column {col})"
                )

    return recursive_inline(expr)


def get_functions(expr):
    match expr:
        case Pair("defun", Pair(name, Pair(args, Pair(body, ())))):
            return {name: (args, body)}
        case list([*seq]):
            funcs = {}
            for x in seq:
                funcs |= get_functions(x)
            return funcs
        case _:
            return {}
