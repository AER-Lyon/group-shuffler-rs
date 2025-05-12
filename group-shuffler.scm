#!/usr/bin/env -S guile -s
!#

;;; Copyright © 2025 Noé Lopez <noe@a-lec.org>

(define-module (group-shuffler)
  #:use-module (ice-9 rdelim)
  #:use-module (ice-9 pretty-print)
  #:use-module (ice-9 match)
  #:use-module (srfi srfi-1)
  #:use-module (srfi srfi-26))

(define* (parse-line port #:key (delimiter #\;))
  (let ((line (read-line port)))
    (if (string? line)
        (string-split line delimiter)
        #f)))

(define (detect-processor header)
  (if (and (member "master" header)
           (member "member0" header))
      (lambda (values)
        (take-right values 2))
      (lambda (values)
        (list (car values)))))

(define (list-students port)
  (let* ((header (parse-line port))
         (processor (detect-processor header)))
    (let loop ((values (parse-line port)))
      (if values
          (append (processor values)
                  (loop (parse-line port)))
          '()))))

(define (shuffle-students students)
  (map cdr
       (sort! (map (lambda (student) (cons (random:uniform) student))
                   students)
              (lambda (p1 p2) (> (car p1) (car p2))))))

(define (make-groups students)
  (match students
    ((member0 member1 . others)
     (cons (cons member0 member1)
           (make-groups others)))
    ((member0)
     (cons (cons member0 "no teammate? 🤨") '()))
    (()
     '())))

(define (write-groups port groups)
  (format port "name;code;master;member0;member1~%")
  (map
   (lambda (group)
     (format port "~a;~a;~a;~a;~a~%"
             (car group)
             (car group)
             (car group)
             (car group)
             (cdr group)))
   groups)
  *unspecified*)

(define (main)
  (match (program-arguments)
    ((_ input-file output-file)
     (let* ((students (call-with-input-file input-file list-students))
            (groups (make-groups (shuffle-students students))))
       (call-with-output-file output-file (cut write-groups <> groups))))
    (_ (format #t "Usage: group-shuffler.scm input-file output-file~%"))))

(main)
