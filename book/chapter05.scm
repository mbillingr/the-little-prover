(((defun memb? (xs)
    (if (atom xs)
        'nil
        (if (equal (car xs) '?)
            't
            (memb? (cdr xs)))))
  (size xs)
  ((Q) (natp/size xs))
  (()
   (if-true
     (if (atom xs)
         't
         (if (equal (car xs) '?)
             't
             (< (size (cdr xs))
                (size xs))))
     'nil))
  ((E E) (size/cdr xs))
  ((E) (if-same (equal (car xs) '?) 't))
  (() (if-same (atom xs) 't)))

 ((defun remb (xs)
    (if (atom xs)
        '()
        (if (equal (car xs) '?)
            (remb (cdr xs))
            (cons
              (car xs)
              (remb (cdr xs))))))
  (size xs)
  ((Q) (natp/size xs))
  (()
   (if-true
     (if (atom xs)
         't
         (< (size (cdr xs)) (size xs)))
     'nil))
  ((E) (size/cdr xs))
  (() (if-same (atom xs) 't)))

 ((dethm memb?/remb0 ()
    (equal (memb? (remb '())) 'nil))
  nil
  ((1 1) (remb '()))
  ((1 1 Q) (atom '()))
  ((1 1)
   (if-true
     '()
     (if (equal (car '()) '?)
         (remb (cdr '()))
         (cons
           (car '())
           (remb (cdr '()))))))
  ((1) (memb? '()))
  ((1 Q) (atom '()))
  ((1)
   (if-true
     'nil
     (if (equal (car '()) '?)
         't
         (memb? (cdr '())))))
  (() (equal 'nil 'nil)))

 ((dethm memb?/remb1 (x1)
    (equal
      (memb? (remb (cons x1 '())))
      'nil))
  nil
  ((1 1) (remb (cons x1 '())))
  ((1 1 Q) (atom/cons x1 '()))
  ((1 1)
   (if-false
     '()
     (if (equal (car (cons x1 '())) '?)
         (remb (cdr (cons x1 '())))
         (cons
           (car (cons x1 '()))
           (remb (cdr (cons x1 '())))))))
  ((1 1 Q 1) (car/cons x1 '()))
  ((1 1 E 1) (car/cons x1 '()))
  ((1 1 A 1) (cdr/cons x1 '()))
  ((1 1 E 2 1) (cdr/cons x1 '()))
  ((1)
   (if-same
     (equal x1 '?)
     (memb?
       (if (equal x1 '?)
           (remb '())
           (cons x1 (remb '()))))))
  ((1 A 1)
   (if-nest-A
     (equal x1 '?)
     (remb '())
     (cons x1 (remb '()))))
  ((1 E 1)
   (if-nest-E
     (equal x1 '?)
     (remb '())
     (cons x1 (remb '()))))
  ((1 A) (memb?/remb0))
  ((1 E) (memb? (cons x1 (remb '()))))
  ((1 E Q) (atom/cons x1 (remb '())))
  ((1 E)
   (if-false
     'nil
     (if
         (equal
           (car (cons x1 (remb '())))
           '?)
         't
         (memb?
           (cdr (cons x1 (remb '())))))))
  ((1 E Q 1) (car/cons x1 (remb '())))
  ((1 E E 1) (cdr/cons x1 (remb '())))
  ((1 E)
   (if-nest-E
     (equal x1 '?)
     't
     (memb? (remb '()))))
  ((1 E) (memb?/remb0))
  ((1) (if-same (equal x1 '?) 'nil))
  (() (equal-same 'nil)))

 ((dethm memb?/remb2 (x1 x2)
    (equal
      (memb?
        (remb (cons x2 (cons x1 '()))))
      'nil))
  nil
  ((1 1) (remb (cons x2 (cons x1 '()))))
  ((1 1 Q) (atom/cons x2 (cons x1 '())))
  ((1 1)
   (if-false
     '()
     (if
         (equal
           (car (cons x2 (cons x1 '())))
           '?)
         (remb
           (cdr (cons x2 (cons x1 '()))))
         (cons
           (car (cons x2 (cons x1 '())))
           (remb
             (cdr
               (cons x2 (cons x1 '()))))))))
  ((1 1 Q 1)
   (car/cons x2 (cons x1 '())))
  ((1 1 E 1)
   (car/cons x2 (cons x1 '())))
  ((1 1 A 1)
   (cdr/cons x2 (cons x1 '())))
  ((1 1 E 2 1)
   (cdr/cons x2 (cons x1 '())))
  ((1)
   (if-same
     (equal x2 '?)
     (memb?
       (if (equal x2 '?)
           (remb (cons x1 '()))
           (cons
             x2
             (remb (cons x1 '())))))))
  ((1 A 1)
   (if-nest-A
     (equal x2 '?)
     (remb (cons x1 '()))
     (cons x2 (remb (cons x1 '())))))
  ((1 E 1)
   (if-nest-E
     (equal x2 '?)
     (remb (cons x1 '()))
     (cons x2 (remb (cons x1 '())))))
  ((1 A) (memb?/remb1 x1))
  ((1 E)
   (memb?
     (cons x2 (remb (cons x1 '())))))
  ((1 E Q)
   (atom/cons x2 (remb (cons x1 '()))))
  ((1 E)
   (if-false
     'nil
     (if
         (equal
           (car
             (cons
               x2
               (remb (cons x1 '()))))
           '?)
         't
         (memb?
           (cdr
             (cons
               x2
               (remb (cons x1 '()))))))))
  ((1 E Q 1)
   (car/cons x2 (remb (cons x1 '()))))
  ((1 E E 1)
   (cdr/cons x2 (remb (cons x1 '()))))
  ((1 E)
   (if-nest-E
     (equal x2 '?)
     't
     (memb? (remb (cons x1 '())))))
  ((1 E) (memb?/remb1 x1))
  ((1) (if-same (equal x2 '?) 'nil))
  (() (equal 'nil 'nil))))
