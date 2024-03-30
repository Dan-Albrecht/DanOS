$ErrorActionPreference = 'Stop'
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    $PSNativeCommandUseErrorActionPreference = $true
    .\build\iterateInternal.ps1

    # Min size bug: https://stackoverflow.com/a/68750259 so cannot use .bin file
    # QEMU will crash if we do a divide by zero and have accel=whpx
    qemu-system-x86_64.exe -machine type=q35 -drive id=disk,file=.\build\DanOS.vhd,format=raw,if=none -device ahci,id=ahci -device ide-hd,drive=disk,bus=ahci.0 -monitor stdio
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
}
