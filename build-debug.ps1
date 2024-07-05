# Path to the .env file
$envFilePath = ".\src\main.rs"

# Read the content of the .env file
$content = Get-Content -Path $envFilePath

# Update the DEPLOYMENT_ENV variable to LOCAL
$content = $content -replace 'pub const DEPLOYMENT_ENV: &str = .*', 'pub const DEPLOYMENT_ENV: &str = "local";'

# Write the updated content back to the .env file
Set-Content -Path $envFilePath -Value $content

# Run the trunk serve command
trunk serve
