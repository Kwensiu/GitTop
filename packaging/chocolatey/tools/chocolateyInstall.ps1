$ErrorActionPreference = 'Stop'

$packageName = 'gittop'
$toolsDir = "$(Split-Path -Parent $MyInvocation.MyCommand.Definition)"

$version = '{{VERSION}}'
$url64 = "https://github.com/AmarBego/GitTop/releases/download/v$version/gittop-windows-x86_64.zip"
$checksum64 = '{{CHECKSUM}}'
$checksumType64 = 'sha256'

$packageArgs = @{
    packageName    = $packageName
    unzipLocation  = $toolsDir
    url64bit       = $url64
    checksum64     = $checksum64
    checksumType64 = $checksumType64
}

Install-ChocolateyZipPackage @packageArgs

$exePath = Join-Path $toolsDir 'gittop-windows-x86_64\gittop.exe'
Install-BinFile -Name 'gittop' -Path $exePath

