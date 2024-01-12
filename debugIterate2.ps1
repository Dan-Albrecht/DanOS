$ErrorActionPreference = 'Stop'
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    $PSNativeCommandUseErrorActionPreference = $true
    .\iterateInternal.ps1
    & 'C:\Program Files\Bochs-2.7\bochsdbg.exe' -f .\bochsrc.bxrc
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
}
