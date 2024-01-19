$ErrorActionPreference = 'Stop'
Push-Location ${PSScriptRoot}
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    cargo build --release

    # For now, always handy to have the assembly around
    rust-objdump.exe -M intel --disassemble-all .\target\x86_64-unknown-none\release\kernel64 > .\target\x86_64-unknown-none\release\kernel64.asm

    # Turn it into the actual bits we'll run
    rust-objcopy.exe -O binary .\target\x86_64-unknown-none\release\kernel64 .\target\x86_64-unknown-none\release\kernel64.bin

    # Re-disassemble above to get a sense we still have what we want
    # https://stackoverflow.com/a/58871420
    rust-objcopy.exe -I binary -O elf64-x86-64 --rename-section=.data=.text,code .\target\x86_64-unknown-none\release\kernel64.bin .\target\x86_64-unknown-none\release\kernel64.elf
    rust-objdump.exe -M intel -d .\target\x86_64-unknown-none\release\kernel64.elf > .\target\x86_64-unknown-none\release\kernel64.elf.asm
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
