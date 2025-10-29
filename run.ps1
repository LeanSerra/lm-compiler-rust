cargo build --release

if ($LASTEXITCODE -eq 0) {
    .\target\release\lm-compiler .\inputs\test.txt
    dot -Tpng inputs/test.dot -o out.png
} else {
    Write-Host "La compilación falló."
}
