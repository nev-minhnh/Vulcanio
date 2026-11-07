//! The six apps Vulcanio hosts, what they open, and how each is created.

use vulcanio_host::{HostContext, Lang, Module, ModuleStorage};
use serde::{Deserialize, Serialize};

/// One hosted app.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModuleId {
    Photo,
    Light,
    Vector,
    Design,
    Fig,
    Cad,
}

impl ModuleId {
    pub const ALL: [Self; 6] = [Self::Photo, Self::Light, Self::Vector, Self::Design, Self::Fig, Self::Cad];

    pub fn index(self) -> usize {
        match self {
            Self::Photo => 0,
            Self::Light => 1,
            Self::Vector => 2,
            Self::Design => 3,
            Self::Fig => 4,
            Self::Cad => 5,
        }
    }

    /// The id as typed on the command line (`--module photo`).
    pub fn key(self) -> &'static str {
        match self {
            Self::Photo => "photo",
            Self::Light => "light",
            Self::Vector => "vector",
            Self::Design => "design",
            Self::Fig => "fig",
            Self::Cad => "cad",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        let key = key.trim().to_ascii_lowercase();
        Self::ALL.into_iter().find(|m| m.key() == key)
    }

    pub fn spec(self) -> &'static Spec {
        &SPECS[self.index()]
    }
}

/// What Vulcanio knows about a hosted app.
pub struct Spec {
    pub id: ModuleId,
    /// Short name on the rail.
    pub short: [&'static str; 2],
    /// Name on the home screen.
    pub title: [&'static str; 2],
    /// One line on what it is for.
    pub blurb: [&'static str; 2],
    /// Label of its New action.
    pub new_label: [&'static str; 2],
    /// The accent of its card and rail icon (sRGB).
    pub accent: [u8; 3],
    /// File types this app is the natural home for: opened here without asking.
    pub primary: &'static [&'static str],
    /// Every file type it opens (File › Open, a drop, Open With).
    pub opens: &'static [&'static str],
    pub create: fn(&HostContext<'_>) -> Box<dyn Module>,
    pub storage: fn() -> ModuleStorage,
}

impl Spec {
    pub fn short(&self, lang: Lang) -> &'static str {
        self.short[lang_index(lang)]
    }
    pub fn title(&self, lang: Lang) -> &'static str {
        self.title[lang_index(lang)]
    }
    pub fn blurb(&self, lang: Lang) -> &'static str {
        self.blurb[lang_index(lang)]
    }
    pub fn new_label(&self, lang: Lang) -> &'static str {
        self.new_label[lang_index(lang)]
    }
    pub fn accent(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.accent[0], self.accent[1], self.accent[2])
    }
    pub fn opens_ext(&self, ext: &str) -> bool {
        self.opens.iter().any(|e| *e == ext)
    }
}

fn lang_index(lang: Lang) -> usize {
    match lang {
        Lang::En => 0,
        Lang::Vi => 1,
    }
}

fn no_storage() -> ModuleStorage {
    ModuleStorage::open(None)
}

pub static SPECS: [Spec; 6] = [
    Spec {
        id: ModuleId::Photo,
        short: ["Photo", "Ảnh"],
        title: ["Photo editor", "Chỉnh sửa ảnh"],
        blurb: [
            "Layers, masks, brushes, filters and PSD files: raster editing for photos and artwork.",
            "Lớp, mặt nạ, cọ vẽ, bộ lọc và tệp PSD: chỉnh sửa ảnh và tranh vẽ dạng điểm ảnh.",
        ],
        new_label: ["New image…", "Ảnh mới…"],
        accent: [0x3B, 0x82, 0xF6],
        primary: &["pcraft", "psd", "psb", "pdn", "ora", "kra", "krz", "exr", "hdr", "qoi", "ico", "tga", "avif"],
        opens: &[
            "pcraft", "psd", "psb", "pdn", "ora", "kra", "krz", "png", "jpg", "jpeg", "tif", "tiff", "webp", "gif", "bmp", "tga", "exr", "hdr",
            "avif", "qoi", "ico", "heic", "heif", "jxl", "dng", "cr2", "cr3", "nef", "arw", "raf", "orf", "rw2", "pef", "srw",
        ],
        create: photocraft_embed::create,
        storage: no_storage,
    },
    Spec {
        id: ModuleId::Light,
        short: ["Library", "Thư viện"],
        title: ["Photo library & RAW", "Thư viện ảnh & RAW"],
        blurb: [
            "Import, organise and develop RAW photos non-destructively, then export or print.",
            "Nhập, sắp xếp và xử lý ảnh RAW không phá hủy, rồi xuất hoặc in.",
        ],
        new_label: ["Import photos…", "Nhập ảnh…"],
        accent: [0xF5, 0x9E, 0x0B],
        primary: &[
            "dng", "cr2", "cr3", "nef", "nrw", "arw", "sr2", "srf", "raf", "orf", "rw2", "rwl", "raw", "pef", "srw", "iiq", "crw", "mrw", "x3f", "kdc",
            "mos", "erf", "3fr", "fff", "mef",
        ],
        opens: &[
            "jpg", "jpeg", "png", "tif", "tiff", "webp", "dng", "cr2", "cr3", "nef", "nrw", "arw", "sr2", "srf", "raf", "orf", "rw2", "rwl", "raw",
            "pef", "srw", "psd", "psb", "jxl", "gif", "bmp", "heic", "heif", "avif", "iiq", "crw", "mrw", "x3f", "kdc", "mos", "erf", "3fr", "fff",
            "mef",
        ],
        create: lightcraft_embed::create,
        storage: no_storage,
    },
    Spec {
        id: ModuleId::Vector,
        short: ["Vector", "Vector"],
        title: ["Vector illustration", "Minh họa vector"],
        blurb: [
            "Paths, shapes, type, gradients and symbols: illustration with SVG, PDF, AI and EPS.",
            "Đường dẫn, hình khối, chữ, dải màu và biểu tượng: minh họa với SVG, PDF, AI và EPS.",
        ],
        new_label: ["New artwork…", "Bản vẽ mới…"],
        accent: [0xEC, 0x48, 0x99],
        primary: &["vectorcraft", "drawcraft", "ai", "ait", "eps", "epsf", "ps", "svgz", "af", "afdesign", "emf", "wmf", "vctemplate", "svg"],
        opens: &[
            "vectorcraft", "drawcraft", "svg", "svgz", "pdf", "ai", "ait", "png", "pns", "jpg", "jpeg", "gif", "webp", "tif", "tiff", "bmp", "dib",
            "rle", "tga", "vda", "icb", "vst", "pcx", "pxr", "jpf", "jpx", "jp2", "j2k", "j2c", "jpc", "psd", "pdd", "psb", "vctemplate", "dxf",
            "emf", "wmf", "eps", "epsf", "ps", "af", "afdesign", "afpub",
        ],
        create: vectorcraft_embed::create,
        storage: no_storage,
    },
    Spec {
        id: ModuleId::Design,
        short: ["Layout", "Dàn trang"],
        title: ["Page layout", "Dàn trang"],
        blurb: [
            "Magazines, books and brochures: master pages, styles, threaded text, PDF and EPUB.",
            "Tạp chí, sách và tờ gấp: trang mẫu, kiểu chữ, khung chữ liên kết, xuất PDF và EPUB.",
        ],
        new_label: ["New document…", "Tài liệu mới…"],
        accent: [0xEF, 0x44, 0x44],
        primary: &["designcraft", "idml", "indd", "indt", "docx", "rtf", "md", "pdf"],
        opens: &["designcraft", "idml", "indd", "indt", "pdf", "docx", "rtf", "md", "txt", "xlsx", "svg", "ai", "eps"],
        create: designcraft_embed::create,
        storage: no_storage,
    },
    Spec {
        id: ModuleId::Fig,
        short: ["Interface", "Giao diện"],
        title: ["Interface design", "Thiết kế giao diện"],
        blurb: [
            "Frames, components, auto layout and prototypes: design apps and websites, together.",
            "Khung, thành phần, bố cục tự động và nguyên mẫu: cùng nhau thiết kế ứng dụng và web.",
        ],
        new_label: ["New design file", "Tệp thiết kế mới"],
        accent: [0x8B, 0x5C, 0xF6],
        primary: &["figcraft", "fig", "jam", "penpot", "sketch"],
        opens: &["figcraft", "fig", "jam", "penpot", "sketch", "json", "svg", "png", "jpg", "jpeg", "webp", "gif", "tif", "tiff"],
        create: figcraft_embed::create,
        storage: no_storage,
    },
    Spec {
        id: ModuleId::Cad,
        short: ["CAD", "CAD"],
        title: ["CAD drafting", "Thiết kế kỹ thuật CAD"],
        blurb: [
            "Precise 2D drafting with layers, blocks, dimensions and a command line: DWG and DXF.",
            "Vẽ kỹ thuật 2D chính xác với lớp, khối, kích thước và dòng lệnh: DWG và DXF.",
        ],
        new_label: ["New drawing…", "Bản vẽ mới…"],
        accent: [0x10, 0xB9, 0x81],
        primary: &["dwg", "dxf", "dwt"],
        opens: &["dwg", "dxf", "dwt", "svg", "svgz"],
        create: cadcraft_embed::create,
        storage: cadcraft_embed::storage,
    },
];

/// The modules that can open a file with this extension, the natural home first.
pub fn candidates(ext: &str) -> Vec<ModuleId> {
    let mut out: Vec<ModuleId> = SPECS.iter().filter(|s| s.primary.contains(&ext)).map(|s| s.id).collect();
    for s in &SPECS {
        if s.opens_ext(ext) && !out.contains(&s.id) {
            out.push(s.id);
        }
    }
    out
}

/// Every extension any module opens (for the Open dialog and file associations).
pub fn all_extensions() -> Vec<&'static str> {
    let mut v: Vec<&'static str> = SPECS.iter().flat_map(|s| s.opens.iter().copied()).collect();
    v.sort_unstable();
    v.dedup();
    v
}
