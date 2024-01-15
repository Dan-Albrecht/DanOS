$ErrorActionPreference = 'Stop'
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    $PSNativeCommandUseErrorActionPreference = $true
    .\iterateInternal.ps1

    # qemu-system-i386 -drive file=.\DanOS.vhd,format=raw
    qemu-system-x86_64.exe  -drive file=.\DanOS.vhd,format=raw
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
}
