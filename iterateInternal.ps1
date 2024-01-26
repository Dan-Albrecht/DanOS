$ErrorActionPreference = 'Stop'
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
Push-Location ${PSScriptRoot}
try {
    $PSNativeCommandUseErrorActionPreference = $true

    # I really hate you PowerShell
    [System.Environment]::CurrentDirectory = ${PSScriptRoot}

    $STAGE_2_LOAD_TARGET = 0xE000
    
    nasm.exe .\bootloaderStage2.asm -DSTAGE_2_LOAD_TARGET="$STAGE_2_LOAD_TARGET" -f bin -o .\bootloaderStage2.bin
    $stage2Bytes = Get-Content .\bootloaderStage2.bin -Raw -AsByteStream
    $stage2Sectors = [Math]::Ceiling($stage2Bytes.Length / 512)

    $STAGE_3_LOAD_TARGET = $STAGE_2_LOAD_TARGET + ($stage2Sectors * 512)

    # Secret handshake to eventaully get this passed to the linker
    $env:KERNEL32_LOAD_TARGET = "0x$(([int]$STAGE_3_LOAD_TARGET).ToString("X"))"
    .\kernel\buildKernel.ps1
    $kernelBytes = Get-Content .\kernel\target\i686-unknown-none\release\kernel.bin -Raw -AsByteStream
    $kernelSectors = [Math]::Ceiling($kernelBytes.Length / 512)

    $STAGE_4_LOAD_TARGET = $STAGE_3_LOAD_TARGET + ($kernelSectors * 512)

    # Secret handshake to eventaully get this passed to the linker
    $env:KERNEL64_LOAD_TARGET = "0x$(([int]$STAGE_4_LOAD_TARGET).ToString("X"))"
    .\kernel64\buildKernel.ps1
    $kernel64Bytes = Get-Content .\kernel64\target\x86_64-unknown-none\release\kernel64.bin -Raw -AsByteStream
    $kernel64Sectors = [Math]::Ceiling($kernel64Bytes.Length / 512)

    $STAGE_1_5_LOAD_TARGET = $STAGE_4_LOAD_TARGET + ($kernel64Sectors * 512)
    nasm.exe .\bootloaderStage1_5.asm -DSTAGE_1_5_LOAD_TARGET="$STAGE_1_5_LOAD_TARGET" -f bin -o .\bootloaderStage1_5.bin
    $stage1_5Bytes = Get-Content .\bootloaderStage1_5.bin -Raw -AsByteStream
    $stage1_5Sectors = [Math]::Ceiling($stage1_5Bytes.Length / 512)
    $stage1_5Segment = $STAGE_1_5_LOAD_TARGET -shr 4

    $neededSectors = $stage2Sectors + $kernelSectors + $kernel64Sectors + $stage1_5Sectors
    # This needs to be in segment so divide by 16
    $stage2Segment = $STAGE_2_LOAD_TARGET -shr 4
    Write-Host "Stage 1 @ 0x7C00 (must be 1 sector), Stage 1.5 @ 0x$(([int]$STAGE_1_5_LOAD_TARGET).ToString("X")) (for 0x$(([int]$stage1_5Sectors).ToString("X")) sectors), Stage 2 @ 0x$(([int]$STAGE_2_LOAD_TARGET).ToString("X")) (which is segment 0x$(([int]$stage2Segment).ToString("X"))) (for 0x$(([int]$stage2Sectors).ToString("X")) sectors), Stage 3 @ 0x$(([int]$STAGE_3_LOAD_TARGET).ToString("X")) (for 0x$(([int]$kernelSectors).ToString("X")) sectors), Stage 4 @ 0x$(([int]$STAGE_4_LOAD_TARGET).ToString("X")) (for 0x$(([int]$kernel64Sectors).ToString("X")) sectors). For a total of 0x$(([int]$neededSectors).ToString("X")) sectors to load from disk."

    nasm.exe .\bootloaderStage1.asm -DSTAGE2_LENGTH_SECTORS="$neededSectors" -DSTAGE1_5_TARGET_MEMORY_SEGMENT="$stage1_5Segment" -DSTAGE2_TARGET_MEMORY_SEGMENT="$stage2Segment" -f bin -o .\bootloaderStage1.bin
    $stage1Bytes = Get-Content .\bootloaderStage1.bin -Raw -AsByteStream
    if ($stage1Bytes.Length -ne 512 ) { Write-Error 'Bootloader should be exactly 512 bytes' }
    $stage1Sectors = 1 # Must be per above and the laws of bios

    if (![System.IO.File]::Exists("empty.vhd")) {
        # Creation is too slow, so just cache an empty one and use it
        New-VHD -Path empty.vhd -Fixed -SizeBytes 3MB
    }

    Copy-Item -Force .\empty.vhd .\DanOS.vhd

    $osBytes = Get-Content .\DanOS.vhd -Raw -AsByteStream
    for ($x = 0; $x -lt $stage1Bytes.Length; $x++ ) { $osBytes[$x] = $stage1Bytes[$x] }
    for ($x = 0; $x -lt $stage2Bytes.Length; $x++ ) { $osBytes[$x + (($stage1Sectors) * 512)] = $stage2Bytes[$x] }
    for ($x = 0; $x -lt $kernelBytes.Length; $x++ ) { $osBytes[$x + (($stage1Sectors + $stage2Sectors) * 512)] = $kernelBytes[$x] }
    for ($x = 0; $x -lt $kernel64Bytes.Length; $x++ ) { $osBytes[$x + (($stage1Sectors + $stage2Sectors + $kernelSectors) * 512)] = $kernel64Bytes[$x] }
    for ($x = 0; $x -lt $stage1_5Bytes.Length; $x++ ) { $osBytes[$x + (($stage1Sectors + $stage2Sectors + $kernelSectors + $kernel64Sectors) * 512)] = $stage1_5Bytes[$x] }

    Write-Host "Writing $($osBytes.Length) bytes"
    [System.IO.File]::WriteAllBytes("${PSScriptRoot}\DanOS.vhd", $osBytes)

    Write-Host "VHD ready"
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
