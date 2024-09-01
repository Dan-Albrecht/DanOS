<#
.SYNOPSIS
Builds the 64 bit kernel

.PARAMETER debug
True to build debug, false to build release
#>

param (
    [bool]$debug = $false
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
        cargo build
    }
    else {
        $buildType = "release"
        cargo build --release
    }

    # Call this an elf as that's what it is
    Copy-Item -Path .\target\x86_64-unknown-none\$buildType\kernel64 -Destination .\target\x86_64-unknown-none\$buildType\kernel64.elf -Force

    # For now, always handy to have the assembly around
    rust-objdump.exe -M intel --disassemble-all .\target\x86_64-unknown-none\$buildType\kernel64.elf > .\target\x86_64-unknown-none\$buildType\kernel64.elf.asm

    $allLines = [System.IO.File]::ReadAllLines("${PSScriptRoot}\target\x86_64-unknown-none\$buildType\kernel64.elf.asm")
    if ($allLines[3] -ne "Disassembly of section .text:") {
        # Our custom linking script is suposed to put this first. Might not need this for much longer as we're finding better ways to load and jump to the .text section.
        Write-Error "Linking seems screwed up again. Text section isn't first. Found: $($allLines[3])"
    }

    # We might change the loader address, but we expect the symbol to be here
    if (!$allLines[5].EndsWith(" <DanMain>:")) {
        Write-Error "Linking seems screwed up again. DanMain wasn't at the start. Found: $($allLines[5])"
    }

    rust-objcopy.exe --only-keep-debug .\target\x86_64-unknown-none\$buildType\kernel64.elf .\target\x86_64-unknown-none\$buildType\kernel64.dbg
    rust-objcopy.exe --strip-debug .\target\x86_64-unknown-none\$buildType\kernel64.elf .\target\x86_64-unknown-none\$buildType\kernel64.stripped
    Copy-Item .\target\x86_64-unknown-none\$buildType\kernel64.stripped .\target\x86_64-unknown-none\$buildType\kernel64.strippedWithDebugLink -Force
    rust-objcopy.exe --add-gnu-debuglink=.\target\x86_64-unknown-none\$buildType\kernel64.dbg .\target\x86_64-unknown-none\$buildType\kernel64.strippedWithDebugLink
    rust-objdump.exe -M intel -d .\target\x86_64-unknown-none\$buildType\kernel64.strippedWithDebugLink > .\target\x86_64-unknown-none\$buildType\kernel64.strippedWithDebugLink.asm

    # Make sure memory location is what previous stage expects it to be
    $codeLine = rust-objdump.exe --headers .\target\x86_64-unknown-none\$buildType\kernel64.strippedWithDebugLink | findstr .text
    $vma = $codeLine.Split(' ', [StringSplitOptions]::RemoveEmptyEntries)[3]
    $loadAddress = "0x" + [System.Convert]::ToInt32("0x$vma", 16).ToString("X")
    $expectedLoadAddress = $env:KERNEL64_LOAD_TARGET

    Write-Host "We requested load at $expectedLoadAddress it is $loadAddress"

    if ($loadAddress -ne $expectedLoadAddress) {
        Write-Error "And that doesn't match"
    }

    # Display sections and size
    # size -Ax kernel64.unstripped
    # or even better
    # readelf -SW kernel64.unstripped

    # Dump section
    # readelf -p .gnu_debuglink kernel64.unstripped

    # Add symbol
    # target symbols add target/x86_64-unknown-none/release/kernel64.dbg

    # Re-disassemble above to get a sense we still have what we want
    # https://stackoverflow.com/a/58871420
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
