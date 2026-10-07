@echo off
cd /d "%~dp0"
title ROSE server
echo ROSE server on port 3000. Leave this window open while people play; close it to stop.
echo The first time, run install-game.bat once while this window is open.
spacetimedb-standalone.exe start --listen-addr 0.0.0.0:3000 --data-dir "%~dp0data" --jwt-pub-key-path "%~dp0keys\id_ecdsa.pub" --jwt-priv-key-path "%~dp0keys\id_ecdsa.p8"
pause
