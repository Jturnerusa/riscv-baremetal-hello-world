        .section .text
        .global start

start:
        csrw satp, 0
        la sp, stack_top
        la gp, global_pointer


        la t5, bss_start
        la t6, bss_end
bss_clear_loop: 
        sd zero, (t5)
        addi t5, t5, 8
        bltu t5, t6, bss_clear_loop

        la t0, kmain
        tail kmain
