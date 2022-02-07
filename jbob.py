import re
from lark import Lark, Transformer


def evaluate(expr):
    if isinstance(expr, str):
        tree = sexpr_parser.parse(expr)
        expr = TreeToSexpr().transform(tree)
        print(to_string(expr))
    raise NotImplementedError()


sexpr_parser = Lark(
    r"""
    ?sexpr: SIGNED_NUMBER   -> number
         | SYMBOL           -> symbol
         | list
         | quote
         
    quote : "'" sexpr
    list : "(" sexpr* ")"    
         
    SYMBOL : ("A".."Z" | "a".."z" | "-" | "/" | "_" | "." | "!")+
    
    %import common.SIGNED_NUMBER
    %import common.WS
    %ignore WS
""",
    start="sexpr",
)


class TreeToSexpr(Transformer):
    def quote(self, q):
        (q,) = q
        return self.list(["quote", q])

    def symbol(self, s):
        (s,) = s
        return s[:]

    def number(self, n):
        (n,) = n
        return int(n)

    def list(self, l):
        out = ()
        for x in l[::-1]:
            out = cons(x, out)
        return out


def is_null(obj):
    return obj == ()


def is_pair(obj):
    return isinstance(obj, tuple) and len(obj) == 2


def cons(a, d):
    return (a, d)


def car(p):
    return p[0]


def cdr(p):
    return p[1]


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
        return f"'{cadr(obj)}"
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


tree = sexpr_parser.parse("( J-Bob/step 'x '42 123 456 foo)")
sexpr = TreeToSexpr().transform(tree)
print(tree.pretty())
print(to_string(sexpr))
print(to_string((1, (2, 3))))


evaluate("(J-Bob/step (prelude) '(car (cons 'ham '(cheese))) '())")
