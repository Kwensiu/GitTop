$ErrorActionPreference = 'Stop'

$toolsDir = "$(Split-Path -Parent $MyInvocation.MyCommand.Definition)"

# Remove shim
Uninstall-BinFile -Name 'gittop'

# Clean up extracted files
$exePath = Join-Path $toolsDir 'gittop.exe'
if (Test-Path $exePath) {
    Remove-Item $exePath -Force
}

$licensePath = Join-Path $toolsDir 'LICENSE.md'
if (Test-Path $licensePath) {
    Remove-Item $licensePath -Force
}

$readmePath = Join-Path $toolsDir 'README.md'
if (Test-Path $readmePath) {
    Remove-Item $readmePath -Force
}
