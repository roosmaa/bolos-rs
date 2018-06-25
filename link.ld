MEMORY
{
  DISCARD (rwx) : ORIGIN = 0xd0000000, LENGTH = 1M

  FLASH   (rx)  : ORIGIN = 0xc0d00000, LENGTH = 400K
  SRAM    (rwx) : ORIGIN = 0x20001800, LENGTH = 4K
}

PAGE_SIZE  = 64;
STACK_SIZE = 768;
END_STACK  = ORIGIN(SRAM) + LENGTH(SRAM);

SECTIONS
{
  ENTRY(bolos_main)

  /****************************************************************/
  /* This section locates the code in FLASH                       */
  /****************************************************************/

  /** put text in Flash memory, VMA will be equal to LMA */
  .text :
  {
    /* provide start code symbol, shall be zero */
    _text = .;
    _nvram = .;

    /* ensure the entry point is always at the start */
    *(.boot*)

    *(.text*)
    *(.rodata)

    . = ALIGN(4);

    /* all code placed */
    _etext = .;

    . = ALIGN(PAGE_SIZE);
    _envram = .;
  } > FLASH = 0x00

  .data (NOLOAD):
  {
    . = ALIGN(4);

    /**
     * Place RAM initialized variables
     */
    _data = .;

    *(vtable)
    *(.data*)

    _edata = .;

  } > DISCARD /*> SRAM AT>FLASH = 0x00 */

  .bss :
  {
    /**
     * Place RAM uninitialized variables
     */
    _bss = .;
    *(.bss*)
    _ebss = .;

    /**
     * Reserve stack size
     */
    . = ALIGN(4);
    _stack = .;
    . = _stack + STACK_SIZE;
    PROVIDE(_stack_size = STACK_SIZE);
    PROVIDE(_estack = ABSOLUTE(END_STACK));
  } > SRAM = 0x00
}
