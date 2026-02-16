$ErrorActionPreference = "Stop"

$repo = "agent-clis/gripe"
$artifact = "gripe-windows-amd64"
$url = "https://github.com/$repo/releases/latest/download/$artifact.zip"
$installDir = "$env:LOCALAPPDATA\gripe\bin"

Write-Host "Downloading gripe for windows-amd64..."
$tmp = New-TemporaryFile | Rename-Item -NewName { $_.Name + ".zip" } -PassThru
Invoke-WebRequest -Uri $url -OutFile $tmp -UseBasicParsing

$extractDir = Join-Path ([System.IO.Path]::GetTempPath()) "gripe-extract"
if (Test-Path $extractDir) { Remove-Item -Recurse -Force $extractDir }
Expand-Archive -Path $tmp -DestinationPath $extractDir
Remove-Item $tmp

New-Item -ItemType Directory -Force -Path $installDir | Out-Null
Move-Item -Force (Join-Path $extractDir "$artifact.exe") (Join-Path $installDir "gripe.exe")
Remove-Item -Recurse -Force $extractDir

# Add to user PATH if not present
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
    $env:Path = "$env:Path;$installDir"
    Write-Host "Added $installDir to user PATH."
}

Write-Host "gripe installed to $installDir\gripe.exe"
