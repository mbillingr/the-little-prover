
(testcase 'recess-frame-03
  (assert-equal
    (J-Bob/step (prelude) '(car (cons 'ham '(cheese))) '())
    '(car (cons 'ham '(cheese)))))

(testcase 'recess-frame-07
  (assert-equal
    (J-Bob/step (prelude)
                '(car (cons 'ham '(cheese)))
                '((() (car/cons 'ham '(cheese)))))
    ''ham))
