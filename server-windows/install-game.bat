@echo off
cd /d "%~dp0"
echo Installing the ROSE game rules on this server (start-server.bat must be running).
echo Running it again later installs a newer version and keeps characters.
spacetimedb-cli.exe publish --server http://127.0.0.1:3000 --bin-path rose_stdb_module.wasm -y rose
pause
