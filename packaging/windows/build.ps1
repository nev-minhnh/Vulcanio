# Build Krasis for Windows 11 and, if Inno Setup is installed, its installer.
#
#   powershell -ExecutionPolicy Bypass -File packaging\windows\build.ps1 [-WithFonts]
#
# Needs: Rust 1.95 or newer (rustup, MSVC toolchain) and the Visual Studio 2022 Build Tools with
# "Desktop development with C++" (the MSVC linker and Windows SDK). Inno Setup 6 is optional.
param([switch]$WithFonts)
$ErrorActionPreference = "Stop"
$root = Resolve-Path "$PSScriptRoot\..\.."
Set-Location $root

$env:CARGO_TERM_COLOR = "always"
# -WithFonts: embed the Craft apps' CJK and extra fonts (storytold/craft-fonts), as their releases
# do. Without it, Latin and Vietnamese text render normally; Chinese and Japanese show as boxes.
if ($WithFonts) {
    $fonts = Join-Path $root "vendor\craft-fonts"
    if (-not (Test-Path $fonts)) { git clone --depth 1 https://github.com/storytold/craft-fonts $fonts }
    $env:CRAFT_FONTS_DIR = $fonts
}
# A release build of six large apps: give the linker room and use every core.
cargo build --release -p krasis
if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }

$exe = Join-Path $root "target\release\krasis.exe"
Write-Host "Built $exe" -ForegroundColor Green

$iscc = @(
    "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe",
    "$env:ProgramFiles\Inno Setup 6\ISCC.exe",
    "$env:LOCALAPPDATA\Programs\Inno Setup 6\ISCC.exe"
) | Where-Object { Test-Path $_ } | Select-Object -First 1
if ($iscc) {
    & $iscc (Join-Path $PSScriptRoot "krasis.iss")
    if ($LASTEXITCODE -ne 0) { throw "Inno Setup failed" }
    Write-Host "Installer in $(Join-Path $root 'dist')" -ForegroundColor Green
} else {
    Write-Host "Inno Setup 6 not found: skipped the installer (winget install JRSoftware.InnoSetup)." -ForegroundColor Yellow
}
