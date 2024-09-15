<#
.SYNOPSIS
Builds the 64 bit kernel

.PARAMETER debug
True to build debug, false to build release
#>

param (
    [bool]$debug = $true
)   

$ErrorActionPreference = 'Stop'
Push-Location ${PSScriptRoot}
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    $PSNativeCommandUseErrorActionPreference = $true
    Push-Location .\src\interupts
    try {
        dotnet run --project ..\..\codeGen\codeGen.csproj
    }
    finally {
        Pop-Location
    }

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
    } -message 'Kernel64 build'

    TimeCommand {
        # Call this an elf as that's what it is
        Copy-Item -Path .\target\x86_64-unknown-none\$buildType\kernel64 -Destination .\target\x86_64-unknown-none\$buildType\kernel64.elf -Force

        # Create our final target binary that'll link to debug file, but not contain extra junk
        rust-objcopy.exe --only-keep-debug .\target\x86_64-unknown-none\$buildType\kernel64.elf .\target\x86_64-unknown-none\$buildType\kernel64.dbg
        rust-objcopy.exe --strip-debug .\target\x86_64-unknown-none\$buildType\kernel64.elf .\target\x86_64-unknown-none\$buildType\kernel64.stripped
        Copy-Item .\target\x86_64-unknown-none\$buildType\kernel64.stripped .\target\x86_64-unknown-none\$buildType\kernel64.strippedWithDebugLink -Force
        rust-objcopy.exe --add-gnu-debuglink=.\target\x86_64-unknown-none\$buildType\kernel64.dbg .\target\x86_64-unknown-none\$buildType\kernel64.strippedWithDebugLink

        # Disassemble this so we can have a reference for debugging. This will reflect our offsets.
        rust-objdump.exe -M intel --disassemble  .\target\x86_64-unknown-none\$buildType\kernel64.strippedWithDebugLink > .\target\x86_64-unknown-none\$buildType\kernel64.strippedWithDebugLink.asm

        # Re-disassemble above to get a sense we still have what we want
        # https://stackoverflow.com/a/58871420
    } -message 'Kernel64 post-build'
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
