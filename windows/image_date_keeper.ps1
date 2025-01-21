# Define the directory to process
$directoryPath = "."

if (-not (Test-Path $directoryPath)) {
    Write-Host "The specified directory does not exist!" -ForegroundColor Red
    exit
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

# Loop over all items in the directory
Get-ChildItem -Path $directoryPath -File | ForEach-Object {
    # Get the file name without extension
    $fileName = $_.BaseName

    # Extract the date string from the file name
    $dateString = Get-DateFromFileName -fileName $fileName

    if (-not $dateString) {
        Write-Host "File $($_.Name) does not match any supported format." -ForegroundColor Yellow
        continue
    }

    # Parse the date string into a DateTime object
    $parsedDate = Parse-DateString -dateString $dateString

    if (-not $parsedDate) {
        Write-Host "Failed to parse date string '$dateString' for file $($_.Name)" -ForegroundColor Yellow
        continue
    }

    try {
        $originalCreationDate = $_.CreationTime

        if ($originalCreationDate.ToString("yyyy-MM-dd") -eq $parsedDate.ToString("yyyy-MM-dd")) {
            Write-Host "Preserving time from original creation date for $($_.Name)" -ForegroundColor Gray
            continue
        }

        $_.CreationTime = $parsedDate
        Write-Host "Updated creation date for $($_.Name) to $parsedDate" -ForegroundColor Green
    } catch {
        Write-Host "Failed to set creation date for $($_.Name)" -ForegroundColor Yellow
    }
}