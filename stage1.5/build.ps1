<#
.SYNOPSIS
Builds the Stage1.5 bootloader. Stage1.5's job is to get us into 32 bit mode.

.PARAMETER origin
The origin of this code.

.PARAMETER memoryMapTarget
The physical memory location to copy the memory map to.

.PARAMETER stage2Address
Location of the Stage2 loader that we'll perform a short jump to.
#>

param (
    [int]$origin = 0xA00
    , [int]$memoryMapTarget = 0x800
    , [int]$stage2Address = 0x900
    , [int]$gdtAddress = 0x500
)   

$ErrorActionPreference = 'Stop'
Push-Location ${PSScriptRoot}
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    $PSNativeCommandUseErrorActionPreference = $true
    
    Write-Host "Assembling Stage1.5 with origin 0x$(([int]$origin).ToString("X")), will find Stage2 at 0x$(([int]$stage2Address).ToString("X")), and will copy GDT to 0x$(([int]$gdtAddress).ToString("X"))"
    nasm.exe .\bootloaderStage1_5.asm -DSTAGE_1_5_LOAD_TARGET="$origin" -DMEMORY_MAP_TARGET="$memoryMapTarget" -DSTAGE_2_JUMP_TARGET="$stage2Address" -DGDT_ADDRESS="$gdtAddress" -f bin -o .\bootloaderStage1_5.bin

    # Dissassemble to see what we actually got
    ndisasm.exe "-o$origin" -b 16 .\bootloaderStage1_5.bin > .\bootloaderStage1_5.disasm.asm

    Write-Host "Finished assembling Stage 1.5"
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
