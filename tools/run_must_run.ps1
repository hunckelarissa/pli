# Run all tests/must-run/L0_MR_*.pli against sibling .expected
# Optional sibling .exitcode (integer; default 0) checked against process exit.
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
Set-Location $Root
$env:Path = "C:\Strawberry\c\bin;" + $env:Path
if (-not $env:PL1C_CC) { $env:PL1C_CC = "gcc" }

cargo build -p pl1c | Out-Host
$pl1c = Join-Path $Root "target\debug\pl1c.exe"
if (-not (Test-Path $pl1c)) { $pl1c = Join-Path $Root "target\debug\pl1c" }

$failed = 0
Get-ChildItem (Join-Path $Root "tests\must-run\L0_MR_*.pli") | Sort-Object Name | ForEach-Object {
  $pli = $_.FullName
  $base = [IO.Path]::GetFileNameWithoutExtension($pli)
  $dir = $_.DirectoryName
  $exe = Join-Path $dir "$base.exe"
  $exp = Join-Path $dir "$base.expected"
  $got = Join-Path $dir "$base.got"
  $exitPath = Join-Path $dir "$base.exitcode"
  if (-not (Test-Path $exp)) {
    Write-Host "SKIP $base (no .expected)"
    return
  }
  $wantExit = 0
  if (Test-Path $exitPath) {
    $wantExit = [int]([IO.File]::ReadAllText($exitPath).Trim())
  }
  & $pl1c $pli -o $exe --cc $env:PL1C_CC
  if ($LASTEXITCODE -ne 0) {
    Write-Host "FAIL $base (compile)"
    $script:failed++
    return
  }
  $stdin = Join-Path $dir "$base.stdin"
  $psi = @{
    FilePath = $exe
    RedirectStandardOutput = $got
    NoNewWindow = $true
    Wait = $true
    PassThru = $true
  }
  if (Test-Path $stdin) {
    $psi.RedirectStandardInput = $stdin
  }
  $proc = Start-Process @psi
  $rc = $proc.ExitCode
  if ($null -eq $rc) { $rc = 0 }
  $out = [IO.File]::ReadAllText($got) -replace "`r`n", "`n" -replace "`r", "`n"
  $e = [IO.File]::ReadAllText($exp) -replace "`r`n", "`n" -replace "`r", "`n"
  if ($rc -ne $wantExit) {
    Write-Host "FAIL $base (exit $rc want $wantExit)"
    $script:failed++
  } elseif ($out -ne $e) {
    Write-Host "FAIL $base (stdout)"
    $script:failed++
  } else {
    Write-Host "OK   $base"
  }
}
if ($failed -gt 0) { exit 1 }
Write-Host "All must-run OK"
