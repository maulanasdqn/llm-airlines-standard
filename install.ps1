param(
    [switch] $BinaryOnly,
    [switch] $AddToPath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'

$InstallArguments = @($args)
$Repository = if ($env:STE_REPOSITORY) { $env:STE_REPOSITORY } else { 'maulanasdqn/llm-airlines-standard' }
$Version = if ($env:STE_VERSION) { $env:STE_VERSION } else { 'latest' }
$DefaultFolder = Join-Path ([Environment]::GetFolderPath('LocalApplicationData')) 'Programs\ste'
$InstallFolder = if ($env:STE_INSTALL_DIR) { $env:STE_INSTALL_DIR } else { $DefaultFolder }
$Archive = 'ste-x86_64-pc-windows-msvc.zip'
$BinaryName = 'ste.exe'

function Assert-Architecture {
    if (-not [Environment]::Is64BitOperatingSystem) {
        throw "Cannot install ste: no release build exists for 32-bit Windows. Run cargo install --git https://github.com/$Repository."
    }
    if ($env:PROCESSOR_ARCHITECTURE -eq 'ARM64') {
        Write-Host 'This Windows uses ARM64. Windows runs the x86_64 build of ste with emulation.'
    }
}

function Get-BaseUrl {
    if ($env:STE_DOWNLOAD_URL) {
        return $env:STE_DOWNLOAD_URL
    }
    if ($Version -eq 'latest') {
        return "https://github.com/$Repository/releases/latest/download"
    }
    return "https://github.com/$Repository/releases/download/$Version"
}

function Save-ReleaseFile {
    param([string] $Name, [string] $Destination)
    $base = Get-BaseUrl
    if (Test-Path -LiteralPath $base -PathType Container) {
        Copy-Item -LiteralPath (Join-Path $base $Name) -Destination $Destination
        return
    }
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    Invoke-WebRequest -Uri "$base/$Name" -OutFile $Destination -UseBasicParsing
}

function Assert-Checksum {
    param([string] $ArchivePath, [string] $ChecksumPath)
    $expected = ((Get-Content -LiteralPath $ChecksumPath -Raw).Trim() -split '\s+')[0].ToLowerInvariant()
    $actual = (Get-FileHash -LiteralPath $ArchivePath -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($expected -ne $actual) {
        throw "The SHA-256 of $Archive does not match the release. Do not use this download. Try again later."
    }
}

function Install-Binary {
    Assert-Architecture
    $work = Join-Path ([IO.Path]::GetTempPath()) ('ste-' + [Guid]::NewGuid())
    New-Item -ItemType Directory -Path $work | Out-Null
    try {
        $archivePath = Join-Path $work $Archive
        Save-ReleaseFile -Name $Archive -Destination $archivePath
        Save-ReleaseFile -Name "$Archive.sha256" -Destination "$archivePath.sha256"
        Assert-Checksum -ArchivePath $archivePath -ChecksumPath "$archivePath.sha256"
        Expand-Archive -LiteralPath $archivePath -DestinationPath $work -Force
        New-Item -ItemType Directory -Path $InstallFolder -Force | Out-Null
        Copy-Item -LiteralPath (Join-Path $work $BinaryName) -Destination (Join-Path $InstallFolder $BinaryName) -Force
    }
    finally {
        Remove-Item -LiteralPath $work -Recurse -Force
    }
    Write-Host "Installed ste in $(Join-Path $InstallFolder $BinaryName)."
}

function Add-PathEntry {
    $userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
    $entries = @($userPath -split ';' | Where-Object { $_ })
    if ($entries -contains $InstallFolder) {
        return
    }
    [Environment]::SetEnvironmentVariable('Path', (($entries + $InstallFolder) -join ';'), 'User')
    $env:Path = "$env:Path;$InstallFolder"
    Write-Host "Added $InstallFolder to the Path of your user account. Open a new terminal to use ste."
}

function Write-PathHint {
    $entries = @($env:Path -split [IO.Path]::PathSeparator)
    if ($entries -notcontains $InstallFolder) {
        Write-Host "Add $InstallFolder to your Path to run ste, or run this script again with -AddToPath."
    }
}

function Install-Skills {
    param([string] $Binary)
    & $Binary install @InstallArguments
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}

$binary = $env:STE_BINARY
if (-not $binary) {
    Install-Binary
    $binary = Join-Path $InstallFolder $BinaryName
    if ($AddToPath) {
        Add-PathEntry
    }
    else {
        Write-PathHint
    }
}
if (-not $BinaryOnly) {
    Install-Skills -Binary $binary
}
