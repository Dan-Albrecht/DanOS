$ErrorActionPreference = 'Stop'
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
Push-Location ${PSScriptRoot}
try {
    $PSNativeCommandUseErrorActionPreference = $true
    $loadMemoryTarget = 0x7E00
    $memoryMapTarget = 0x6000

    # I really hate you PowerShell
    [System.Environment]::CurrentDirectory = ${PSScriptRoot}

    $STAGE_4_LOAD_TARGET = $loadMemoryTarget

    # Secret handshake to eventaully get this passed to the linker
    $env:KERNEL64_LOAD_TARGET = "0x$(([int]$STAGE_4_LOAD_TARGET).ToString("X"))"
    .\kernel64\buildKernel.ps1
    $kernel64Bytes = Get-Content .\kernel64\target\x86_64-unknown-none\release\kernel64.bin -Raw -AsByteStream
    $kernel64Sectors = [Math]::Ceiling($kernel64Bytes.Length / 512)

    $STAGE_3_LOAD_TARGET = $STAGE_4_LOAD_TARGET + ($kernel64Sectors * 512)
    $env:KERNEL32_LOAD_TARGET = "0x$(([int]$STAGE_3_LOAD_TARGET).ToString("X"))"
    .\kernel\buildKernel.ps1
    $kernelBytes = Get-Content .\kernel\target\i686-unknown-none\release\kernel.bin -Raw -AsByteStream
    $kernelSectors = [Math]::Ceiling($kernelBytes.Length / 512)

    $STAGE_2_LOAD_TARGET = $STAGE_3_LOAD_TARGET + ($kernelSectors * 512)    
    nasm.exe .\bootloaderStage2.asm -DSTAGE_2_LOAD_TARGET="$STAGE_2_LOAD_TARGET" -DKERNEL32_JUMP_TARGET="$STAGE_3_LOAD_TARGET" -f bin -o .\bootloaderStage2.bin
    $stage2Bytes = Get-Content .\bootloaderStage2.bin -Raw -AsByteStream
    $stage2Sectors = [Math]::Ceiling($stage2Bytes.Length / 512)

    $STAGE_1_5_LOAD_TARGET = $STAGE_2_LOAD_TARGET + ($stage2Sectors * 512)
    nasm.exe .\bootloaderStage1_5.asm -DSTAGE_1_5_LOAD_TARGET="$STAGE_1_5_LOAD_TARGET" -DMEMORY_MAP_TARGET="$memoryMapTarget" -f bin -o .\bootloaderStage1_5.bin
    $stage1_5Bytes = Get-Content .\bootloaderStage1_5.bin -Raw -AsByteStream
    $stage1_5Sectors = [Math]::Ceiling($stage1_5Bytes.Length / 512)
    $stage1_5Segment = $STAGE_1_5_LOAD_TARGET -shr 4

    $neededSectors = $stage2Sectors + $kernelSectors + $kernel64Sectors + $stage1_5Sectors

    # Divide by 16 to get to segment
    $diskDataSegment = $loadMemoryTarget -shr 4

    Write-Host "Stage 1 @ 0x7C00 (must be 1 sector)"
    Write-Host "Stage 1.5 @ 0x$(([int]$STAGE_1_5_LOAD_TARGET).ToString("X")) (for 0x$(([int]$stage1_5Sectors).ToString("X")) sectors)"
    Write-Host "Stage 2 @ 0x$(([int]$STAGE_2_LOAD_TARGET).ToString("X")) (for 0x$(([int]$stage2Sectors).ToString("X")) sectors)"
    Write-Host "Stage 3 @ 0x$(([int]$STAGE_3_LOAD_TARGET).ToString("X")) (for 0x$(([int]$kernelSectors).ToString("X")) sectors)"
    Write-Host "Stage 4 @ 0x$(([int]$STAGE_4_LOAD_TARGET).ToString("X")) (for 0x$(([int]$kernel64Sectors).ToString("X")) sectors)."
    Write-Host "This is a total of 0x$(([int]$neededSectors).ToString("X")) sectors to load from disk to segment 0x$(([int]$diskDataSegment).ToString("X"))."

    # BUGUBG: Don't harcode load target, just change address to sector
    nasm.exe .\bootloaderStage1.asm -DDISK_DATA_SECTOR_LOAD_COUNT="$neededSectors" -DDISK_DATA_MEMORY_SEGMENT="$diskDataSegment" -DSTAGE1_5_TARGET_MEMORY_SEGMENT="$stage1_5Segment" -DSTAGE_2_JUMP_TARGET="$STAGE_2_LOAD_TARGET" -f bin -o .\bootloaderStage1.bin
    $stage1Bytes = Get-Content .\bootloaderStage1.bin -Raw -AsByteStream
    if ($stage1Bytes.Length -ne 512 ) { Write-Error 'Bootloader should be exactly 512 bytes' }
    $stage1Sectors = 1 # Must be per above and the laws of bios

    if (![System.IO.File]::Exists("empty.vhd")) {
        # Creation is too slow, so just cache an empty one and use it
        New-VHD -Path empty.vhd -Fixed -SizeBytes 3MB
    }

    Copy-Item -Force .\empty.vhd .\DanOS.vhd

    $osBytes = Get-Content .\DanOS.vhd -Raw -AsByteStream
    $writeIndex = 0

    for ($x = 0; $x -lt ($stage1Sectors * 512); $x++) {
        if ($x -lt $stage1Bytes.Length) {
            $osBytes[$writeIndex] = $stage1Bytes[$x]
        }
        else {
            $osBytes[$writeIndex] = 0xDA
        }
        $writeIndex++
    }

    for ($x = 0; $x -lt ($kernel64Sectors * 512); $x++) {
        if ($x -lt $kernel64Bytes.Length) {
            $osBytes[$writeIndex] = $kernel64Bytes[$x]
        }
        else {
            $osBytes[$writeIndex] = 0xDB
        }
        $writeIndex++
    }

    for ($x = 0; $x -lt ($kernelSectors * 512); $x++) {
        if ($x -lt $kernelBytes.Length) {
            $osBytes[$writeIndex] = $kernelBytes[$x]
        }
        else {
            $osBytes[$writeIndex] = 0xDC
        }
        $writeIndex++
    }

    for ($x = 0; $x -lt ($stage2Sectors * 512); $x++) {
        if ($x -lt $stage2Bytes.Length) {
            $osBytes[$writeIndex] = $stage2Bytes[$x]
        }
        else {
            $osBytes[$writeIndex] = 0xDD
        }
        $writeIndex++
    }

    for ($x = 0; $x -lt ($stage1_5Sectors * 512); $x++) {
        if ($x -lt $stage1_5Bytes.Length) {
            $osBytes[$writeIndex] = $stage1_5Bytes[$x]
        }
        else {
            $osBytes[$writeIndex] = 0xDE
        }
        $writeIndex++
    }

    Write-Host "Writing $($osBytes.Length) bytes"
    [System.IO.File]::WriteAllBytes("${PSScriptRoot}\DanOS.vhd", $osBytes)

    Write-Host "VHD ready"
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
