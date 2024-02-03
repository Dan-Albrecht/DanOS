$ErrorActionPreference = 'Stop'
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    $PSNativeCommandUseErrorActionPreference = $true
    .\iterateInternal.ps1
    # qemu-system-i386 -drive file=.\DanOS.vhd,format=raw -S -gdb tcp::3333 -d cpu_reset
    qemu-system-x86_64 -drive file=.\DanOS.vhd,format=raw -S -gdb tcp::3333 -d cpu_reset
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
}
