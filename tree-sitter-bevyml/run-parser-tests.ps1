<#
.SYNOPSIS
Runs the Tree-sitter grammar tests against every fixture under the parse directory.
#>

$scriptRoot = $PSScriptRoot
if (-not $scriptRoot) {
  $scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Definition
}

Push-Location $scriptRoot
try {
  $treeSitterCmd = Get-Command tree-sitter -ErrorAction SilentlyContinue
  if (-not $treeSitterCmd) {
    Write-Host "tree-sitter CLI not found in PATH." -ForegroundColor Red
    Write-Host "Please install the tree-sitter CLI and ensure it is accessible before running this script." -ForegroundColor Yellow
    exit 1
  }

  Write-Host "tree-sitter CLI found at $($treeSitterCmd.Path)." -ForegroundColor Green

  Write-Host ""
  Write-Host "Running `tree-sitter generate`..." -ForegroundColor Cyan
  $generateOutput = & tree-sitter generate 2>&1
  $generateExit = $LASTEXITCODE
  if ($generateExit -ne 0) {
    Write-Host "tree-sitter generate failed with exit code $generateExit." -ForegroundColor Red
    Write-Host $generateOutput
  }
  else {
    Write-Host "tree-sitter generate completed successfully." -ForegroundColor Green
  }

  $parseDir = Join-Path $scriptRoot "parse"
  $allHtml = Get-ChildItem -Path $parseDir -File -Filter '*.html' | Where-Object { -not $_.PSIsContainer }

  if (-not $allHtml) {
    Write-Host "No HTML fixtures found under $parseDir." -ForegroundColor Yellow
    exit 1
  }

  $orderedFiles = @()
  $indexFile = $allHtml | Where-Object { $_.Name -ieq 'index.html' }
  if ($indexFile) {
    $orderedFiles += $indexFile
  }

  $otherFiles = $allHtml | Where-Object { $_.Name -ine 'index.html' }
  $otherFilesSorted = $otherFiles | Sort-Object -Property @{
    Expression = {
      if ($PSItem.Name -match '^(\d+)') {
        [int]$Matches[1]
      }
      else {
        [int]::MaxValue
      }
    }
  }
  $orderedFiles += $otherFilesSorted

  Write-Host ""
  Write-Host "=== Parser Test Targets ===" -ForegroundColor Cyan
  for ($i = 0; $i -lt $orderedFiles.Count; $i++) {
    $prefix = '{0,2}.' -f ($i + 1)
    Write-Host " $prefix $($orderedFiles[$i].Name)"
  }

  $results = @()
  foreach ($file in $orderedFiles) {
    Write-Host ""
    Write-Host "Running: tree-sitter parse $($file.Name)" -ForegroundColor DarkCyan
    $lineCount = (Get-Content -LiteralPath $file.FullName | Measure-Object -Line).Lines
    $stopwatch = [System.Diagnostics.Stopwatch]::StartNew()
    $parseOutput = & tree-sitter parse $file.FullName 2>&1
    $stopwatch.Stop()
    $parseExit = $LASTEXITCODE
    $results += [pscustomobject]@{
      File = $file.Name
      Path = $file.FullName
      ExitCode = $parseExit
      Duration = $stopwatch.Elapsed
      SizeBytes = $file.Length
      SizeKb = [math]::Round($file.Length / 1KB, 2)
      LineCount = $lineCount
      Output = $parseOutput
    }
  }

  Write-Host ""
  Write-Host "=== Parse Results ===" -ForegroundColor Cyan
  $rowFormat = "{0,-12} {1,-32} {2,10} {3,8} {4,12}"
  $maxFileNameLength = 32
  Write-Host ($rowFormat -f "Status", "File", "SizeKB", "Lines", "Duration ms")
  Write-Host ($rowFormat -f "------", "----", "------", "-----", "-----------")
  foreach ($result in $results) {
    $durationMs = [math]::Round($result.Duration.TotalMilliseconds, 3)
    $statusLabel = if ($result.ExitCode -eq 0) { "[PASS]" } else { "[FAIL $($result.ExitCode)]" }
    $sizeDisplay = "{0:N2}" -f $result.SizeKb
    $displayFile = if ($result.File.Length -gt $maxFileNameLength) {
      $result.File.Substring(0, $maxFileNameLength - 3) + '...'
    }
    else {
      $result.File
    }
    $row = $rowFormat -f $statusLabel, $displayFile, $sizeDisplay, $result.LineCount, $durationMs
    if ($result.ExitCode -eq 0) {
      Write-Host $row -ForegroundColor Green
    }
    else {
      Write-Host $row -ForegroundColor Red
      if ($result.Output) {
        Write-Host $result.Output
      }
    }
  }

  Write-Host ""
  Write-Host "Parser test run complete. Have a wonderful day and happy Bevy building!" -ForegroundColor Yellow
}
finally {
  Pop-Location
}
