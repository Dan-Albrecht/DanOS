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
    $STAGE_2_LOAD_TARGET = "0x7E00"    # Slap this right after boot sector for now, we're just going to assume it'll stay small and fit...
    $KERNEL32_LOAD_TARGET = "0x100000" # Nice round number; nothing significant
    $BOOTLOADER_MAX_SIZE = 1MB         # Total ammount of space we have before the bootloaders before they'd start overwriting the first partition of our image
    $OUTPUT_FILE = "DanOS.img"

    if ($debug) {
        $targetType = "debug"
    }
    else {
        $targetType = "release"
    }

    # Forcing to release for now to try and get smaller size which seem to be the source of break after 1.88 upgrade
    TimeCommand { ../stage2_rust/build.ps1 -loadTarget $STAGE_2_LOAD_TARGET -debug $false } -message 'Stage 2'
    $stage2Path = "../stage2_rust/target/i386-unknown-none/release/stage2_rust.bin"

    $stage2Bytes = Get-Content $stage2Path -Raw -AsByteStream
    $stage2Item = Get-ChildItem $stage2Path
    $stage2Sectors = [Math]::Ceiling($stage2Bytes.Length / 512)
    $stage2Padding = $stage2Sectors * 512 - $stage2Bytes.Length

    TimeCommand { ../stage1/build.ps1 -sectorsToLoad $stage2Sectors -addressToLoadTo $STAGE_2_LOAD_TARGET } -message 'Stage 1'
    $stage1Path = "../stage1/bootloaderStage1.bin"
    
    TimeCommand { ../kernel/buildKernel.ps1 -debug $debug -loadTarget $KERNEL32_LOAD_TARGET } -message 'Kernel32'
    $stage3Path = "../kernel/target/i686-unknown-none/$targetType/kernel.bin"
    
    TimeCommand { ../kernel64/buildKernel.ps1 -debug $debug } -message 'Kernel64'
    $stage4Path = "../kernel64/target/x86_64-unknown-none/$targetType/kernel64"

    $stage1Bytes = Get-Content $stage1Path -Raw -AsByteStream
    $stage1Item = Get-ChildItem $stage1Path
    Write-Host "Stage1 is @ $stage1Path size is $($stage1Item.Length) written @ $($stage1Item.LastWriteTime)"
    
    if ($stage1Bytes.Length -ne 440) {
        # 440, not 512 since that's just the code space
        # We'll pre-create an empty image that'll have parition info present
        Write-Error "Stage 1 must be exactly 440 bytes"
    }

    if ($stage1Bytes.Length + $stage2Bytes.Length -gt $BOOTLOADER_MAX_SIZE) {
        Write-Error "Bootloaders are too big"
    }

    if ([System.IO.File]::Exists($OUTPUT_FILE)) {
        Remove-Item -Path $OUTPUT_FILE -Force
    }

    TimeCommand {
        # Create small OS image
        dd if=/dev/zero of=$OUTPUT_FILE bs=6M count=1

        # 1 Meg gap at front it for early stage bootloaders, kernel(s) will appear on the filesystem
        /usr/sbin/parted --script $OUTPUT_FILE mklabel msdos mkpart primary fat16 1MiB 100% set 1 boot on

        # Shenanagins to force a small FAT16 partition:
        #   - Above created a 6MB total image
        #   - It reserves 1MB for bootloaders
        #   - So we have 5MB left for the actual data
        #   - Force a very small cluster size so total cluster count is high enough for FAT16; default would have ended up with FAT12
        mformat -c 2 -i $OUTPUT_FILE@@1M ::

        "Built at $([DateTime]::Now)" | mcopy -i $OUTPUT_FILE@@1M - ::HI.TXT

    } -message 'Create empty image'

    TimeCommand {
        mcopy -i $OUTPUT_FILE@@1M $stage3Path ::
        mcopy -i $OUTPUT_FILE@@1M $stage4Path ::KERNEL64.ELF
    } -message 'Copy kernels'

    TimeCommand {
        $fs = [System.IO.File]::Open($OUTPUT_FILE, [System.IO.FileMode]::Open, [System.IO.FileAccess]::ReadWrite)
        $fs.Position = 0
        
        $fs.Write($stage1Bytes)

        # Move to next sector so we don't overwrite the parition info
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
    Write-Host "Disk image contents:"
    mdir -i $OUTPUT_FILE@@1M ::
    Write-Host "`n`nKicking off DanOS...`n`n"
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
