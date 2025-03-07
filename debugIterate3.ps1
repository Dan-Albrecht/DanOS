$ErrorActionPreference = 'Stop'
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    $PSNativeCommandUseErrorActionPreference = $true
    .\build\iterateInternal.ps1

    qemu-system-i386.exe -machine type=q35 -drive id=disk,file=.\build\DanOS.img,format=raw,if=none -device ahci,id=ahci -device ide-hd,drive=disk,bus=ahci.0 -monitor stdio -S -gdb tcp::3333 -d cpu_reset
    # In WSL, find host IP
    # ip route show | grep -i default | awk '{ print $3}'
    # In GDB connect: target remote XXX:3333
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
}
