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

    if ((-not (Test-Path ./src/interupts/setup.rs)) -or 
        (-not (Test-Path ./src/interupts/table.rs)) -or 
        ((Get-Item ./src/interupts/setup.rs).LastWriteTime -lt (Get-Item ./codeGen/Program.cs).LastWriteTime) -or 
        ((Get-Item ./src/interupts/table.rs).LastWriteTime -lt (Get-Item ./codeGen/Program.cs).LastWriteTime)) {

        Write-Host 'Running codeGen...'
        
        Push-Location ./src/interupts
        try {
            dotnet run --project ../../codeGen/codeGen.csproj

            # For reasons I don't understand, PSNativeCommandUseErrorActionPreference doesn't seem to be working here
            if ($LastExitCode -ne 0) {
                Write-Error 'Codegen failed'
            }
        }
        finally {
            Pop-Location
        }
    }
    else {
        Write-Host 'Skipping codeGen...'
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

    # Disassemble this so we can have a reference for debugging. This will reflect our offsets.
    objdump -M intel --disassemble  ./target/x86_64-unknown-none/$buildType/kernel64 > ./target/x86_64-unknown-none/$buildType/kernel64.asm
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
