
(testcase 'frame-03
  (assert-equal
    (J-Bob/step (prelude) '(car (cons 'ham '(cheese))) '())
    '(car (cons 'ham '(cheese)))))

(testcase 'frame-07
  (assert-equal
    (J-Bob/step (prelude)
      '(car (cons 'ham '(cheese)))
      '((() (car/cons 'ham '(cheese)))))
    ''ham))

(testcase 'frame-09
  (assert-equal
    (J-Bob/step (prelude)
      '(equal 'flapjack (atom (cons 'ham '(cheese))))
      '(((2) (atom/cons 'ham '(cheese)))
        (() (equal 'flapjack 'nil))))
    ''nil))

(testcase 'frame-10
  (assert-equal
    (J-Bob/step (prelude)
      '(atom (cdr (cons (car (cons p q)) '())))
      '(((1 1 1) (car/cons p q))
        ((1) (cdr/cons p '()))
        (() (atom '()))))
    ''t))

(testcase 'frame-14
  (assert-equal
    (J-Bob/step (prelude)
      '(if a c c)
      '())
    '(if a c c)))

(testcase 'frame-15
  (assert-equal
    (J-Bob/step (prelude)
      '(if a c c)
      '((() (if-same a c))))
    'c))

(testcase 'frame-16-simplified
  (assert-equal
    (J-Bob/step (prelude)
      '(if a c c)
      '((() (if-same a c))
        (() (if-same (if 'x 'y 'z) c))))
    '(if (if 'x 'y 'z) c c)))

(testcase 'frame-17-simplified
  (assert-equal
    (J-Bob/step (prelude)
      '(if a c c)
      '((() (if-same a c))
        (() (if-same (if 'x 'y (cons 'z '(z))) c))
        ((Q E) (cons 'z '(z)))))
    '(if (if 'x 'y '(z z)) c c)))

(testcase 'frame-29
  (assert-equal
    (J-Bob/prove (prelude) '())
    ''t))

(testcase 'frame-30
  (assert-equal
    (J-Bob/prove (prelude)
      '(((defun pair (x y) (cons x (cons y '())))
         nil)))
    ''t))

(testcase 'frame-33
  (assert-equal
    (J-Bob/prove (prelude)
      '(((defun pair (x y) (cons x (cons y '())))
         nil)
        ((defun first-of (x) (car x))
         nil)
        ((defun second-of (x) (car (cdr x)))
         nil)))
    ''t))

(testcase 'frame-34
  (assert-equal
    (J-Bob/prove (prelude)
      '(((defun pair (x y) (cons x (cons y '())))
         nil)
        ((defun first-of (x) (car x))
         nil)
        ((defun second-of (x) (car (cdr x)))
         nil)
        ((dethm first-of-pair (a b)
           (equal (first-of (pair a b)) a))
         nil)))
    '(equal (first-of (pair a b)) a)))

(testcase 'frame-35
  (assert-equal
    (J-Bob/prove (prelude)
      '(((defun pair (x y) (cons x (cons y '())))
         nil)
        ((defun first-of (x) (car x))
         nil)
        ((defun second-of (x) (car (cdr x)))
         nil)
        ((dethm first-of-pair (a b)
           (equal (first-of (pair a b)) a))
         nil
         ((1 1) (pair a b)))))
    '(equal (first-of (cons a (cons b '()))) a)))

(testcase 'frame-38
  (assert-equal
    (J-Bob/prove (prelude)
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
         (() (equal-same a)))))
    ''t))

(define prelude+first-of-pair
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
       (() (equal-same a))))))

(testcase 'frame-45
  (assert-equal
    (J-Bob/prove prelude+first-of-pair
      '(((dethm second-of-pair (a b)
           (equal (second-of (pair a b)) b))
         nil)))
    '(equal (second-of (pair a b)) b)))

(testcase 'frame-46
  (assert-equal
    (J-Bob/prove prelude+first-of-pair
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
          nil)))
    '(equal (in-pair? (pair a '?)) 't)))

(testcase 'frame-50
  (assert-equal
    (J-Bob/prove (prelude)
      '(((defun list? (x)
           (if (atom x)
               (equal x '())
               (list? (cdr x))))
         nil)))
   ''nil))

(testcase 'frame-52
  (assert-equal
    (J-Bob/prove (prelude)
      '(((defun list? (x)
           (if (atom x)
               (equal x '())
               (list? (cdr x))))
         (size x))))
   '(if (natp (size x)) (if (atom x) 't (< (size (cdr x)) (size x))) 'nil)))

(testcase 'frame-53
  (assert-equal
    (J-Bob/prove (prelude)
      '(((defun list? (x)
           (if (atom x)
               (equal x '())
               (list? (cdr x))))
         (size x)
         ((Q) (natp/size x))
         (() (if-true (if (atom x) 't (< (size (cdr x)) (size x))) 'nil))
         ((E) (size/cdr x))
         (() (if-same (atom x) 't)))))

   ''t))

(define prelude+memb?+remb
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
       (() (if-same (atom xs) 't))))))

(testcase 'frame-61
  (assert-equal
    (J-Bob/prove prelude+memb?+remb
      '(((dethm memb?/remb (xs)
            (equal (memb? (remb xs)) 'nil))
         (list-induction xs))))
   '(if (atom xs)
        (equal (memb? (remb xs)) 'nil)
        (if (equal (memb? (remb (cdr xs))) 'nil)
            (equal (memb? (remb xs)) 'nil)
            't))))
