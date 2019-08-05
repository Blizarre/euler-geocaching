; Compatible with Chicken Scheme 5

(import (chicken process-context)
        (chicken format)
        srfi-1) ; for null-list?

(define (one? n) (eq? n 1))
(define (inc n) (+ n 1))

; Naive approach
;================

; to compute fib(x), we compute fib(1), fib(2), fib(3), fib(4).... until fib(n)
; fibinternal provide the next fibonnaci number based on the previous (fib-1) and the
; one before (fib-2)
(define (fib n)
  (cond
    ((eq? n 1) 1)
    ((eq? n 2) 1)
    (else
      (let fibinternal ((steps (- n 2)) (fib-2 (fib 1)) (fib-1 (fib 2)))
        (cond
          ((one? steps) (+ fib-2  fib-1))
          (else (fibinternal (- steps 1) fib-1 (+ fib-1 fib-2))))))))

; give the index of the fibonnaci number with n digits. Iterate over all numbers
; until we find one with the right size
(define (numdigits n)
  (let next ((target n) (index 1))
    (if
      (>= (string-length (number->string (fib index)))
          target)
      index
      (next target (+ 1 index)))))


; Less naive approach
;======================

; Much better way of doing it: register a callback that will be called at each step
; the callback stop the computation by returning #f, and use dynamic programming to
; compute fib in O(n)
(define (fibcallback callback)
  (if (callback 1 1) 1
    (if (callback 2 1) 1
      (let fibinternal ((step 3) (fib-2 1) (fib-1 1))
        (if (not (callback step (+ fib-2 fib-1)))
          (fibinternal (inc step) fib-1 (+ fib-1 fib-2)))))))


; Some quick and dirty logic to parse the first argument as a number (or return #f)
(define (parse-args args)
     (if (null-list? args) (print "Error: Missing length argument")
        (let ((num (string->number (car args))))
          (begin
            (if (eq? #f num) (print "Error: argument must be int"))
            num))))

; define the callback using the argument and use it in fibcallback to stop the
; computation once the right number of digits is reached
(let ((num (parse-args (command-line-arguments))))
     (if (not (eqv? #f num))
        (fibcallback
               (lambda (step value)
                   (let ((value-len (string-length (number->string value))))
                      (if (>= value-len num)
                       (begin
                        (print (format "Step ~a (~a)" step value-len))
                        (print value)
                        #t)
                      #f)))
             )))


; other callbacks, used for debugging
(define (displaycallback step value)
  (begin (print (format "Step ~a: ~a" step value))
         (> value 1000000000)))

(define (samplecallback step value)
        (let ((value-len (string-length (number->string value))))
           (if (>= value-len 500)
            (begin
             (print (format "Step ~a (~a)" step value-len))
             (print value)
             #t)
           #f)))
