param(
    [switch]$DryRun
)

$ErrorActionPreference = "Stop"

$tools = @(
    "cargo-deny",
    "cargo-audit",
    "cargo-cyclonedx",
    "release-plz",
    "cargo-machete"
)

function Invoke-Step {
    param(
        [string[]]$Arguments
    )

    if ($DryRun) {
        Write-Host "+ $($Arguments -join ' ')"
        return
    }

    $command = $Arguments[0]
    $remaining = @()

    if ($Arguments.Length -gt 1) {
        $remaining = $Arguments[1..($Arguments.Length - 1)]
    }

    & $command @remaining

    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "cargo is required to bootstrap dev tools."
}

foreach ($tool in $tools) {
    Invoke-Step -Arguments @("cargo", "install", "--locked", $tool)
}
