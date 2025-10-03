use super::context_switch::isr_switch_thread_context;
use super::exceptions::*;
use super::idt::Idt;
use super::ipis::isr_interprocessor_interrupt;
use crate::isa::init::gdt;

pub fn register_fixed_isr_gates(idt: &mut Idt) {
    /* CPU Exceptions */
    idt.set_gate(0, isr_divide_by_zero, gdt::KERNEL_CODE_SELECTOR, true, true);
    idt.set_gate(1, isr_debug, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(2, isr_non_maskable_interrupt, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(3, isr_breakpoint, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(4, isr_overflow, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(5, isr_bound_range_exceeded, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(6, isr_invalid_opcode, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(7, isr_device_not_available, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(8, isr_double_fault, gdt::KERNEL_CODE_SELECTOR, true, true);
    idt.set_gate(10, isr_invalid_tss, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(11, isr_segment_not_present, gdt::KERNEL_CODE_SELECTOR, true, true);
    idt.set_gate(12, isr_stack_segment_fault, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(13, isr_general_protection_fault, gdt::KERNEL_CODE_SELECTOR, true, true);
    idt.set_gate(14, isr_page_fault, gdt::KERNEL_CODE_SELECTOR, true, true);
    idt.set_gate(16, isr_x87_floating_point, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(17, isr_alignment_check, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(18, isr_machine_check, gdt::KERNEL_CODE_SELECTOR, true, true);
    idt.set_gate(19, isr_simd_floating_point, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(20, isr_virtualization, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(21, isr_control_protection, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(28, isr_hypervisor_injection, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(29, isr_vmm_communication, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(30, isr_security_exception, gdt::KERNEL_CODE_SELECTOR, true, false);
    /* Kernel Defined Fixed Interrupts */
    idt.set_gate(32, isr_switch_thread_context, gdt::KERNEL_CODE_SELECTOR, false, true);
    idt.set_gate(33, isr_interprocessor_interrupt, gdt::KERNEL_CODE_SELECTOR, false, true);
}
