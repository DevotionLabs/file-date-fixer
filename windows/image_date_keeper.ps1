function Update-FileCreationDate {
    param (
        [System.IO.FileInfo]$file,
        [datetime]$parsedDate
    )

    try {
        $originalDate = $file.CreationTime

        $originalDateStr = $originalDate.ToString("yyyy-MM-dd")
        $parsedDateStr = $parsedDate.ToString("yyyy-MM-dd")

        Write-Host "Comparing original creation date ($originalDateStr) with parsed date ($parsedDateStr)" -ForegroundColor Gray

        if ($originalDateStr -eq $parsedDateStr) {
            Write-Host "Preserving time from original creation date for $($file.Name)" -ForegroundColor Gray
            return
        }

        $file.CreationTime = $parsedDate
        Write-Host "Updated creation date for $($file.Name) to $parsedDate" -ForegroundColor Green
    } catch {
        Write-Host "Failed to set creation date for $($file.Name)" -ForegroundColor Yellow
    }
}

function Process-FilesInDirectory {
    param (
        [string]$directoryPath
    )

    # Loop over all items in the directory
    Get-ChildItem -Path $directoryPath -File | ForEach-Object {
        Process-File -file $_
    }
}

function Process-File {
    param (
        [System.IO.FileInfo]$file
    )

    $fileName = $file.BaseName # Without extension

    $dateString = Get-DateFromFileName -fileName $fileName

    if (-not $dateString) {
        Write-Host "File $($file.Name) does not match any supported format." -ForegroundColor Yellow
        return
    }

    # Parse the date string into a DateTime object
    $parsedDate = Parse-DateString -dateString $dateString

    if (-not $parsedDate) {
        Write-Host "Failed to parse date string '$dateString' for file $($file.Name)" -ForegroundColor Yellow
        return
    }

    Update-FileCreationDate -file $file -parsedDate $parsedDate
}

function Get-DateFromFileName {
    param (
        [string]$fileName
    )

    $patterns = @(
        "IMG-(\d{8})-", # IMG-YYYYMMDD-<whatever>
        "IMG_(\d{8})", # IMG_YYYYMMDD<whatever>
        "Screenshot_(\d{4}-\d{2}-\d{2})-" # Screenshot_YYYY-MM-DD-<whatever>
    )

    foreach ($pattern in $patterns) {
        if ($fileName -match $pattern) {
            return $matches[1]
        }
    }

    return $null
}

function Parse-DateString {
    param (
        [string]$dateString
    )

    if ($dateString -match "^\d{8}$") {
        return [datetime]::ParseExact($dateString, "yyyyMMdd", $null)

    } elseif ($dateString -match "^\d{4}-\d{2}-\d{2}$") {
        return [datetime]::ParseExact($dateString, "yyyy-MM-dd", $null)
    }

    return $null
}

# Main script

$directoryPath = "." # TODO: Get path from command line arguments

if (-not (Test-Path $directoryPath)) {
    Write-Host "The specified directory does not exist!" -ForegroundColor Red
    exit
}


Process-FilesInDirectory -directoryPath $directoryPath
