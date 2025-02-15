<#
.SYNOPSIS
Builds the 16-bit Stage2 boot loader

.PARAMETER debug
True to build debug, false to build release
#>

param (
    [bool]$debug = $true
    , [string]$loadTarget = "0x7E00"
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

    # We're building an elf file; so call it that
    Copy-Item -Path .\target\i386-unknown-none\$buildType\stage2_rust -Destination .\target\i386-unknown-none\$buildType\stage2_rust.elf -Force

    # Get a flat binary we can throw right into memory and jump to
    rust-objcopy.exe --input-target=elf32-i386 -O binary .\target\i386-unknown-none\$buildType\stage2_rust.elf .\target\i386-unknown-none\$buildType\stage2_rust.bin

    # Disassemble so we can see what we got. The real Linux tools are better than the rust-objdump one.
    wsl -- objdump -D -m i8086 -M intel -j .text ./target/i386-unknown-none/$buildType/stage2_rust.elf > ./target/i386-unknown-none/$buildType/stage2_rust.elf.asm
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
