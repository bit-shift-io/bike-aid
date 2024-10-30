MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* NRF52840 with Softdevice S140 7.0.1 */
  /* in nrf52840 RAM is 256k, so 0x2000_0000..0x2004_0000 */
  
  FLASH : ORIGIN = 0x00027000, LENGTH = 868K
  RAM : ORIGIN = 0x20005490, LENGTH = 0x3a770
  PANDUMP: ORIGIN = 0x2003fc00, LENGTH = 1K
}

_panic_dump_start = ORIGIN(PANDUMP);
_panic_dump_end   = ORIGIN(PANDUMP) + LENGTH(PANDUMP);