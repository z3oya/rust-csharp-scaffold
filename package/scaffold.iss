#define MyAppName "rust-csharp-scaffold"
#define MyAppVerFull GetFileVersionString("publish\Scaffold.Wpf.dll")
#define MyAppVersion Copy(MyAppVerFull, 1, RPos(".", MyAppVerFull) - 1)
#define MyAppExe "Scaffold.Wpf.exe"

[Setup]
AppId={{8C4E9A72-6B3D-4E8A-9F27-3D5A1E2C7B84}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=yes
UninstallDisplayIcon={app}\{#MyAppExe}
OutputDir=dist
OutputBaseFilename=scaffold-{#MyAppVersion}-framework-setup
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
PrivilegesRequiredOverridesAllowed=dialog
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
Source: "publish\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExe}"
Name: "{group}\Uninstall {#MyAppName}"; Filename: "{uninstallexe}"

[Code]
var
  DesktopIconCheckBox: TNewCheckBox;

procedure InitializeWizard();
begin
  DesktopIconCheckBox := TNewCheckBox.Create(WizardForm);
  DesktopIconCheckBox.Parent := WizardForm.FinishedPage;
  DesktopIconCheckBox.Left := WizardForm.FinishedLabel.Left;
  DesktopIconCheckBox.Top := WizardForm.FinishedLabel.Top + ScaleY(96);
  DesktopIconCheckBox.Width := WizardForm.FinishedLabel.Width;
  DesktopIconCheckBox.Height := ScaleY(17);
  DesktopIconCheckBox.Caption := ExpandConstant('{cm:CreateDesktopIcon}');
  DesktopIconCheckBox.Checked := True;
end;

function NextButtonClick(CurPageID: Integer): Boolean;
begin
  Result := True;
  if (CurPageID = wpFinished) and (not WizardSilent()) and DesktopIconCheckBox.Checked then
  begin
    CreateShellLink(
      ExpandConstant('{autodesktop}\{#MyAppName}.lnk'),
      '{#MyAppName}',
      ExpandConstant('{app}\{#MyAppExe}'),
      '',
      ExpandConstant('{app}'),
      '', 0, SW_SHOWNORMAL);
  end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usUninstall then
    DeleteFile(ExpandConstant('{autodesktop}\{#MyAppName}.lnk'));
end;
