param(
    [ValidateSet("small", "medium", "large", "all")]
    [string]$Size = "all",
    [string]$Output = (Join-Path $PSScriptRoot "out"),
    [switch]$Force
)

$ErrorActionPreference = "Stop"

$sizes = [ordered]@{
    small = 8
    medium = 64
    large = 256
}

function New-Section {
    param(
        [string]$Id,
        [string[]]$Path,
        [int]$Level,
        [int]$Line,
        [System.Collections.IDictionary]$Metadata,
        [string]$Text
    )
    [ordered]@{
        id = $Id
        path = $Path
        level = $Level
        line = $Line
        metadata = $Metadata
        text = $Text
    }
}

function New-Fixture {
    param(
        [string]$Name,
        [int]$Count
    )

    $root = Join-Path $Output $Name
    if ((Test-Path $root) -and -not $Force) {
        throw "Fixture '$Name' already exists at $root. Re-run with -Force to replace it."
    }
    Remove-Item -Recurse -Force $root -ErrorAction SilentlyContinue

    $mdRoot = Join-Path $root "md"
    $pebbleRoot = Join-Path $root "pebble"
    New-Item -ItemType Directory -Path $mdRoot, $pebbleRoot | Out-Null

    $utf8NoBom = New-Object System.Text.UTF8Encoding($false)
    $task = "ready proof context boundary topic 0 cluster 0"
    $clusterSize = 16

    for ($i = 0; $i -lt $Count; $i++) {
        $topic = "topic-{0:D4}" -f $i
        $title = "Topic {0:D4}" -f $i
        $cluster = [math]::Floor($i / $clusterSize)
        $next = "topic-{0:D4}.md" -f (($i + 1) % $Count)
        $prev = "topic-{0:D4}.md" -f (($i + $Count - 1) % $Count)
        $hubIndex = [int]($cluster * $clusterSize)
        $hub = "topic-{0:D4}.md" -f $hubIndex
        $focus = if ($cluster -eq 0) {
            "ready proof context boundary seed evidence cluster zero benchmark"
        } else {
            "background corpus contrast material cluster $cluster benchmark"
        }

        $frontmatter = @(
            "---",
            "title: $title",
            "tags: [proof, benchmark, cluster-$cluster]",
            "status: ready",
            "version: 1.0",
            "---"
        ) -join "`n"
        $metadata = [ordered]@{
            title = $title
            tags = "[proof, benchmark, cluster-$cluster]"
            status = "ready"
            version = "1.0"
        }
        $intro = "# $title`n`n$focus. This source links to [next]($next), [previous]($prev), and [cluster hub]($hub)."
        $evidence = "## Evidence`n`nThe evidence paragraph for $title repeats the task vocabulary when relevant: $focus."
        $details = "## Details`n`nThe details paragraph gives CROP another local unit with stable source identity, section shape, and linkable provenance."
        $handoff = "## Handoff`n`nThe handoff paragraph describes how a selected crop should preserve enough context for an AI reader without carrying the whole corpus."
        $markdown = @($frontmatter, $intro, $evidence, $details, $handoff) -join "`n`n"

        [System.IO.File]::WriteAllText((Join-Path $mdRoot "$topic.md"), $markdown, $utf8NoBom)

        $refs = New-Object System.Collections.ArrayList
        foreach ($ref in @($next, $prev, $hub)) {
            if (-not $refs.Contains($ref)) {
                [void]$refs.Add($ref)
            }
        }
        $sections = New-Object System.Collections.ArrayList
        $contextText = @($intro, $evidence) -join "`n`n"
        $provenanceText = @(
            "## Provenance",
            "",
            "Source $topic belongs to cluster $cluster and preserves refs to $next, $prev, and $hub for context boundary reconstruction.",
            "The compact Pebble section keeps proof context provenance without carrying Markdown frontmatter as separate evidence."
        ) -join "`n"
        [void]$sections.Add((New-Section "$topic-context" @($title, "Context") 1 7 $metadata $contextText))
        [void]$sections.Add((New-Section "$topic-provenance" @($title, "Provenance") 2 15 $metadata $provenanceText))

        $document = [ordered]@{
            schema = "pebble.v1"
            kind = "document"
            title = $title
            source = "$topic.md"
            format = "markdown"
            metadata = $metadata
            sections = $sections
            refs = $refs
        }
        [System.IO.File]::WriteAllText(
            (Join-Path $pebbleRoot "$topic.pebble.json"),
            ($document | ConvertTo-Json -Depth 10 -Compress),
            $utf8NoBom
        )
    }

    $manifest = [ordered]@{
        schema_version = "crop.bench-fixture.v1"
        name = $Name
        document_count = $Count
        task = $task
        budget = 2000
        md_root = "md"
        pebble_root = "pebble"
    }
    [System.IO.File]::WriteAllText(
        (Join-Path $root "manifest.json"),
        ($manifest | ConvertTo-Json -Depth 5),
        $utf8NoBom
    )
}

New-Item -ItemType Directory -Path $Output -Force | Out-Null

$selected = if ($Size -eq "all") { $sizes.Keys } else { @($Size) }
foreach ($name in $selected) {
    New-Fixture $name $sizes[$name]
}
