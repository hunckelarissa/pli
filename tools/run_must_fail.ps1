# Run tests/must-fail/L0_MF_*.pli — expect non-zero exit from pl1c --parse-only
$ErrorActionPreference = "Continue"
$Root = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
Set-Location $Root

cargo build -p pl1c | Out-Host
$pl1c = Join-Path $Root "target\debug\pl1c.exe"
if (-not (Test-Path $pl1c)) { $pl1c = Join-Path $Root "target\debug\pl1c" }

$failed = 0
$dir = Join-Path $Root "tests\must-fail"
if (-not (Test-Path $dir)) {
  Write-Host "No must-fail tests"
  exit 0
}
Get-ChildItem (Join-Path $dir "L0_MF_*.pli") -ErrorAction SilentlyContinue | Sort-Object Name | ForEach-Object {
  $pli = $_.FullName
  $base = $_.BaseName
  $errFile = Join-Path $dir "$base.err"
  cmd /c "`"$pl1c`" `"$pli`" --parse-only >NUL 2> `"$errFile`""
  $code = $LASTEXITCODE
  $err = if (Test-Path $errFile) { [IO.File]::ReadAllText($errFile) } else { "" }
  if ($code -eq 0) {
    Write-Host "FAIL $base (expected error, got success)"
    $script:failed++
  } elseif ($err -notmatch "not implemented") {
    Write-Host "FAIL $base (missing not-implemented diagnostic)"
    Write-Host $err
    $script:failed++
  } else {
    Write-Host "OK   $base"
  }
}
if ($failed -gt 0) { exit 1 }
Write-Host "All must-fail OK"
