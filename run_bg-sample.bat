@echo off

set exe_path="C:\path_to_exe\restart_3day.exe"
powershell -Command "Start-Process -FilePath '%exe_path%' -WindowStyle Hidden"
