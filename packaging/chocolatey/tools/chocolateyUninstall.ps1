$ErrorActionPreference = 'Stop'

$toolsDir = "$(Split-Path -Parent $MyInvocation.MyCommand.Definition)"

Uninstall-BinFile -Name 'gittop'

$extractedDir = Join-Path $toolsDir 'gittop-windows-x86_64'
if (Test-Path $extractedDir) {
    Remove-Item $extractedDir -Recurse -Force
}

