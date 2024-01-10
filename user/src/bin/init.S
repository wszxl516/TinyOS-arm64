.global _start
_start:
    mov  x0, 1
    adr  x1, init
    mov  x2, 6
    mov  x8, #64
    svc  #0
    bl _start
init:
  .string "hello\0"