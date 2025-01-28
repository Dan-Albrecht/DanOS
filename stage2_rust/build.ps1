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

    # Get the image of what we'll use
    rust-objcopy.exe --input-target=elf32-i386 -O binary .\target\i386-unknown-none\$buildType\stage2_rust .\target\i386-unknown-none\$buildType\stage2_rust.bin
    
    # See what we actually got
    rust-objdump.exe -d .\target\i386-unknown-none\$buildType\stage2_rust > .\target\i386-unknown-none\$buildType\stage2_rust.asm

    # Alt from the flat binary
    ndisasm.exe "-o$loadTarget" -b 16 .\target\i386-unknown-none\$buildType\stage2_rust.bin > .\target\i386-unknown-none\$buildType\stage2_rust.bin.asm

}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
