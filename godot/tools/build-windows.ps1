# Builds the Windows client from Windows:
# - Compiles rose_gd.dll with Rust + MinGW-w64
# - Exports ROSE.exe using Godot's Windows export template, with ROSE.pck (scripts,
#   interface and zone scenes, about 5 MB) and assets.pck (the converted ROSE assets,
#   about 320 MB), which the game opens next to ROSE.exe
# - Packages both together with README.txt (in a folder ROSE, or ROSE-dev for -Dev)
#
# Switches:
#   -Dev     the dev client: GM commands in (chat and rose_gd.dll built with the dev
#            feature), Godot's debug template with a console window, the "Windows Desktop
#            Dev" preset. Output ROSE-windows-dev instead of ROSE-windows.
#   -Folder  a folder instead of a zip (<output>\ROSE-windows[-dev]\), for testing. A
#            rebuild replaces only the game's own files, so the themes folder, the ROSE
#            client data and anything else put next to ROSE.exe stay.
#   -GameOnly an update without assets.pck (ROSE-windows[-dev]-update.zip, or with
#            -Folder the folder's assets.pck is kept): for players who already have the
#            assets of this version. A change to the import (IMPORT_VERSION in
#            godot\rust\src\assets.rs) or to anything in assets\rose needs the full build.
#   -DataIdx the ROSE client's data.idx (or set ROSE_DATA_IDX). The build first converts
#            the ROSE models, textures and animations into godot\assets\rose\
#            (--import-rose), because assets.pck carries them and the zone scenes need them.
#
# It refuses to build when overrides\motions.toml (the server's hit times),
# overrides\zones\ (spawns and NPCs) or overrides\data\ (items, monsters, NPCs, skills) don't match the
# animations, the zone scenes and the game data resources in godot\data\; run
# --export-motions, --export-zones or --export-data and commit the files first. The game
# reads its items, monsters, NPCs and skills from overrides\data\ next to ROSE.exe, as the server does.
#
# The production build checks that no GM code got in and stops if it did.
#
# Prerequisites:
#   - Rustup
#   - Rust target: x86_64-pc-windows-gnu
#   - MinGW-w64 / MSYS2
#   - Godot 4.7.2
#   - Godot 4.7.2 Windows Desktop export template (fetched when missing)
#   - zip functionality (PowerShell Compress-Archive is used)
#   - The ROSE client data (data.idx and its VFS files)
#
# Usage:
#   .\godot\tools\build-windows.ps1 -DataIdx C:\Path\To\ROSE-client\data.idx
#
#   .\godot\tools\build-windows.ps1 .\dist
#
#   .\godot\tools\build-windows.ps1 -Dev -Folder
#
#   .\godot\tools\build-windows.ps1 -GameOnly
#
# Environment variables:
#   GODOT            Path to godot.exe
#   ROSE_DATA_IDX    Path to the ROSE client's data.idx (instead of -DataIdx)
#   CARGO_TARGET_DIR Custom Cargo target directory

param(
    [switch]$Dev,
    [switch]$Folder,
    [switch]$GameOnly,
    [string]$DataIdx,
    [Parameter(Position = 0)]
    [string]$OutDir
)

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

$Out = if ($OutDir) {
    [System.IO.Path]::GetFullPath($OutDir)
} else {
    Join-Path (Get-Location) "dist"
}

$TargetDir = if ($env:CARGO_TARGET_DIR) {
    $env:CARGO_TARGET_DIR
} else {
    Join-Path (Get-Location) "target"
}

$RustTarget = "x86_64-pc-windows-gnu"

if (-not $DataIdx) {
    $DataIdx = $env:ROSE_DATA_IDX
}

if ($Dev) {
    $Name = "ROSE-windows-dev"
    $DirName = "ROSE-dev"
    $Preset = "Windows Desktop Dev"
    $ExportFlag = "--export-debug"
    # The console window comes from its own small template.
    $Templates = @("windows_debug_x86_64.exe", "windows_debug_x86_64_console.exe")
} else {
    $Name = "ROSE-windows"
    $DirName = "ROSE"
    $Preset = "Windows Desktop"
    $ExportFlag = "--export-release"
    $Templates = @("windows_release_x86_64.exe")
}
if ($GameOnly) {
    $Name = "$Name-update"
}

Write-Host "==> Repository root: $(Get-Location)"
Write-Host "==> Godot: $Godot"
Write-Host "==> Output: $Out ($Name, $(if ($Folder) { 'folder' } else { 'zip' }))"
Write-Host "==> Cargo target: $TargetDir"
Write-Host "==> ROSE data: $DataIdx"

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

if (-not $DataIdx) {
    throw @"
The ROSE client data was not given.

Pass the client's data.idx:

  .\godot\tools\build-windows.ps1 -DataIdx C:\Path\To\ROSE-client\data.idx

or set:

  `$env:ROSE_DATA_IDX = "C:\Path\To\ROSE-client\data.idx"
"@
}

if (-not (Test-Path -PathType Leaf $DataIdx)) {
    throw "data.idx was not found: $DataIdx"
}
$DataIdx = (Resolve-Path $DataIdx).Path

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
# Export template (must match the editor's version)
# ------------------------------------------------------------

# godot.exe only hands back its output when piped (see the export below).
$GodotVersion = (& $Godot --version | Out-String).Trim()
if ($GodotVersion -notmatch '^(\d+\.\d+(\.\d+)?)\.([a-z0-9]+)') {
    throw "Could not read the Godot version from: $GodotVersion"
}
$TemplateVersion = "$($Matches[1]).$($Matches[3])"
$TemplateDir = Join-Path $env:APPDATA "Godot\export_templates\$TemplateVersion"
$Missing = @($Templates | Where-Object { -not (Test-Path (Join-Path $TemplateDir $_)) })

if ($Missing.Count -gt 0) {
    $Release = "$($Matches[1])-$($Matches[3])"
    $Url = "https://github.com/godotengine/godot/releases/download/$Release/Godot_v${Release}_export_templates.tpz"
    Write-Host ""
    Write-Host "==> Fetching $($Missing -join ', ') from $Url ..."

    $Tpz = Join-Path ([System.IO.Path]::GetTempPath()) "godot-templates-$([Guid]::NewGuid().ToString()).tpz"
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    $PreviousProgress = $ProgressPreference
    $ProgressPreference = "SilentlyContinue"
    try {
        Invoke-WebRequest -Uri $Url -OutFile $Tpz -UseBasicParsing
        Add-Type -AssemblyName System.IO.Compression.FileSystem
        New-Item -ItemType Directory -Force -Path $TemplateDir | Out-Null
        $Zip = [System.IO.Compression.ZipFile]::OpenRead($Tpz)
        try {
            foreach ($Template in $Missing) {
                $Entry = $Zip.GetEntry("templates/$Template")
                if (-not $Entry) {
                    throw "$Template is not in $Url"
                }
                [System.IO.Compression.ZipFileExtensions]::ExtractToFile(
                    $Entry, (Join-Path $TemplateDir $Template), $true)
            }
        }
        finally {
            $Zip.Dispose()
        }
    }
    finally {
        $ProgressPreference = $PreviousProgress
        if (Test-Path $Tpz) {
            Remove-Item -Force $Tpz
        }
    }
}

Write-Host "==> Templates: $TemplateDir ($($Templates -join ', '))"

# ------------------------------------------------------------
# Build Rust DLL
# ------------------------------------------------------------

Write-Host ""
Write-Host "==> Building rose-gd..."

$env:CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "gcc"
$env:CARGO_TARGET_DIR = $TargetDir

$Features = @()
if ($Dev) {
    $Features = @("--features", "dev")
}

cargo "+$GnuToolchain" build `
    --release `
    -p rose-gd `
    --target $RustTarget `
    @Features

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
# Convert the ROSE assets and check the server's files
# ------------------------------------------------------------

# godot.exe is a GUI-subsystem program, so PowerShell only waits for it
# when its output is piped. Godot also prints harmless ERROR lines to
# stderr, which Windows PowerShell 5.1 turns into terminating errors under
# "Stop", so relax that for the call and print each line as plain text.
# Returns Godot's exit code.
function Invoke-Godot {
    param([string[]]$Arguments)
    $PreviousPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    try {
        & $Godot @Arguments 2>&1 | ForEach-Object { Write-Host "$_" }
        return $LASTEXITCODE
    }
    finally {
        $ErrorActionPreference = $PreviousPreference
    }
}

# The converted assets go into assets.pck, so make sure they are all there
# and current. The project's own files are imported first, as on a fresh
# clone nothing is.
Write-Host ""
Write-Host "==> Importing the project..."
if ((Invoke-Godot @("--headless", "--path", "godot", "--import")) -ne 0) {
    throw "Godot import failed."
}

Write-Host ""
Write-Host "==> Converting the ROSE assets (--import-rose)..."
if ((Invoke-Godot @("--headless", "--path", "godot", "--", "--import-rose", "--data-idx=$DataIdx")) -ne 0) {
    throw "Converting the ROSE assets failed."
}

# The server times hits from overrides/motions.toml, so the animations
# shipped must match it.
Write-Host ""
Write-Host "==> Checking overrides\motions.toml..."
if ((Invoke-Godot @("--headless", "--path", "godot", "--", "--export-motions=check", "--data-idx=$DataIdx")) -ne 0) {
    throw @"
overrides\motions.toml doesn't match the player animations.

Run this and commit the file:

  godot --headless --path godot -- --export-motions --data-idx=$DataIdx
"@
}

# The server places spawns, NPCs and named spots from overrides/zones/,
# exported from the zone scenes.
Write-Host ""
Write-Host "==> Checking overrides\zones\..."
if ((Invoke-Godot @("--headless", "--path", "godot", "--", "--export-zones=check", "--data-idx=$DataIdx")) -ne 0) {
    throw @"
overrides\zones\ doesn't match the zone scenes in godot\zones\.

Run this and commit the files:

  godot --headless --path godot -- --export-zones --data-idx=$DataIdx
"@
}

# The server and the game read the items, monsters, NPCs and skills from overrides/data/, exported
# from the game data resources in godot/data/.
Write-Host ""
Write-Host "==> Checking overrides\data\..."
if ((Invoke-Godot @("--headless", "--path", "godot", "--", "--export-data=check")) -ne 0) {
    throw @"
overrides\data\ doesn't match the game data resources in godot\data\.

Run this and commit the files:

  godot --headless --path godot -- --export-data
"@
}

# ------------------------------------------------------------
# Create temporary staging directory
# ------------------------------------------------------------

Write-Host ""
Write-Host "==> Creating staging directory..."

$Stage = Join-Path ([System.IO.Path]::GetTempPath()) `
    "rose-windows-$([Guid]::NewGuid().ToString())"

$StageRose = Join-Path $Stage $DirName
$VersionFile = Join-Path (Get-Location) "godot\build_version.txt"

New-Item -ItemType Directory -Force -Path $StageRose | Out-Null

try {

    # --------------------------------------------------------
    # Export Godot project
    # --------------------------------------------------------

    Write-Host ""
    Write-Host "==> Exporting Godot Windows ($Preset)..."

    # The version line shows the commit the client was built from
    # (scripts/build_info.gd).
    # git's stderr would stop Windows PowerShell 5.1 under "Stop", see below.
    $Commit = "unknown"
    if (Get-Command git -ErrorAction SilentlyContinue) {
        $PreviousPreference = $ErrorActionPreference
        $ErrorActionPreference = "Continue"
        try {
            $GitOut = git rev-parse --short=7 HEAD 2>$null
            if ($LASTEXITCODE -eq 0 -and $GitOut) {
                $Commit = "$GitOut".Trim()
            }
        }
        finally {
            $ErrorActionPreference = $PreviousPreference
        }
    }
    [System.IO.File]::WriteAllText($VersionFile, "$Commit`n")

    # Godot keeps the binary copies of the text scenes it exported in
    # .godot\exported\ and reuses one while its own file is unchanged, even
    # when a scene it uses is new or changed. A zone scene exported before
    # --import-rose made its model scenes would keep empty decorations and a
    # grey ground, so convert everything again.
    $ExportCache = Join-Path (Get-Location) "godot\.godot\exported"
    if (Test-Path $ExportCache) {
        Remove-Item -Recurse -Force $ExportCache
    }

    # godot.exe is a GUI-subsystem program, so PowerShell only waits for it
    # when its output is piped. Godot also prints harmless ERROR lines to
    # stderr during a headless export, which Windows PowerShell 5.1 turns
    # into terminating errors under "Stop", so relax that for this call and
    # print each line as plain text.
    $ExportedExe = Join-Path $StageRose "ROSE.exe"
    $PreviousPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    try {
        & $Godot `
            --headless `
            --path godot `
            $ExportFlag $Preset `
            $ExportedExe 2>&1 | ForEach-Object { Write-Host "$_" }
        $GodotExit = $LASTEXITCODE
    }
    finally {
        $ErrorActionPreference = $PreviousPreference
    }

    # Godot can report a failed export with exit code 0, so also check
    # that the executable was written.
    if ($GodotExit -ne 0 -or -not (Test-Path $ExportedExe)) {
        throw "Godot export failed."
    }

    # The converted assets go in their own pack (the "ROSE assets"
    # preset), which the game opens next to ROSE.exe, so a game update
    # can leave them out.
    if (-not $GameOnly) {
        Write-Host ""
        Write-Host "==> Exporting assets.pck..."
        $AssetsPck = Join-Path $StageRose "assets.pck"
        $GodotExit = Invoke-Godot @("--headless", "--path", "godot", "--export-pack", "ROSE assets", $AssetsPck)
        if ($GodotExit -ne 0 -or -not (Test-Path $AssetsPck)) {
            throw "Exporting assets.pck failed."
        }
    }

    # --------------------------------------------------------
    # GM commands are for dev builds only: the production client
    # must have neither the GM chat scripts (godot/gm/) nor the
    # extension's gm_command. A file in the pck is listed as
    # "gm/gm_chat.gdc" and so on; the "res://gm/..." the pck also
    # lists are only the project's uid cache naming every file.
    # --------------------------------------------------------

    if (-not $Dev) {
        $Pck = [System.Text.Encoding]::ASCII.GetString(
            [System.IO.File]::ReadAllBytes((Join-Path $StageRose "ROSE.pck")))
        $GmFiles = @([regex]::Matches($Pck, '(?s)(.{6})gm/gm_[a-z_]+\.gd') |
            Where-Object { $_.Groups[1].Value -ne "res://" })
        $Dll = [System.Text.Encoding]::ASCII.GetString(
            [System.IO.File]::ReadAllBytes((Join-Path $StageRose "rose_gd.dll")))
        if ($GmFiles.Count -gt 0 -or $Dll.Contains("gm_command")) {
            throw "GM code got into the production build (godot/gm/ or gm_command); not packaging it."
        }
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

    # The items the game reads, the same export the server loads.
    $StageOverrides = Join-Path $StageRose "overrides"
    New-Item -ItemType Directory -Force -Path $StageOverrides | Out-Null
    Copy-Item `
        -Path (Join-Path (Get-Location) "overrides\data") `
        -Destination $StageOverrides `
        -Recurse

    # --------------------------------------------------------
    # Create output directory
    # --------------------------------------------------------

    New-Item `
        -ItemType Directory `
        -Force `
        -Path $Out | Out-Null

    if ($Folder) {

        # ----------------------------------------------------
        # Copy into the folder, keeping what isn't ours
        # ----------------------------------------------------

        $Dest = Join-Path $Out $Name
        New-Item -ItemType Directory -Force -Path $Dest | Out-Null
        Get-ChildItem -Path $Dest -File |
            Where-Object { $_.Extension -in ".exe", ".dll" -or $_.Name -in "README.txt", "ROSE.pck" -or ($_.Name -eq "assets.pck" -and -not $GameOnly) } |
            Remove-Item -Force
        $DestData = Join-Path $Dest "overrides\data"
        if (Test-Path $DestData) {
            Remove-Item -Recurse -Force $DestData
        }
        Copy-Item -Path (Join-Path $StageRose "*") -Destination $Dest -Recurse -Force

        Write-Host ""
        Write-Host "==> Build complete!"
        Write-Host ""
        Get-ChildItem $Dest | Format-Table Name, Length, LastWriteTime

    } else {

        $ZipPath = Join-Path $Out "$Name.zip"

        if (Test-Path $ZipPath) {
            Remove-Item -Force $ZipPath
        }

        # ----------------------------------------------------
        # Create ZIP
        # ----------------------------------------------------

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

}
finally {

    # --------------------------------------------------------
    # Cleanup
    # --------------------------------------------------------

    if (Test-Path $VersionFile) {
        Remove-Item -Force $VersionFile
    }

    if (Test-Path $Stage) {
        Remove-Item `
            -Recurse `
            -Force `
            $Stage
    }
}
