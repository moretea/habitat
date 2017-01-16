@echo off
"%~dp0powershell/powershell.exe" -NoProfile -ExecutionPolicy bypass -NoLogo -NoExit -Command ". '%~dp0hab-studio.ps1' %*"
