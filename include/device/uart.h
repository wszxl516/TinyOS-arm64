#ifndef __UART_H__
#define __UART_H__
#include "common.h"
#include "config.h"
#include "spinlock.h"
typedef struct OPTIMIZATION_ALIGN(4) {
  REG u8 data;
  _Reserved _Reserved0[3];
  // Receive Status
  // reset 0x0
  REG u32 receive_status;
  _Reserved _Reserved1[16];
  // reset 0b-10010---
  REG u32 flag;
  _Reserved _Reserved2[4];
  // rest 0x00
  REG u32 lp_counter;
  REG u32 baud_rate[2];
  // reset 0x00
  REG u32 line_control;
  // reset 0x0300
  REG u32 control;
} Pl011Uart;

#define UART_FLAGS() (UART->flag)

// flag bit 4; Receive FIFO is full.
#define RX_IS_FULL() ((UART_FLAGS() & (1 << 4)) != 0)

// flag bit 5; Transmit FIFO is full.
#define TX_IS_FULL() ((UART_FLAGS() & (1 << 5)) != 0)

#define IS_BUSY() ((UART_FLAGS() & (1 << 3)) == 1)

#define RX_READY() (!IS_BUSY() && !RX_IS_FULL())

#define TX_READY() (!IS_BUSY() && !TX_IS_FULL())

void init_uart();
char getc();
void putc(char c);
void puts(char *buffer);
u32 gets(char *buffer, u32 size);
#endif  //__UART_H__