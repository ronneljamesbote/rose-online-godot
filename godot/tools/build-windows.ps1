# Builds the Windows release zip from Windows:
# - Compiles rose_gd.dll with Rust + MinGW-w64
# - Exports ROSE.exe using Godot's Windows release template
# - Packages both together with README.txt
#
# Prerequisites:
#   - Rustup
#   - Rust target: x86_64-pc-windows-gnu
#   - MinGW-w64 / MSYS2
#   - Godot 4.7.2
#   - Godot 4.7.2 Windows Desktop export template
#   - zip functionality (PowerShell Compress-Archive is used)
#
# Usage:
#   .\godot\tools\build-windows.ps1
#
#   .\godot\tools\build-windows.ps1 .\dist
#
# Environment variables:
#   GODOT            Path to godot.exe
#   CARGO_TARGET_DIR Custom Cargo target directory

$ErrorActionPreference = "Stop"

# Go to repository root.
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location (Join-Path $ScriptDir "..\..")

# Configuration
$Godot = if ($env:GODOT) {
    $env:GODOT
} else {
    "godot"
}

$Out = if ($args.Count -ge 1) {
    [System.IO.Path]::GetFullPath($args[0])
} else {
    Join-Path (Get-Location) "dist"
}

$TargetDir = if ($env:CARGO_TARGET_DIR) {
    $env:CARGO_TARGET_DIR
} else {
    Join-Path (Get-Location) "target"
}

$RustTarget = "x86_64-pc-windows-gnu"

Write-Host "==> Repository root: $(Get-Location)"
Write-Host "==> Godot: $Godot"
Write-Host "==> Output: $Out"
Write-Host "==> Cargo target: $TargetDir"

# ------------------------------------------------------------
# Check prerequisites
# ------------------------------------------------------------

Write-Host ""
Write-Host "==> Checking prerequisites..."

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw "cargo was not found. Install Rust using rustup."
}

if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
    throw "rustc was not found. Install Rust using rustup."
}

if (-not (Get-Command gcc -ErrorAction SilentlyContinue)) {
    throw @"
gcc was not found.

Install MinGW-w64 through MSYS2:

  pacman -S --needed mingw-w64-ucrt-x86_64-gcc

Then make sure the MSYS2 UCRT64 bin directory is on PATH.
"@
}

if (-not (Get-Command strip -ErrorAction SilentlyContinue)) {
    throw @"
strip was not found.

Install MinGW-w64 through MSYS2:

  pacman -S --needed mingw-w64-ucrt-x86_64-binutils
"@
}

if (-not (Get-Command $Godot -ErrorAction SilentlyContinue)) {
    throw @"
Godot was not found.

Either add godot.exe to PATH or set:

  `$env:GODOT = "C:\Path\To\Godot.exe"
"@
}

# The MinGW GNU toolchain must be the *host* toolchain, not just an added
# target: cargo compiles proc-macro and build-script crates for the host, and
# an MSVC host looks for link.exe. Use a GNU host so both host and target
# link with gcc.
$GnuToolchain = "stable-$RustTarget"

$HasGnu = rustup toolchain list | Select-String -SimpleMatch $GnuToolchain -Quiet
if (-not $HasGnu) {
    Write-Host "==> Installing GNU host toolchain: $GnuToolchain"
    rustup toolchain install $GnuToolchain
}

# ------------------------------------------------------------
# Build Rust DLL
# ------------------------------------------------------------

Write-Host ""
Write-Host "==> Building rose-gd..."

$env:CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "gcc"
$env:CARGO_TARGET_DIR = $TargetDir

cargo "+$GnuToolchain" build `
    --release `
    -p rose-gd `
    --target $RustTarget

if ($LASTEXITCODE -ne 0) {
    throw "Cargo build failed."
}

# ------------------------------------------------------------
# Strip DLL
# ------------------------------------------------------------

Write-Host ""
Write-Host "==> Preparing rose_gd.dll..."

$BinDir = Join-Path (Get-Location) "godot\bin"

New-Item -ItemType Directory -Force -Path $BinDir | Out-Null

$BuiltDll = Join-Path `
    $TargetDir `
    "$RustTarget\release\rose_gd.dll"

$OutputDll = Join-Path $BinDir "rose_gd.dll"

if (-not (Test-Path $BuiltDll)) {
    throw "Built DLL was not found: $BuiltDll"
}

strip `
    -o $OutputDll `
    $BuiltDll

if ($LASTEXITCODE -ne 0) {
    throw "Failed to strip rose_gd.dll."
}

Write-Host "==> DLL: $OutputDll"

# ------------------------------------------------------------
# Create temporary staging directory
# ------------------------------------------------------------

Write-Host ""
Write-Host "==> Creating staging directory..."

$Stage = Join-Path ([System.IO.Path]::GetTempPath()) `
    "rose-windows-$([Guid]::NewGuid().ToString())"

$StageRose = Join-Path $Stage "ROSE"

New-Item -ItemType Directory -Force -Path $StageRose | Out-Null

try {

    # --------------------------------------------------------
    # Export Godot project
    # --------------------------------------------------------

    Write-Host ""
    Write-Host "==> Exporting Godot Windows release..."

    & $Godot `
        --headless `
        --path godot `
        --export-release "Windows Desktop" `
        (Join-Path $StageRose "ROSE.exe") 2>&1 | Out-Host

    if ($LASTEXITCODE -ne 0) {
        throw "Godot export failed."
    }

    # --------------------------------------------------------
    # Copy README
    # --------------------------------------------------------

    $Readme = Join-Path `
        (Get-Location) `
        "godot\tools\README-windows.txt"

    if (-not (Test-Path $Readme)) {
        throw "README not found: $Readme"
    }

    Copy-Item `
        $Readme `
        (Join-Path $StageRose "README.txt")

    # --------------------------------------------------------
    # Create output directory
    # --------------------------------------------------------

    New-Item `
        -ItemType Directory `
        -Force `
        -Path $Out | Out-Null

    $ZipPath = Join-Path $Out "ROSE-windows.zip"

    if (Test-Path $ZipPath) {
        Remove-Item -Force $ZipPath
    }

    # --------------------------------------------------------
    # Create ZIP
    # --------------------------------------------------------

    Write-Host ""
    Write-Host "==> Creating ZIP..."

    Compress-Archive `
        -Path $StageRose `
        -DestinationPath $ZipPath `
        -CompressionLevel Optimal

    Write-Host ""
    Write-Host "==> Build complete!"
    Write-Host ""
    Get-Item $ZipPath | Format-List FullName, Length, LastWriteTime

}
finally {

    # --------------------------------------------------------
    # Cleanup
    # --------------------------------------------------------

    if (Test-Path $Stage) {
        Remove-Item `
            -Recurse `
            -Force `
            $Stage
    }
}
