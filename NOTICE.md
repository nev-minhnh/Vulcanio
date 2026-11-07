# Notices

Krasis hosts six open-source apps, each included here unchanged except for a small embedding
crate (`upstream/<app>/apps/<app>-embed`) and the changes listed below. Each is licensed
MIT OR Apache-2.0; their own `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE` and `ATTRIBUTION.md`/`ASSETS.md`
files are kept in `upstream/<app>/`.

| App | Source | Copyright |
|---|---|---|
| PhotoCraft | https://github.com/storytold/photocraft | ArtCraft Team and the PhotoCraft contributors |
| LightCraft | https://github.com/storytold/lightcraft | ArtCraft Team and the LightCraft contributors |
| VectorCraft | https://github.com/storytold/vectorcraft | ArtCraft Team and the VectorCraft contributors |
| DesignCraft | https://github.com/storytold/designcraft | ArtCraft Team and the DesignCraft contributors |
| FigCraft | https://github.com/storytold/figcraft | ArtCraft Team and the FigCraft contributors |
| CADCraft | https://github.com/storytold/cadcraft | ArtCraft Team and the CADCraft contributors |

The egui fork in `vendor/egui` is https://github.com/storytold/egui (MIT OR Apache-2.0, the egui
authors), with four additions for Krasis: `CreationContext::for_embedded`,
`Context::retain_viewport_commands`, `Context::effective_font_definitions` and a configurable
mouse-wheel easing time.

PhotoCraft, LightCraft, VectorCraft, DesignCraft, FigCraft, CADCraft and ArtCraft are names of
their owners; Krasis uses them only to credit the apps it is built on. Krasis draws its own icons
and does not ship the apps' brand icons as its own.

## Changes to the hosted apps

- Display names: inside Krasis the apps are called Krasis Photo, Krasis Library, Krasis Vector,
  Krasis Layout, Krasis Interface and Krasis CAD; their icons are replaced by Krasis' and their
  brand artwork (`docs/brand`) and packaging are removed. File formats, extensions, settings folders
  and code names are unchanged, so files and settings from the original apps keep working.
- Community links (Discord, websites, GitHub) and the apps' own self-updaters are removed from the
  interface.
- Startup: LightCraft opens without its demo library; FigCraft indexes system fonts on demand;
  VectorCraft installs its fonts when it first opens.

- DesignCraft: a Vietnamese interface catalog (`crates/ui-egui/src/i18n/vi.rs`) and its entry in
  the language list.
- LightCraft: the embedding crate draws with wgpu only (no OpenGL `glow` window renderer), since
  Krasis owns one wgpu window for every app.
