# Définir le chemin du répertoire principal où sont stockés les projets
$projectsDir = "D:/Travail/Rust project/the_rust_programming_language/projects"

# Trouver et supprimer tous les dossiers .git
Get-ChildItem -Path $projectsDir -Recurse -Directory -Filter ".git" | ForEach-Object { Remove-Item $_.FullName -Recurse -Force }

Write-Host "Tous les dossiers .git ont été supprimés dans le répertoire $projectsDir."
