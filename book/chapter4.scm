(((defun list0? (x) (equal x '())) nil)

 ((defun list1? (x)
    (if (atom x) 'nil (list0? (cdr x))))
  nil)

 ((defun list2? (x)
    (if (atom x) 'nil (list1? (cdr x))))
  nil)

 ((defun list? (x)
    (if (atom x)
        (equal x '())
        (list? (cdr x))))
  (size x)
  ((Q) (natp/size x))
  (()
   (if-true
     (if (atom x)
         't
         (< (size (cdr x)) (size x)))
     'nil))
  ((E) (size/cdr x))
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
  (() (if-same (atom y) 't))))
