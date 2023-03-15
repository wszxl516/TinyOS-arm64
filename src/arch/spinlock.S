 #include "macros.S"
 /* void spin_lock(spinlock_t *lock);*/
func_def spin_lock
	mov	    w2, #1
1:	mov     w1, wzr
    /*cmp value*/
2:	casa	w1, w2, [x0]
	cbz	    w1, 3f
	ldxr	w1, [x0]
	cbz	    w1, 2b
	wfe
	b	    1b
3:
	ret
func_end spin_lock

 /* void spin_unlock(spinlock_t *lock);*/
func_def spin_unlock
	stlr	wzr, [x0]
	ret
func_end spin_unlock