#!/usr/bin/env powershell

<#
.SYNOPSIS
Installs a Cargo plugin to a directory suitable for caching

.DESCRIPTION
The script will run `cargo install <PLUGIN>` to target a non-default directory,
allowing a user to cache the installation directory for caching in a CI
context. The executables in the `bin` directory are linked back into
`$env:CARGO_HOME\bin` so that no further PATH manipulation is necessary.

.EXAMPLE
.\cargo-cache-install.ps1 cargo-make
#>

param (
    # The crate plugin to build and install
    [Parameter(Mandatory=$True)]
    [string]$Plugin
)

function main([string]$Plugin) {
    Install-Plugin "$Plugin" "$env:CARGO_HOME\opt\$plugin"
}

function Install-Plugin([string]$Plugin, [string]$Root) {
    if (-Not (Test-Path "$Root")) {
        New-Item -Type Directory "$Root" | Out-Null
    }

    Write-Debug "Installing $Plugin to $Root"
    rustup install stable
    cargo +stable install --root "$Root" --force --verbose "$Plugin"

    # Create symbolic links for all execuatbles into $env:CARGO_HOME\bin
    Get-ChildItem "$Root\bin\*.exe" | ForEach-Object {
        $dst = "$env:CARGO_HOME\bin\$($_.Name)"

        if (-Not (Test-Path "$dst")) {
            Write-Debug "Symlinking $_ to $dst"
            New-Item -Path "$dst" -Type SymbolicLink -Value "$_" | Out-Null
        }
    }
}

main "$Plugin"
