
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

; page 205 - left

(defun if-QAE (e) (list3 (if.Q e) (if.A e) (if.E e)))
(defun QAE-if (es) (if-c (elem1 es) (elem2 es) (elem3 es)))

(defun rator? (name)
  (member? name '(equal atom car cdr cons natp size + <)))

(defun rator.formals (rator)
  (if (member? rator '(atom car cdr natp size))
      '(x)
      (if (member? rator '(equal cons + <))
          '(x y)
          'nil)))

(defun def.name (def)
  (if (defun? def)
      (defun.name def)
      (if (dethm? def)
          (dethm.name def)
          def)))

(defun def.formals (def)
  (if (dethm? def)
      (dethm.formals def)
      (if (defun? def)
          (defun.formals def)
          '())))

(defun if-c-when-necessary (Q A E)
  (if (equal A E) A (if-c Q A E)))

(defun conjunction (es)
  (if (atom es)
      (quote-c 't)
      (if (atom (cdr es))
          (car es)
          (if-c (car es)
                (conjunction (cdr es))
                (quote-c 'nil)))))

(defun implication (es e)
  (if (atom es)
      e
      (if-c (car es)
            (implication (cdr es) e)
            (quote-c 't))))

; page 205 - right

(defun args-arity? (def args)
  (if (dethm? def)
      'nil
      (if (defun? def)
          (arity? (defun.formals def) args)
          (if (rator? def)
              (arity? (rator.formals def) args)
              'nil))))

(defun app-arity? (defs app)
  (args-arity? (lookup (app.name app) defs) (app.args app)))

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

(defun bound? (var vars)
  (if (equal vars 'any) 't (member? var vars)))

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

(defun subset? (xs ys)
  (if (atom xs)
      't
      (if (member? (car xs) ys)
          (subset? (cdr xs) ys)
          'nil)))

(defun list-extend (xs x)
  (if (atom xs)
      (list1 x)
      (if (equal (car xs) x)
          xs
          (cons (car xs)
                (list-extend (cdr xs) x)))))

(defun list-union (xs ys)
  (if (atom ys)
      xs
      (list-union (list-extend xs (car ys))
                  (cdr ys))))

; page 206 - right

(defun get-arg-from (n args from)
  (if (atom args)
      'nil
      (if (equal n from)
          (car args)
          (get-arg from n (cdr args) (+ from '1)))))

(defun get-arg (n args) (get-arg-from n args '1))

(defun set-arg-from (n args y from)
  (if (atom args)
      '()
      (if (equal n from)
          (cons y (cdr args))
          (cons (car args)
                (set-arg-from n (cdr args) y (+ from '1))))))

(defun set-arg (n args y) (set-arg-from n args y '1))

(defun <=len-from (n args from)
  (if (atom args)
      'nil
      (if (equal n from)
          't
          (<=len-from n (cdr args) (+ from '1)))))

(defun <=len (n args)
  (if (< '0 n) (<=len-from n args '1) 'nil))

(defun arity? (vars es)
  (if (atom vars)
      (atom es)
      (if (atom es)
          'nil
          (arity? (cdr vars) (cdr es)))))

(defun formals? (vars)
  (if (atom vars)
      't
      (if (var? (car vars))
          (if (member? (car vars) (cdr vars))
              'nil
              (formals? (cdr vars)))
          'nil)))

; page 207 - left

(defun direction? (dir)
  (if (natp dir) 't (member? dir '(Q A E))))

(defun path? (path)
  (if (atom path)
      't
      (if (direction? (car path))
          (path? (cdr path))
          'nil)))

(defun quoted-exprs? (args)
  (if (atom args)
      't
      (if (quote? (car args))
          (quoted-exprs? (cdr args))
          'nil)))

(defun step-args? (defs def args)
  (if (dethm? def)
      (if (arity? (dethm.formals def) args)
          (exprs? defs 'any args)
          'nil)
      (if (defun? def)
          (if (arity? (defun.formals def) args)
              (exprs? defs 'any args)
              'nil)
          (if (rator? def)
              (if (arity? (rator.formals def) args)
                  (quoted-exprs? args)
                  'nil)
              'nil))))

(defun step-app? (defs app)
  (step-args? defs
              (lookup (app.name app) defs)
              (app.args app)))

; page 207 - right

(defun step? (defs step)
  (if (path? (elem1 step))
      (if (app? (elem2 step))
          (step-app? defs (elem2 step))
          'nil)
      'nil))

(defun steps? (defs steps)
  (if (atom steps)
      't
      (if (step? defs (car steps))
          (steps? defs (cdr steps))
          'nil)))

(defun induction-scheme-for? (def vars e)
  (if (defun? def)
      (if (arity? (defun.formals def) (app.args e))
          (if (formals? (app.args e))
              (subset? (app.args e) vars)
              'nil)
          'nil)
      'nil))

(defun induction-scheme? (defs vars e)
  (if (app? e)
      (induction-scheme-for?
        (lookup (app.name e) defs)
        vars
        e)
      'nil))

(defun seed? (defs def seed)
  (if (equal seed 'nil)
      't
      (if (defun? def)
          (expr? defs (defun.formals def) seed)
          (if (dethm? def)
              (induction-scheme? defs
                                 (dethm.formals def)
                                 seed)
              'nil))))

; page 208 - left

(defun extend-rec (defs def)
  (if (defun? def)
      (list-extend defs
        (defun-c (defun.name def)
                 (defun.formals def)
                 (app-c (defun.name def)
                        (defun.formals def))))
      defs))

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

; page 208 - right

(defun list2-or-more? (pf)
  (if (atom pf)
      'nil
      (if (atom (cdr pf))
          'nil
          't)))

(defun proof? (defs pf)
  (if (list2-or-more? pf)
      (if (def? defs (elem1 pf))
          (if (seed? defs (elem1 pf) (elem2 pf))
              (steps? (extend-rec defs (elem1 pf))
                      (cdr (cdr pf)))
              'nil)
          'nil)
      'nil))

(defun proofs? (defs pfs)
  (if (atom pfs)
      't
      (if (proof? defs (car pfs))
          (proofs? (list-extend defs (elem1 (car pfs)))
                   (cdr pfs))
          'nil)))

(defun sub-var (vars args var)
  (if (atom vars)
      var
      (if (equal (car vars) var)
          (car args)
          (sub-var (cdr vars) (cdr args) var))))

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
      (equal (car (cons x y)) x))
    ; todo: more...
    (dethm size/cdr (x)
      (if (atom x)
          't
          (equal (< (size (car x)) (size x)) 't)))))
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
