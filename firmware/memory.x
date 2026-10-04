/* STM32WL33CC: 256K flash, 32K SRAM. */
MEMORY
{
  FLASH : ORIGIN = 0x10040000, LENGTH = 256K
  /* Bootloader RAM_VR at 0x20000004 + crash info at 0x20000034 (ST .ld); skip 256 B. */
  RAM   : ORIGIN = 0x20000100, LENGTH = 32K - 0x100
}
