$ErrorActionPreference = 'Stop'
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUserDeclaredVarsMoreThanAssignments', 'This is a global PS state variable')]
$oldErrorState = $PSNativeCommandUseErrorActionPreference
Push-Location ${PSScriptRoot}
try {
    # I really hate you PowerShell
    [System.Environment]::CurrentDirectory = ${PSScriptRoot}
    $PSNativeCommandUseErrorActionPreference = $true
    
    $memoryMapTarget = 0x1000
    $debug = $true

    if ($debug) {
        $targetType = "debug"
    } else {
        $targetType = "release"
    }

    $STAGE_2_LOAD_TARGET = 0x7E00 # Slap this right after boot sector for now, we're just going to assume it'll stay small and fit...
    TimeCommand { ..\stage2_rust\build.ps1 -loadTarget $STAGE_2_LOAD_TARGET -debug $debug } -message 'Stage 2'
    $stage2Bytes = Get-Content ..\stage2_rust\target\i386-unknown-none\$targetType\stage2_rust.bin -Raw -AsByteStream
    $stage2Sectors = [Math]::Ceiling($stage2Bytes.Length / 512)
    $stage2Segment = $STAGE_2_LOAD_TARGET -shr 4

<#
    $STAGE_4_LOAD_TARGET = 0x8000
    TimeCommand { ..\kernel64\buildKernel.ps1 -debug $debug } -message 'Kernel64'
    $kernel64Bytes = Get-Content ..\kernel64\target\x86_64-unknown-none\$targetType\kernel64.strippedWithDebugLink -Raw -AsByteStream    
    $kernel64Sectors = [Math]::Ceiling($kernel64Bytes.Length / 512)

    $STAGE_3_LOAD_TARGET = $STAGE_4_LOAD_TARGET + ($kernel64Sectors * 512)
    $env:KERNEL32_LOAD_TARGET = "0x$(([int]$STAGE_3_LOAD_TARGET).ToString("X"))"
    TimeCommand { ..\kernel\buildKernel.ps1 -debug $debug } -message 'Kernel32'
    $kernelBytes = Get-Content ..\kernel\target\i686-unknown-none\$targetType\kernel.bin -Raw -AsByteStream    
    $kernelSectors = [Math]::Ceiling($kernelBytes.Length / 512)
#>
    

    $neededSectors = $stage2Sectors + $kernelSectors + $kernel64Sectors

    # Divide by 16 to get to segment
    #$diskDataSegment = $STAGE_4_LOAD_TARGET -shr 4
    $diskDataSegment = $stage2Segment

    Write-Host "Stage 1 @ 0x7C00 (must be 1 sector)"
    Write-Host "Stage 2 @ 0x$(([int]$STAGE_2_LOAD_TARGET).ToString("X")) (for 0x$(([int]$stage2Sectors).ToString("X")) sectors)"
    Write-Host "Stage 3 @ 0x$(([int]$STAGE_3_LOAD_TARGET).ToString("X")) (for 0x$(([int]$kernelSectors).ToString("X")) sectors)"
    Write-Host "Stage 4 @ 0x$(([int]$STAGE_4_LOAD_TARGET).ToString("X")) (for 0x$(([int]$kernel64Sectors).ToString("X")) sectors)."
    Write-Host "This is a total of 0x$(([int]$neededSectors).ToString("X")) sectors to load from disk to segment 0x$(([int]$diskDataSegment).ToString("X"))."

    # BUGUBG: Don't harcode load target, just change address to sector
    TimeCommand { ..\stage1\build.ps1 -sectorsToLoad $neededSectors -targetMemorySegment $diskDataSegment -handoffToSegment $stage2Segment } -message 'Stage 1'
    
    # Slap on some partition info to make this look like an actual MBR disk so we can boot from it on real hardware
    TimeCommand { dotnet run --runtime win-x64 --no-launch-profile --project ..\diskTools\diskTools.csproj merge ..\stage1\bootloaderStage1.bin \temp\usb2.bin ..\mergedStage1.bin } -message 'Partition Stage 1'
    
    $stage1Bytes = Get-Content ..\mergedStage1.bin -Raw -AsByteStream
    if ($stage1Bytes.Length -ne 512 ) { Write-Error 'Bootloader should be exactly 512 bytes' }
    $stage1Sectors = 1 # Must be per above and the laws of bios

    # +1 as we also need Stage1
    $danOSBin = [byte[]]::new((($neededSectors + 1) * 512))
    Write-Host "Allocated $($danOSBin.Count) bytes"

    $writeIndex = 0

    for ($x = 0; $x -lt ($stage1Sectors * 512); $x++) {
        if ($x -lt $stage1Bytes.Length) {
            $danOSBin[$writeIndex] = $stage1Bytes[$x]
        }
        else {
            # We pad all these to fill the sector
            $danOSBin[$writeIndex] = 0xDA
        }
        $writeIndex++
    }

    for ($x = 0; $x -lt ($stage2Sectors * 512); $x++) {
        if ($x -lt $stage2Bytes.Length) {
            $danOSBin[$writeIndex] = $stage2Bytes[$x]
        }
        else {
            $danOSBin[$writeIndex] = 0xDD
        }
        $writeIndex++
    }

    <#
    for ($x = 0; $x -lt ($kernel64Sectors * 512); $x++) {
        if ($x -lt $kernel64Bytes.Length) {
            $danOSBin[$writeIndex] = $kernel64Bytes[$x]
        }
        else {
            $danOSBin[$writeIndex] = 0xDB
        }
        $writeIndex++
    }

    for ($x = 0; $x -lt ($kernelSectors * 512); $x++) {
        if ($x -lt $kernelBytes.Length) {
            $danOSBin[$writeIndex] = $kernelBytes[$x]
        }
        else {
            $danOSBin[$writeIndex] = 0xDC
        }
        $writeIndex++
    }
    #>

    Write-Host "Writing $($danOSBin.Length) bytes to DanOS.bin"
    TimeCommand { [System.IO.File]::WriteAllBytes("${PSScriptRoot}\DanOS.bin", $danOSBin) } -message 'Write DanOS.bin'
    $emptyVhdSize = 3MB

    # The VHD has a mandatory footer we cannot overwrite
    $footerLength = 512

    $vhdFreeSpace = $emptyVhdSize - $footerLength

    if ($danOSBin.Length -gt $vhdFreeSpace) {
        Write-Error "VHD needs to be made bigger"
    }

    if (![System.IO.File]::Exists("empty.vhd")) {
        # Creation is too slow, so just cache an empty one and use it
        Write-Host "Creating empty VHD"
        TimeCommand { New-VHD -Path empty.vhd -Fixed -SizeBytes $emptyVhdSize } -message 'Create empty VHD'
    }

    Copy-Item -Force .\empty.vhd .\DanOS.vhd

    Write-Host "Bulding VHD"
    $osBytes = Get-Content .\DanOS.vhd -Raw -AsByteStream
    for ($x = 0; $x -lt $danOSBin.Count; $x++) {
        $osBytes[$x] = $danOSBin[$x]
    }

    Write-Host "Writing $($osBytes.Length) bytes to VHD"
    TimeCommand { [System.IO.File]::WriteAllBytes("${PSScriptRoot}\DanOS.vhd", $osBytes) } -message 'Build VHD'
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
