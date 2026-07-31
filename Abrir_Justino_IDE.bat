@echo off
title Justino Studio IDE (.jucode)
echo Launching Self-Hosting Justino Studio IDE (.jucode + .css)...
start msedge --app="file:///%~dp0justino_ide\ui\index.html" --window-size=1280,800
