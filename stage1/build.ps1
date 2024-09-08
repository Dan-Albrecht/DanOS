<#
.SYNOPSIS
Builds the Stage1 bootloader. Stage1's sole job is to load the rest of the loaders to memory from disk and constrain itself to a single sector.

.PARAMETER sectorsToLoad
Numer of sector to load from disk. Reading will start the sector after this bootloader is read from.

.PARAMETER targetMemorySegment
The memory segment that data will be loaded. Data will be loaded at an offset of 0.

.PARAMETER handoffToSegment
After Stage1 finishes it will far jump to this segment at an offset of 0.
#>

param (
    [int]$sectorsToLoad = 1
    , [int]$targetMemorySegment = 0x800
    , [int]$handoffToSegment = 0x900
)   

$ErrorActionPreference = 'Stop'
Push-Location ${PSScriptRoot}
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    $PSNativeCommandUseErrorActionPreference = $true

    # There's a max we can load at a time, so we have to chunk it
    $maxSectorReadCount = 0x7F
    $fullBlocks = [System.Math]::Floor($sectorsToLoad / $maxSectorReadCount)
    $remainingSectors = $sectorsToLoad - ($fullBlocks * $maxSectorReadCount)

    Write-Host "Assembling Stage1 with 0x$(([int]$sectorsToLoad).ToString("X")) sector to load across 0x$(([int]$fullBlocks).ToString("X")) full (0x$(([int]$maxSectorReadCount).ToString("X"))) and 0x$(([int]$remainingSectors).ToString("X")) remaining, which will be loaded to segment 0x$(([int]$targetMemorySegment).ToString("X")), and we'll hand off to 0x$(([int]$handoffToSegment).ToString("X"))"
    nasm.exe .\bootloaderStage1.asm -Werror -DMAX_SECTOR_READ_COUNT="$maxSectorReadCount" -DFULL_SECTOR_BLOCKS="$fullBlocks" -DREMAINING_SECTORS="$remainingSectors" -DDISK_DATA_MEMORY_SEGMENT="$targetMemorySegment" -DSTAGE1_5_TARGET_MEMORY_SEGMENT="$handoffToSegment" -f bin -o .\bootloaderStage1.bin

    # Dissassemble to see what we actually got
    ndisasm.exe -o0x7c00 -b 16 .\bootloaderStage1.bin > .\bootloaderStage1.disasm.asm

    Write-Host "Finished assembling Stage1"
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
