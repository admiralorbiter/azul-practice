# Build script for WASM module (PowerShell)
# Usage: .\build-wasm.ps1 [-Mode dev|release]

param(
    [ValidateSet("dev", "release")]
    [string]$Mode = "dev"
)

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir
$RustDir = Join-Path (Join-Path $ProjectRoot "rust") "engine"
$OutDir = Join-Path (Join-Path (Join-Path (Join-Path $ProjectRoot "web") "src") "wasm") "pkg"

# Check if wasm-pack is installed
if (-not (Get-Command wasm-pack -ErrorAction SilentlyContinue)) {
    Write-Host "Error: wasm-pack is not installed." -ForegroundColor Red
    Write-Host "Install it with: cargo install wasm-pack" -ForegroundColor Yellow
    exit 1
}

# Determine build target
$Target = if ($Mode -eq "release") { "--release" } else { "--dev" }

Write-Host "Building WASM in $Mode mode..." -ForegroundColor Cyan

Set-Location $RustDir

# Build with wasm-pack
wasm-pack build --target web --out-dir $OutDir $Target

Write-Host "WASM build complete! Output: $OutDir" -ForegroundColor Green
