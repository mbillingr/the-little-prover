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
  nil))
