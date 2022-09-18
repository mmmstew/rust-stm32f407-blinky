/* memory.x - Linker script for the STM32F407VG */
/*MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 1M
  RAM : ORIGIN = 0x20000000, LENGTH = 192K
}*/
MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  /* TODO Adjust these memory regions to match your device memory layout */
  FLASH : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM : ORIGIN = 0x20000000, LENGTH = 112K
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);
