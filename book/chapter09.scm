(((defun member? (x ys)
    (if (atom ys)
        'nil
        (if (equal x (car ys))
            't
            (member? x (cdr ys)))))
  (size ys)
  ((Q) (natp/size ys))
  (()
   (if-true
     (if (atom ys)
         't
         (if (equal x (car ys))
             't
             (< (size (cdr ys))
                (size ys))))
     'nil))
  ((E E) (size/cdr ys))
  ((E) (if-same (equal x (car ys)) 't))
  (() (if-same (atom ys) 't)))

 ((defun set? (xs)
    (if (atom xs)
        't
        (if (member? (car xs) (cdr xs))
            'nil
            (set? (cdr xs)))))
  (size xs)
  ((Q) (natp/size xs))
  (()
   (if-true
     (if (atom xs)
         't
         (if
             (member?
               (car xs)
               (cdr xs))
             't
             (< (size (cdr xs))
                (size xs))))
     'nil))
  ((E E) (size/cdr xs))
  ((E)
   (if-same
     (member? (car xs) (cdr xs))
     't))
  (() (if-same (atom xs) 't)))

 ((defun add-atoms (x ys)
    (if (atom x)
        (if (member? x ys)
            ys
            (cons x ys))
        (add-atoms
          (car x)
          (add-atoms (cdr x) ys))))
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

 ((defun atoms (x) (add-atoms x '()))
  nil)

 ((dethm set?/t (xs)
    (if (set? xs)
        (equal (set? xs) 't)
        't))
  (list-induction xs)
  ((A A 1) (set? xs))
  ((A A 1)
   (if-nest-A
     (atom xs)
     't
     (if (member? (car xs) (cdr xs))
         'nil
         (set? (cdr xs)))))
  ((A A) (equal-same 't))
  ((A) (if-same (set? xs) 't))
  ((E A A 1) (set? xs))
  ((E A A 1)
   (if-nest-E
     (atom xs)
     't
     (if (member? (car xs) (cdr xs))
         'nil
         (set? (cdr xs)))))
  ((E A Q) (set? xs))
  ((E A Q)
   (if-nest-E
     (atom xs)
     't
     (if (member? (car xs) (cdr xs))
         'nil
         (set? (cdr xs)))))
  ((E A)
   (if-same
     (member? (car xs) (cdr xs))
     (if
         (if (member? (car xs) (cdr xs))
             'nil
             (set? (cdr xs)))
         (equal
           (if
               (member?
                 (car xs)
                 (cdr xs))
               'nil
               (set? (cdr xs)))
           't)
         't)))
  ((E A A Q)
   (if-nest-A
     (member? (car xs) (cdr xs))
     'nil
     (set? (cdr xs))))
  ((E A A A 1)
   (if-nest-A
     (member? (car xs) (cdr xs))
     'nil
     (set? (cdr xs))))
  ((E A E Q)
   (if-nest-E
     (member? (car xs) (cdr xs))
     'nil
     (set? (cdr xs))))
  ((E A E A 1)
   (if-nest-E
     (member? (car xs) (cdr xs))
     'nil
     (set? (cdr xs))))
  ((E A A)
   (if-false (equal 'nil 't) 't))
  ((E)
   (if-same
     (set? (cdr xs))
     (if
         (if (set? (cdr xs))
             (equal (set? (cdr xs)) 't)
             't)
         (if (member? (car xs) (cdr xs))
             't
             (if (set? (cdr xs))
                 (equal
                   (set? (cdr xs))
                   't)
                 't))
         't)))
  ((E A Q)
   (if-nest-A
     (set? (cdr xs))
     (equal (set? (cdr xs)) 't)
     't))
  ((E A A E)
   (if-nest-A
     (set? (cdr xs))
     (equal (set? (cdr xs)) 't)
     't))
  ((E E Q)
   (if-nest-E
     (set? (cdr xs))
     (equal (set? (cdr xs)) 't)
     't))
  ((E E A E)
   (if-nest-E
     (set? (cdr xs))
     (equal (set? (cdr xs)) 't)
     't))
  ((E E)
   (if-true
     (if (member? (car xs) (cdr xs))
         't
         't)
     't))
  ((E E)
   (if-same
     (member? (car xs) (cdr xs))
     't))
  ((E A A E 1)
   (equal-if (set? (cdr xs)) 't))
  ((E A A E) (equal 't 't))
  ((E A A)
   (if-same
     (member? (car xs) (cdr xs))
     't))
  ((E A)
   (if-same
     (equal (set? (cdr xs)) 't)
     't))
  ((E) (if-same (set? (cdr xs)) 't))
  (() (if-same (atom xs) 't)))

 ((dethm set?/nil (xs)
    (if (set? xs)
        't
        (equal (set? xs) 'nil)))
  (list-induction xs)
  ((A Q) (set? xs))
  ((A Q)
   (if-nest-A
     (atom xs)
     't
     (if (member? (car xs) (cdr xs))
         'nil
         (set? (cdr xs)))))
  ((A)
   (if-true 't (equal (set? xs) 'nil)))
  ((E A E 1) (set? xs))
  ((E A E 1)
   (if-nest-E
     (atom xs)
     't
     (if (member? (car xs) (cdr xs))
         'nil
         (set? (cdr xs)))))
  ((E A Q) (set? xs))
  ((E A Q)
   (if-nest-E
     (atom xs)
     't
     (if (member? (car xs) (cdr xs))
         'nil
         (set? (cdr xs)))))
  ((E A)
   (if-same
     (member? (car xs) (cdr xs))
     (if
         (if (member? (car xs) (cdr xs))
             'nil
             (set? (cdr xs)))
         't
         (equal
           (if
               (member?
                 (car xs)
                 (cdr xs))
               'nil
               (set? (cdr xs)))
           'nil))))
  ((E A A Q)
   (if-nest-A
     (member? (car xs) (cdr xs))
     'nil
     (set? (cdr xs))))
  ((E A A)
   (if-false
     't
     (equal
       (if (member? (car xs) (cdr xs))
           'nil
           (set? (cdr xs)))
       'nil)))
  ((E A A 1)
   (if-nest-A
     (member? (car xs) (cdr xs))
     'nil
     (set? (cdr xs))))
  ((E A A) (equal 'nil 'nil))
  ((E A E Q)
   (if-nest-E
     (member? (car xs) (cdr xs))
     'nil
     (set? (cdr xs))))
  ((E A E E 1)
   (if-nest-E
     (member? (car xs) (cdr xs))
     'nil
     (set? (cdr xs))))
  ((E)
   (if-same
     (set? (cdr xs))
     (if
         (if (set? (cdr xs))
             't
             (equal
               (set? (cdr xs))
               'nil))
         (if (member? (car xs) (cdr xs))
             't
             (if (set? (cdr xs))
                 't
                 (equal
                   (set? (cdr xs))
                   'nil)))
         't)))
  ((E A Q)
   (if-nest-A
     (set? (cdr xs))
     't
     (equal (set? (cdr xs)) 'nil)))
  ((E A A E)
   (if-nest-A
     (set? (cdr xs))
     't
     (equal (set? (cdr xs)) 'nil)))
  ((E E Q)
   (if-nest-E
     (set? (cdr xs))
     't
     (equal (set? (cdr xs)) 'nil)))
  ((E E A E)
   (if-nest-E
     (set? (cdr xs))
     't
     (equal (set? (cdr xs)) 'nil)))
  ((E A A)
   (if-same
     (member? (car xs) (cdr xs))
     't))
  ((E A) (if-same 't 't))
  ((E E A E 1)
   (equal-if (set? (cdr xs)) 'nil))
  ((E E A E) (equal 'nil 'nil))
  ((E E A)
   (if-same
     (member? (car xs) (cdr xs))
     't))
  ((E E)
   (if-same
     (equal (set? (cdr xs)) 'nil)
     't))
  ((E) (if-same (set? (cdr xs)) 't))
  (() (if-same (atom xs) 't)))

 ((dethm set?/add-atoms (a bs)
    (if (set? bs)
        (equal
          (set? (add-atoms a bs))
          't)
        't))
  (add-atoms a bs)
  ((A A 1 1) (add-atoms a bs))
  ((A A 1 1)
   (if-nest-A
     (atom a)
     (if (member? a bs) bs (cons a bs))
     (add-atoms
       (car a)
       (add-atoms (cdr a) bs))))
  ((A A 1)
   (if-same
     (member? a bs)
     (set?
       (if (member? a bs)
           bs
           (cons a bs)))))
  ((A A 1 A 1)
   (if-nest-A
     (member? a bs)
     bs
     (cons a bs)))
  ((A A 1 E 1)
   (if-nest-E
     (member? a bs)
     bs
     (cons a bs)))
  ((A A 1 A) (set?/t bs))
  ((A A 1 E) (set? (cons a bs)))
  ((A A 1 E Q) (atom/cons a bs))
  ((A A 1 E)
   (if-false
     't
     (if
         (member?
           (car (cons a bs))
           (cdr (cons a bs)))
         'nil
         (set? (cdr (cons a bs))))))
  ((A A 1 E Q 1) (car/cons a bs))
  ((A A 1 E Q 2) (cdr/cons a bs))
  ((A A 1 E E 1) (cdr/cons a bs))
  ((A A 1 E E) (set?/t bs))
  ((A A 1 E)
   (if-nest-E (member? a bs) 'nil 't))
  ((A A 1) (if-same (member? a bs) 't))
  ((A A) (equal-same 't))
  ((A) (if-same (set? bs) 't))
  ((E)
   (if-same
     (set? bs)
     (if
         (if
             (set?
               (add-atoms (cdr a) bs))
             (equal
               (set?
                 (add-atoms
                   (car a)
                   (add-atoms
                     (cdr a)
                     bs)))
               't)
             't)
         (if
             (if (set? bs)
                 (equal
                   (set?
                     (add-atoms
                       (cdr a)
                       bs))
                   't)
                 't)
             (if (set? bs)
                 (equal
                   (set?
                     (add-atoms a bs))
                   't)
                 't)
             't)
         't)))
  ((E A A Q)
   (if-nest-A
     (set? bs)
     (equal
       (set? (add-atoms (cdr a) bs))
       't)
     't))
  ((E A A A)
   (if-nest-A
     (set? bs)
     (equal (set? (add-atoms a bs)) 't)
     't))
  ((E E A Q)
   (if-nest-E
     (set? bs)
     (equal
       (set? (add-atoms (cdr a) bs))
       't)
     't))
  ((E E A A)
   (if-nest-E
     (set? bs)
     (equal (set? (add-atoms a bs)) 't)
     't))
  ((E E A) (if-same 't 't))
  ((E E)
   (if-same
     (if (set? (add-atoms (cdr a) bs))
         (equal
           (set?
             (add-atoms
               (car a)
               (add-atoms (cdr a) bs)))
           't)
         't)
     't))
  ((E A)
   (if-same
     (set? (add-atoms (cdr a) bs))
     (if
         (if
             (set?
               (add-atoms (cdr a) bs))
             (equal
               (set?
                 (add-atoms
                   (car a)
                   (add-atoms
                     (cdr a)
                     bs)))
               't)
             't)
         (if
             (equal
               (set?
                 (add-atoms (cdr a) bs))
               't)
             (equal
               (set? (add-atoms a bs))
               't)
             't)
         't)))
  ((E A A Q)
   (if-nest-A
     (set? (add-atoms (cdr a) bs))
     (equal
       (set?
         (add-atoms
           (car a)
           (add-atoms (cdr a) bs)))
       't)
     't))
  ((E A E Q)
   (if-nest-E
     (set? (add-atoms (cdr a) bs))
     (equal
       (set?
         (add-atoms
           (car a)
           (add-atoms (cdr a) bs)))
       't)
     't))
  ((E A E)
   (if-true
     (if
         (equal
           (set? (add-atoms (cdr a) bs))
           't)
         (equal
           (set? (add-atoms a bs))
           't)
         't)
     't))
  ((E A A A Q 1)
   (set?/t (add-atoms (cdr a) bs)))
  ((E A A A Q) (equal 't 't))
  ((E A A A)
   (if-true
     (equal (set? (add-atoms a bs)) 't)
     't))
  ((E A E Q 1)
   (set?/nil (add-atoms (cdr a) bs)))
  ((E A E Q) (equal 'nil 't))
  ((E A E)
   (if-false
     (equal (set? (add-atoms a bs)) 't)
     't))
  ((E A A A 1 1) (add-atoms a bs))
  ((E A A A 1 1)
   (if-nest-E
     (atom a)
     (if (member? a bs) bs (cons a bs))
     (add-atoms
       (car a)
       (add-atoms (cdr a) bs))))
  ((E A A A 1)
   (equal-if
     (set?
       (add-atoms
         (car a)
         (add-atoms (cdr a) bs)))
     't))
  ((E A A A) (equal 't 't))
  ((E A A)
   (if-same
     (equal
       (set?
         (add-atoms
           (car a)
           (add-atoms (cdr a) bs)))
       't)
     't))
  ((E A)
   (if-same
     (set? (add-atoms (cdr a) bs))
     't))
  ((E) (if-same (set? bs) 't))
  (() (if-same (atom a) 't)))
  
 ((dethm set?/atoms (a)
    (equal (set? (atoms a)) 't))
  nil
  ((1 1) (atoms a))
  (()
   (if-true
     (equal
       (set? (add-atoms a '()))
       't)
     't))
  ((Q)
   (if-true
     't
     (if (member? (car '()) (cdr '()))
         'nil
         (set? (cdr '())))))
  ((Q Q) (atom '()))
  ((Q) (set? '()))
  ((A 1) (set?/add-atoms a '()))
  ((A) (equal 't 't))
  (() (if-same (set? '()) 't))))
