
; page 204 - left

(defun list0 () '())
(defun list0? (x) (equal x '()))

(defun list1 (x) (cons x (list0)))
(defun list1? (x)
  (if (atom x) 'nil (list0? (cdr x))))
(defun elem1 (xs) (car xs))

(defun list2 (x y) (cons x (list1 y)))
(defun list2? (x)
  (if (atom x) 'nil (list1? (cdr x))))
(defun elem2 (xs) (elem1 (cdr xs)))

(defun list3 (x y z) (cons x (list2 y z)))
(defun list3? (x)
  (if (atom x) 'nil (list2? (cdr x))))
(defun elem3 (xs) (elem2 (cdr xs)))

(defun tag (sym x) (cons sym x))
(defun tag? (sym x)
  (if (atom x) 'nil (equal (car x) sym)))
(defun untag (x) (cdr x))

(defun member? (x ys)
  (if (atom ys)
      'nil
      (if (equal x (car ys))
          't
          (member? x (cdr ys)))))

; page 204 - right

(defun quote-c (value) (tag 'quote (list1 value)))
(defun quote? (x) (if (tag? 'quote x) (list1? (untag x)) 'nil))
(defun quote.value (e) (elem1 (untag e)))

(defun if-c (Q A E) (tag 'if (list3 Q A E)))
(defun if? (x) (if (tag? 'if x) (list3? (untag x)) 'nil))
(defun if.Q (e) (elem1 (untag e)))
(defun if.A (e) (elem2 (untag e)))
(defun if.E (e) (elem3 (untag e)))

(defun app-c (name args) (cons name args))
(defun app? (x)
  (if (atom x)
      'nil
      (if (quote? x)
          'nil
          (if (if? x)
              'nil
              't))))
(defun app.name (e) (car e))
(defun app.args (e) (cdr e))

(defun var? (x)
  (if (equal x 't)
      'nil
      (if (equal x 'nil)
          'nil
          (if (natp x)
              'nil
              (atom x)))))

(defun defun-c (name formals body)
  (tag 'defun (list3 name formals body)))
(defun defun? (x) (if (tag? 'defun x) (list3? (untag x)) 'nil))
(defun defun.name (def) (elem1 (untag def)))
(defun defun.formals (def) (elem2 (untag def)))
(defun defun.body (def) (elem3 (untag def)))

(defun dethm-c (name formals body)
  (tag 'dethm (list3 name formals body)))
(defun dethm? (x) (if (tag? 'dethm x) (list3? (untag x)) 'nil))
(defun dethm.name (def) (elem1 (untag def)))
(defun dethm.formals (def) (elem2 (untag def)))
(defun dethm.body (def) (elem3 (untag def)))

; page 205

(defun lookup (name defs)
  (if (atom defs)
      name
      (if (equal (def.name (car defs)) name)
          (car defs)
          (lookup name (cdr defs)))))

(defun undefined? (name defs)
  (if (var? name)
      (equal (lookup name defs) name)
      'nil))

; page 206 - left

(defun exprs? (defs vars es)
  (if (atom es)
      't
      (if (var? (car es))
          (if (bound? (car es) vars)
              (exprs? defs vars (cdr es))
              'nil)
          (if (quote? (car es))
              (exprs? defs vars (cdr es))
              (if (if? (car es))
                  (if (exprs? defs vars (if-QAE (car es)))
                      (exprs? defs vars (cdr es))
                      'nil)
                  (if (app? (car es))
                      (if (app-arity? defs (car es))
                          (if (exprs? defs vars (app.args (car es)))
                              (exprs? defs vars (cdr es))
                              'nil)
                          'nil)
                      'nil))))))

(defun expr? (defs vars e)
  (exprs? defs vars (list1 e)))

; page 206 - right

(defun formals? (vars)
  (if (atom vars)
      't
      (if (var? (car vars))
          (if (member? (car vars) (cdr vars))
              'nil
              (formals? (cdr vars)))
          'nil)))

; page 207

; page 208

(defun def-contents? (known-defs formals body)
  (if (formals? formals)
      (expr? known-defs formals body)
      'nil))

(defun def? (known-defs def)
  (if (dethm? def)
      (if (undefined? (dethm.name def) known-defs)
          (def-contents? known-defs
                         (dethm.formals def)
                         (dethm.body def))
          'nil)
      (if (defun? def)
          (if (undefined? (defun.name def) known-defs)
              (def-contents? (extend-rec known-defs def)
                             (defun.formals def)
                             (defun.body def))
              'nil)
          'nil)))

(defun defs? (known-defs defs)
  (if (atom defs)
      't
      (if (def? known-defs (car defs))
          (defs? (list-extend known-defs (car defs))
                 (cdr defs))
          'nil)))


; page 209

; page 210

; page 211

; page 212

; page 213

(defun J-Bob/step (defs e steps)
  (if (defs? '() defs)
      (if (expr? defs 'any e)
          (if (steps? defs steps)
              (rewrite/steps defs e steps)
              e)
          e)
      e))

(defun J-Bob/prove (defs pfs)
  TODO)

(defun J-Bob/define (defs pfs)
  (if (defs? '() defs)
      (if (proofs? defs pfs)
          (rewrite/define+ defs pfs)
          defs)
      defs))

; page 214

(defun axioms ()
  '((dethm atom/cons (x y)
      (equal (atom (cons x y)) 'nil))
    (dethm car/cons (x y)
      (equal (car (cons x y)) x))))
    ; todo: the rest...

(defun prelude ()
  (J-Bob/define (axioms)
    '(((defun list-induction (x)
         (if (atom x)
             '()
             (cons (car x)
                   (list-induction (cdr x)))))
       (size x)
       ((A E) (size/cdr x))
       ((A) (if-same (atom x) 't))
       ((Q) (natp/size x))
       (() (if-true 't 'nil)))
      ((defun star-induction (x)
         (if (atom x)
             x
             (cons (star-induction (car x))
                   (star-induction (cdr x)))))
       (size x)
       ((A E A) (size/cdr x))
       ((A E Q) (size/car x))
       ((A E) (if-true 't 'nil))
       ((A) (if-same (atom x) 't))
       ((Q) (natp/size x))
       (() (if-true 't 'nil))))))
