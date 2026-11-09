; Vulcanio — The all-in-one studio
; Inno Setup 6 script. Build vulcanio.exe first (packaging\windows\build.ps1), then:
;   iscc packaging\windows\vulcanio.iss
; Installs per user (no admin rights) or for all users, adds Start menu and desktop shortcuts,
; and registers Vulcanio in "Open with" and Settings › Apps › Default apps for every file type its
; apps open. Windows lets only the user pick the default app for a type.

#define AppName "Vulcanio"
#define AppVersion "1.3.0"
#define AppExe "vulcanio.exe"

[Setup]
AppId={{6E0F5B2C-4B8C-4D2B-9A1E-6B7C1C2D4A11}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppPublisher=Vulcanio
VersionInfoDescription=Vulcanio - The all-in-one studio
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
ArchitecturesAllowed=x64compatible arm64
ArchitecturesInstallIn64BitMode=x64compatible arm64
MinVersion=10.0.22000
OutputDir=..\..\dist
OutputBaseFilename=Vulcanio-Setup-{#AppVersion}
SetupIconFile=..\..\apps\vulcanio\assets\vulcanio.ico
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
Name: "assoc"; Description: "Open supported files with Vulcanio / Mở các tệp được hỗ trợ bằng Vulcanio"; GroupDescription: "Files / Tệp"

[Files]
Source: "..\..\target\release\{#AppExe}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\LICENSE-MIT"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\LICENSE-APACHE"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\NOTICE.md"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#AppName}"; Filename: "{app}\{#AppExe}"; Comment: "Vulcanio - The all-in-one studio"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExe}"; Tasks: desktopicon

[Registry]
; Startup boost: Vulcanio adds this sign-in entry itself (Settings > Startup); remove it on uninstall.
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: none; ValueName: "Vulcanio Startup Boost"; Flags: uninsdeletevalue dontcreatekey
Root: HKA; Subkey: "Software\Classes\Vulcanio.Document"; ValueType: string; ValueName: ""; ValueData: "Vulcanio document"; Flags: uninsdeletekey; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Vulcanio.Document\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExe}"",0"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Vulcanio.Document\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExe}"" ""%1"""; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe"; ValueType: string; ValueName: "FriendlyAppName"; ValueData: "Vulcanio"; Flags: uninsdeletekey; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExe}"" ""%1"""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities"; ValueType: string; ValueName: "ApplicationName"; ValueData: "Vulcanio"; Flags: uninsdeletekey; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities"; ValueType: string; ValueName: "ApplicationDescription"; ValueData: "Vulcanio - The all-in-one studio"; Tasks: assoc
Root: HKA; Subkey: "Software\RegisteredApplications"; ValueType: string; ValueName: "Vulcanio"; ValueData: "Software\Vulcanio\Capabilities"; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.3fr\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".3fr"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".3fr"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.af\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".af"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".af"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.afdesign\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".afdesign"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".afdesign"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.afpub\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".afpub"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".afpub"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.ai\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".ai"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".ai"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.ait\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".ait"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".ait"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.arw\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".arw"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".arw"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.avif\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".avif"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".avif"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.bmp\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".bmp"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".bmp"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.cr2\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".cr2"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".cr2"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.cr3\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".cr3"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".cr3"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.crw\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".crw"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".crw"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.designcraft\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".designcraft"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".designcraft"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.dib\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".dib"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".dib"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.dng\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".dng"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".dng"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.docx\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".docx"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".docx"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.drawcraft\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".drawcraft"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".drawcraft"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.dwg\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".dwg"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".dwg"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.dwt\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".dwt"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".dwt"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.dxf\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".dxf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".dxf"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.emf\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".emf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".emf"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.eps\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".eps"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".eps"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.epsf\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".epsf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".epsf"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.erf\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".erf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".erf"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.exr\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".exr"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".exr"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.fff\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".fff"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".fff"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.fig\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".fig"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".fig"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.figcraft\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".figcraft"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".figcraft"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.gif\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".gif"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".gif"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.hdr\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".hdr"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".hdr"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.heic\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".heic"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".heic"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.heif\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".heif"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".heif"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.icb\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".icb"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".icb"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.ico\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".ico"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".ico"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.idml\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".idml"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".idml"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.iiq\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".iiq"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".iiq"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.indd\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".indd"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".indd"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.indt\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".indt"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".indt"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.j2c\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".j2c"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".j2c"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.j2k\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".j2k"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".j2k"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.jam\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".jam"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".jam"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.jp2\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".jp2"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".jp2"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.jpc\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".jpc"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".jpc"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.jpeg\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".jpeg"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".jpeg"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.jpf\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".jpf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".jpf"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.jpg\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".jpg"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".jpg"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.jpx\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".jpx"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".jpx"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.json\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".json"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".json"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.jxl\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".jxl"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".jxl"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.kdc\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".kdc"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".kdc"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.kra\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".kra"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".kra"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.krz\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".krz"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".krz"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.md\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".md"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".md"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.mef\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".mef"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".mef"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.mos\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".mos"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".mos"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.mrw\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".mrw"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".mrw"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.nef\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".nef"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".nef"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.nrw\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".nrw"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".nrw"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.ora\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".ora"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".ora"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.orf\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".orf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".orf"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.pcraft\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".pcraft"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pcraft"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.pcx\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".pcx"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pcx"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.pdd\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".pdd"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pdd"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.pdf\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".pdf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pdf"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.pdn\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".pdn"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pdn"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.pef\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".pef"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pef"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.penpot\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".penpot"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".penpot"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.png\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".png"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".png"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.pns\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".pns"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pns"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.ps\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".ps"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".ps"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.psb\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".psb"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".psb"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.psd\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".psd"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".psd"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.pxr\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".pxr"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pxr"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.qoi\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".qoi"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".qoi"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.raf\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".raf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".raf"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.raw\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".raw"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".raw"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.rle\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".rle"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".rle"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.rtf\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".rtf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".rtf"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.rw2\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".rw2"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".rw2"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.rwl\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".rwl"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".rwl"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.sketch\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".sketch"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".sketch"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.sr2\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".sr2"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".sr2"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.srf\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".srf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".srf"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.srw\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".srw"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".srw"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.svg\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".svg"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".svg"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.svgz\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".svgz"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".svgz"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.tga\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".tga"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".tga"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.tif\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".tif"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".tif"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.tiff\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".tiff"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".tiff"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.txt\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".txt"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".txt"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.vctemplate\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".vctemplate"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".vctemplate"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.vda\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".vda"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".vda"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.vectorcraft\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".vectorcraft"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".vectorcraft"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.vst\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".vst"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".vst"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.webp\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".webp"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".webp"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.wmf\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".wmf"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".wmf"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.x3f\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".x3f"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".x3f"; ValueData: "Vulcanio.Document"; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\.xlsx\OpenWithProgids"; ValueType: string; ValueName: "Vulcanio.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc
Root: HKA; Subkey: "Software\Classes\Applications\vulcanio.exe\SupportedTypes"; ValueType: string; ValueName: ".xlsx"; ValueData: ""; Tasks: assoc
Root: HKA; Subkey: "Software\Vulcanio\Capabilities\FileAssociations"; ValueType: string; ValueName: ".xlsx"; ValueData: "Vulcanio.Document"; Tasks: assoc

[Run]
Filename: "{app}\{#AppExe}"; Description: "{cm:LaunchProgram,{#AppName}}"; Flags: nowait postinstall skipifsilent
