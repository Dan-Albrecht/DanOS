$ErrorActionPreference = 'Stop'
$oldErrorState = $PSNativeCommandUseErrorActionPreference
Push-Location ${PSScriptRoot}
try {
    # I really hate you PowerShell
    [System.Environment]::CurrentDirectory = ${PSScriptRoot}
    $PSNativeCommandUseErrorActionPreference = $true

    TimeCommand {
        $imgItem = Get-ChildItem "DanOS.img"

        # VHD has a small footer. 1MB is more than enough to accomodate it.
        $neededLength = $imgItem.Length + 1MB

        if ([System.IO.File]::Exists("DanOS.vhd")) {
            Remove-Item "DanOS.vhd"
        }

        New-VHD -Path "DanOS.vhd" -SizeBytes $neededLength -Fixed

        $fs = [System.IO.File]::Open("DanOS.vhd", [System.IO.FileMode]::Open, [System.IO.FileAccess]::ReadWrite)
        $fs.Position = 0
        
        $imgBytes = Get-Content $imgItem -Raw -AsByteStream
        $fs.Write($imgBytes)
        $fs.Close()
    } -message 'Build VHD'
}
finally {
    $PSNativeCommandUseErrorActionPreference = $oldErrorState
    Pop-Location
}
