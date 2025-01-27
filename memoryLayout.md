# Memory Layout

## Start of Stage1 (16-bit Real mode)

BOIS loads the following and jumps to Stage1:

| Physical Address | Virtual Address | What |
| - | - | - |
| 0x7C00 | N/A | Start of Stage1
| 0x7DFF | N/A | End of Stage1 (last 72 bytes are MBR info)

## Start of Stage1.5 (16-bit Real mode)

Stage1 loads the following and jumps to Stage1.5:

| Physical Address | Virtual Address | What |
| - | - | - |
| 0x7C00 | N/A | Start of Stage1
| 0x7DFF | N/A | End of Stage1 (last 72 bytes are MBR info)
| 0x8000 | N/A | Start of Stage4 (Kernel64)
| Next sector after Stage4 end | N/A | Start of Stage3 (Kernel32)
| Next sector after Stag3 end | N/A | Start of Stage2
| Next sector after Stage2 end | N/A | Start of Stage1.5

## Start of Stage2 (32-bit Protected mode)

Stage1.5 enters 32-bit and identity maps:

| Physical Address | Virtual Address | What |
| - | - | - |
| 0x7C00 | 0x7C00 | Start of Stage1
| 0x7DFF | 0x7DFF | End of Stage1 (last 72 bytes are MBR info)
| 0x8000 | 0x8000 | Start of Stage4 (Kernel64)
| Next sector after Stage4 end | Same as physical | Start of Stage3 (Kernel32)
| Next sector after Stag3 end | Same as physical | Start of Stage2
| Next sector after Stage2 end | Same as physical | Start of Stage1.5

## Start of Stage3 (32-bit Protected mode)

Stage2 doesn't change anything and the map is the same as above. Stage3 doesn't need relocation as the build scripts can calculate where it'll end up getting loaded to (Stage4 address + Stage4's length).

## Start of Stage4 (64-bit Long mode)

Stage3 continue the identiy map and overall memory layout. It does relocate the Kernel64 code to wherver it ended up landing.