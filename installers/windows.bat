@echo off

set URL=https://github.com/Jupiee/ezpie/releases/download/Latest/ezpie.exe
set DESTINATION=C:\Users\%USERNAME%\AppData\Local\Microsoft\WindowsApps

curl -L "%URL%" -o "ezpie.exe"

move "ezpie.exe" "%DESTINATION%"