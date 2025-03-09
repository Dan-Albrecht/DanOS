<#
.SYNOPSIS
Builds the Stage1 bootloader. Stage1's sole job is to load the rest of the loaders to memory from disk and constrain itself to a single sector.

.PARAMETER sectorsToLoad
Numer of sector to load from disk. Reading will start the sector after this bootloader is read from.

.PARAMETER addressToLoadTo
The disk data will be loaded to this address (CS=0) and execution will be handed off to it.

#>

param (
    [int]$sectorsToLoad = 1
    , [int]$addressToLoadTo = 0x8000
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

    Write-Host "Assembling Stage1 with 0x$(([int]$sectorsToLoad).ToString("X")) sector(s) to load across 0x$(([int]$fullBlocks).ToString("X")) full (0x$(([int]$maxSectorReadCount).ToString("X"))) and 0x$(([int]$remainingSectors).ToString("X")) remaining, which will be loaded to and executed from address 0x$(([int]$addressToLoadTo).ToString("X"))"
    nasm ./bootloaderStage1.asm -Werror -DMAX_SECTOR_READ_COUNT="$maxSectorReadCount" -DFULL_SECTOR_BLOCKS="$fullBlocks" -DREMAINING_SECTORS="$remainingSectors" -DSTAGE2_ADDRESS="$addressToLoadTo" -f bin -o ./bootloaderStage1.bin

    # Dissassemble to see what we actually got
    ndisasm -o0x7c00 -b 16 ./bootloaderStage1.bin > ./bootloaderStage1.disasm.asm

    Write-Host "Finished assembling Stage1"
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
