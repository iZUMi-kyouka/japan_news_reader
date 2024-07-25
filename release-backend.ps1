# Define source and target directories
$sourceDir = ".\dist"
$targetDir = "C:\Users\Tanu Aryanto Tan\RustProjects\aninfo_server\aninfo-server\assets_nhkreader"
$indexFilePath = Join-Path -Path $targetDir -ChildPath "index.html"

# Ensure the target directory exists
if (!(Test-Path -Path $targetDir)) {
    New-Item -ItemType Directory -Path $targetDir
}

# Delete all files in the target directory
Get-ChildItem -Path $targetDir -Recurse | Remove-Item -Force

# Copy all files from the source directory to the target directory
Copy-Item -Path "$sourceDir\*" -Destination $targetDir -Recurse

# Get the filename of the sole .wasm file in the target directory
$wasmFile = Get-ChildItem -Path $targetDir -Filter *.wasm | Select-Object -First 1

if ($wasmFile) {
    $wasmFilePath = $wasmFile.FullName
    $optimizedWasmFileName = "OPT_" + $wasmFile.Name
    $optimizedWasmFilePath = Join-Path -Path $targetDir -ChildPath $optimizedWasmFileName

    # Run the wasm-opt command
    wasm-opt -Oz -o $optimizedWasmFilePath $wasmFilePath
} else {
    Write-Host "No .wasm file found in the target directory."
}
