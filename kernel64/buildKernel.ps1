<#
.SYNOPSIS
Builds the 64 bit kernel

.PARAMETER debug
True to build debug, false to build release

.PARAMETER expectedOffset
Where the upstream caller of us expected to jump to. If after building we find they're wrong we'll fail so they can update.
#>

param (
    [bool]$debug = $true
    , [int]$expectedOffset = 0x1000
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

        # Make sure our code is at the offset the calling code will expect and our memory load target is expected
        # we'll ultimatley end up jumping to.
        #
        # WSL output will be somethin like:
        #
        # There are 16 section headers, starting at offset 0x2ae588:
        #
        #Section Headers:
        #[Nr] Name              Type            Address          Off    Size   ES Flg Lk Inf Al
        #[ 0]                   NULL            0000000000000000 000000 000000 00      0   0  0
        #[ 1] .text             PROGBITS        0000000000009100 001100 03b1be 00  AX  0   0 16
        # ...
        $textSection = wsl -- readelf -SW target/x86_64-unknown-none/$buildType/kernel64.strippedWithDebugLink | findstr .text

        $textOffset = $textSection.Split(' ', [StringSplitOptions]::RemoveEmptyEntries)[5]
        $textOffset = [System.Convert]::ToInt32($textOffset, 16)
        
        if ($textOffset -ne $expectedOffset) {
            # For reasons I haven't figured out yet, the elf header somtimes changes sizes. Until we can control that, detect it and then just have upstream
            # take into account the new jump target
            Write-Error ".text section moved. Expected offset: 0x$($expectedOffset.ToString("X")) Actual offset: 0x$($textOffset.ToString("X"))"
        }

        # Re-disassemble above to get a sense we still have what we want
        # https://stackoverflow.com/a/58871420
    } -message 'Kernel64 post-build'
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
