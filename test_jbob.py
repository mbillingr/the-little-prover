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

    def test_frame_29_simplified(self):
        assert_same_value(
            "(J-Bob/prove (prelude) '())",
            "''t",
        )
