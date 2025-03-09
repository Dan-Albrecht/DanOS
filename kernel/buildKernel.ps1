<#
.SYNOPSIS
Builds the 32 bit kernel

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
    } -message 'Kernel32 build'

    # Having the assembly to look at when debuggin in Bochs is quite handy
    objdump -M intel --disassemble ./target/i686-unknown-none/$buildType/kernel > ./target/i686-unknown-none/$buildType/kernel.asm

    # Stage2 requires, for now, a staticaly linked kernel. Verify it is where Stage2 will expect it to be.
    $danMainAddress = readelf --symbols ./target/i686-unknown-none/$buildType/kernel | grep DanMain | awk '{print $2}' | sed 's/^0*//'

    if ($danMainAddress -ne "100000") {
        Write-Error "DanMain was not found at the expected address. Found: $danMainAddress"
    }

    # Translate to flat binary so Stage2 can just jump to it.
    objcopy -O binary ./target/i686-unknown-none/$buildType/kernel ./target/i686-unknown-none/$buildType/kernel.bin
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
