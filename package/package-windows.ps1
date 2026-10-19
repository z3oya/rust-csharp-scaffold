$ErrorActionPreference = "Stop"

$packageDir = $PSScriptRoot
$repoRoot = Split-Path -Parent $packageDir
$publishDir = Join-Path $packageDir "publish"
$portableRoot = Join-Path $packageDir "portable"
$distDir = Join-Path $packageDir "dist"

$props = [xml](Get-Content (Join-Path $repoRoot "Directory.Build.props"))
$version = $props.Project.PropertyGroup.Version
$portableDir = Join-Path $portableRoot "scaffold-$version"

$iscc = $null
try { $iscc = (Get-Command iscc -ErrorAction Stop).Source } catch {}
if (-not $iscc) {
    $candidates = @(
        "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe",
        "$env:ProgramFiles\Inno Setup 6\ISCC.exe",
        (Join-Path $env:USERPROFILE "scoop\apps\inno-setup\current\ISCC.exe")
    )
    $iscc = $candidates | Where-Object { Test-Path $_ } | Select-Object -First 1
}
if (-not $iscc) { throw "Inno Setup 6 (ISCC.exe) not found" }

Remove-Item -Recurse -Force -ErrorAction SilentlyContinue $publishDir
Remove-Item -Recurse -Force -ErrorAction SilentlyContinue $portableRoot
New-Item -ItemType Directory -Force -Path $distDir | Out-Null

$project = Join-Path $repoRoot "apps\wpf\Scaffold.Wpf.csproj"

dotnet publish $project -c Release -r win-x64 --self-contained false -o $publishDir
if ($LASTEXITCODE -ne 0) { throw "dotnet publish failed (installer)" }

& $iscc (Join-Path $packageDir "scaffold.iss")
if ($LASTEXITCODE -ne 0) { throw "Inno Setup compile failed" }

dotnet publish $project -c Release -r win-x64 --self-contained false -o $portableDir
if ($LASTEXITCODE -ne 0) { throw "dotnet publish failed (portable)" }

$zip = Join-Path $distDir "scaffold-$version-framework-portable.zip"
Compress-Archive -Path $portableDir -DestinationPath $zip -Force

Write-Host "installer: $(Join-Path $distDir "scaffold-$version-framework-setup.exe")"
Write-Host "portable:  $zip"
