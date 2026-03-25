$date = Get-Date -Format "ddMMyy"
$msg = "vnc-$date"

git add .
git commit -m "$msg"
git pull origin main --rebase
git push origin main
