$ErrorActionPreference = 'Stop'
Push-Location ${PSScriptRoot}
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    cargo build --release

    # For now, always handy to have the assembly around
    rust-objdump.exe -M intel --disassemble-all .\target\i686-unknown-none\release\kernel > .\target\i686-unknown-none\release\kernel.asm

    $allLines = [System.IO.File]::ReadAllLines("${PSScriptRoot}\target\i686-unknown-none\release\kernel.asm")
    if ($allLines[3] -ne "Disassembly of section .text:") {
        Write-Error "Linking seems screwed up again. Text section isn't first. Found: $($allLines[3])"
    }

    # We might change the loader address, but we expect the symbol to be here
    if (!$allLines[5].EndsWith(" <DanMain>:")) {
        Write-Error "Linking seems screwed up again. DanMain wasn't at the start. Found: $($allLines[5])"
    }

    # Turn it into the actual bits we'll run
    rust-objcopy.exe -O binary .\target\i686-unknown-none\release\kernel .\target\i686-unknown-none\release\kernel.bin

    # Re-disassemble above to get a sense we still have what we want
    # https://stackoverflow.com/a/58871420
    rust-objcopy.exe -I binary -O elf32-i386 --rename-section=.data=.text,code .\target\i686-unknown-none\release\kernel.bin .\target\i686-unknown-none\release\kernel.elf
    rust-objdump.exe -M intel -d .\target\i686-unknown-none\release\kernel.elf > .\target\i686-unknown-none\release\kernel.elf.asm
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
