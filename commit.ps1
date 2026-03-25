$ErrorActionPreference = "SilentlyContinue"

$date = Get-Date -Format "ddMMyy"
$msg = "vnc-$date"

Write-Host "--- Cleaning git's status ---" -ForegroundColor Cyan
git rebase --abort 2>$null
git merge --abort 2>$null

Write-Host "--- Check file change ---" -ForegroundColor Cyan
git add . 2>$null

git commit -m "$msg" --allow-empty 2>&1

Write-Host "--- Push the project to GitHub ---" -ForegroundColor Cyan
git push origin main --force 2>&1

if ($LASTEXITCODE -eq 0) {
    Write-Host "`n==========================================" -ForegroundColor Green
    Write-Host "      PUSH PROJECT HAS COMPLETED!         " -ForegroundColor Green
    Write-Host "==========================================`n" -ForegroundColor Green
} else {
    Write-Host "`n[!] REAL ERROR DETECTED. CHECK CONNECTION." -ForegroundColor Red
} $ErrorActionPreference = "Continue"