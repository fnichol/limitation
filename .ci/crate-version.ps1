#!/usr/bin/env powershell

<#
.SYNOPSIS
Prints latest version of a Cargo crate

.DESCRIPTION
The script prints the latest version string of the given crate.

.EXAMPLE
.\crate-version.ps1 cargo-make
#>

Param(
    # The crate name
    [Parameter(Mandatory=$True)]
    [String[]]
    $Crate
)

function main([string]$Crate) {
    Get-CrateVersion "$Crate"
}

function Get-CrateVersion([string]$Crate) {
    (cargo search --limit 1 --quiet "$Crate" | Select-Object -First 1).
        Split('"')[1]
}

main "$Crate"
