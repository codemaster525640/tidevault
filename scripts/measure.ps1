$root = Split-Path (Split-Path $PSScriptRoot -Parent) -Parent
if (-not (Test-Path (Join-Path $PSScriptRoot "..\Cargo.toml"))) {
  $root = Resolve-Path (Join-Path $PSScriptRoot "..")
} else {
  $root = Resolve-Path (Join-Path $PSScriptRoot "..")
}
function Is-Test([string]$p) {
  return ($p.Replace('\','/').ToLower() -match '/tests/')
}
$src = Get-ChildItem $root -Recurse -Filter *.rs | Where-Object {
  $_.FullName -notmatch '\\target\\|\\vendor\\|\\scripts\\' -and -not (Is-Test $_.FullName)
}
$test = Get-ChildItem $root -Recurse -Filter *.rs | Where-Object {
  $_.FullName -notmatch '\\target\\|\\vendor\\' -and (Is-Test $_.FullName)
}
$sl = 0; foreach ($f in $src) { $sl += @(Get-Content $f.FullName).Count }
$tl = 0; foreach ($f in $test) { $tl += @(Get-Content $f.FullName).Count }
$bugs = @(Get-ChildItem (Join-Path $root "tests\tests") -Filter "bug_*.rs" -EA SilentlyContinue).Count
$mods = @(Get-ChildItem (Join-Path $root "domains") -Directory).Count
Write-Output "src_files=$($src.Count)"
Write-Output "src_loc=$sl"
Write-Output "test_files=$($test.Count)"
Write-Output "test_loc=$tl"
Write-Output "bug_scenarios=$bugs"
Write-Output "domain_modules=$mods"
