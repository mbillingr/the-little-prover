from dataclasses import dataclass
from typing import Any


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

    def __repr__(self):
        return to_string(self)

    def __hash__(self):
        return hash((self.car, self.cdr))

    def __len__(self):
        return sum(1 for _ in self)


def map_list(func, x):
    if is_pair(x):
        return cons(func(x.car), map_list(func, x.cdr))
    else:
        return ()


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


def pythonize(obj):
    match obj:
        case _ if is_list(obj):
            return list(map(pythonize, obj))
        case Pair(a, d):
            return {"pair": (a, d)}
        case _: return obj


def display(*objs):
    print(" ".join(map(to_string, objs)))
