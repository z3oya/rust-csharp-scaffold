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
ChangesEnvironment=yes
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
  AddPathCheckBox: TNewCheckBox;
  AutostartCheckBox: TNewCheckBox;

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

  AddPathCheckBox := TNewCheckBox.Create(WizardForm);
  AddPathCheckBox.Parent := WizardForm.FinishedPage;
  AddPathCheckBox.Left := WizardForm.FinishedLabel.Left;
  AddPathCheckBox.Top := WizardForm.FinishedLabel.Top + ScaleY(118);
  AddPathCheckBox.Width := WizardForm.FinishedLabel.Width;
  AddPathCheckBox.Height := ScaleY(17);
  AddPathCheckBox.Caption := 'Add install directory to PATH';

  AutostartCheckBox := TNewCheckBox.Create(WizardForm);
  AutostartCheckBox.Parent := WizardForm.FinishedPage;
  AutostartCheckBox.Left := WizardForm.FinishedLabel.Left;
  AutostartCheckBox.Top := WizardForm.FinishedLabel.Top + ScaleY(140);
  AutostartCheckBox.Width := WizardForm.FinishedLabel.Width;
  AutostartCheckBox.Height := ScaleY(17);
  AutostartCheckBox.Caption := 'Start automatically when Windows starts';
end;

procedure AddToPath();
var
  AppDir: string;
  OrigPath: string;
begin
  AppDir := ExpandConstant('{app}');
  if RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', OrigPath) then
  begin
    if Pos(';' + Uppercase(AppDir) + ';', ';' + Uppercase(OrigPath) + ';') = 0 then
      RegWriteExpandStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', AppDir + ';' + OrigPath);
  end
  else
    RegWriteExpandStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', AppDir);
end;

procedure AddToAutostart();
begin
  RegWriteStringValue(HKEY_CURRENT_USER,
    'Software\Microsoft\Windows\CurrentVersion\Run', '{#MyAppName}',
    '"' + ExpandConstant('{app}\{#MyAppExe}') + '"');
end;

function NextButtonClick(CurPageID: Integer): Boolean;
begin
  Result := True;
  if (CurPageID = wpFinished) and (not WizardSilent()) then
  begin
    if DesktopIconCheckBox.Checked then
      CreateShellLink(
        ExpandConstant('{autodesktop}\{#MyAppName}.lnk'),
        '{#MyAppName}',
        ExpandConstant('{app}\{#MyAppExe}'),
        '',
        ExpandConstant('{app}'),
        '', 0, SW_SHOWNORMAL);
    if AddPathCheckBox.Checked then
      AddToPath();
    if AutostartCheckBox.Checked then
      AddToAutostart();
  end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  Path: string;
  AppDir: string;
begin
  if CurUninstallStep = usUninstall then
  begin
    DeleteFile(ExpandConstant('{autodesktop}\{#MyAppName}.lnk'));
    RegDeleteValue(HKEY_CURRENT_USER, 'Software\Microsoft\Windows\CurrentVersion\Run', '{#MyAppName}');
    AppDir := ExpandConstant('{app}');
    if RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path) then
    begin
      StringChangeEx(Path, ';' + AppDir, '', True);
      StringChangeEx(Path, AppDir + ';', '', True);
      if Uppercase(Path) = Uppercase(AppDir) then
        Path := '';
      RegWriteExpandStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path);
    end;
  end;
end;
