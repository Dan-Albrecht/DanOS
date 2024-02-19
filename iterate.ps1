$ErrorActionPreference = 'Stop'
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    $PSNativeCommandUseErrorActionPreference = $true
    .\iterateInternal.ps1

    qemu-system-x86_64.exe  -drive file=.\DanOS.bin,format=raw
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
}
