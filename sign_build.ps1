# =========================================================================
# Justino Studio: Code Signing Script (Windows Authenticode)
# =========================================================================
# Este script automatiza o processo de assinatura digital do seu executável.
# Quando você comprar o seu Certificado de Desenvolvedor Windows, basta rodar 
# este script para garantir que o Windows Defender não bloqueie sua IDE!

param(
    [string]$CertThumbprint = "COLOQUE_SEU_THUMBPRINT_AQUI",
    [string]$ExePath = ".\justino-studio.exe",
    [string]$TimestampServer = "http://timestamp.digicert.com"
)

Write-Host "Iniciando compilação de Produção..." -ForegroundColor Cyan
Set-Location .\justino_ide\desktop
cargo build --release

Write-Host "Copiando binário para a raiz..." -ForegroundColor Cyan
Copy-Item target\release\justino-studio.exe ..\..\justino-studio.exe -Force
Set-Location ..\..

if ($CertThumbprint -eq "COLOQUE_SEU_THUMBPRINT_AQUI") {
    Write-Host "[AVISO] O Thumbprint do certificado não foi configurado." -ForegroundColor Yellow
    Write-Host "O executável foi gerado, mas NÃO foi assinado." -ForegroundColor Yellow
    Write-Host "Compre um certificado (SignPath, DigiCert) para evitar alertas do SmartScreen." -ForegroundColor Yellow
    exit
}

Write-Host "Assinando o executável Justino Studio..." -ForegroundColor Cyan
# Requer o Windows SDK SignTool instalado
$SignToolPath = "C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64\signtool.exe"

if (Test-Path $SignToolPath) {
    & $SignToolPath sign /sha1 $CertThumbprint /fd SHA256 /tr $TimestampServer /td SHA256 $ExePath
    Write-Host "✅ Assinatura concluída com sucesso!" -ForegroundColor Green
} else {
    Write-Host "❌ Signtool.exe não encontrado. Instale o Windows SDK." -ForegroundColor Red
}
