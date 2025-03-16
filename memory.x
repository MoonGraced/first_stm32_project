MEMORY
{
    /* ОЗУ (RAM) */
    RAM (xrw) : ORIGIN = 0x20000000, LENGTH = 128K

    /* Flash-память */
    FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 512K
}