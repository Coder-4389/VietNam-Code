$date = Get-Date -Format "ddMMyy"
$msg = "vnc-$date"

git add .

git commit -m "$msg"

git push origin main
