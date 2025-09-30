cargo build --release

if ($LASTEXITCODE -eq 0) {
    .\target\release\lm-compiler .\inputs\test.txt
} else {
    Write-Host "La compilación falló."
}