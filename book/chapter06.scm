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

 ((dethm memb?/remb (xs)
    (equal (memb? (remb xs)) 'nil))
  (list-induction xs)
  ((A 1 1) (remb xs))
  ((A 1 1)
   (if-nest-A
     (atom xs)
     '()
     (if (equal (car xs) '?)
         (remb (cdr xs))
         (cons (car xs) (remb (cdr xs))))))
  ((A 1) (memb? '()))
  ((A 1 Q) (atom '()))
  ((A 1)
   (if-true
     'nil
     (if (equal (car '()) '?)
         't
         (memb? (cdr '())))))
  ((A) (equal-same 'nil))
  ((E A 1 1) (remb xs))
  ((E A 1 1)
   (if-nest-E
     (atom xs)
     '()
     (if (equal (car xs) '?)
         (remb (cdr xs))
         (cons (car xs) (remb (cdr xs))))))
  ((E A 1)
   (if-same
     (equal (car xs) '?)
     (memb?
       (if (equal (car xs) '?)
           (remb (cdr xs))
           (cons
             (car xs)
             (remb (cdr xs)))))))
  ((E A 1 A 1)
   (if-nest-A
     (equal (car xs) '?)
     (remb (cdr xs))
     (cons (car xs) (remb (cdr xs)))))
  ((E A 1 E 1)
   (if-nest-E
     (equal (car xs) '?)
     (remb (cdr xs))
     (cons (car xs) (remb (cdr xs)))))
  ((E A 1 A)
   (equal-if
     (memb? (remb (cdr xs)))
     'nil))
  ((E A 1 E)
   (memb?
     (cons (car xs) (remb (cdr xs)))))
  ((E A 1 E Q)
   (atom/cons (car xs) (remb (cdr xs))))
  ((E A 1 E)
   (if-false
     'nil
     (if
         (equal
           (car
             (cons
               (car xs)
               (remb (cdr xs))))
           '?)
         't
         (memb?
           (cdr
             (cons
               (car xs)
               (remb (cdr xs))))))))
  ((E A 1 E Q 1)
   (car/cons (car xs) (remb (cdr xs))))
  ((E A 1 E E 1)
   (cdr/cons (car xs) (remb (cdr xs))))
  ((E A 1 E)
   (if-nest-E
     (equal (car xs) '?)
     't
     (memb? (remb (cdr xs)))))
  ((E A 1 E)
   (equal-if
     (memb? (remb (cdr xs)))
     'nil))
  ((E A 1)
   (if-same (equal (car xs) '?) 'nil))
  ((E A) (equal-same 'nil))
  ((E)
   (if-same
     (equal
       (memb? (remb (cdr xs)))
       'nil)
     't))
  (() (if-same (atom xs) 't))))
