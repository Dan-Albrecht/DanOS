$ErrorActionPreference = 'Stop'
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    $PSNativeCommandUseErrorActionPreference = $true
    ./build/iterateInternal.ps1
    & '/mnt/c/Program Files/Bochs-2.7/bochsdbg.exe' -f ./bochsrc.bxrc
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
}
