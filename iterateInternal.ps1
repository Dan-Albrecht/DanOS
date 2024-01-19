$ErrorActionPreference = 'Stop'
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
try {
    $PSNativeCommandUseErrorActionPreference = $true
    nasm.exe .\bootloaderStage1.asm -f bin -o .\bootloaderStage1.bin
    nasm.exe .\bootloaderStage2.asm -f bin -o .\bootloaderStage2.bin
    .\kernel\buildKernel.ps1
    .\kernel64\buildKernel.ps1

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
    $kernelSectors = [Math]::Ceiling($kernelBytes.Length / 512)
    $kernel64Bytes = Get-Content .\kernel64\target\x86_64-unknown-none\release\kernel64.bin -Raw -AsByteStream
    $kernel64Sectors = [Math]::Ceiling($kernel64Bytes.Length / 512)

    # Do this in sector count so obvious what we have to update the loader to.
    $neededSectors = $kernelSectors + $kernel64Sectors
    Write-Host "Kernel32 is $($kernelBytes.Length) bytes and $kernelSectors sectors. Kernel64 is $($kernel64Bytes.Length) bytes and $kernel64Sectors sectors. So we need a total of $neededSectors sectors loaded from disk for kernels."
    if ($neededSectors -gt 0x15 ) { Write-Error "Kernel has grown again, update the loader. Need $neededSectors sector for kernel." }

    $osBytes = Get-Content .\DanOS.vhd -Raw -AsByteStream
    for ($x = 0; $x -lt $stage1Bytes.Length; $x++ ) { $osBytes[$x] = $stage1Bytes[$x] }
    for ($x = 0; $x -lt $stage2Bytes.Length; $x++ ) { $osBytes[$x + 512] = $stage2Bytes[$x] }
    for ($x = 0; $x -lt $kernelBytes.Length; $x++ ) { $osBytes[$x + 1024] = $kernelBytes[$x] }
    for ($x = 0; $x -lt $kernel64Bytes.Length; $x++ ) { $osBytes[$x + 1024 + ($kernelSectors * 512)] = $kernel64Bytes[$x] }

    Write-Host "Writing $($osBytes.Length) bytes"
    [System.IO.File]::WriteAllBytes("${PSScriptRoot}\DanOS.vhd", $osBytes)

    Write-Host "VHD ready"
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
}
