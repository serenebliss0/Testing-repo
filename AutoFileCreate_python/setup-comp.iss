; =====================================================
; AutoFileCreate Installer - FINAL VERSION
; =====================================================

#define MyAppName "AutoFileCreate (for Jupyter Notebook)"
#define MyAppVersion "3.0"
#define MyAppPublisher "Semire Designs and Animations"
#define MyAppURL "https://github.com/serenebliss0"
#define MyAppExeName "autofilecreate-py.exe"

[Setup]

AppId={{0094FBBB-67B3-4CD7-9522-D4A363F412FD}}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}

DefaultDirName={autopf}\AutoFileCreate_py
UninstallDisplayIcon={app}\{#MyAppExeName}

ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible

DisableProgramGroupPage=yes

OutputBaseFilename=autofilecreate-py
SolidCompression=yes
WizardStyle=modern dynamic windows11

SetupIconFile=C:\Users\Semire\Downloads\My Programming Journey 3\Testing-repo\AutoFileCreate_python\autofilecreate-py.ico

InfoBeforeFile=C:\Users\Semire\Downloads\My Programming Journey 3\Testing-repo\AutoFileCreate_python\before_info.txt
InfoAfterFile=C:\Users\Semire\Downloads\My Programming Journey 3\Testing-repo\AutoFileCreate_python\after_info.txt

PrivilegesRequired=admin


[Languages]

Name: "english"; MessagesFile: "compiler:Default.isl"


[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]

Source: "C:\Users\Semire\Downloads\My Programming Journey 3\Testing-repo\AutoFileCreate_python\dist\autofilecreate-py.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "C:\Users\Semire\Downloads\My Programming Journey 3\Testing-repo\AutoFileCreate_python\autofilecreate-py.ico"; DestDir: "{app}"; Flags: ignoreversion


[Icons]

[Icons]
Name: "{autoprograms}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\{#MyAppExeName}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon; IconFilename: "{app}\{#MyAppExeName}"


[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "Launch AutoFileCreate"; Flags: nowait postinstall skipifsilent
; =====================================================
; 🔥 PATH AUTOMATION
; =====================================================

[Registry]
Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; Flags: preservestringtype uninsdeletekeyifempty uninsdeletevalue; Check: NeedsAddPath(ExpandConstant('{app}'))


[Code]
function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', OrigPath) then
  begin
    Result := True;
    exit;
  end;
  Result := Pos(';' + Param + ';', ';' + OrigPath + ';') = 0;
end;