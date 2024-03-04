$ErrorActionPreference = 'Stop'
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    $PSNativeCommandUseErrorActionPreference = $true
    .\iterateInternal.ps1

    qemu-system-x86_64.exe -machine type=q35,accel=whpx -drive file=.\DanOS.vhd,format=raw -S -gdb tcp::3333 -d cpu_reset -monitor stdio
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
}
