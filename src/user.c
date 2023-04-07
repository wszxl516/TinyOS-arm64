#include "user.h"

void FUNC_NORETURN test_user() {
  while (1) {
    SYSCALL(0);
    SYSCALL(2);
    SYSCALL(1);
  }
}
