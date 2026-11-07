; Krasis — Your creative oasis
; Inno Setup 6 script. Build krasis.exe first (packaging\windows\build.ps1), then:
;   iscc packaging\windows\krasis.iss
; Installs per user (no admin rights) or for all users, adds Start menu and desktop shortcuts,
; and registers Krasis in "Open with" and Settings › Apps › Default apps for every file type its
; apps open. Windows lets only the user pick the default app for a type.

#define AppName "Krasis"
#define AppVersion "1.1.0"
#define AppExe "krasis.exe"

[Setup]
AppId={{6E0F5B2C-4B8C-4D2B-9A1E-6B7C1C2D4A11}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppPublisher=Krasis
VersionInfoDescription=Krasis - Your creative oasis
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
ArchitecturesAllowed=x64compatible arm64
ArchitecturesInstallIn64BitMode=x64compatible arm64
MinVersion=10.0.22000
OutputDir=..\..\dist
OutputBaseFilename=Krasis-Setup-{#AppVersion}
SetupIconFile=..\..\apps\krasis\assets\krasis.ico
UninstallDisplayIcon={app}\{#AppExe}
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
ChangesAssociations=yes
LicenseFile=..\..\LICENSE-MIT

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
; Vietnamese is an unofficial Inno Setup translation: used when installed beside the compiler.
#if FileExists(AddBackslash(CompilerPath) + "Languages\Vietnamese.isl")
Name: "vietnamese"; MessagesFile: "compiler:Languages\Vietnamese.isl"
#endif

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"
Name: "assoc"; Description: "Open supported files with Krasis / Mở các tệp được hỗ trợ bằng Krasis"; GroupDescription: "Files / Tệp"

[Files]
Source: "..\..\target\release\{#AppExe}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\LICENSE-MIT"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\LICENSE-APACHE"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\NOTICE.md"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#AppName}"; Filename: "{app}\{#AppExe}"; Comment: "Krasis - Your creative oasis"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExe}"; Tasks: desktopicon

[Registry]
Root: HKA; Subkey: "Software\Classes\Krasis.Document"; ValueType: string; ValueName: ""; ValueData: "Krasis document"; Flags: uninsdeletekey; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Krasis.Document\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExe}"",0"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Krasis.Document\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExe}"" ""%1"""; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe"; ValueType: string; ValueName: "FriendlyAppName"; ValueData: "Krasis"; Flags: uninsdeletekey; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExe}"" ""%1"""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities"; ValueType: string; ValueName: "ApplicationName"; ValueData: "Krasis"; Flags: uninsdeletekey; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities"; ValueType: string; ValueName: "ApplicationDescription"; ValueData: "Krasis - Your creative oasis"; Tasks: assoc
Root: HKA; Subkey: "Software\RegisteredApplications"; ValueType: string; ValueName: "Krasis"; ValueData: "Software\Krasis\Capabilities"; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.3fr\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".3fr"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".3fr"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.af\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".af"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".af"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.afdesign\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".afdesign"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".afdesign"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.afpub\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".afpub"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".afpub"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.ai\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".ai"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".ai"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.ait\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".ait"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".ait"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.arw\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".arw"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".arw"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.avif\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".avif"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".avif"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.bmp\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".bmp"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".bmp"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.cr2\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".cr2"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".cr2"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.cr3\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".cr3"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".cr3"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.crw\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".crw"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".crw"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.designcraft\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".designcraft"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".designcraft"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.dib\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".dib"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".dib"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.dng\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".dng"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".dng"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.docx\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".docx"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".docx"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.drawcraft\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".drawcraft"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".drawcraft"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.dwg\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".dwg"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".dwg"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.dwt\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".dwt"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".dwt"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.dxf\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".dxf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".dxf"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.emf\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".emf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".emf"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.eps\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".eps"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".eps"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.epsf\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".epsf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".epsf"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.erf\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".erf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".erf"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.exr\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".exr"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".exr"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.fff\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".fff"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".fff"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.fig\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".fig"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".fig"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.figcraft\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".figcraft"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".figcraft"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.gif\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".gif"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".gif"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.hdr\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".hdr"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".hdr"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.heic\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".heic"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".heic"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.heif\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".heif"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".heif"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.icb\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".icb"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".icb"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.ico\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".ico"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".ico"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.idml\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".idml"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".idml"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.iiq\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".iiq"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".iiq"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.indd\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".indd"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".indd"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.indt\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".indt"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".indt"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.j2c\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".j2c"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".j2c"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.j2k\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".j2k"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".j2k"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.jam\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".jam"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".jam"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.jp2\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".jp2"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".jp2"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.jpc\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".jpc"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".jpc"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.jpeg\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".jpeg"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".jpeg"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.jpf\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".jpf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".jpf"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.jpg\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".jpg"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".jpg"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.jpx\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".jpx"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".jpx"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.json\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".json"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".json"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.jxl\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".jxl"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".jxl"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.kdc\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".kdc"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".kdc"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.kra\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".kra"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".kra"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.krz\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".krz"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".krz"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.md\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".md"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".md"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.mef\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".mef"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".mef"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.mos\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".mos"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".mos"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.mrw\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".mrw"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".mrw"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.nef\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".nef"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".nef"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.nrw\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".nrw"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".nrw"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.ora\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".ora"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".ora"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.orf\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".orf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".orf"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.pcraft\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".pcraft"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pcraft"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.pcx\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".pcx"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pcx"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.pdd\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".pdd"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pdd"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.pdf\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".pdf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pdf"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.pdn\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".pdn"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pdn"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.pef\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".pef"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pef"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.penpot\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".penpot"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".penpot"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.png\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".png"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".png"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.pns\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".pns"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pns"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.ps\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".ps"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".ps"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.psb\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".psb"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".psb"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.psd\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".psd"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".psd"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.pxr\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".pxr"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pxr"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.qoi\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".qoi"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".qoi"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.raf\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".raf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".raf"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.raw\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".raw"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".raw"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.rle\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".rle"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".rle"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.rtf\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".rtf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".rtf"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.rw2\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".rw2"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".rw2"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.rwl\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".rwl"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".rwl"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.sketch\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".sketch"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".sketch"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.sr2\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".sr2"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".sr2"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.srf\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".srf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".srf"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.srw\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".srw"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".srw"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.svg\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".svg"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".svg"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.svgz\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".svgz"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".svgz"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.tga\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".tga"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".tga"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.tif\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".tif"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".tif"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.tiff\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".tiff"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".tiff"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.txt\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".txt"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".txt"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.vctemplate\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".vctemplate"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".vctemplate"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.vda\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".vda"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".vda"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.vectorcraft\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".vectorcraft"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".vectorcraft"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.vst\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".vst"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".vst"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.webp\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".webp"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".webp"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.wmf\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".wmf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".wmf"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.x3f\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".x3f"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".x3f"; ValueData: "Krasis.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.xlsx\OpenWithProgids"; ValueType: string; ValueName: "Krasis.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\krasis.exe\SupportedTypes"; ValueType: string; ValueName: ".xlsx"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Krasis\Capabilities\FileAssociations"; ValueType: string; ValueName: ".xlsx"; ValueData: "Krasis.Document"; Tasks: assoc

[Run]
Filename: "{app}\{#AppExe}"; Description: "{cm:LaunchProgram,{#AppName}}"; Flags: nowait postinstall skipifsilent
