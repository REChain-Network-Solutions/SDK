# PowerShell script to update all company branding references
Write-Host "Updating all company branding references to REChain Network Solutions LLC..."

# Update Cargo.toml files
$tomlFiles = Get-ChildItem -Path "." -Name "*.toml" -Recurse
foreach ($file in $tomlFiles) {
    Write-Host "Updating company references in $file..."
    $content = Get-Content $file.FullName -Raw -ErrorAction SilentlyContinue
    if ($content) {
        # Update author references
        $content = $content -replace "Parity Technologies <admin@parity.io>", "REChain Network Solutions LLC <info@rechain.network>"
        $content = $content -replace "paritytech", "rechain-network"
        $content = $content -replace "parity.io", "rechain.network"
        $content = $content -replace "security@parity.io", "info@rechain.network"
        Set-Content -Path $file.FullName -Value $content
    }
}

# Update README files
$readmeFiles = Get-ChildItem -Path "." -Name "README.md" -Recurse
foreach ($file in $readmeFiles) {
    Write-Host "Updating company references in $file..."
    $content = Get-Content $file.FullName -Raw -ErrorAction SilentlyContinue
    if ($content) {
        $content = $content -replace "paritytech", "rechain-network"
        $content = $content -replace "parity.io", "rechain.network"
        $content = $content -replace "Polkadot", "Rechain"
        Set-Content -Path $file.FullName -Value $content
    }
}

# Update other documentation files
$docFiles = Get-ChildItem -Path "." -Name "*.md" -Recurse
foreach ($file in $docFiles) {
    Write-Host "Updating company references in $file..."
    $content = Get-Content $file.FullName -Raw -ErrorAction SilentlyContinue
    if ($content) {
        $content = $content -replace "paritytech", "rechain-network"
        $content = $content -replace "parity.io", "rechain.network"
        Set-Content -Path $file.FullName -Value $content
    }
}

Write-Host "Company branding update completed successfully!"