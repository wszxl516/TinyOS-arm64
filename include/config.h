#ifndef __CONFIG_H__
#define __CONFIG_H__
#include "stdtypes.h"
typedef void (ld_script_pointer_t)();

//uart
#define UART_REGISTER_ADDR               (0x09000000LLU)
#define GIC_BASE_ADDR                    (0x8000000LLU)
//
#define SMP_CORE_COUNT                   (2)
//4MB
#define PAGE_FRAME_SIZE                  (0x100000 * 4)
//ld script section address
extern ld_script_pointer_t stack_top, stack_bottom;
extern ld_script_pointer_t bss_start, bss_end;
extern ld_script_pointer_t heap_start, text_start;
extern ld_script_pointer_t device_start, device_end;
extern ld_script_pointer_t trap_vector, page_table_start;
extern ld_script_pointer_t frame_start, frame_end;
extern ld_script_pointer_t user_stack_top, user_stack_bottom;
#endif //__CONFIG_H__
