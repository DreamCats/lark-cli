# Lark CLI Installer for Windows
# https://github.com/DreamCats/lark-cli

$ErrorActionPreference = "Stop"

$REPO = "DreamCats/lark-cli"
$BINARY_NAME = "lark-cli"
$INSTALL_DIR = "$env:USERPROFILE\.local\bin"

function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] " -ForegroundColor Blue -NoNewline
    Write-Host $Message
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] " -ForegroundColor Green -NoNewline
    Write-Host $Message
}

function Write-Warn {
    param([string]$Message)
    Write-Host "[WARN] " -ForegroundColor Yellow -NoNewline
    Write-Host $Message
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] " -ForegroundColor Red -NoNewline
    Write-Host $Message
    exit 1
}

function Get-Platform {
    $arch = [System.Environment]::GetEnvironmentVariable("PROCESSOR_ARCHITECTURE")
    switch ($arch) {
        "AMD64" { return "x86_64-pc-windows-msvc" }
        "ARM64" { return "aarch64-pc-windows-msvc" }
        default { Write-Error "Unsupported architecture: $arch" }
    }
}

function Get-LatestVersion {
    $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$REPO/releases/latest"
    return $response.tag_name
}

function Install-LarkCli {
    Write-Info "Detecting platform..."
    $platform = Get-Platform
    Write-Info "Platform: $platform"

    Write-Info "Fetching latest version..."
    $version = Get-LatestVersion
    if (-not $version) {
        Write-Error "Failed to get latest version"
    }
    Write-Info "Version: $version"

    $downloadUrl = "https://github.com/$REPO/releases/download/$version/$BINARY_NAME-$platform.zip"
    Write-Info "Downloading from: $downloadUrl"

    $tmpDir = New-TemporaryFile | ForEach-Object { Remove-Item $_; New-Item -ItemType Directory -Path $_ }
    $zipPath = Join-Path $tmpDir "lark-cli.zip"

    try {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath -UseBasicParsing

        Write-Info "Extracting..."
        Expand-Archive -Path $zipPath -DestinationPath $tmpDir -Force

        Write-Info "Installing to $INSTALL_DIR..."
        if (-not (Test-Path $INSTALL_DIR)) {
            New-Item -ItemType Directory -Path $INSTALL_DIR -Force | Out-Null
        }

        $exePath = Join-Path $tmpDir "$BINARY_NAME.exe"
        Copy-Item $exePath -Destination $INSTALL_DIR -Force

        Write-Success "Lark CLI $version installed successfully!"

        # Check if INSTALL_DIR is in PATH
        $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
        if ($userPath -notlike "*$INSTALL_DIR*") {
            Write-Warn "$INSTALL_DIR is not in your PATH"
            Write-Host ""
            Write-Host "To add it to your PATH, run:"
            Write-Host ""
            Write-Host "  `$env:Path += `";$INSTALL_DIR`"" -ForegroundColor Cyan
            Write-Host "  [Environment]::SetEnvironmentVariable('Path', `$env:Path + ';$INSTALL_DIR', 'User')" -ForegroundColor Cyan
            Write-Host ""
        }

        Write-Host ""
        Write-Info "Run 'lark-cli --help' to get started"
    }
    finally {
        Remove-Item -Recurse -Force $tmpDir -ErrorAction SilentlyContinue
    }
}

# Run installer
Install-LarkCli
