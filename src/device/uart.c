#include "uart.h"
STATIC_INIT_SPIN_LOCK(SECTION("device") UART_LOCK);
static Pl011Uart SECTION("device") *UART = (Pl011Uart*)UART_REGISTER_ADDR;

char getc(){
    while (!RX_READY());
    char c = 0;
    spin_lock(&UART_LOCK);
    c = UART->data;
    spin_unlock(&UART_LOCK);
    return c;
}

void putc(char c) {
    while (!TX_READY());
    spin_lock(&UART_LOCK);
        UART->data = c;
    spin_unlock(&UART_LOCK);
}

void init_uart()
{
    UART->receive_status = 0;
    UART->flag = 0x90;
    UART->lp_counter = 0x00;
    //115200
    UART->baud_rate[0] = 0x1c200;
    UART->baud_rate[1] = 0x0;
    UART->line_control = 0x300;
}


void puts(char *buffer) {
    while (!TX_READY());
    spin_lock(&UART_LOCK);
    while (*buffer)
    {
        UART->data = *buffer;
        buffer ++;
    }
    spin_unlock(&UART_LOCK);
}

u32 gets(char *buffer, u32 size){
    u32 i = 0;
    memset(buffer, 0, size);
    for (; i < size; i++)
    {   
        char ch = getc();
        if ('\n' == ch || '\r' == ch)
        {
            buffer[i] = 0;
            puts("\n\r");
            break;
        }
        buffer[i] = ch;
        putc(buffer[i]);
    }
    return i;
}