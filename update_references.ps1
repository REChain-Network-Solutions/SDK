# PowerShell script to update all polkadot references to rechain
Write-Host "Updating all polkadot references to rechain..."

# Get all .toml files recursively
$tomlFiles = Get-ChildItem -Path "." -Name "*.toml" -Recurse

foreach ($file in $tomlFiles) {
    Write-Host "Processing $file..."
    $content = Get-Content $file -Raw

    # Replace polkadot/ with rechain/
    $content = $content -replace "polkadot/", "rechain/"

    # Replace polkadot- with rechain-
    $content = $content -replace "polkadot-", "rechain-"

    # Replace polkadot_ with rechain_ (for logo files)
    $content = $content -replace "polkadot_", "rechain_"

    # Replace polkadot. with rechain. (for URLs and domains)
    $content = $content -replace "polkadot\.", "rechain."

    # Write back the updated content
    Set-Content -Path $file -Value $content
}

Write-Host "All references updated successfully!"