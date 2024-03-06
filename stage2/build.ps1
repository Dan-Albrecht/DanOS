<#
.SYNOPSIS
Builds the Stage2 bootloader.

.PARAMETER origin
The origin of this code.

.PARAMETER kernel32Address
Location of the 32-bit kernel that we'll perform a short jump to.
#>

param (
    [int]$origin = 0xB00
    , [int]$kernel32Address  = 0x900
)   

$ErrorActionPreference = 'Stop'
Push-Location ${PSScriptRoot}
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    $PSNativeCommandUseErrorActionPreference = $true

    Write-Host "Assembling Stage2 with origin 0x$(([int]$origin).ToString("X")), will find Kernel32 at 0x$(([int]$kernel32Address).ToString("X"))"
    nasm.exe .\bootloaderStage2.asm -DSTAGE_2_LOAD_TARGET="$origin" -DKERNEL32_JUMP_TARGET="$kernel32Address" -f bin -o .\bootloaderStage2.bin

    # Dissassemble to see what we actually got
    ndisasm.exe "-o$origin" -b 32 .\bootloaderStage2.bin > .\bootloaderStage2.disasm.asm

    Write-Host "Finished assembling Stage2"
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
