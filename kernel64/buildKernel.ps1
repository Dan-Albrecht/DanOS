$ErrorActionPreference = 'Stop'
Push-Location ${PSScriptRoot}
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    $PSNativeCommandUseErrorActionPreference = $true
    Push-Location .\src\interupts
    try{
        dotnet run --project ..\..\codeGen\codeGen.csproj
    }
    finally{
        Pop-Location
    }

    cargo build --release

    # For now, always handy to have the assembly around
    rust-objdump.exe -M intel --disassemble-all .\target\x86_64-unknown-none\release\kernel64 > .\target\x86_64-unknown-none\release\kernel64.asm

    $allLines = [System.IO.File]::ReadAllLines("${PSScriptRoot}\target\x86_64-unknown-none\release\kernel64.asm")
    if ($allLines[3] -ne "Disassembly of section .text:") {
        Write-Error "Linking seems screwed up again. Text section isn't first. Found: $($allLines[3])"
    }

    # We might change the loader address, but we expect the symbol to be here
    if (!$allLines[5].EndsWith(" <DanMain>:")) {
        Write-Error "Linking seems screwed up again. DanMain wasn't at the start. Found: $($allLines[5])"
    }

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
