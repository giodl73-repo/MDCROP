param(
    [ValidateSet("small", "medium", "large", "all")]
    [string]$Size = "all",
    [int]$Runs = 5,
    [string]$Output = (Join-Path $PSScriptRoot "out")
)

$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
$generator = Join-Path $PSScriptRoot "generate.ps1"

if (-not (Test-Path $Output)) {
    & $generator -Size $Size -Output $Output -Force
}

$sizes = if ($Size -eq "all") { @("small", "medium", "large") } else { @($Size) }
$rows = New-Object System.Collections.ArrayList

Push-Location $repoRoot
try {
    foreach ($name in $sizes) {
        $fixtureRoot = Join-Path $Output $name
        if (-not (Test-Path $fixtureRoot)) {
            & $generator -Size $name -Output $Output -Force
        }
        $manifest = Get-Content (Join-Path $fixtureRoot "manifest.json") -Raw | ConvertFrom-Json
        foreach ($mode in @("md", "mdport")) {
            $relativeRoot = if ($mode -eq "md") { $manifest.md_root } else { $manifest.mdport_root }
            $root = Join-Path $fixtureRoot $relativeRoot
            $measurements = New-Object System.Collections.ArrayList
            for ($i = 0; $i -lt $Runs; $i++) {
                $json = cargo run -q -- bench --root $root --task $manifest.task --budget $manifest.budget | ConvertFrom-Json
                [void]$measurements.Add($json)
            }
            $byteCount = [int64]((Get-ChildItem $root -File | Measure-Object Length -Sum).Sum)
            [void]$rows.Add([pscustomobject]@{
                size = $name
                mode = $mode
                documents = $manifest.document_count
                bytes = $byteCount
                source_count = $measurements[0].source_count
                unit_count = $measurements[0].unit_count
                edge_count = $measurements[0].edge_count
                selected_unit_count = $measurements[0].selected_unit_count
                selected_token_estimate = $measurements[0].selected_token_estimate
                boundary_cut_edges = $measurements[0].boundary_cut_edges
                boundary_conductance = $measurements[0].boundary_conductance
                relevance_mean = [math]::Round((($measurements | Measure-Object relevance_mean -Average).Average), 6)
                quality_score = [math]::Round((($measurements | Measure-Object quality_score -Average).Average), 6)
                ingest_ms_avg = [math]::Round((($measurements | ForEach-Object { $_.timings.ingest_ms } | Measure-Object -Average).Average), 3)
                score_ms_avg = [math]::Round((($measurements | ForEach-Object { $_.timings.score_ms } | Measure-Object -Average).Average), 3)
                crop_ms_avg = [math]::Round((($measurements | ForEach-Object { $_.timings.crop_ms } | Measure-Object -Average).Average), 3)
                total_ms_avg = [math]::Round((($measurements | ForEach-Object { $_.timings.total_ms } | Measure-Object -Average).Average), 3)
                total_ms_min = ($measurements | ForEach-Object { $_.timings.total_ms } | Measure-Object -Minimum).Minimum
                total_ms_max = ($measurements | ForEach-Object { $_.timings.total_ms } | Measure-Object -Maximum).Maximum
            })
        }
    }
}
finally {
    Pop-Location
}

$rows | ConvertTo-Json -Depth 5
