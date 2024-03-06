$ErrorActionPreference = 'Stop'
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    $PSNativeCommandUseErrorActionPreference = $true
    .\build\iterateInternal.ps1

    # Min size bug: https://stackoverflow.com/a/68750259 so cannot use .bin file
    qemu-system-x86_64.exe -machine type=q35,accel=whpx -drive file=.\build\DanOS.vhd,format=raw -monitor stdio
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
}
