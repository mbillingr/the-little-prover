from jbob import evaluate


def assert_same_value(expr1, expr2):
    assert evaluate(expr1) == evaluate(expr2)


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
