(((defun ctx? (x)
    (if (atom x)
        (equal x '?)
        (if (ctx? (car x))
            't
            (ctx? (cdr x)))))
  (size x)
  ((Q) (natp/size x))
  (()
   (if-true
     (if (atom x)
         't
         (if
             (< (size (car x))
                (size x))
             (if (ctx? (car x))
                 't
                 (< (size (cdr x))
                    (size x)))
             'nil))
     'nil))
  ((E Q) (size/car x))
  ((E A E) (size/cdr x))
  ((E A) (if-same (ctx? (car x)) 't))
  ((E) (if-true 't 'nil))
  (() (if-same (atom x) 't)))

 ((defun sub (x y)
    (if (atom y)
        (if (equal y '?) x y)
        (cons
          (sub x (car y))
          (sub x (cdr y)))))
  (size y)
  ((Q) (natp/size y))
  (()
   (if-true
     (if (atom y)
         't
         (if
             (< (size (car y))
                (size y))
             (< (size (cdr y))
                (size y))
             'nil))
     'nil))
  ((E Q) (size/car y))
  ((E A) (size/cdr y))
  ((E) (if-true 't 'nil))
  (() (if-same (atom y) 't)))

 ((dethm ctx?/t (x)
    (if (ctx? x) (equal (ctx? x) 't) 't))
  (star-induction x)
  ((A A 1) (ctx? x))
  ((A Q) (ctx? x))
  ((A Q)
   (if-nest-A
     (atom x)
     (equal x '?)
     (if (ctx? (car x))
         't
         (ctx? (cdr x)))))
  ((A A 1)
   (if-nest-A
     (atom x)
     (equal x '?)
     (if (ctx? (car x))
         't
         (ctx? (cdr x)))))
  ((A A 1 1) (equal-if x '?))
  ((A A 1) (equal-same '?))
  ((A A) (equal-same 't))
  ((A) (if-same (equal x '?) 't))
  ((E A A A 1) (ctx? x))
  ((E A A Q) (ctx? x))
  ((E A A A 1)
   (if-nest-E
     (atom x)
     (equal x '?)
     (if (ctx? (car x))
         't
         (ctx? (cdr x)))))
  ((E A A Q)
   (if-nest-E
     (atom x)
     (equal x '?)
     (if (ctx? (car x))
         't
         (ctx? (cdr x)))))
  ((E)
   (if-same
     (ctx? (car x))
     (if
         (if (ctx? (car x))
             (equal (ctx? (car x)) 't)
             't)
         (if
             (if (ctx? (cdr x))
                 (equal
                   (ctx? (cdr x))
                   't)
                 't)
             (if
                 (if (ctx? (car x))
                     't
                     (ctx? (cdr x)))
                 (equal
                   (if (ctx? (car x))
                       't
                       (ctx? (cdr x)))
                   't)
                 't)
             't)
         't)))
  ((E A Q)
   (if-nest-A
     (ctx? (car x))
     (equal (ctx? (car x)) 't)
     't))
  ((E E Q)
   (if-nest-E
     (ctx? (car x))
     (equal (ctx? (car x)) 't)
     't))
  ((E A A A Q)
   (if-nest-A
     (ctx? (car x))
     't
     (ctx? (cdr x))))
  ((E A A A A 1)
   (if-nest-A
     (ctx? (car x))
     't
     (ctx? (cdr x))))
  ((E E A A Q)
   (if-nest-E
     (ctx? (car x))
     't
     (ctx? (cdr x))))
  ((E E A A A 1)
   (if-nest-E
     (ctx? (car x))
     't
     (ctx? (cdr x))))
  ((E A A A) (if-true (equal 't 't) 't))
  ((E A A A) (equal-same 't))
  ((E A A)
   (if-same
     (if (ctx? (cdr x))
         (equal (ctx? (cdr x)) 't)
         't)
     't))
  ((E A)
   (if-same
     (equal (ctx? (car x)) 't)
     't))
  ((E E)
   (if-true
     (if
         (if (ctx? (cdr x))
             (equal (ctx? (cdr x)) 't)
             't)
         (if (ctx? (cdr x))
             (equal (ctx? (cdr x)) 't)
             't)
         't)
     't))
  ((E E)
   (if-same
     (ctx? (cdr x))
     (if
         (if (ctx? (cdr x))
             (equal (ctx? (cdr x)) 't)
             't)
         (if (ctx? (cdr x))
             (equal (ctx? (cdr x)) 't)
             't)
         't)))
  ((E E A Q)
   (if-nest-A
     (ctx? (cdr x))
     (equal (ctx? (cdr x)) 't)
     't))
  ((E E A A)
   (if-nest-A
     (ctx? (cdr x))
     (equal (ctx? (cdr x)) 't)
     't))
  ((E E E Q)
   (if-nest-E
     (ctx? (cdr x))
     (equal (ctx? (cdr x)) 't)
     't))
  ((E E E A)
   (if-nest-E
     (ctx? (cdr x))
     (equal (ctx? (cdr x)) 't)
     't))
  ((E E E) (if-same 't 't))
  ((E E A A 1)
   (equal-if (ctx? (cdr x)) 't))
  ((E E A A) (equal-same 't))
  ((E E A)
   (if-same
     (equal (ctx? (cdr x)) 't)
     't))
  ((E E) (if-same (ctx? (cdr x)) 't))
  ((E) (if-same (ctx? (car x)) 't))
  (() (if-same (atom x) 't)))

 ((dethm ctx?/sub (x y)
    (if (ctx? x)
        (if (ctx? y)
            (equal (ctx? (sub x y)) 't)
            't)
        't))
  (star-induction y)
  (()
   (if-same
     (ctx? x)
     (if (atom y)
         (if (ctx? x)
             (if (ctx? y)
                 (equal
                   (ctx? (sub x y))
                   't)
                 't)
             't)
         (if
             (if (ctx? x)
                 (if (ctx? (car y))
                     (equal
                       (ctx?
                         (sub
                           x
                           (car y)))
                       't)
                     't)
                 't)
             (if
                 (if (ctx? x)
                     (if (ctx? (cdr y))
                         (equal
                           (ctx?
                             (sub
                               x
                               (cdr y)))
                           't)
                         't)
                     't)
                 (if (ctx? x)
                     (if (ctx? y)
                         (equal
                           (ctx?
                             (sub x y))
                           't)
                         't)
                     't)
                 't)
             't))))
  ((A A)
   (if-nest-A
     (ctx? x)
     (if (ctx? y)
         (equal (ctx? (sub x y)) 't)
         't)
     't))
  ((A E Q)
   (if-nest-A
     (ctx? x)
     (if (ctx? (car y))
         (equal
           (ctx? (sub x (car y)))
           't)
         't)
     't))
  ((A E A Q)
   (if-nest-A
     (ctx? x)
     (if (ctx? (cdr y))
         (equal
           (ctx? (sub x (cdr y)))
           't)
         't)
     't))
  ((A E A A)
   (if-nest-A
     (ctx? x)
     (if (ctx? y)
         (equal (ctx? (sub x y)) 't)
         't)
     't))
  ((E A)
   (if-nest-E
     (ctx? x)
     (if (ctx? y)
         (equal (ctx? (sub x y)) 't)
         't)
     't))
  ((E E Q)
   (if-nest-E
     (ctx? x)
     (if (ctx? (car y))
         (equal
           (ctx? (sub x (car y)))
           't)
         't)
     't))
  ((E E A Q)
   (if-nest-E
     (ctx? x)
     (if (ctx? (cdr y))
         (equal
           (ctx? (sub x (cdr y)))
           't)
         't)
     't))
  ((E E A A)
   (if-nest-E
     (ctx? x)
     (if (ctx? y)
         (equal (ctx? (sub x y)) 't)
         't)
     't))
  ((E E A) (if-same 't 't))
  ((E E) (if-same 't 't))
  ((E) (if-same (atom y) 't))
  ((A A A 1 1) (sub x y))
  ((A A A 1 1)
   (if-nest-A
     (atom y)
     (if (equal y '?) x y)
     (cons
       (sub x (car y))
       (sub x (cdr y)))))
  ((A A A)
   (if-same
     (equal y '?)
     (equal
       (ctx? (if (equal y '?) x y))
       't)))
  ((A A A A 1 1)
   (if-nest-A (equal y '?) x y))
  ((A A A E 1 1)
   (if-nest-E (equal y '?) x y))
  ((A A A A 1) (ctx?/t x))
  ((A A A E 1) (ctx?/t y))
  ((A A A A) (equal 't 't))
  ((A A A E) (equal 't 't))
  ((A A A) (if-same (equal y '?) 't))
  ((A A) (if-same (ctx? y) 't))
  ((A E A A A 1 1) (sub x y))
  ((A E A A A 1 1)
   (if-nest-E
     (atom y)
     (if (equal y '?) x y)
     (cons
       (sub x (car y))
       (sub x (cdr y)))))
  ((A E A A A 1)
   (ctx?
     (cons
       (sub x (car y))
       (sub x (cdr y)))))
  ((A E A A A 1 Q)
   (atom/cons
     (sub x (car y))
     (sub x (cdr y))))
  ((A E A A A 1)
   (if-false
     (equal
       (cons
         (sub x (car y))
         (sub x (cdr y)))
       '?)
     (if
         (ctx?
           (car
             (cons
               (sub x (car y))
               (sub x (cdr y)))))
         't
         (ctx?
           (cdr
             (cons
               (sub x (car y))
               (sub x (cdr y))))))))
  ((A E A A A 1 Q 1)
   (car/cons
     (sub x (car y))
     (sub x (cdr y))))
  ((A E A A A 1 E 1)
   (cdr/cons
     (sub x (car y))
     (sub x (cdr y))))
  ((A E A A Q) (ctx? y))
  ((A E A A Q)
   (if-nest-E
     (atom y)
     (equal y '?)
     (if (ctx? (car y))
         't
         (ctx? (cdr y)))))
  ((A E)
   (if-same
     (ctx? (car y))
     (if
         (if (ctx? (car y))
             (equal
               (ctx? (sub x (car y)))
               't)
             't)
         (if
             (if (ctx? (cdr y))
                 (equal
                   (ctx?
                     (sub x (cdr y)))
                   't)
                 't)
             (if
                 (if (ctx? (car y))
                     't
                     (ctx? (cdr y)))
                 (equal
                   (if
                       (ctx?
                         (sub x (car y)))
                       't
                       (ctx?
                         (sub x (cdr y))))
                   't)
                 't)
             't)
         't)))
  ((A E A Q)
   (if-nest-A
     (ctx? (car y))
     (equal (ctx? (sub x (car y))) 't)
     't))
  ((A E A A A Q)
   (if-nest-A
     (ctx? (car y))
     't
     (ctx? (cdr y))))
  ((A E E Q)
   (if-nest-E
     (ctx? (car y))
     (equal (ctx? (sub x (car y))) 't)
     't))
  ((A E E A A Q)
   (if-nest-E
     (ctx? (car y))
     't
     (ctx? (cdr y))))
  ((A E E)
   (if-true
     (if
         (if (ctx? (cdr y))
             (equal
               (ctx? (sub x (cdr y)))
               't)
             't)
         (if (ctx? (cdr y))
             (equal
               (if
                   (ctx?
                     (sub x (car y)))
                   't
                   (ctx?
                     (sub x (cdr y))))
               't)
             't)
         't)
     't))
  ((A E A A A)
   (if-true
     (equal
       (if (ctx? (sub x (car y)))
           't
           (ctx? (sub x (cdr y))))
       't)
     't))
  ((A E A A A 1 Q)
   (equal-if (ctx? (sub x (car y))) 't))
  ((A E A A A 1)
   (if-true 't (ctx? (sub x (cdr y)))))
  ((A E A A A) (equal 't 't))
  ((A E A A)
   (if-same
     (if (ctx? (cdr y))
         (equal
           (ctx? (sub x (cdr y)))
           't)
         't)
     't))
  ((A E A)
   (if-same
     (equal (ctx? (sub x (car y))) 't)
     't))
  ((A E E)
   (if-same
     (ctx? (cdr y))
     (if
         (if (ctx? (cdr y))
             (equal
               (ctx? (sub x (cdr y)))
               't)
             't)
         (if (ctx? (cdr y))
             (equal
               (if
                   (ctx?
                     (sub x (car y)))
                   't
                   (ctx?
                     (sub x (cdr y))))
               't)
             't)
         't)))
  ((A E E A Q)
   (if-nest-A
     (ctx? (cdr y))
     (equal (ctx? (sub x (cdr y))) 't)
     't))
  ((A E E A A)
   (if-nest-A
     (ctx? (cdr y))
     (equal
       (if (ctx? (sub x (car y)))
           't
           (ctx? (sub x (cdr y))))
       't)
     't))
  ((A E E E Q)
   (if-nest-E
     (ctx? (cdr y))
     (equal (ctx? (sub x (cdr y))) 't)
     't))
  ((A E E E A)
   (if-nest-E
     (ctx? (cdr y))
     (equal
       (if (ctx? (sub x (car y)))
           't
           (ctx? (sub x (cdr y))))
       't)
     't))
  ((A E E E) (if-same 't 't))
  ((A E E A A 1 E)
   (equal-if (ctx? (sub x (cdr y))) 't))
  ((A E E A A 1)
   (if-same (ctx? (sub x (car y))) 't))
  ((A E E A A) (equal-same 't))
  ((A E E A)
   (if-same
     (equal (ctx? (sub x (cdr y))) 't)
     't))
  ((A E E) (if-same (ctx? (cdr y)) 't))
  ((A E) (if-same (ctx? (car y)) 't))
  ((A) (if-same (atom y) 't))
  (() (if-same (ctx? x) 't))))
