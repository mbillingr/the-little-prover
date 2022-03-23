// This source file is automatically generated. Changes may get lost!
#![allow(non_snake_case)]
#![allow(unused_variables)]

use crate::jbob_runtime::*;

const C_QUOTE: S<'static> = S::Symbol("quote");
const C_IF: S<'static> = S::Symbol("if");
const C_DEFUN: S<'static> = S::Symbol("defun");
const C_DETHM: S<'static> = S::Symbol("dethm");
const C__LT: S<'static> = S::Symbol("<");
const C__PLUS: S<'static> = S::Symbol("+");
const C_SIZE: S<'static> = S::Symbol("size");
const C_NATP: S<'static> = S::Symbol("natp");
const C_CONS: S<'static> = S::Symbol("cons");
const C_CDR: S<'static> = S::Symbol("cdr");
const C_CAR: S<'static> = S::Symbol("car");
const C_ATOM: S<'static> = S::Symbol("atom");
const C_EQUAL: S<'static> = S::Symbol("equal");
const C_X: S<'static> = S::Symbol("x");
const C_Y: S<'static> = S::Symbol("y");
const C_ANY: S<'static> = S::Symbol("any");
const C_E: S<'static> = S::Symbol("E");
const C_A: S<'static> = S::Symbol("A");
const C_Q: S<'static> = S::Symbol("Q");
const C_IDENTITY__PLUS: S<'static> = S::Symbol("identity-+");
const C_Z: S<'static> = S::Symbol("z");
const C_COMMON_ADDENDS__LT: S<'static> = S::Symbol("common-addends-<");
const C_POSITIVES__PLUS: S<'static> = S::Symbol("positives-+");
const C_NATP_SLASH__PLUS: S<'static> = S::Symbol("natp/+");
const C_COMMUTE__PLUS: S<'static> = S::Symbol("commute-+");
const C_C: S<'static> = S::Symbol("c");
const C_B: S<'static> = S::Symbol("b");
const C_ASSOCIATE__PLUS: S<'static> = S::Symbol("associate-+");
const C_SIZE_SLASH_CDR: S<'static> = S::Symbol("size/cdr");
const C_SIZE_SLASH_CAR: S<'static> = S::Symbol("size/car");
const C_NATP_SLASH_SIZE: S<'static> = S::Symbol("natp/size");
const C_EQUAL_IF: S<'static> = S::Symbol("equal-if");
const C_CONS_SLASH_CAR_PLUS_CDR: S<'static> = S::Symbol("cons/car+cdr");
const C_IF_NEST_A: S<'static> = S::Symbol("if-nest-A");
const C_IF_NEST_E: S<'static> = S::Symbol("if-nest-E");
const C_IF_FALSE: S<'static> = S::Symbol("if-false");
const C_IF_TRUE: S<'static> = S::Symbol("if-true");
const C_IF_SAME: S<'static> = S::Symbol("if-same");
const C_EQUAL_SWAP: S<'static> = S::Symbol("equal-swap");
const C_EQUAL_SAME: S<'static> = S::Symbol("equal-same");
const C_CDR_SLASH_CONS: S<'static> = S::Symbol("cdr/cons");
const C_CAR_SLASH_CONS: S<'static> = S::Symbol("car/cons");
const C_ATOM_SLASH_CONS: S<'static> = S::Symbol("atom/cons");
const C_STAR_INDUCTION: S<'static> = S::Symbol("star-induction");
const C_LIST_INDUCTION: S<'static> = S::Symbol("list-induction");
const C_PAIR_0: S<'static> = cons!(C__LT, S::Empty);
const C_PAIR_1: S<'static> = cons!(C__PLUS, C_PAIR_0);
const C_PAIR_8: S<'static> = cons!(C_EQUAL, C_ATOM, C_CAR, C_CDR, C_CONS, C_NATP, C_SIZE, C_PAIR_1);
const C_PAIR_9: S<'static> = cons!(C_SIZE, S::Empty);
const C_PAIR_10: S<'static> = cons!(C_NATP, C_PAIR_9);
const C_PAIR_11: S<'static> = cons!(C_CDR, C_PAIR_10);
const C_PAIR_12: S<'static> = cons!(C_CAR, C_PAIR_11);
const C_PAIR_13: S<'static> = cons!(C_ATOM, C_PAIR_12);
const C_PAIR_14: S<'static> = cons!(C_X, S::Empty);
const C_PAIR_15: S<'static> = cons!(C_CONS, C_PAIR_1);
const C_PAIR_16: S<'static> = cons!(C_EQUAL, C_PAIR_15);
const C_PAIR_17: S<'static> = cons!(C_Y, S::Empty);
const C_PAIR_18: S<'static> = cons!(C_X, C_PAIR_17);
const C_PAIR_19: S<'static> = cons!(C_E, S::Empty);
const C_PAIR_20: S<'static> = cons!(C_A, C_PAIR_19);
const C_PAIR_21: S<'static> = cons!(C_Q, C_PAIR_20);
const C_PAIR_22: S<'static> = cons!(C_T, S::Empty);
const C_PAIR_23: S<'static> = cons!(C_QUOTE, C_PAIR_22);
const C_PAIR_24: S<'static> = cons!(C_PAIR_23, S::Empty);
const C_PAIR_25: S<'static> = cons!(S::Num(0), S::Empty);
const C_PAIR_26: S<'static> = cons!(C_QUOTE, C_PAIR_25);
const C_PAIR_27: S<'static> = cons!(C_PAIR_26, C_PAIR_14);
const C_PAIR_32: S<'static> = cons!(C_NATP, C_PAIR_14);
const C_PAIR_42: S<'static> = cons!(C_Z, S::Empty);
const C_PAIR_43: S<'static> = cons!(C_Y, C_PAIR_42);
const C_PAIR_53: S<'static> = cons!(C_X, C_PAIR_43);
const C_PAIR_58: S<'static> = cons!(C__PLUS, C_PAIR_18);
const C_PAIR_59: S<'static> = cons!(C_PAIR_58, S::Empty);
const C_PAIR_93: S<'static> = cons!(C_Y, C_PAIR_14);
const C_PAIR_103: S<'static> = cons!(C_C, S::Empty);
const C_PAIR_104: S<'static> = cons!(C_B, C_PAIR_103);
const C_PAIR_123: S<'static> = cons!(C_SIZE, C_PAIR_14);
const C_PAIR_124: S<'static> = cons!(C_PAIR_123, S::Empty);
const C_PAIR_125: S<'static> = cons!(C_CDR, C_PAIR_14);
const C_PAIR_126: S<'static> = cons!(C_PAIR_125, S::Empty);
const C_PAIR_134: S<'static> = cons!(C_ATOM, C_PAIR_14);
const C_PAIR_142: S<'static> = cons!(C_CAR, C_PAIR_14);
const C_PAIR_143: S<'static> = cons!(C_PAIR_142, S::Empty);
const C_PAIR_166: S<'static> = cons!(C_EQUAL, C_PAIR_18);
const C_PAIR_188: S<'static> = cons!(C_IF, C_PAIR_53);
const C_PAIR_210: S<'static> = cons!(C_NIL, S::Empty);
const C_PAIR_211: S<'static> = cons!(C_QUOTE, C_PAIR_210);
const C_PAIR_258: S<'static> = cons!(C_CONS, C_PAIR_18);
const C_PAIR_259: S<'static> = cons!(C_PAIR_258, S::Empty);
const C_PAIR_276: S<'static> = cons!(C_PAIR_211, S::Empty);
const C_PAIR_284: S<'static> = cons!(
    cons!(
        C_DETHM,
        C_ATOM_SLASH_CONS,
        C_PAIR_18,
        cons!(C_EQUAL, cons!(C_ATOM, C_PAIR_259), C_PAIR_276),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_CAR_SLASH_CONS,
        C_PAIR_18,
        cons!(C_EQUAL, cons!(C_CAR, C_PAIR_259), C_PAIR_14),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_CDR_SLASH_CONS,
        C_PAIR_18,
        cons!(C_EQUAL, cons!(C_CDR, C_PAIR_259), C_PAIR_17),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_EQUAL_SAME,
        C_PAIR_14,
        cons!(C_EQUAL, cons!(C_EQUAL, C_X, C_PAIR_14), C_PAIR_24),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_EQUAL_SWAP,
        C_PAIR_18,
        cons!(C_EQUAL, C_PAIR_166, cons!(C_EQUAL, C_PAIR_93), S::Empty),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_IF_SAME,
        C_PAIR_18,
        cons!(C_EQUAL, cons!(C_IF, C_X, C_Y, C_PAIR_17), C_PAIR_17),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_IF_TRUE,
        C_PAIR_18,
        cons!(C_EQUAL, cons!(C_IF, C_PAIR_23, C_PAIR_18), C_PAIR_14),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_IF_FALSE,
        C_PAIR_18,
        cons!(C_EQUAL, cons!(C_IF, C_PAIR_211, C_PAIR_18), C_PAIR_17),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_IF_NEST_E,
        C_PAIR_53,
        cons!(
            C_IF,
            C_X,
            C_PAIR_23,
            cons!(C_EQUAL, C_PAIR_188, C_PAIR_42),
            S::Empty
        ),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_IF_NEST_A,
        C_PAIR_53,
        cons!(C_IF, C_X, cons!(C_EQUAL, C_PAIR_188, C_PAIR_17), C_PAIR_24),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_CONS_SLASH_CAR_PLUS_CDR,
        C_PAIR_14,
        cons!(
            C_IF,
            C_PAIR_134,
            C_PAIR_23,
            cons!(C_EQUAL, cons!(C_CONS, C_PAIR_142, C_PAIR_126), C_PAIR_14),
            S::Empty
        ),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_EQUAL_IF,
        C_PAIR_18,
        cons!(C_IF, C_PAIR_166, C_PAIR_166, C_PAIR_24),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_NATP_SLASH_SIZE,
        C_PAIR_14,
        cons!(C_EQUAL, cons!(C_NATP, C_PAIR_124), C_PAIR_24),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_SIZE_SLASH_CAR,
        C_PAIR_14,
        cons!(
            C_IF,
            C_PAIR_134,
            C_PAIR_23,
            cons!(
                C_EQUAL,
                cons!(C__LT, cons!(C_SIZE, C_PAIR_143), C_PAIR_124),
                C_PAIR_24
            ),
            S::Empty
        ),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_SIZE_SLASH_CDR,
        C_PAIR_14,
        cons!(
            C_IF,
            C_PAIR_134,
            C_PAIR_23,
            cons!(
                C_EQUAL,
                cons!(C__LT, cons!(C_SIZE, C_PAIR_126), C_PAIR_124),
                C_PAIR_24
            ),
            S::Empty
        ),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_ASSOCIATE__PLUS,
        cons!(C_A, C_PAIR_104),
        cons!(
            C_EQUAL,
            cons!(C__PLUS, cons!(C__PLUS, C_A, C_B, S::Empty), C_PAIR_103),
            cons!(C__PLUS, C_A, cons!(C__PLUS, C_PAIR_104), S::Empty),
            S::Empty
        ),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_COMMUTE__PLUS,
        C_PAIR_18,
        cons!(C_EQUAL, C_PAIR_58, cons!(C__PLUS, C_PAIR_93), S::Empty),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_NATP_SLASH__PLUS,
        C_PAIR_18,
        cons!(
            C_IF,
            C_PAIR_32,
            cons!(
                C_IF,
                cons!(C_NATP, C_PAIR_17),
                cons!(C_EQUAL, cons!(C_NATP, C_PAIR_59), C_PAIR_24),
                C_PAIR_24
            ),
            C_PAIR_24
        ),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_POSITIVES__PLUS,
        C_PAIR_18,
        cons!(
            C_IF,
            cons!(C__LT, C_PAIR_27),
            cons!(
                C_IF,
                cons!(C__LT, C_PAIR_26, C_PAIR_17),
                cons!(C_EQUAL, cons!(C__LT, C_PAIR_26, C_PAIR_59), C_PAIR_24),
                C_PAIR_24
            ),
            C_PAIR_24
        ),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_COMMON_ADDENDS__LT,
        C_PAIR_53,
        cons!(
            C_EQUAL,
            cons!(
                C__LT,
                cons!(C__PLUS, C_X, C_PAIR_42),
                cons!(C__PLUS, C_PAIR_43),
                S::Empty
            ),
            cons!(C__LT, C_PAIR_18),
            S::Empty
        ),
        S::Empty
    ),
    cons!(
        C_DETHM,
        C_IDENTITY__PLUS,
        C_PAIR_14,
        cons!(
            C_IF,
            C_PAIR_32,
            cons!(C_EQUAL, cons!(C__PLUS, C_PAIR_27), C_PAIR_14),
            C_PAIR_24
        ),
        S::Empty
    ),
    S::Empty
);
const C_PAIR_285: S<'static> = cons!(C_PAIR_23, C_PAIR_276);
const C_PAIR_286: S<'static> = cons!(C_IF_TRUE, C_PAIR_285);
const C_PAIR_287: S<'static> = cons!(C_PAIR_286, S::Empty);
const C_PAIR_288: S<'static> = cons!(S::Empty, C_PAIR_287);
const C_PAIR_289: S<'static> = cons!(C_PAIR_288, S::Empty);
const C_PAIR_290: S<'static> = cons!(C_NATP_SLASH_SIZE, C_PAIR_14);
const C_PAIR_291: S<'static> = cons!(C_PAIR_290, S::Empty);
const C_PAIR_292: S<'static> = cons!(C_Q, S::Empty);
const C_PAIR_293: S<'static> = cons!(C_PAIR_292, C_PAIR_291);
const C_PAIR_294: S<'static> = cons!(C_PAIR_293, C_PAIR_289);
const C_PAIR_295: S<'static> = cons!(C_PAIR_134, C_PAIR_24);
const C_PAIR_296: S<'static> = cons!(C_IF_SAME, C_PAIR_295);
const C_PAIR_297: S<'static> = cons!(C_PAIR_296, S::Empty);
const C_PAIR_298: S<'static> = cons!(C_A, S::Empty);
const C_PAIR_299: S<'static> = cons!(C_PAIR_298, C_PAIR_297);
const C_PAIR_300: S<'static> = cons!(C_PAIR_299, C_PAIR_294);
const C_PAIR_309: S<'static> = cons!(C_SIZE_SLASH_CDR, C_PAIR_14);
const C_PAIR_310: S<'static> = cons!(C_PAIR_309, S::Empty);
const C_PAIR_349: S<'static> = cons!(
    cons!(
        cons!(
            C_DEFUN,
            C_LIST_INDUCTION,
            C_PAIR_14,
            cons!(
                C_IF,
                C_PAIR_134,
                cons!(C_QUOTE, S::Empty, S::Empty),
                cons!(
                    C_CONS,
                    C_PAIR_142,
                    cons!(C_LIST_INDUCTION, C_PAIR_126),
                    S::Empty
                ),
                S::Empty
            ),
            S::Empty
        ),
        C_PAIR_123,
        cons!(C_PAIR_20, C_PAIR_310),
        C_PAIR_300
    ),
    cons!(
        cons!(
            C_DEFUN,
            C_STAR_INDUCTION,
            C_PAIR_14,
            cons!(
                C_IF,
                C_PAIR_134,
                C_X,
                cons!(
                    C_CONS,
                    cons!(C_STAR_INDUCTION, C_PAIR_143),
                    cons!(C_STAR_INDUCTION, C_PAIR_126),
                    S::Empty
                ),
                S::Empty
            ),
            S::Empty
        ),
        C_PAIR_123,
        cons!(cons!(C_A, C_E, C_PAIR_298), C_PAIR_310),
        cons!(
            cons!(C_A, C_E, C_PAIR_292),
            cons!(C_SIZE_SLASH_CAR, C_PAIR_14),
            S::Empty
        ),
        cons!(C_PAIR_20, C_PAIR_287),
        C_PAIR_300
    ),
    S::Empty
);

pub fn list0<'a>(context: &Context<'a>) -> S<'a> {
    S::Empty
}

pub fn is_list0<'a>(context: &Context<'a>, x: S<'a>) -> S<'a> {
    equal(context, x, S::Empty)
}

pub fn list1<'a>(context: &Context<'a>, x: S<'a>) -> S<'a> {
    cons(context, x, list0(context))
}

pub fn is_list1<'a>(context: &Context<'a>, x: S<'a>) -> S<'a> {
    if atom(context, x) != C_NIL {
        C_NIL
    } else {
        is_list0(context, cdr(context, x))
    }
}

pub fn elem1<'a>(context: &Context<'a>, xs: S<'a>) -> S<'a> {
    car(context, xs)
}

pub fn list2<'a>(context: &Context<'a>, x: S<'a>, y: S<'a>) -> S<'a> {
    cons(context, x, list1(context, y))
}

pub fn is_list2<'a>(context: &Context<'a>, x: S<'a>) -> S<'a> {
    if atom(context, x) != C_NIL {
        C_NIL
    } else {
        is_list1(context, cdr(context, x))
    }
}

pub fn elem2<'a>(context: &Context<'a>, xs: S<'a>) -> S<'a> {
    elem1(context, cdr(context, xs))
}

pub fn list3<'a>(context: &Context<'a>, x: S<'a>, y: S<'a>, z: S<'a>) -> S<'a> {
    cons(context, x, list2(context, y, z))
}

pub fn is_list3<'a>(context: &Context<'a>, x: S<'a>) -> S<'a> {
    if atom(context, x) != C_NIL {
        C_NIL
    } else {
        is_list2(context, cdr(context, x))
    }
}

pub fn elem3<'a>(context: &Context<'a>, xs: S<'a>) -> S<'a> {
    elem2(context, cdr(context, xs))
}

pub fn tag<'a>(context: &Context<'a>, sym: S<'a>, x: S<'a>) -> S<'a> {
    cons(context, sym, x)
}

pub fn is_tag<'a>(context: &Context<'a>, sym: S<'a>, x: S<'a>) -> S<'a> {
    if atom(context, x) != C_NIL {
        C_NIL
    } else {
        equal(context, car(context, x), sym)
    }
}

pub fn untag<'a>(context: &Context<'a>, x: S<'a>) -> S<'a> {
    cdr(context, x)
}

pub fn is_member<'a>(context: &Context<'a>, x: S<'a>, ys: S<'a>) -> S<'a> {
    if atom(context, ys) != C_NIL {
        C_NIL
    } else {
        if equal(context, x, car(context, ys)) != C_NIL {
            C_T
        } else {
            is_member(context, x, cdr(context, ys))
        }
    }
}

pub fn quote_c<'a>(context: &Context<'a>, value: S<'a>) -> S<'a> {
    tag(context, C_QUOTE, list1(context, value))
}

pub fn is_quote<'a>(context: &Context<'a>, x: S<'a>) -> S<'a> {
    if is_tag(context, C_QUOTE, x) != C_NIL {
        is_list1(context, untag(context, x))
    } else {
        C_NIL
    }
}

pub fn quote_dot_value<'a>(context: &Context<'a>, e: S<'a>) -> S<'a> {
    elem1(context, untag(context, e))
}

pub fn if_c<'a>(context: &Context<'a>, q: S<'a>, a: S<'a>, e: S<'a>) -> S<'a> {
    tag(context, C_IF, list3(context, q, a, e))
}

pub fn is_if<'a>(context: &Context<'a>, x: S<'a>) -> S<'a> {
    if is_tag(context, C_IF, x) != C_NIL {
        is_list3(context, untag(context, x))
    } else {
        C_NIL
    }
}

pub fn if_dot_q<'a>(context: &Context<'a>, e: S<'a>) -> S<'a> {
    elem1(context, untag(context, e))
}

pub fn if_dot_a<'a>(context: &Context<'a>, e: S<'a>) -> S<'a> {
    elem2(context, untag(context, e))
}

pub fn if_dot_e<'a>(context: &Context<'a>, e: S<'a>) -> S<'a> {
    elem3(context, untag(context, e))
}

pub fn app_c<'a>(context: &Context<'a>, name: S<'a>, args: S<'a>) -> S<'a> {
    cons(context, name, args)
}

pub fn is_app<'a>(context: &Context<'a>, x: S<'a>) -> S<'a> {
    if atom(context, x) != C_NIL {
        C_NIL
    } else {
        if is_quote(context, x) != C_NIL {
            C_NIL
        } else {
            if is_if(context, x) != C_NIL {
                C_NIL
            } else {
                C_T
            }
        }
    }
}

pub fn app_dot_name<'a>(context: &Context<'a>, e: S<'a>) -> S<'a> {
    car(context, e)
}

pub fn app_dot_args<'a>(context: &Context<'a>, e: S<'a>) -> S<'a> {
    cdr(context, e)
}

pub fn is_var<'a>(context: &Context<'a>, x: S<'a>) -> S<'a> {
    if equal(context, x, C_T) != C_NIL {
        C_NIL
    } else {
        if equal(context, x, C_NIL) != C_NIL {
            C_NIL
        } else {
            if natp(context, x) != C_NIL {
                C_NIL
            } else {
                atom(context, x)
            }
        }
    }
}

pub fn defun_c<'a>(context: &Context<'a>, name: S<'a>, formals: S<'a>, body: S<'a>) -> S<'a> {
    tag(context, C_DEFUN, list3(context, name, formals, body))
}

pub fn is_defun<'a>(context: &Context<'a>, x: S<'a>) -> S<'a> {
    if is_tag(context, C_DEFUN, x) != C_NIL {
        is_list3(context, untag(context, x))
    } else {
        C_NIL
    }
}

pub fn defun_dot_name<'a>(context: &Context<'a>, def: S<'a>) -> S<'a> {
    elem1(context, untag(context, def))
}

pub fn defun_dot_formals<'a>(context: &Context<'a>, def: S<'a>) -> S<'a> {
    elem2(context, untag(context, def))
}

pub fn defun_dot_body<'a>(context: &Context<'a>, def: S<'a>) -> S<'a> {
    elem3(context, untag(context, def))
}

pub fn dethm_c<'a>(context: &Context<'a>, name: S<'a>, formals: S<'a>, body: S<'a>) -> S<'a> {
    tag(context, C_DETHM, list3(context, name, formals, body))
}

pub fn is_dethm<'a>(context: &Context<'a>, x: S<'a>) -> S<'a> {
    if is_tag(context, C_DETHM, x) != C_NIL {
        is_list3(context, untag(context, x))
    } else {
        C_NIL
    }
}

pub fn dethm_dot_name<'a>(context: &Context<'a>, def: S<'a>) -> S<'a> {
    elem1(context, untag(context, def))
}

pub fn dethm_dot_formals<'a>(context: &Context<'a>, def: S<'a>) -> S<'a> {
    elem2(context, untag(context, def))
}

pub fn dethm_dot_body<'a>(context: &Context<'a>, def: S<'a>) -> S<'a> {
    elem3(context, untag(context, def))
}

pub fn if_qae<'a>(context: &Context<'a>, e: S<'a>) -> S<'a> {
    list3(
        context,
        if_dot_q(context, e),
        if_dot_a(context, e),
        if_dot_e(context, e),
    )
}

pub fn qae_if<'a>(context: &Context<'a>, es: S<'a>) -> S<'a> {
    if_c(
        context,
        elem1(context, es),
        elem2(context, es),
        elem3(context, es),
    )
}

pub fn is_rator<'a>(context: &Context<'a>, name: S<'a>) -> S<'a> {
    is_member(context, name, C_PAIR_8)
}

pub fn rator_dot_formals<'a>(context: &Context<'a>, rator: S<'a>) -> S<'a> {
    if is_member(context, rator, C_PAIR_13) != C_NIL {
        C_PAIR_14
    } else {
        if is_member(context, rator, C_PAIR_16) != C_NIL {
            C_PAIR_18
        } else {
            C_NIL
        }
    }
}

pub fn def_dot_name<'a>(context: &Context<'a>, def: S<'a>) -> S<'a> {
    if is_defun(context, def) != C_NIL {
        defun_dot_name(context, def)
    } else {
        if is_dethm(context, def) != C_NIL {
            dethm_dot_name(context, def)
        } else {
            def
        }
    }
}

pub fn def_dot_formals<'a>(context: &Context<'a>, def: S<'a>) -> S<'a> {
    if is_dethm(context, def) != C_NIL {
        dethm_dot_formals(context, def)
    } else {
        if is_defun(context, def) != C_NIL {
            defun_dot_formals(context, def)
        } else {
            S::Empty
        }
    }
}

pub fn if_c_when_necessary<'a>(context: &Context<'a>, q: S<'a>, a: S<'a>, e: S<'a>) -> S<'a> {
    if equal(context, a, e) != C_NIL {
        a
    } else {
        if_c(context, q, a, e)
    }
}

pub fn conjunction<'a>(context: &Context<'a>, es: S<'a>) -> S<'a> {
    if atom(context, es) != C_NIL {
        quote_c(context, C_T)
    } else {
        if atom(context, cdr(context, es)) != C_NIL {
            car(context, es)
        } else {
            if_c(
                context,
                car(context, es),
                conjunction(context, cdr(context, es)),
                quote_c(context, C_NIL),
            )
        }
    }
}

pub fn implication<'a>(context: &Context<'a>, es: S<'a>, e: S<'a>) -> S<'a> {
    if atom(context, es) != C_NIL {
        e
    } else {
        if_c(
            context,
            car(context, es),
            implication(context, cdr(context, es), e),
            quote_c(context, C_T),
        )
    }
}

pub fn is_args_arity<'a>(context: &Context<'a>, def: S<'a>, args: S<'a>) -> S<'a> {
    if is_dethm(context, def) != C_NIL {
        C_NIL
    } else {
        if is_defun(context, def) != C_NIL {
            is_arity(context, defun_dot_formals(context, def), args)
        } else {
            if is_rator(context, def) != C_NIL {
                is_arity(context, rator_dot_formals(context, def), args)
            } else {
                C_NIL
            }
        }
    }
}

pub fn is_app_arity<'a>(context: &Context<'a>, defs: S<'a>, app: S<'a>) -> S<'a> {
    is_args_arity(
        context,
        lookup(context, app_dot_name(context, app), defs),
        app_dot_args(context, app),
    )
}

pub fn lookup<'a>(context: &Context<'a>, name: S<'a>, defs: S<'a>) -> S<'a> {
    if atom(context, defs) != C_NIL {
        name
    } else {
        if equal(context, def_dot_name(context, car(context, defs)), name) != C_NIL {
            car(context, defs)
        } else {
            lookup(context, name, cdr(context, defs))
        }
    }
}

pub fn is_undefined<'a>(context: &Context<'a>, name: S<'a>, defs: S<'a>) -> S<'a> {
    if is_var(context, name) != C_NIL {
        equal(context, lookup(context, name, defs), name)
    } else {
        C_NIL
    }
}

pub fn is_bound<'a>(context: &Context<'a>, var: S<'a>, vars: S<'a>) -> S<'a> {
    if equal(context, vars, C_ANY) != C_NIL {
        C_T
    } else {
        is_member(context, var, vars)
    }
}

pub fn is_exprs<'a>(context: &Context<'a>, defs: S<'a>, vars: S<'a>, es: S<'a>) -> S<'a> {
    if atom(context, es) != C_NIL {
        C_T
    } else {
        if is_var(context, car(context, es)) != C_NIL {
            if is_bound(context, car(context, es), vars) != C_NIL {
                is_exprs(context, defs, vars, cdr(context, es))
            } else {
                C_NIL
            }
        } else {
            if is_quote(context, car(context, es)) != C_NIL {
                is_exprs(context, defs, vars, cdr(context, es))
            } else {
                if is_if(context, car(context, es)) != C_NIL {
                    if is_exprs(context, defs, vars, if_qae(context, car(context, es))) != C_NIL {
                        is_exprs(context, defs, vars, cdr(context, es))
                    } else {
                        C_NIL
                    }
                } else {
                    if is_app(context, car(context, es)) != C_NIL {
                        if is_app_arity(context, defs, car(context, es)) != C_NIL {
                            if is_exprs(
                                context,
                                defs,
                                vars,
                                app_dot_args(context, car(context, es)),
                            ) != C_NIL
                            {
                                is_exprs(context, defs, vars, cdr(context, es))
                            } else {
                                C_NIL
                            }
                        } else {
                            C_NIL
                        }
                    } else {
                        C_NIL
                    }
                }
            }
        }
    }
}

pub fn is_expr<'a>(context: &Context<'a>, defs: S<'a>, vars: S<'a>, e: S<'a>) -> S<'a> {
    is_exprs(context, defs, vars, list1(context, e))
}

pub fn is_subset<'a>(context: &Context<'a>, xs: S<'a>, ys: S<'a>) -> S<'a> {
    if atom(context, xs) != C_NIL {
        C_T
    } else {
        if is_member(context, car(context, xs), ys) != C_NIL {
            is_subset(context, cdr(context, xs), ys)
        } else {
            C_NIL
        }
    }
}

pub fn list_extend<'a>(context: &Context<'a>, xs: S<'a>, x: S<'a>) -> S<'a> {
    if atom(context, xs) != C_NIL {
        list1(context, x)
    } else {
        if equal(context, car(context, xs), x) != C_NIL {
            xs
        } else {
            cons(
                context,
                car(context, xs),
                list_extend(context, cdr(context, xs), x),
            )
        }
    }
}

pub fn list_union<'a>(context: &Context<'a>, xs: S<'a>, ys: S<'a>) -> S<'a> {
    if atom(context, ys) != C_NIL {
        xs
    } else {
        list_union(
            context,
            list_extend(context, xs, car(context, ys)),
            cdr(context, ys),
        )
    }
}

pub fn get_arg_from<'a>(context: &Context<'a>, n: S<'a>, args: S<'a>, from: S<'a>) -> S<'a> {
    if atom(context, args) != C_NIL {
        C_NIL
    } else {
        if equal(context, n, from) != C_NIL {
            car(context, args)
        } else {
            get_arg_from(
                context,
                n,
                cdr(context, args),
                _plus(context, from, S::Num(1)),
            )
        }
    }
}

pub fn get_arg<'a>(context: &Context<'a>, n: S<'a>, args: S<'a>) -> S<'a> {
    get_arg_from(context, n, args, S::Num(1))
}

pub fn set_arg_from<'a>(
    context: &Context<'a>,
    n: S<'a>,
    args: S<'a>,
    y: S<'a>,
    from: S<'a>,
) -> S<'a> {
    if atom(context, args) != C_NIL {
        S::Empty
    } else {
        if equal(context, n, from) != C_NIL {
            cons(context, y, cdr(context, args))
        } else {
            cons(
                context,
                car(context, args),
                set_arg_from(
                    context,
                    n,
                    cdr(context, args),
                    y,
                    _plus(context, from, S::Num(1)),
                ),
            )
        }
    }
}

pub fn set_arg<'a>(context: &Context<'a>, n: S<'a>, args: S<'a>, y: S<'a>) -> S<'a> {
    set_arg_from(context, n, args, y, S::Num(1))
}

pub fn _lt__eq_len_from<'a>(context: &Context<'a>, n: S<'a>, args: S<'a>, from: S<'a>) -> S<'a> {
    if atom(context, args) != C_NIL {
        C_NIL
    } else {
        if equal(context, n, from) != C_NIL {
            C_T
        } else {
            _lt__eq_len_from(
                context,
                n,
                cdr(context, args),
                _plus(context, from, S::Num(1)),
            )
        }
    }
}

pub fn _lt__eq_len<'a>(context: &Context<'a>, n: S<'a>, args: S<'a>) -> S<'a> {
    if _lt(context, S::Num(0), n) != C_NIL {
        _lt__eq_len_from(context, n, args, S::Num(1))
    } else {
        C_NIL
    }
}

pub fn is_arity<'a>(context: &Context<'a>, vars: S<'a>, es: S<'a>) -> S<'a> {
    if atom(context, vars) != C_NIL {
        atom(context, es)
    } else {
        if atom(context, es) != C_NIL {
            C_NIL
        } else {
            is_arity(context, cdr(context, vars), cdr(context, es))
        }
    }
}

pub fn is_formals<'a>(context: &Context<'a>, vars: S<'a>) -> S<'a> {
    if atom(context, vars) != C_NIL {
        C_T
    } else {
        if is_var(context, car(context, vars)) != C_NIL {
            if is_member(context, car(context, vars), cdr(context, vars)) != C_NIL {
                C_NIL
            } else {
                is_formals(context, cdr(context, vars))
            }
        } else {
            C_NIL
        }
    }
}

pub fn is_direction<'a>(context: &Context<'a>, dir: S<'a>) -> S<'a> {
    if natp(context, dir) != C_NIL {
        C_T
    } else {
        is_member(context, dir, C_PAIR_21)
    }
}

pub fn is_path<'a>(context: &Context<'a>, path: S<'a>) -> S<'a> {
    if atom(context, path) != C_NIL {
        C_T
    } else {
        if is_direction(context, car(context, path)) != C_NIL {
            is_path(context, cdr(context, path))
        } else {
            C_NIL
        }
    }
}

pub fn is_quoted_exprs<'a>(context: &Context<'a>, args: S<'a>) -> S<'a> {
    if atom(context, args) != C_NIL {
        C_T
    } else {
        if is_quote(context, car(context, args)) != C_NIL {
            is_quoted_exprs(context, cdr(context, args))
        } else {
            C_NIL
        }
    }
}

pub fn is_step_args<'a>(context: &Context<'a>, defs: S<'a>, def: S<'a>, args: S<'a>) -> S<'a> {
    if is_dethm(context, def) != C_NIL {
        if is_arity(context, dethm_dot_formals(context, def), args) != C_NIL {
            is_exprs(context, defs, C_ANY, args)
        } else {
            C_NIL
        }
    } else {
        if is_defun(context, def) != C_NIL {
            if is_arity(context, defun_dot_formals(context, def), args) != C_NIL {
                is_exprs(context, defs, C_ANY, args)
            } else {
                C_NIL
            }
        } else {
            if is_rator(context, def) != C_NIL {
                if is_arity(context, rator_dot_formals(context, def), args) != C_NIL {
                    is_quoted_exprs(context, args)
                } else {
                    C_NIL
                }
            } else {
                C_NIL
            }
        }
    }
}

pub fn is_step_app<'a>(context: &Context<'a>, defs: S<'a>, app: S<'a>) -> S<'a> {
    is_step_args(
        context,
        defs,
        lookup(context, app_dot_name(context, app), defs),
        app_dot_args(context, app),
    )
}

pub fn is_step<'a>(context: &Context<'a>, defs: S<'a>, step: S<'a>) -> S<'a> {
    if is_path(context, elem1(context, step)) != C_NIL {
        if is_app(context, elem2(context, step)) != C_NIL {
            is_step_app(context, defs, elem2(context, step))
        } else {
            C_NIL
        }
    } else {
        C_NIL
    }
}

pub fn is_steps<'a>(context: &Context<'a>, defs: S<'a>, steps: S<'a>) -> S<'a> {
    if atom(context, steps) != C_NIL {
        C_T
    } else {
        if is_step(context, defs, car(context, steps)) != C_NIL {
            is_steps(context, defs, cdr(context, steps))
        } else {
            C_NIL
        }
    }
}

pub fn is_induction_scheme_for<'a>(
    context: &Context<'a>,
    def: S<'a>,
    vars: S<'a>,
    e: S<'a>,
) -> S<'a> {
    if is_defun(context, def) != C_NIL {
        if is_arity(
            context,
            defun_dot_formals(context, def),
            app_dot_args(context, e),
        ) != C_NIL
        {
            if is_formals(context, app_dot_args(context, e)) != C_NIL {
                is_subset(context, app_dot_args(context, e), vars)
            } else {
                C_NIL
            }
        } else {
            C_NIL
        }
    } else {
        C_NIL
    }
}

pub fn is_induction_scheme<'a>(context: &Context<'a>, defs: S<'a>, vars: S<'a>, e: S<'a>) -> S<'a> {
    if is_app(context, e) != C_NIL {
        is_induction_scheme_for(
            context,
            lookup(context, app_dot_name(context, e), defs),
            vars,
            e,
        )
    } else {
        C_NIL
    }
}

pub fn is_seed<'a>(context: &Context<'a>, defs: S<'a>, def: S<'a>, seed: S<'a>) -> S<'a> {
    if equal(context, seed, C_NIL) != C_NIL {
        C_T
    } else {
        if is_defun(context, def) != C_NIL {
            is_expr(context, defs, defun_dot_formals(context, def), seed)
        } else {
            if is_dethm(context, def) != C_NIL {
                is_induction_scheme(context, defs, dethm_dot_formals(context, def), seed)
            } else {
                C_NIL
            }
        }
    }
}

pub fn extend_rec<'a>(context: &Context<'a>, defs: S<'a>, def: S<'a>) -> S<'a> {
    if is_defun(context, def) != C_NIL {
        list_extend(
            context,
            defs,
            defun_c(
                context,
                defun_dot_name(context, def),
                defun_dot_formals(context, def),
                app_c(
                    context,
                    defun_dot_name(context, def),
                    defun_dot_formals(context, def),
                ),
            ),
        )
    } else {
        defs
    }
}

pub fn is_def_contents<'a>(
    context: &Context<'a>,
    known_defs: S<'a>,
    formals: S<'a>,
    body: S<'a>,
) -> S<'a> {
    if is_formals(context, formals) != C_NIL {
        is_expr(context, known_defs, formals, body)
    } else {
        C_NIL
    }
}

pub fn is_def<'a>(context: &Context<'a>, known_defs: S<'a>, def: S<'a>) -> S<'a> {
    if is_dethm(context, def) != C_NIL {
        if is_undefined(context, dethm_dot_name(context, def), known_defs) != C_NIL {
            is_def_contents(
                context,
                known_defs,
                dethm_dot_formals(context, def),
                dethm_dot_body(context, def),
            )
        } else {
            C_NIL
        }
    } else {
        if is_defun(context, def) != C_NIL {
            if is_undefined(context, defun_dot_name(context, def), known_defs) != C_NIL {
                is_def_contents(
                    context,
                    extend_rec(context, known_defs, def),
                    defun_dot_formals(context, def),
                    defun_dot_body(context, def),
                )
            } else {
                C_NIL
            }
        } else {
            C_NIL
        }
    }
}

pub fn is_defs<'a>(context: &Context<'a>, known_defs: S<'a>, defs: S<'a>) -> S<'a> {
    if atom(context, defs) != C_NIL {
        C_T
    } else {
        if is_def(context, known_defs, car(context, defs)) != C_NIL {
            is_defs(
                context,
                list_extend(context, known_defs, car(context, defs)),
                cdr(context, defs),
            )
        } else {
            C_NIL
        }
    }
}

pub fn is_list2_or_more<'a>(context: &Context<'a>, pf: S<'a>) -> S<'a> {
    if atom(context, pf) != C_NIL {
        C_NIL
    } else {
        if atom(context, cdr(context, pf)) != C_NIL {
            C_NIL
        } else {
            C_T
        }
    }
}

pub fn is_proof<'a>(context: &Context<'a>, defs: S<'a>, pf: S<'a>) -> S<'a> {
    if is_list2_or_more(context, pf) != C_NIL {
        if is_def(context, defs, elem1(context, pf)) != C_NIL {
            if is_seed(context, defs, elem1(context, pf), elem2(context, pf)) != C_NIL {
                is_steps(
                    context,
                    extend_rec(context, defs, elem1(context, pf)),
                    cdr(context, cdr(context, pf)),
                )
            } else {
                C_NIL
            }
        } else {
            C_NIL
        }
    } else {
        C_NIL
    }
}

pub fn is_proofs<'a>(context: &Context<'a>, defs: S<'a>, pfs: S<'a>) -> S<'a> {
    if atom(context, pfs) != C_NIL {
        C_T
    } else {
        if is_proof(context, defs, car(context, pfs)) != C_NIL {
            is_proofs(
                context,
                list_extend(context, defs, elem1(context, car(context, pfs))),
                cdr(context, pfs),
            )
        } else {
            C_NIL
        }
    }
}

pub fn sub_var<'a>(context: &Context<'a>, vars: S<'a>, args: S<'a>, var: S<'a>) -> S<'a> {
    if atom(context, vars) != C_NIL {
        var
    } else {
        if equal(context, car(context, vars), var) != C_NIL {
            car(context, args)
        } else {
            sub_var(context, cdr(context, vars), cdr(context, args), var)
        }
    }
}

pub fn sub_es<'a>(context: &Context<'a>, vars: S<'a>, args: S<'a>, es: S<'a>) -> S<'a> {
    if atom(context, es) != C_NIL {
        S::Empty
    } else {
        if is_var(context, car(context, es)) != C_NIL {
            cons(
                context,
                sub_var(context, vars, args, car(context, es)),
                sub_es(context, vars, args, cdr(context, es)),
            )
        } else {
            if is_quote(context, car(context, es)) != C_NIL {
                cons(
                    context,
                    car(context, es),
                    sub_es(context, vars, args, cdr(context, es)),
                )
            } else {
                if is_if(context, car(context, es)) != C_NIL {
                    cons(
                        context,
                        qae_if(
                            context,
                            sub_es(context, vars, args, if_qae(context, car(context, es))),
                        ),
                        sub_es(context, vars, args, cdr(context, es)),
                    )
                } else {
                    cons(
                        context,
                        app_c(
                            context,
                            app_dot_name(context, car(context, es)),
                            sub_es(context, vars, args, app_dot_args(context, car(context, es))),
                        ),
                        sub_es(context, vars, args, cdr(context, es)),
                    )
                }
            }
        }
    }
}

pub fn sub_e<'a>(context: &Context<'a>, vars: S<'a>, args: S<'a>, e: S<'a>) -> S<'a> {
    elem1(context, sub_es(context, vars, args, list1(context, e)))
}

pub fn exprs_recs<'a>(context: &Context<'a>, f: S<'a>, es: S<'a>) -> S<'a> {
    if atom(context, es) != C_NIL {
        S::Empty
    } else {
        if is_var(context, car(context, es)) != C_NIL {
            exprs_recs(context, f, cdr(context, es))
        } else {
            if is_quote(context, car(context, es)) != C_NIL {
                exprs_recs(context, f, cdr(context, es))
            } else {
                if is_if(context, car(context, es)) != C_NIL {
                    list_union(
                        context,
                        exprs_recs(context, f, if_qae(context, car(context, es))),
                        exprs_recs(context, f, cdr(context, es)),
                    )
                } else {
                    if equal(context, app_dot_name(context, car(context, es)), f) != C_NIL {
                        list_union(
                            context,
                            list1(context, car(context, es)),
                            list_union(
                                context,
                                exprs_recs(context, f, app_dot_args(context, car(context, es))),
                                exprs_recs(context, f, cdr(context, es)),
                            ),
                        )
                    } else {
                        list_union(
                            context,
                            exprs_recs(context, f, app_dot_args(context, car(context, es))),
                            exprs_recs(context, f, cdr(context, es)),
                        )
                    }
                }
            }
        }
    }
}

pub fn expr_recs<'a>(context: &Context<'a>, f: S<'a>, e: S<'a>) -> S<'a> {
    exprs_recs(context, f, list1(context, e))
}

pub fn totality_slash__lt<'a>(
    context: &Context<'a>,
    meas: S<'a>,
    formals: S<'a>,
    app: S<'a>,
) -> S<'a> {
    app_c(
        context,
        C__LT,
        list2(
            context,
            sub_e(context, formals, app_dot_args(context, app), meas),
            meas,
        ),
    )
}

pub fn totality_slash_meas<'a>(
    context: &Context<'a>,
    meas: S<'a>,
    formals: S<'a>,
    apps: S<'a>,
) -> S<'a> {
    if atom(context, apps) != C_NIL {
        S::Empty
    } else {
        cons(
            context,
            totality_slash__lt(context, meas, formals, car(context, apps)),
            totality_slash_meas(context, meas, formals, cdr(context, apps)),
        )
    }
}

pub fn totality_slash_if<'a>(
    context: &Context<'a>,
    meas: S<'a>,
    f: S<'a>,
    formals: S<'a>,
    e: S<'a>,
) -> S<'a> {
    if is_if(context, e) != C_NIL {
        conjunction(
            context,
            list_extend(
                context,
                totality_slash_meas(
                    context,
                    meas,
                    formals,
                    expr_recs(context, f, if_dot_q(context, e)),
                ),
                if_c_when_necessary(
                    context,
                    if_dot_q(context, e),
                    totality_slash_if(context, meas, f, formals, if_dot_a(context, e)),
                    totality_slash_if(context, meas, f, formals, if_dot_e(context, e)),
                ),
            ),
        )
    } else {
        conjunction(
            context,
            totality_slash_meas(context, meas, formals, expr_recs(context, f, e)),
        )
    }
}

pub fn totality_slash_claim<'a>(context: &Context<'a>, meas: S<'a>, def: S<'a>) -> S<'a> {
    if equal(context, meas, C_NIL) != C_NIL {
        if equal(
            context,
            expr_recs(
                context,
                defun_dot_name(context, def),
                defun_dot_body(context, def),
            ),
            S::Empty,
        ) != C_NIL
        {
            quote_c(context, C_T)
        } else {
            quote_c(context, C_NIL)
        }
    } else {
        if_c(
            context,
            app_c(context, C_NATP, list1(context, meas)),
            totality_slash_if(
                context,
                meas,
                defun_dot_name(context, def),
                defun_dot_formals(context, def),
                defun_dot_body(context, def),
            ),
            quote_c(context, C_NIL),
        )
    }
}

pub fn induction_slash_prems<'a>(
    context: &Context<'a>,
    vars: S<'a>,
    claim: S<'a>,
    apps: S<'a>,
) -> S<'a> {
    if atom(context, apps) != C_NIL {
        S::Empty
    } else {
        cons(
            context,
            sub_e(
                context,
                vars,
                app_dot_args(context, car(context, apps)),
                claim,
            ),
            induction_slash_prems(context, vars, claim, cdr(context, apps)),
        )
    }
}

pub fn induction_slash_if<'a>(
    context: &Context<'a>,
    vars: S<'a>,
    claim: S<'a>,
    f: S<'a>,
    e: S<'a>,
) -> S<'a> {
    if is_if(context, e) != C_NIL {
        implication(
            context,
            induction_slash_prems(
                context,
                vars,
                claim,
                expr_recs(context, f, if_dot_q(context, e)),
            ),
            if_c_when_necessary(
                context,
                if_dot_q(context, e),
                induction_slash_if(context, vars, claim, f, if_dot_a(context, e)),
                induction_slash_if(context, vars, claim, f, if_dot_e(context, e)),
            ),
        )
    } else {
        implication(
            context,
            induction_slash_prems(context, vars, claim, expr_recs(context, f, e)),
            claim,
        )
    }
}

pub fn induction_slash_defun<'a>(
    context: &Context<'a>,
    vars: S<'a>,
    claim: S<'a>,
    def: S<'a>,
) -> S<'a> {
    induction_slash_if(
        context,
        vars,
        claim,
        defun_dot_name(context, def),
        sub_e(
            context,
            defun_dot_formals(context, def),
            vars,
            defun_dot_body(context, def),
        ),
    )
}

pub fn induction_slash_claim<'a>(
    context: &Context<'a>,
    defs: S<'a>,
    seed: S<'a>,
    def: S<'a>,
) -> S<'a> {
    if equal(context, seed, C_NIL) != C_NIL {
        dethm_dot_body(context, def)
    } else {
        induction_slash_defun(
            context,
            app_dot_args(context, seed),
            dethm_dot_body(context, def),
            lookup(context, app_dot_name(context, seed), defs),
        )
    }
}

pub fn find_focus_at_direction<'a>(context: &Context<'a>, dir: S<'a>, e: S<'a>) -> S<'a> {
    if equal(context, dir, C_Q) != C_NIL {
        if_dot_q(context, e)
    } else {
        if equal(context, dir, C_A) != C_NIL {
            if_dot_a(context, e)
        } else {
            if equal(context, dir, C_E) != C_NIL {
                if_dot_e(context, e)
            } else {
                get_arg(context, dir, app_dot_args(context, e))
            }
        }
    }
}

pub fn rewrite_focus_at_direction<'a>(
    context: &Context<'a>,
    dir: S<'a>,
    e1: S<'a>,
    e2: S<'a>,
) -> S<'a> {
    if equal(context, dir, C_Q) != C_NIL {
        if_c(context, e2, if_dot_a(context, e1), if_dot_e(context, e1))
    } else {
        if equal(context, dir, C_A) != C_NIL {
            if_c(context, if_dot_q(context, e1), e2, if_dot_e(context, e1))
        } else {
            if equal(context, dir, C_E) != C_NIL {
                if_c(context, if_dot_q(context, e1), if_dot_a(context, e1), e2)
            } else {
                app_c(
                    context,
                    app_dot_name(context, e1),
                    set_arg(context, dir, app_dot_args(context, e1), e2),
                )
            }
        }
    }
}

pub fn is_focus_is_at_direction<'a>(context: &Context<'a>, dir: S<'a>, e: S<'a>) -> S<'a> {
    if equal(context, dir, C_Q) != C_NIL {
        is_if(context, e)
    } else {
        if equal(context, dir, C_A) != C_NIL {
            is_if(context, e)
        } else {
            if equal(context, dir, C_E) != C_NIL {
                is_if(context, e)
            } else {
                if is_app(context, e) != C_NIL {
                    _lt__eq_len(context, dir, app_dot_args(context, e))
                } else {
                    C_NIL
                }
            }
        }
    }
}

pub fn is_focus_is_at_path<'a>(context: &Context<'a>, path: S<'a>, e: S<'a>) -> S<'a> {
    if atom(context, path) != C_NIL {
        C_T
    } else {
        if is_focus_is_at_direction(context, car(context, path), e) != C_NIL {
            is_focus_is_at_path(
                context,
                cdr(context, path),
                find_focus_at_direction(context, car(context, path), e),
            )
        } else {
            C_NIL
        }
    }
}

pub fn find_focus_at_path<'a>(context: &Context<'a>, path: S<'a>, e: S<'a>) -> S<'a> {
    if atom(context, path) != C_NIL {
        e
    } else {
        find_focus_at_path(
            context,
            cdr(context, path),
            find_focus_at_direction(context, car(context, path), e),
        )
    }
}

pub fn rewrite_focus_at_path<'a>(
    context: &Context<'a>,
    path: S<'a>,
    e1: S<'a>,
    e2: S<'a>,
) -> S<'a> {
    if atom(context, path) != C_NIL {
        e2
    } else {
        rewrite_focus_at_direction(
            context,
            car(context, path),
            e1,
            rewrite_focus_at_path(
                context,
                cdr(context, path),
                find_focus_at_direction(context, car(context, path), e1),
                e2,
            ),
        )
    }
}

pub fn is_prem_a<'a>(context: &Context<'a>, prem: S<'a>, path: S<'a>, e: S<'a>) -> S<'a> {
    if atom(context, path) != C_NIL {
        C_NIL
    } else {
        if equal(context, car(context, path), C_A) != C_NIL {
            if equal(context, if_dot_q(context, e), prem) != C_NIL {
                C_T
            } else {
                is_prem_a(
                    context,
                    prem,
                    cdr(context, path),
                    find_focus_at_direction(context, car(context, path), e),
                )
            }
        } else {
            is_prem_a(
                context,
                prem,
                cdr(context, path),
                find_focus_at_direction(context, car(context, path), e),
            )
        }
    }
}

pub fn is_prem_e<'a>(context: &Context<'a>, prem: S<'a>, path: S<'a>, e: S<'a>) -> S<'a> {
    if atom(context, path) != C_NIL {
        C_NIL
    } else {
        if equal(context, car(context, path), C_E) != C_NIL {
            if equal(context, if_dot_q(context, e), prem) != C_NIL {
                C_T
            } else {
                is_prem_e(
                    context,
                    prem,
                    cdr(context, path),
                    find_focus_at_direction(context, car(context, path), e),
                )
            }
        } else {
            is_prem_e(
                context,
                prem,
                cdr(context, path),
                find_focus_at_direction(context, car(context, path), e),
            )
        }
    }
}

pub fn follow_prems<'a>(context: &Context<'a>, path: S<'a>, e: S<'a>, thm: S<'a>) -> S<'a> {
    if is_if(context, thm) != C_NIL {
        if is_prem_a(context, if_dot_q(context, thm), path, e) != C_NIL {
            follow_prems(context, path, e, if_dot_a(context, thm))
        } else {
            if is_prem_e(context, if_dot_q(context, thm), path, e) != C_NIL {
                follow_prems(context, path, e, if_dot_e(context, thm))
            } else {
                thm
            }
        }
    } else {
        thm
    }
}

pub fn unary_op<'a>(context: &Context<'a>, rator: S<'a>, rand: S<'a>) -> S<'a> {
    if equal(context, rator, C_ATOM) != C_NIL {
        atom(context, rand)
    } else {
        if equal(context, rator, C_CAR) != C_NIL {
            car(context, rand)
        } else {
            if equal(context, rator, C_CDR) != C_NIL {
                cdr(context, rand)
            } else {
                if equal(context, rator, C_NATP) != C_NIL {
                    natp(context, rand)
                } else {
                    if equal(context, rator, C_SIZE) != C_NIL {
                        size(context, rand)
                    } else {
                        C_NIL
                    }
                }
            }
        }
    }
}

pub fn binary_op<'a>(context: &Context<'a>, rator: S<'a>, rand1: S<'a>, rand2: S<'a>) -> S<'a> {
    if equal(context, rator, C_EQUAL) != C_NIL {
        equal(context, rand1, rand2)
    } else {
        if equal(context, rator, C_CONS) != C_NIL {
            cons(context, rand1, rand2)
        } else {
            if equal(context, rator, C__PLUS) != C_NIL {
                _plus(context, rand1, rand2)
            } else {
                if equal(context, rator, C__LT) != C_NIL {
                    _lt(context, rand1, rand2)
                } else {
                    C_NIL
                }
            }
        }
    }
}

pub fn apply_op<'a>(context: &Context<'a>, rator: S<'a>, rands: S<'a>) -> S<'a> {
    if is_member(context, rator, C_PAIR_13) != C_NIL {
        unary_op(context, rator, elem1(context, rands))
    } else {
        if is_member(context, rator, C_PAIR_16) != C_NIL {
            binary_op(context, rator, elem1(context, rands), elem2(context, rands))
        } else {
            C_NIL
        }
    }
}

pub fn rands<'a>(context: &Context<'a>, args: S<'a>) -> S<'a> {
    if atom(context, args) != C_NIL {
        S::Empty
    } else {
        cons(
            context,
            quote_dot_value(context, car(context, args)),
            rands(context, cdr(context, args)),
        )
    }
}

pub fn eval_op<'a>(context: &Context<'a>, app: S<'a>) -> S<'a> {
    quote_c(
        context,
        apply_op(
            context,
            app_dot_name(context, app),
            rands(context, app_dot_args(context, app)),
        ),
    )
}

pub fn is_app_of_equal<'a>(context: &Context<'a>, e: S<'a>) -> S<'a> {
    if is_app(context, e) != C_NIL {
        equal(context, app_dot_name(context, e), C_EQUAL)
    } else {
        C_NIL
    }
}

pub fn equality<'a>(context: &Context<'a>, focus: S<'a>, a: S<'a>, b: S<'a>) -> S<'a> {
    if equal(context, focus, a) != C_NIL {
        b
    } else {
        if equal(context, focus, b) != C_NIL {
            a
        } else {
            focus
        }
    }
}

pub fn equality_slash_equation<'a>(
    context: &Context<'a>,
    focus: S<'a>,
    concl_inst: S<'a>,
) -> S<'a> {
    if is_app_of_equal(context, concl_inst) != C_NIL {
        equality(
            context,
            focus,
            elem1(context, app_dot_args(context, concl_inst)),
            elem2(context, app_dot_args(context, concl_inst)),
        )
    } else {
        focus
    }
}

pub fn equality_slash_path<'a>(context: &Context<'a>, e: S<'a>, path: S<'a>, thm: S<'a>) -> S<'a> {
    if is_focus_is_at_path(context, path, e) != C_NIL {
        rewrite_focus_at_path(
            context,
            path,
            e,
            equality_slash_equation(
                context,
                find_focus_at_path(context, path, e),
                follow_prems(context, path, e, thm),
            ),
        )
    } else {
        e
    }
}

pub fn equality_slash_def<'a>(
    context: &Context<'a>,
    claim: S<'a>,
    path: S<'a>,
    app: S<'a>,
    def: S<'a>,
) -> S<'a> {
    if is_rator(context, def) != C_NIL {
        equality_slash_path(
            context,
            claim,
            path,
            app_c(context, C_EQUAL, list2(context, app, eval_op(context, app))),
        )
    } else {
        if is_defun(context, def) != C_NIL {
            equality_slash_path(
                context,
                claim,
                path,
                sub_e(
                    context,
                    defun_dot_formals(context, def),
                    app_dot_args(context, app),
                    app_c(
                        context,
                        C_EQUAL,
                        list2(
                            context,
                            app_c(
                                context,
                                defun_dot_name(context, def),
                                defun_dot_formals(context, def),
                            ),
                            defun_dot_body(context, def),
                        ),
                    ),
                ),
            )
        } else {
            if is_dethm(context, def) != C_NIL {
                equality_slash_path(
                    context,
                    claim,
                    path,
                    sub_e(
                        context,
                        dethm_dot_formals(context, def),
                        app_dot_args(context, app),
                        dethm_dot_body(context, def),
                    ),
                )
            } else {
                claim
            }
        }
    }
}

pub fn rewrite_slash_step<'a>(
    context: &Context<'a>,
    defs: S<'a>,
    claim: S<'a>,
    step: S<'a>,
) -> S<'a> {
    equality_slash_def(
        context,
        claim,
        elem1(context, step),
        elem2(context, step),
        lookup(context, app_dot_name(context, elem2(context, step)), defs),
    )
}

pub fn rewrite_slash_continue<'a>(
    context: &Context<'a>,
    defs: S<'a>,
    steps: S<'a>,
    old: S<'a>,
    new: S<'a>,
) -> S<'a> {
    if equal(context, new, old) != C_NIL {
        new
    } else {
        if atom(context, steps) != C_NIL {
            new
        } else {
            rewrite_slash_continue(
                context,
                defs,
                cdr(context, steps),
                new,
                rewrite_slash_step(context, defs, new, car(context, steps)),
            )
        }
    }
}

pub fn rewrite_slash_steps<'a>(
    context: &Context<'a>,
    defs: S<'a>,
    claim: S<'a>,
    steps: S<'a>,
) -> S<'a> {
    if atom(context, steps) != C_NIL {
        claim
    } else {
        rewrite_slash_continue(
            context,
            defs,
            cdr(context, steps),
            claim,
            rewrite_slash_step(context, defs, claim, car(context, steps)),
        )
    }
}

pub fn rewrite_slash_prove<'a>(
    context: &Context<'a>,
    defs: S<'a>,
    def: S<'a>,
    seed: S<'a>,
    steps: S<'a>,
) -> S<'a> {
    if is_defun(context, def) != C_NIL {
        rewrite_slash_steps(
            context,
            defs,
            totality_slash_claim(context, seed, def),
            steps,
        )
    } else {
        if is_dethm(context, def) != C_NIL {
            rewrite_slash_steps(
                context,
                defs,
                induction_slash_claim(context, defs, seed, def),
                steps,
            )
        } else {
            quote_c(context, C_NIL)
        }
    }
}

pub fn rewrite_slash_prove_plus_1<'a>(
    context: &Context<'a>,
    defs: S<'a>,
    pf: S<'a>,
    e: S<'a>,
) -> S<'a> {
    if equal(context, e, quote_c(context, C_T)) != C_NIL {
        rewrite_slash_prove(
            context,
            defs,
            elem1(context, pf),
            elem2(context, pf),
            cdr(context, cdr(context, pf)),
        )
    } else {
        e
    }
}

pub fn rewrite_slash_prove_plus<'a>(context: &Context<'a>, defs: S<'a>, pfs: S<'a>) -> S<'a> {
    if atom(context, pfs) != C_NIL {
        quote_c(context, C_T)
    } else {
        rewrite_slash_prove_plus_1(
            context,
            defs,
            car(context, pfs),
            rewrite_slash_prove_plus(
                context,
                list_extend(context, defs, elem1(context, car(context, pfs))),
                cdr(context, pfs),
            ),
        )
    }
}

pub fn rewrite_slash_define<'a>(
    context: &Context<'a>,
    defs: S<'a>,
    def: S<'a>,
    seed: S<'a>,
    steps: S<'a>,
) -> S<'a> {
    if equal(
        context,
        rewrite_slash_prove(context, defs, def, seed, steps),
        quote_c(context, C_T),
    ) != C_NIL
    {
        list_extend(context, defs, def)
    } else {
        defs
    }
}

pub fn rewrite_slash_define_plus_1<'a>(
    context: &Context<'a>,
    defs1: S<'a>,
    defs2: S<'a>,
    pfs: S<'a>,
) -> S<'a> {
    if equal(context, defs1, defs2) != C_NIL {
        defs1
    } else {
        if atom(context, pfs) != C_NIL {
            defs2
        } else {
            rewrite_slash_define_plus_1(
                context,
                defs2,
                rewrite_slash_define(
                    context,
                    defs2,
                    elem1(context, car(context, pfs)),
                    elem2(context, car(context, pfs)),
                    cdr(context, cdr(context, car(context, pfs))),
                ),
                cdr(context, pfs),
            )
        }
    }
}

pub fn rewrite_slash_define_plus<'a>(context: &Context<'a>, defs: S<'a>, pfs: S<'a>) -> S<'a> {
    if atom(context, pfs) != C_NIL {
        defs
    } else {
        rewrite_slash_define_plus_1(
            context,
            defs,
            rewrite_slash_define(
                context,
                defs,
                elem1(context, car(context, pfs)),
                elem2(context, car(context, pfs)),
                cdr(context, cdr(context, car(context, pfs))),
            ),
            cdr(context, pfs),
        )
    }
}

pub fn j_bob_slash_step<'a>(context: &Context<'a>, defs: S<'a>, e: S<'a>, steps: S<'a>) -> S<'a> {
    if is_defs(context, S::Empty, defs) != C_NIL {
        if is_expr(context, defs, C_ANY, e) != C_NIL {
            if is_steps(context, defs, steps) != C_NIL {
                rewrite_slash_steps(context, defs, e, steps)
            } else {
                e
            }
        } else {
            e
        }
    } else {
        e
    }
}

pub fn j_bob_slash_prove<'a>(context: &Context<'a>, defs: S<'a>, pfs: S<'a>) -> S<'a> {
    if is_defs(context, S::Empty, defs) != C_NIL {
        if is_proofs(context, defs, pfs) != C_NIL {
            rewrite_slash_prove_plus(context, defs, pfs)
        } else {
            quote_c(context, C_NIL)
        }
    } else {
        quote_c(context, C_NIL)
    }
}

pub fn j_bob_slash_define<'a>(context: &Context<'a>, defs: S<'a>, pfs: S<'a>) -> S<'a> {
    if is_defs(context, S::Empty, defs) != C_NIL {
        if is_proofs(context, defs, pfs) != C_NIL {
            rewrite_slash_define_plus(context, defs, pfs)
        } else {
            defs
        }
    } else {
        defs
    }
}

pub fn axioms<'a>(context: &Context<'a>) -> S<'a> {
    C_PAIR_284
}

pub fn prelude<'a>(context: &Context<'a>) -> S<'a> {
    j_bob_slash_define(context, axioms(context), C_PAIR_349)
}
