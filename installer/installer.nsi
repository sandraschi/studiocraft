; StudioCraft current-user installer (no admin UAC).
!define APPNAME "StudioCraft"
!define APPVERSION "0.1.0"
!define BINARY "studiocraft-app.exe"
Name "${APPNAME} ${APPVERSION}"
OutFile "studiocraft-0.1.0-setup.exe"
InstallDir "$LOCALAPPDATA\StudioCraft"
RequestExecutionLevel user
Page directory
Page instfiles
Section "Install"
  SetOutPath "$INSTDIR"
  File "..\target\release\${BINARY}"
  File "..\target\release\studiocraft-cli.exe"
  WriteUninstaller "$INSTDIR\uninstall.exe"
SectionEnd
Section "Uninstall"
  Delete "$INSTDIR\${BINARY}"
  Delete "$INSTDIR\studiocraft-cli.exe"
  Delete "$INSTDIR\uninstall.exe"
  RMDir "$INSTDIR"
SectionEnd
