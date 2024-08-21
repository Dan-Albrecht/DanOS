$ErrorActionPreference = 'Stop'
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    $PSNativeCommandUseErrorActionPreference = $true
    .\build\iterateInternal.ps1

    qemu-system-x86_64.exe -machine type=q35 -drive file=.\build\DanOS.vhd,format=raw -S -gdb tcp::3333 -d cpu_reset -monitor stdio
    # In WSL, find host IP
    # ip route show | grep -i default | awk '{ print $3}'
    # In GDB connect: target remote XXX:3333
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
}
