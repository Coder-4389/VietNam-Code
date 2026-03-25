# 1. Lấy ngày giờ để đặt tên Commit (Ví dụ: vnc-250326)
$date = Get-Date -Format "ddMMyy"
$msg = "vnc-$date"

Write-Host "--- DANG DON DEP TRANG THAI GIT ---" -ForegroundColor Cyan
# 2. Hủy bỏ mọi lệnh Rebase/Merge đang bị kẹt (Fix lỗi 'already a rebase-merge directory')
git rebase --abort 2>$null
git merge --abort 2>$null

Write-Host "--- DANG KIEM TRA FILE THAY DOI ---" -ForegroundColor Cyan
# 3. Thêm tất cả file (bao gồm cả parser.rs mới đổi tên)
git add .

# 4. Ghi lại thay đổi (Commit)
# Dùng --allow-empty để tránh lỗi nếu không có gì thay đổi so với lần trước
git commit -m "$msg" --allow-empty

Write-Host "--- DANG DAY CODE LEN GITHUB PUBLIC ---" -ForegroundColor Cyan
# 5. Dùng FORCE PUSH để ghi đè GitHub bằng code ở máy (Sửa lỗi 'rejected' và 'non-fast-forward')
git push origin main --force

if ($LASTEXITCODE -eq 0) {
    Write-Host "`n==========================================" -ForegroundColor Green
    Write-Host "      push project have was completed       " -ForegroundColor Green
    Write-Host "==========================================`n" -ForegroundColor Green
} else {
    Write-Host "have an error" -ForegroundColor Red
}