# Run all tests/must-parse/L0_MP_*.pli with pl1c --parse-only
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
Set-Location $Root
$env:Path = "C:\Strawberry\c\bin;" + $env:Path

cargo build -p pl1c | Out-Host
$pl1c = Join-Path $Root "target\debug\pl1c.exe"
if (-not (Test-Path $pl1c)) { $pl1c = Join-Path $Root "target\debug\pl1c" }

$failed = 0
Get-ChildItem (Join-Path $Root "tests\must-parse\L0_MP_*.pli") -ErrorAction SilentlyContinue | Sort-Object Name | ForEach-Object {
  & $pl1c $_.FullName --parse-only
  if ($LASTEXITCODE -ne 0) {
    Write-Host "FAIL $($_.BaseName)"
    $script:failed++
  } else {
    Write-Host "OK   $($_.BaseName)"
  }
}
if ($failed -gt 0) { exit 1 }
Write-Host "All must-parse OK"
