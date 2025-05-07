<#
.SYNOPSIS
Builds the 16-bit Stage2 boot loader

.PARAMETER debug
True to build debug, false to build release
#>

param (
    [bool]$debug = $true
    , [string]$loadTarget = "0x7E01"
)   

$ErrorActionPreference = 'Stop'
Push-Location ${PSScriptRoot}
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference

try {
    $PSNativeCommandUseErrorActionPreference = $true
    $env:STAGE2_RUST_LOAD_TARGET = $loadTarget

    if ($debug) {
        $buildType = "debug"
    }
    else {
        $buildType = "release"
    }

    TimeCommand {

        if ($debug) {
            cargo build
        }
        else {
            cargo build --release
        }
    } -message 'Stage2 build'

    TimeCommand {
        # Get a flat binary we can throw right into memory and jump to
        objcopy --input-target=elf32-i386 -O binary ./target/i386-unknown-none/$buildType/stage2_rust ./target/i386-unknown-none/$buildType/stage2_rust.bin
    } -message 'Stage2 flatten'

    TimeCommand {
        # Disassemble so we can see what we got
        objdump -D -m i8086 -M intel -j .text ./target/i386-unknown-none/$buildType/stage2_rust > ./target/i386-unknown-none/$buildType/stage2_rust.asm
    } -message 'Stage2 disassemble'
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
