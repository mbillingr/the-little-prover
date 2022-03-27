(((defun rotate (x)
    (cons
      (car (car x))
      (cons (cdr (car x)) (cdr x))))
  nil)

 ((dethm rotate/cons (x y z)
    (equal
      (rotate (cons (cons x y) z))
      (cons x (cons y z))))
  nil
  ((1) (rotate (cons (cons x y) z)))
  ((1 1 1) (car/cons (cons x y) z))
  ((1 1) (car/cons x y))
  ((1 2 1 1) (car/cons (cons x y) z))
  ((1 2 1) (cdr/cons x y))
  ((1 2 2) (cdr/cons (cons x y) z))
  (() (equal-same (cons x (cons y z)))))

 ((defun wt (x)
    (if (atom x)
        '1
        (+ (+ (wt (car x)) (wt (car x)))
           (wt (cdr x)))))
  (size x)
  ((Q) (natp/size x))
  (()
   (if-true
     (if (atom x)
         't
         (if
             (< (size (car x))
                (size x))
             (< (size (cdr x))
                (size x))
             'nil))
     'nil))
  ((E Q) (size/car x))
  ((E A) (size/cdr x))
  ((E) (if-true 't 'nil))
  (() (if-same (atom x) 't)))

 ((dethm natp/wt (x)
    (equal (natp (wt x)) 't))
  (star-induction x)
  ((A 1 1) (wt x))
  ((A 1 1)
   (if-nest-A
     (atom x)
     '1
     (+ (+ (wt (car x)) (wt (car x)))
        (wt (cdr x)))))
  ((A 1) (natp '1))
  ((A) (equal 't 't))
  ((E A A 1 1) (wt x))
  ((E A A 1 1)
   (if-nest-E
     (atom x)
     '1
     (+ (+ (wt (car x)) (wt (car x)))
        (wt (cdr x)))))
  ((E A A)
   (if-true
     (equal
       (natp
         (+
            (+ (wt (car x))
               (wt (car x)))
            (wt (cdr x))))
       't)
     't))
  ((E A A Q)
   (equal-if (natp (wt (car x))) 't))
  ((E A A A)
   (if-true
     (equal
       (natp
         (+
            (+ (wt (car x))
               (wt (car x)))
            (wt (cdr x))))
       't)
     't))
  ((E A A A Q)
   (natp/+ (wt (car x)) (wt (car x))))
  ((E A A Q)
   (equal-if (natp (wt (car x))) 't))
  ((E A A Q)
   (equal-if (natp (wt (cdr x))) 't))
  ((E A A A A 1)
   (natp/+
     (+ (wt (car x)) (wt (car x)))
     (wt (cdr x))))
  ((E A A A A) (equal 't 't))
  ((E A A A)
   (if-same
     (natp
       (+ (wt (car x)) (wt (car x))))
     't))
  ((E A A)
   (if-same (natp (wt (cdr x))) 't))
  ((E A)
   (if-same
     (equal (natp (wt (cdr x))) 't)
     't))
  ((E)
   (if-same
     (equal (natp (wt (car x))) 't)
     't))
  (() (if-same (atom x) 't)))

 ((dethm positive/wt (x)
    (equal (< '0 (wt x)) 't))
  (star-induction x)
  ((A 1 2) (wt x))
  ((A 1 2)
   (if-nest-A
     (atom x)
     '1
     (+ (+ (wt (car x)) (wt (car x)))
        (wt (cdr x)))))
  ((A 1) (< '0 '1))
  ((A) (equal 't 't))
  ((E A A 1 2) (wt x))
  ((E A A 1 2)
   (if-nest-E
     (atom x)
     '1
     (+ (+ (wt (car x)) (wt (car x)))
        (wt (cdr x)))))
  ((E A A)
   (if-true
     (equal
       (< '0
          (+
             (+ (wt (car x))
                (wt (car x)))
             (wt (cdr x))))
       't)
     't))
  ((E A A Q)
   (equal-if (< '0 (wt (car x))) 't))
  ((E A A A)
   (if-true
     (equal
       (< '0
          (+
             (+ (wt (car x))
                (wt (car x)))
             (wt (cdr x))))
       't)
     't))
  ((E A A A Q)
   (positives-+
     (wt (car x))
     (wt (car x))))
  ((E A A Q)
   (equal-if (< '0 (wt (car x))) 't))
  ((E A A Q)
   (equal-if (< '0 (wt (cdr x))) 't))
  ((E A A A A 1)
   (positives-+
     (+ (wt (car x)) (wt (car x)))
     (wt (cdr x))))
  ((E A A A A) (equal 't 't))
  ((E A A A)
   (if-same
     (< '0
        (+ (wt (car x)) (wt (car x))))
     't))
  ((E A A)
   (if-same (< '0 (wt (cdr x))) 't))
  ((E A)
   (if-same
     (equal (< '0 (wt (cdr x))) 't)
     't))
  ((E)
   (if-same
     (equal (< '0 (wt (car x))) 't)
     't))
  (() (if-same (atom x) 't)))

 ((defun align (x)
    (if (atom x)
        x
        (if (atom (car x))
            (cons
              (car x)
              (align (cdr x)))
            (align (rotate x)))))
  (wt x)
  ((Q) (natp/wt x))
  (()
   (if-true
     (if (atom x)
         't
         (if (atom (car x))
             (< (wt (cdr x)) (wt x))
             (< (wt (rotate x)) (wt x))))
     'nil))
  ((E A 2) (wt x))
  ((E A 2)
   (if-nest-E
     (atom x)
     '1
     (+ (+ (wt (car x)) (wt (car x)))
        (wt (cdr x)))))
  ((E A)
   (if-true
     (< (wt (cdr x))
        (+ (+ (wt (car x)) (wt (car x)))
           (wt (cdr x))))
     't))
  ((E A Q) (natp/wt (cdr x)))
  ((E A A 1) (identity-+ (wt (cdr x))))
  ((E A A)
   (common-addends-<
     '0
     (+ (wt (car x)) (wt (car x)))
     (wt (cdr x))))
  ((E A Q) (natp/wt (cdr x)))
  ((E A Q) (positive/wt (car x)))
  ((E A A)
   (positives-+
     (wt (car x))
     (wt (car x))))
  ((E A)
   (if-same (< '0 (wt (car x))) 't))
  ((E E 1 1) (rotate x))
  ((E E 1)
   (wt
       (cons
         (car (car x))
         (cons (cdr (car x)) (cdr x)))))
  ((E E 1 Q)
   (atom/cons
     (car (car x))
     (cons (cdr (car x)) (cdr x))))
  ((E E 1)
   (if-false
     '1
     (+
        (+
           (wt
               (car
                 (cons
                   (car (car x))
                   (cons
                     (cdr (car x))
                     (cdr x)))))
           (wt
               (car
                 (cons
                   (car (car x))
                   (cons
                     (cdr (car x))
                     (cdr x))))))
        (wt
            (cdr
              (cons
                (car (car x))
                (cons
                  (cdr (car x))
                  (cdr x))))))))
  ((E E 1 1 1 1)
   (car/cons
     (car (car x))
     (cons (cdr (car x)) (cdr x))))
  ((E E 1 1 2 1)
   (car/cons
     (car (car x))
     (cons (cdr (car x)) (cdr x))))
  ((E E 1 2 1)
   (cdr/cons
     (car (car x))
     (cons (cdr (car x)) (cdr x))))
  ((E E 1 2)
   (wt (cons (cdr (car x)) (cdr x))))
  ((E E 1 2 Q)
   (atom/cons (cdr (car x)) (cdr x)))
  ((E E 1 2)
   (if-false
     '1
     (+
        (+
           (wt
               (car
                 (cons
                   (cdr (car x))
                   (cdr x))))
           (wt
               (car
                 (cons
                   (cdr (car x))
                   (cdr x)))))
        (wt
            (cdr
              (cons
                (cdr (car x))
                (cdr x)))))))
  ((E E 1 2 1 1 1)
   (car/cons (cdr (car x)) (cdr x)))
  ((E E 1 2 1 2 1)
   (car/cons (cdr (car x)) (cdr x)))
  ((E E 1 2 2 1)
   (cdr/cons (cdr (car x)) (cdr x)))
  ((E E 2) (wt x))
  ((E E 2)
   (if-nest-E
     (atom x)
     '1
     (+ (+ (wt (car x)) (wt (car x)))
        (wt (cdr x)))))
  ((E E 2 1 1) (wt (car x)))
  ((E E 2 1 2) (wt (car x)))
  ((E E 2 1 1)
   (if-nest-E
     (atom (car x))
     '1
     (+
        (+ (wt (car (car x)))
           (wt (car (car x))))
        (wt (cdr (car x))))))
  ((E E 2 1 2)
   (if-nest-E
     (atom (car x))
     '1
     (+
        (+ (wt (car (car x)))
           (wt (car (car x))))
        (wt (cdr (car x))))))
  ((E E 1)
   (associate-+
     (+ (wt (car (car x)))
        (wt (car (car x))))
     (+ (wt (cdr (car x)))
        (wt (cdr (car x))))
     (wt (cdr x))))
  ((E E)
   (common-addends-<
     (+
        (+ (wt (car (car x)))
           (wt (car (car x))))
        (+ (wt (cdr (car x)))
           (wt (cdr (car x)))))
     (+
        (+
           (+ (wt (car (car x)))
              (wt (car (car x))))
           (wt (cdr (car x))))
        (+
           (+ (wt (car (car x)))
              (wt (car (car x))))
           (wt (cdr (car x)))))
     (wt (cdr x))))
  ((E E 1)
   (associate-+
     (+ (wt (car (car x)))
        (wt (car (car x))))
     (wt (cdr (car x)))
     (wt (cdr (car x)))))
  ((E E 1)
   (commute-+
     (+
        (+ (wt (car (car x)))
           (wt (car (car x))))
        (wt (cdr (car x))))
     (wt (cdr (car x)))))
  ((E E)
   (common-addends-<
     (wt (cdr (car x)))
     (+
        (+ (wt (car (car x)))
           (wt (car (car x))))
        (wt (cdr (car x))))
     (+
        (+ (wt (car (car x)))
           (wt (car (car x))))
        (wt (cdr (car x))))))
  ((E E 1)
   (if-true (wt (cdr (car x))) 't))
  ((E E 1 Q) (natp/wt (cdr (car x))))
  ((E E 1 A)
   (identity-+ (wt (cdr (car x)))))
  ((E E 1 Q) (natp/wt (cdr (car x))))
  ((E E 1)
   (if-true
     (+ '0 (wt (cdr (car x))))
     't))
  ((E E)
   (common-addends-<
     '0
     (+ (wt (car (car x)))
        (wt (car (car x))))
     (wt (cdr (car x)))))
  ((E E)
   (if-true
     (< '0
        (+ (wt (car (car x)))
           (wt (car (car x)))))
     't))
  ((E E Q) (positive/wt (car (car x))))
  ((E E A)
   (positives-+
     (wt (car (car x)))
     (wt (car (car x)))))
  ((E E)
   (if-same
     (< '0 (wt (car (car x))))
     't))
  ((E) (if-same (atom (car x)) 't))
  (() (if-same (atom x) 't)))

 ((dethm align/align (x)
    (equal (align (align x)) (align x)))
  (align x)
  ((A 2) (align x))
  ((A 1 1) (align x))
  ((A 1 1)
   (if-nest-A
     (atom x)
     x
     (if (atom (car x))
         (cons (car x) (align (cdr x)))
         (align (rotate x)))))
  ((A 2)
   (if-nest-A
     (atom x)
     x
     (if (atom (car x))
         (cons (car x) (align (cdr x)))
         (align (rotate x)))))
  ((A 1) (align x))
  ((A 1)
   (if-nest-A
     (atom x)
     x
     (if (atom (car x))
         (cons (car x) (align (cdr x)))
         (align (rotate x)))))
  ((A) (equal-same x))
  ((E A A 2) (align x))
  ((E A A 1 1) (align x))
  ((E A A 1 1)
   (if-nest-E
     (atom x)
     x
     (if (atom (car x))
         (cons (car x) (align (cdr x)))
         (align (rotate x)))))
  ((E A A 1 1)
   (if-nest-A
     (atom (car x))
     (cons (car x) (align (cdr x)))
     (align (rotate x))))
  ((E A A 2)
   (if-nest-E
     (atom x)
     x
     (if (atom (car x))
         (cons (car x) (align (cdr x)))
         (align (rotate x)))))
  ((E A A 2)
   (if-nest-A
     (atom (car x))
     (cons (car x) (align (cdr x)))
     (align (rotate x))))
  ((E A A 1)
   (align
     (cons (car x) (align (cdr x)))))
  ((E A A 1 Q)
   (atom/cons (car x) (align (cdr x))))
  ((E A A 1 E Q 1)
   (car/cons (car x) (align (cdr x))))
  ((E A A 1 E)
   (if-nest-A
     (atom (car x))
     (cons
       (car
         (cons (car x) (align (cdr x))))
       (align
         (cdr
           (cons
             (car x)
             (align (cdr x))))))
     (align
       (rotate
         (cons (car x) (align (cdr x)))))))
  ((E A A 1)
   (if-false
     (cons (car x) (align (cdr x)))
     (cons
       (car
         (cons (car x) (align (cdr x))))
       (align
         (cdr
           (cons
             (car x)
             (align (cdr x))))))))
  ((E A A 1 1)
   (car/cons (car x) (align (cdr x))))
  ((E A A 1 2 1)
   (cdr/cons (car x) (align (cdr x))))
  ((E A A 1 2)
   (equal-if
     (align (align (cdr x)))
     (align (cdr x))))
  ((E A A)
   (equal-same
     (cons (car x) (align (cdr x)))))
  ((E A)
   (if-same
     (equal
       (align (align (cdr x)))
       (align (cdr x)))
     't))
  ((E E A 2) (align x))
  ((E E A 2)
   (if-nest-E
     (atom x)
     x
     (if (atom (car x))
         (cons (car x) (align (cdr x)))
         (align (rotate x)))))
  ((E E A 2)
   (if-nest-E
     (atom (car x))
     (cons (car x) (align (cdr x)))
     (align (rotate x))))
  ((E E A 1 1) (align x))
  ((E E A 1 1 E)
   (if-nest-E
     (atom (car x))
     (cons (car x) (align (cdr x)))
     (align (rotate x))))
  ((E E A 1 1)
   (if-nest-E
     (atom x)
     x
     (align (rotate x))))
  ((E E A 1)
   (equal-if
     (align (align (rotate x)))
     (align (rotate x))))
  ((E E A)
   (equal-same (align (rotate x))))
  ((E E)
   (if-same
     (equal
       (align (align (rotate x)))
       (align (rotate x)))
     't))
  ((E) (if-same (atom (car x)) 't))
  (() (if-same (atom x) 't))))
