$ErrorActionPreference = 'Stop'
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
Push-Location ${PSScriptRoot}
try {
    # I really hate you PowerShell
    [System.Environment]::CurrentDirectory = ${PSScriptRoot}
    $PSNativeCommandUseErrorActionPreference = $true

    # Preferneces
    $debug = $true

    # Magic constants    
    $STAGE_2_LOAD_TARGET = 0x7E00 # Slap this right after boot sector for now, we're just going to assume it'll stay small and fit...
    $BOOTLOADER_MAX_SIZE = 1MB # Total ammount of space we have before the bootloaders before they'd start overwriting the first partition of our image
    $OUTPUT_FILE = "DanOS.img"

    if ($debug) {
        $targetType = "debug"
    }
    else {
        $targetType = "release"
    }

    TimeCommand { ../stage2_rust/build.ps1 -loadTarget $STAGE_2_LOAD_TARGET -debug $debug } -message 'Stage 2'
    $stage2Path = "../stage2_rust/target/i386-unknown-none/$targetType/stage2_rust.bin"

    $stage2Bytes = Get-Content $stage2Path -Raw -AsByteStream
    $stage2Item = Get-ChildItem $stage2Path
    $stage2Sectors = [Math]::Ceiling($stage2Bytes.Length / 512)
    $stage2Padding = $stage2Sectors * 512 - $stage2Bytes.Length

    TimeCommand { ../stage1/build.ps1 -sectorsToLoad $stage2Sectors -addressToLoadTo $STAGE_2_LOAD_TARGET } -message 'Stage 1'
    $stage1Path = "../stage1/bootloaderStage1.bin"
    
    TimeCommand { ../kernel/buildKernel.ps1 -debug $debug } -message 'Kernel32'
    $stage3Path = "../kernel/target/i686-unknown-none/$targetType/kernel.bin"
    
    TimeCommand { ../kernel64/buildKernel.ps1 -debug $debug } -message 'Kernel64'
    $stage4Path = "../kernel64/target/x86_64-unknown-none/$targetType/kernel64"

    $stage1Bytes = Get-Content $stage1Path -Raw -AsByteStream
    $stage1Item = Get-ChildItem $stage1Path
    Write-Host "Stage1 is @ $stage1Path size is $($stage1Item.Length) written @ $($stage1Item.LastWriteTime)"
    
    if ($stage1Bytes.Length -ne 440) {
        # 440, not 512 since that's just the code space
        # The partition info is already in the empty.img
        Write-Error "Stage 1 must be exactly 440 bytes"
    }

    if ($stage1Bytes.Length + $stage2Bytes.Length -gt $BOOTLOADER_MAX_SIZE) {
        Write-Error "Bootloaders are too big"
    }

    
    # BUGBUG: This name is confusing
    if (![System.IO.File]::Exists("empty.img")) {
        Write-Error "empty.img doesn't exist. You'll need to create it by hand. Follow the README.md."
    }

    # BUGUBG: Probably a better way to check this (that the img is mounted), but we're always hand backing this file into the image, so good enough for now
    if (![System.IO.File]::Exists("/mnt/danOS/hi.txt")) {
        Write-Error "It appears empty.img isn't mounted. You'll need to mount it to /mnt/danOS."
    }

    TimeCommand {
        cp $stage3Path /mnt/danOS/kernel.bin
        cp $stage4Path /mnt/danOS/kernel64.elf
        sync
    } -message 'Copy kernels'

    Copy-Item -Path 'empty.img' -Destination $OUTPUT_FILE -Force

    TimeCommand {
        $fs = [System.IO.File]::Open($OUTPUT_FILE, [System.IO.FileMode]::Open, [System.IO.FileAccess]::ReadWrite)
        $fs.Position = 0
        
        $fs.Write($stage1Bytes)

        # Need to leave the parition info intact either from Stage1 itself, or the prebuild empty.img
        $fs.Position = 512
        $fs.Write($stage2Bytes)

        $fs.Close()
    } -message 'Assemble image'

    Write-Host "Stage 1 @ 0x7C00 (must be 1 sector)"
    Write-Host "$($stage1Path)"
    Write-Host "Stage 2 @ 0x$(([int]$STAGE_2_LOAD_TARGET).ToString("X")) (for 0x$(([int]$stage2Sectors).ToString("X")) sectors)"
    Write-Host "Stage2 is @ $stage2Path size is $($stage2Item.Length) written @ $($stage2Item.LastWriteTime) and will need $stage2Padding padding"
    Write-Host "This is a total of 0x$(([int]$stage2Sectors).ToString("X")) sectors to load from disk to address 0x$(([int]$STAGE_2_LOAD_TARGET).ToString("X"))."
    Write-Host "$($stage2Path)"
    Write-Host "$($stage3Path)"
    Write-Host "$($stage4Path)"
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
