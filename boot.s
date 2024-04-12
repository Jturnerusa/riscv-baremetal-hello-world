        .section .text
        .global start

start:
        csrw satp, 0
        la sp, stack_top
        la gp, global_pointer
        la t0, kmain
        tail kmain
