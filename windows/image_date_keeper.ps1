
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
        "IMG-(\d{8})-",          # Matches IMG-YYYYMMDD-<whatever>
        "IMG_(\d{8})",          # Matches IMG_YYYYMMDD<whatever>
        "Screenshot_(\d{4}-\d{2}-\d{2})-" # Matches Screenshot_YYYY-MM-DD-<whatever>
    )

    foreach ($pattern in $patterns) {
        if ($fileName -match $pattern) {
            return $matches[1]  # Return the matched date string
        }
    }

    return $null
}

function Parse-DateString {
    param (
        [string]$dateString
    )

    # Try parsing as "YYYYMMDD"
    if ($dateString -match "^\d{8}$") {
        return [datetime]::ParseExact($dateString, "yyyyMMdd", $null)
    }
    # Try parsing as "YYYY-MM-DD"
    elseif ($dateString -match "^\d{4}-\d{2}-\d{2}$") {
        return [datetime]::ParseExact($dateString, "yyyy-MM-dd", $null)
    }

    # Return null if parsing fails
    return $null
}

# Loop over all items in the directory
Get-ChildItem -Path $directoryPath -File | ForEach-Object {
    # Get the file name without extension
    $fileName = $_.BaseName

    # Extract the date string from the file name
    $dateString = Get-DateFromFileName -fileName $fileName

    if(-not $dateString) {
        Write-Host "File $($_.Name) does not match any supported format." -ForegroundColor Yellow
        continue
    }

    # Parse the date string into a DateTime object
    $creationDate = Parse-DateString -dateString $dateString

    if(-not $creationDate) {
    	Write-Host "Failed to parse date string '$dateString' for file $($_.Name)" -ForegroundColor Yellow
    	continue
    }

    try {
        # Set the creation time of the file
        $_.CreationTime = $creationDate
        Write-Host "Updated creation date for $($_.Name) to $creationDate" -ForegroundColor Green
    } catch {
        Write-Host "Failed to set creation date for $($_.Name)" -ForegroundColor Yellow
    }
}