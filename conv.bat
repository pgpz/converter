@echo off
REM 

REM 
setlocal

set SCRIPT_DIR=%~dp0
set CONV_EXE=%SCRIPT_DIR%conv.exe

if not exist "%CONV_EXE%" (
    echo ERROR: conv.exe not found in %SCRIPT_DIR%
    exit /b 1
)

"%CONV_EXE%" %*
endlocal
