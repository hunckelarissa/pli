# Build pl1c, compile MR001, compare stdout to .expected
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
Set-Location $Root

$gcc = $env:PL1C_CC
if (-not $gcc) { $gcc = "gcc" }

Write-Host "== cargo build =="
cargo build -p pl1c
$pl1c = Join-Path $Root "target\debug\pl1c.exe"
if (-not (Test-Path $pl1c)) {
  $pl1c = Join-Path $Root "target\debug\pl1c"
}

$pli = Join-Path $Root "tests\must-run\L0_MR_001_hello.pli"
$exe = Join-Path $Root "tests\must-run\L0_MR_001_hello.exe"
$expectedPath = Join-Path $Root "tests\must-run\L0_MR_001_hello.expected"
$gotPath = Join-Path $Root "tests\must-run\L0_MR_001_hello.got"

Write-Host "== pl1c =="
& $pl1c $pli -o $exe --cc $gcc
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "== run =="
# Capture raw stdout bytes via redirection (avoid Out-String artifacts)
cmd /c "`"$exe`" > `"$gotPath`""
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$out = [System.IO.File]::ReadAllText($gotPath) -replace "`r`n", "`n" -replace "`r", "`n"
$exp = [System.IO.File]::ReadAllText($expectedPath) -replace "`r`n", "`n" -replace "`r", "`n"
if ($out -ne $exp) {
  Write-Host "FAIL stdout mismatch"
  Write-Host "--- expected ---"
  [System.BitConverter]::ToString([System.Text.Encoding]::UTF8.GetBytes($exp))
  Write-Host $exp
  Write-Host "--- actual ---"
  [System.BitConverter]::ToString([System.Text.Encoding]::UTF8.GetBytes($out))
  Write-Host $out
  exit 1
}
Write-Host "OK MR001 hello"
