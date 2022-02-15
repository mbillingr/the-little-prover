import pytest
from jbob import evaluate, undefine
from jbob_parser import parse
from jbob_testing import evaluate_testsuite


def assert_same_value(expr1, expr2):
    assert evaluate(expr1) == evaluate(expr2)


@pytest.mark.parametrize("testsuite", ["test_recess.scm"])
def test_jbob(testsuite):
    with open(testsuite) as fd:
        ast = parse(fd.read())
    evaluate_testsuite(ast)


class TestRecess:
    def test_frame_03(self):
        assert_same_value(
            "(J-Bob/step (prelude) '(car (cons 'ham '(cheese))) '())",
            "'(car (cons 'ham '(cheese)))",
        )

    def test_frame_07(self):
        assert_same_value(
            """(J-Bob/step (prelude) 
                '(car (cons 'ham '(cheese))) 
                '((() (car/cons 'ham '(cheese)))))""",
            "''ham",
        )

    def test_frame_09(self):
        assert_same_value(
            """(J-Bob/step (prelude) 
                '(equal 'flapjack (atom (cons 'ham '(cheese)))) 
                '(((2) (atom/cons 'ham '(cheese)))
                  (() (equal 'flapjack 'nil))))""",
            "''nil",
        )

    def test_frame_10(self):
        assert_same_value(
            """(J-Bob/step (prelude) 
                '(atom (cdr (cons (car (cons p q)) '()))) 
                '(((1 1 1) (car/cons p q))
                  ((1) (cdr/cons p '()))
                  (() (atom '()))))""",
            "''t",
        )

    def test_frame_14(self):
        assert_same_value(
            """(J-Bob/step (prelude) 
                '(if a c c) 
                '())""",
            "'(if a c c)",
        )

    def test_frame_15(self):
        assert_same_value(
            """(J-Bob/step (prelude) 
                '(if a c c) 
                '((() (if-same a c))))""",
            "'c",
        )

    def test_frame_16_simplified(self):
        assert_same_value(
            """(J-Bob/step (prelude) 
                '(if a c c) 
                '((() (if-same a c))
                  (() (if-same (if 'x 'y 'z) c))
                ))""",
            "'(if (if 'x 'y 'z) c c)",
        )

    def test_frame_17_simplified(self):
        assert_same_value(
            """(J-Bob/step (prelude) 
                '(if a c c) 
                '((() (if-same a c))
                  (() (if-same (if 'x 'y (cons 'z '(z))) c))
                  ((Q E) (cons 'z '(z)))
                ))""",
            "'(if (if 'x 'y '(z z)) c c)",
        )

    def test_frame_29(self):
        assert_same_value(
            "(J-Bob/prove (prelude) '())",
            "''t",
        )

    def test_frame_30(self):
        assert_same_value(
            """(J-Bob/prove (prelude) 
                '(((defun pair (x y) (cons x (cons y '())))
                   nil)))""",
            "''t",
        )

    def test_frame_33(self):
        assert_same_value(
            """(J-Bob/prove (prelude) 
                '(((defun pair (x y) (cons x (cons y '())))
                   nil)
                  ((defun first-of (x) (car x))
                   nil)
                  ((defun second-of (x) (car (cdr x)))
                   nil)))""",
            "''t",
        )

    def test_frame_34(self):
        assert_same_value(
            """(J-Bob/prove (prelude) 
                '(((defun pair (x y) (cons x (cons y '())))
                   nil)
                  ((defun first-of (x) (car x))
                   nil)
                  ((defun second-of (x) (car (cdr x)))
                   nil)
                  ((dethm first-of-pair (a b) 
                     (equal (first-of (pair a b)) a))
                   nil)))""",
            "'(equal (first-of (pair a b)) a)",
        )

    def test_frame_35(self):
        assert_same_value(
            """(J-Bob/prove (prelude) 
                '(((defun pair (x y) (cons x (cons y '())))
                   nil)
                  ((defun first-of (x) (car x))
                   nil)
                  ((defun second-of (x) (car (cdr x)))
                   nil)
                  ((dethm first-of-pair (a b) 
                     (equal (first-of (pair a b)) a))
                   nil
                   ((1 1) (pair a b)))))""",
            "'(equal (first-of (cons a (cons b '()))) a)",
        )

    def test_frame_38(self):
        assert_same_value(
            """(J-Bob/prove (prelude) 
                '(((defun pair (x y) (cons x (cons y '())))
                   nil)
                  ((defun first-of (x) (car x))
                   nil)
                  ((defun second-of (x) (car (cdr x)))
                   nil)
                  ((dethm first-of-pair (a b) 
                     (equal (first-of (pair a b)) a))
                   nil
                   ((1 1) (pair a b))
                   ((1) (first-of (cons a (cons b '()))))
                   ((1) (car/cons a (cons b '())))
                   (() (equal-same a))
                   )))""",
            "''t",
        )

    @pytest.fixture()
    def frame42(self):
        evaluate(
            """(defun prelude+first-of-pair ()
                 (J-Bob/define (prelude) 
                   '(((defun pair (x y) (cons x (cons y '())))
                      nil)
                     ((defun first-of (x) (car x))
                      nil)
                     ((defun second-of (x) (car (cdr x)))
                      nil)
                     ((dethm first-of-pair (a b) 
                        (equal (first-of (pair a b)) a))
                      nil
                     ((1 1) (pair a b))
                     ((1) (first-of (cons a (cons b '()))))
                     ((1) (car/cons a (cons b '())))
                     (() (equal-same a))))))"""
        )
        yield
        undefine("prelude+first-of-pair")

    def test_frame_45(self, frame42):
        assert_same_value(
            """(J-Bob/prove (prelude+first-of-pair)
                  '(((dethm second-of-pair (a b)
                       (equal (second-of (pair a b)) b))
                     nil)))""",
            "'(equal (second-of (pair a b)) b)",
        )

    def test_frame_46(self, frame42):
        assert_same_value(
            """(J-Bob/prove (prelude+first-of-pair)
                  '(((dethm second-of-pair (a b)
                       (equal (second-of (pair a b)) b))
                     nil)
                    ((defun in-pair? (xs)
                       (if (equal (first-of xs) '?)
                           't
                           (equal (second-of xs) '?)))
                     nil)
                    ((dethm in-first-of-pair (b)
                       (equal (in-pair? (pair '? b)) 't))
                     nil)
                    ((dethm in-second-of-pair (a)
                       (equal (in-pair? (pair a '?)) 't))
                     nil)
                ))""",
            "'(equal (in-pair? (pair a '?)) 't)",
        )

    def test_frame_50(self):
        assert_same_value(
            """(J-Bob/prove (prelude)
                  '(((defun list? (x)
                       (if (atom x)
                           (equal x '())
                           (list? (cdr x))))
                     nil)))""",
            "''nil",
        )

    def test_frame_52(self):
        assert_same_value(
            """(J-Bob/prove (prelude)
                  '(((defun list? (x)
                       (if (atom x)
                           (equal x '())
                           (list? (cdr x))))
                     (size x))))""",
            "'(if (natp (size x)) (if (atom x) 't (< (size (cdr x)) (size x))) 'nil)",
        )

    def test_frame_53(self):
        assert_same_value(
            """(J-Bob/prove (prelude)
                  '(((defun list? (x)
                       (if (atom x)
                           (equal x '())
                           (list? (cdr x))))
                     (size x)
                     ((Q) (natp/size x))
                     (() (if-true (if (atom x) 't (< (size (cdr x)) (size x))) 'nil))
                     ((E) (size/cdr x))
                     (() (if-same (atom x) 't))
                )))""",
            "''t",
        )

    @pytest.fixture()
    def frame55(self):
        evaluate(
            """(defun prelude+memb?+remb ()
                 (J-Bob/define (prelude) 
                   '(((defun memb? (xs)
                        (if (atom xs)
                            'nil
                            (if (equal (car xs) '?)
                                't
                                (memb? (cdr xs)))))
                      (size xs)
                      ((Q) (natp/size xs))
                      (() (if-true (if (atom xs) 't (if (equal (car xs) '?) 't (< (size (cdr xs)) (size xs)))) 'nil))
                      ((E E) (size/cdr xs))
                      ((E) (if-same (equal (car xs) '?) 't))
                      (() (if-same (atom xs) 't)))
                     ((defun remb (xs)
                        (if (atom xs)
                            '()
                            (if (equal (car xs) '?)
                                (remb (cdr xs))
                                (cons (car xs) (remb (cdr xs))))))
                      (size xs)
                      ((Q) (natp/size xs))
                      (() (if-true (if (atom xs) 't (< (size (cdr xs)) (size xs))) 'nil))
                      ((E) (size/cdr xs))
                      (() (if-same (atom xs) 't)))
                )))"""
        )
        yield
        undefine("prelude+memb?+remb")

    def test_frame_61(self, frame55):
        assert_same_value(
            """(J-Bob/prove (prelude+memb?+remb)
                  '(((dethm memb?/remb (xs)
                        (equal (memb? (remb xs)) 'nil))
                     (list-induction xs))
                ))""",
            """'(if (atom xs)
                    (equal (memb? (remb xs)) 'nil)
                    (if (equal (memb? (remb (cdr xs))) 'nil)
                        (equal (memb? (remb xs)) 'nil)
                        't))""",
        )


class TestChapter02:
    def test_frame_11pp(self):
        assert_same_value(
            """(J-Bob/step (prelude) 
                '(if (if (equal a 't) (if (equal 'nil 'nil) a b) (equal 'or (cons 'black '(coffee)))) c c)
                '(((Q E 2) (cons 'black '(coffee)))
                  ((Q A Q) (equal 'nil 'nil))
                  ((Q A) (if-true a b))
                  ((Q A) (equal-if a 't))))""",
            "'(if (if (equal a 't) 't (equal 'or '(black coffee))) c c)",
        )

    def test_frame_60pp(self):
        assert_same_value(
            """(J-Bob/step (prelude) 
                '(if (atom (car a)) 
                     (if (equal (car a) (cdr a)) 'hominy 'grits) 
                     (if (equal (cdr (car a)) '(hash browns)) 
                         (cons 'ketchup (car a)) 
                         (cons 'mustard (car a))))
                '(((E A 2) (cons/car+cdr (car a)))
                  ((E A 2 2) (equal-if (cdr (car a)) '(hash browns)))
                ))""",
            """'(if (atom (car a)) 
                    (if (equal (car a) (cdr a)) 'hominy 'grits) 
                    (if (equal (cdr (car a)) '(hash browns)) 
                        (cons 'ketchup (cons (car (car a)) '(hash browns))) 
                        (cons 'mustard (car a))))""",
        )


class TestChapter03:
    def test_frame_05(self, chapter03):
        assert_functions_defined("(chapter03)", "pair", "first-of", "second-of")

    def test_frame_06(self, chapter03):
        assert_theorems_proved("(chapter03)", "first-of-pair")

    def test_frame_17(self, chapter03):
        assert_theorems_proved("(chapter03)", "second-of-pair")

    def test_frame_27(self, chapter03):
        assert_functions_defined("(chapter03)", "in-pair?")

    def test_frame_28(self, chapter03):
        assert_theorems_proved("(chapter03)", "in-first-of-pair")

    def proof_wip_template(self):
        assert_same_value(
            """(J-Bob/prove (chapter03)
                  '(((dethm name (args)
                        (body))
                     nil)
                ))""",
            "''t",
        )

    @pytest.fixture()
    def chapter03(self):
        evaluate(
            """
(defun chapter03 ()
    (J-Bob/define (prelude)
        '(((defun pair (x y) 
             (cons x (cons y '())))
           nil)
           
          ((defun first-of (x) 
             (car x))
           nil)
           
          ((defun second-of (x) 
             (car (cdr x)))
           nil)
           
          ((dethm first-of-pair (a b) 
             (equal (first-of (pair a b)) a))
           nil
           ((1 1) (pair a b))
           ((1) (first-of (cons a (cons b '()))))
           ((1) (car/cons a (cons b '())))
           (() (equal-same a)))
           
          ((dethm second-of-pair (a b)
             (equal (second-of (pair a b)) b))
           nil
           ((1 1) (pair a b))
           ((1) (second-of (cons a (cons b '()))))
           ((1 1) (cdr/cons a (cons b '())))
           ((1) (car/cons b '()))
           (() (equal-same b))
           )
           
          ((defun in-pair? (xs)
             (if (equal (first-of xs) '?) 
                 't 
                 (equal (second-of xs) '?)))
           nil)
           
          ((dethm in-first-of-pair (b)
             (equal (in-pair? (pair '? b)) 't))
           nil
           ((1 1) (pair '? b))
           ((1) (in-pair? (cons '? (cons b '()))))
           ((1 Q 1) (first-of (cons '? (cons b '()))))
           ((1 Q 1) (car/cons '? (cons b '())))
           ((1 Q) (equal-same '?))
           ((1) (if-true 't (equal (second-of (cons '? (cons b '()))) '?)))
           (() (equal-same 't)))
)))"""
        )
        yield
        undefine("chapter03")

    @pytest.fixture()
    def first_of_pair(self):
        evaluate(
            """(defun pairs ()
                (J-Bob/define (pairs)
                  '(((dethm first-of-pair (a b) 
                       (equal (first-of (pair a b)) a))
                     nil
                     ((1 1) (pair a b))
                     ((1) (first-of (cons a (cons b '()))))
                     ((1) (car/cons a (cons b '())))
                     (() (equal-same a))))))"""
        )
        yield
        undefine("pairs")


def assert_functions_defined(expr, *fnames):
    assert_defined("defun", expr, fnames)


def assert_theorems_proved(expr, *fnames):
    assert_defined("dethm", expr, fnames)


def assert_defined(definition_kind, expr, fnames):
    defs = map(list, evaluate(expr))
    defuns = filter(lambda x: x[0] == definition_kind, defs)
    defun_names = map(lambda x: x[1], defuns)
    undefined_names = set(fnames).difference(defun_names)
    assert not undefined_names, f"Undefined: {undefined_names}"
