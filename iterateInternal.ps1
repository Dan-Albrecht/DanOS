$ErrorActionPreference = 'Stop'
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    $PSNativeCommandUseErrorActionPreference = $true
    nasm.exe .\bootloaderStage1.asm -f bin -o .\bootloaderStage1.bin
    nasm.exe .\bootloaderStage2.asm -f bin -o .\bootloaderStage2.bin
    .\kernel\buildKernel.ps1

    # Really hate PowerShell sometimes
    if (![System.IO.File]::Exists("${PSScriptRoot}\empty.vhd")) {
        # Creation is too slow, so just cache an empty one and use it
        New-VHD -Path empty.vhd -Fixed -SizeBytes 3MB
    }

    Copy-Item -Force .\empty.vhd .\DanOS.vhd

    $stage1Bytes = Get-Content .\bootloaderStage1.bin -Raw -AsByteStream
    if ($stage1Bytes.Length -ne 512 ) { Write-Error 'Bootloader should be exactly 512 bytes' }

    $stage2Bytes = Get-Content .\bootloaderStage2.bin -Raw -AsByteStream

    # This doesn't currenty pad, we're relying on the VHD to be 0'd
    $kernelBytes = Get-Content .\kernel\target\i686-unknown-none\release\kernel.bin -Raw -AsByteStream
    # Do this in sector count so obvious what we have to update the loader to
    if ($kernelBytes.Length -gt (0x13 * 0x200) ) { Write-Error 'Kernel has grown again, update the loader' }

    $osBytes = Get-Content .\DanOS.vhd -Raw -AsByteStream
    for ($x = 0; $x -lt $stage1Bytes.Length; $x++ ) { $osBytes[$x] = $stage1Bytes[$x] }
    for ($x = 0; $x -lt $stage2Bytes.Length; $x++ ) { $osBytes[$x + 512] = $stage2Bytes[$x] }
    for ($x = 0; $x -lt $kernelBytes.Length; $x++ ) { $osBytes[$x + 1024] = $kernelBytes[$x] }

    Write-Host "Writing $($osBytes.Length) bytes"
    [System.IO.File]::WriteAllBytes("${PSScriptRoot}\DanOS.vhd", $osBytes)

    Write-Host "VHD ready"
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
}
