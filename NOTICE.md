# Notices

Vulcanio hosts six open-source apps, each included here unchanged except for a small embedding
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
authors), with these additions for Vulcanio: `CreationContext::for_embedded`,
`Context::retain_viewport_commands`, `Context::effective_font_definitions`, a configurable
mouse-wheel easing time, and an optional slide-in for panels as they appear
(`set_panel_appear_seconds`, `suppress_panel_appear`).

PhotoCraft, LightCraft, VectorCraft, DesignCraft, FigCraft, CADCraft and ArtCraft are names of
their owners; Vulcanio uses them only to credit the apps it is built on. Vulcanio does not ship the
apps' logos or brand artwork: every app icon file in `upstream/<app>/assets/app-icon` is replaced by
the Vulcanio logo, and the apps' own icon artwork (SVG sources and their notes) is removed.

## Changes to the hosted apps

- Display names: inside Vulcanio the apps are called Vulcanio Photo, Vulcanio Library, Vulcanio Vector,
  Vulcanio Layout, Vulcanio Interface and Vulcanio CAD; their icons are replaced by Vulcanio's and their
  brand artwork (`docs/brand`, icon SVGs) and packaging are removed. File formats, extensions, settings folders
  and code names are unchanged, so files and settings from the original apps keep working.
- Community links (Discord, websites, GitHub) and the apps' own self-updaters and update checks
  are removed from the interface.
- Startup: every app starts on its own start page — Vulcanio Photo and Vulcanio Vector show their home
  screens, Vulcanio Interface shows Home without making a new file, Vulcanio CAD shows its Start page
  instead of a blank drawing, and Vulcanio Library starts on its library chooser.
- LightCraft opens without its demo library; FigCraft indexes system fonts on demand;
  VectorCraft installs its fonts when it first opens.

- DesignCraft: a Vietnamese interface catalog (`crates/ui-egui/src/i18n/vi.rs`) and its entry in
  the language list.
- LightCraft: the embedding crate draws with wgpu only (no OpenGL `glow` window renderer), since
  Vulcanio owns one wgpu window for every app.
