# PowerShell script to update CI configuration files for rechain rebranding
Write-Host "Updating CI configuration files..."

# Update .gitlab-ci.yml if it exists
if (Test-Path ".gitlab-ci.yml") {
    Write-Host "Updating .gitlab-ci.yml..."
    $content = Get-Content ".gitlab-ci.yml" -Raw
    $content = $content -replace "polkadot", "rechain"
    Set-Content -Path ".gitlab-ci.yml" -Value $content
}

# Update any other CI-related files
$ciFiles = @(
    ".github/workflows/*.yml",
    ".github/workflows/*.yaml",
    "scripts/ci/*.sh",
    "scripts/ci/**/*.sh"
)

foreach ($pattern in $ciFiles) {
    $files = Get-ChildItem -Path $pattern -ErrorAction SilentlyContinue
    foreach ($file in $files) {
        Write-Host "Updating $($file.FullName)..."
        $content = Get-Content $file.FullName -Raw
        $content = $content -replace "polkadot", "rechain"
        Set-Content -Path $file.FullName -Value $content
    }
}

Write-Host "CI configuration files updated successfully!"