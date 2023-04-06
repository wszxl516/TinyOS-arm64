#include "syscalls.h"
#include "printf.h"
#include "uart.h"


isize do_syscall(syscall_args_t *args){
    switch (args->syscall_nr) {
        case SYSCALL_HELLO:
            return pr_notice("SYSCALL_HELLO\n");
            break;
        case SYSCALL_WORLD:
            return pr_notice("SYSCALL_WORLD\n");
            break;
        case SYSCALL_PAUSE:
            return getc();
            break;
    }
    return 0;
}