import lark
from lark import Lark, Transformer, v_args

from jbob_runtime import cons


if int(lark.__version__.split(".")[0]) < 1:

    def with_meta(func):
        return v_args(meta=True)(lambda self, x, meta: func(self, meta, x))

else:

    def with_meta(func):
        return v_args(meta=True)(func)


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
    @with_meta
    def quote(self, meta, q):
        (q,) = q
        return self._list(meta, ["quote", q])

    @with_meta
    def symbol(self, meta, s):
        (s,) = s
        name = s[:]
        SRCMAP[id(name)] = ((meta.line, meta.column), (meta.end_line, meta.end_column))
        return name

    def number(self, n):
        (n,) = n
        return int(n)

    @with_meta
    def list(self, meta, l):
        return self._list(meta, l)

    def _list(self, meta, l):
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
