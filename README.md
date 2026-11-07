# Krasis — Your creative oasis

*Krasis — Ốc đảo sáng tạo của bạn*

**Phiên bản 1.1.0**

Krasis gộp sáu ứng dụng sáng tạo mã nguồn mở viết bằng Rust vào **một ứng dụng, một cửa sổ** cho
Windows 11:

| Ứng dụng trong Krasis | Dùng để |
|---|---|
| Krasis Photo — Chỉnh sửa ảnh | Lớp, mặt nạ, cọ, bộ lọc, PSD |
| Krasis Library — Thư viện ảnh & RAW | Nhập, sắp xếp, xử lý RAW không phá hủy |
| Krasis Vector — Minh họa vector | Đường dẫn, hình khối, SVG/PDF/AI/EPS |
| Krasis Layout — Dàn trang | Tạp chí, sách, IDML, xuất PDF/EPUB |
| Krasis Interface — Thiết kế giao diện | Khung, thành phần, bố cục tự động, nguyên mẫu |
| Krasis CAD — Thiết kế kỹ thuật | Vẽ 2D, DWG/DXF, dòng lệnh |

Mã nguồn gốc và giấy phép của từng thành phần: xem [NOTICE.md](NOTICE.md).

Mỗi ứng dụng giữ **nguyên toàn bộ tính năng và giao diện** của nó (menu, bảng điều khiển, hộp
thoại, tạo/mở/lưu/xuất tệp, phím tắt, kênh điều khiển…): Krasis chạy chính mã nguồn của ứng dụng
đó bên trong cửa sổ của mình.

## Có gì trong Krasis

- **Thanh ứng dụng** bên trái (như thanh tab dọc của Edge): Trang chủ và sáu ứng dụng; chấm vàng
  cho biết ứng dụng đang chạy. Nhấp chuột phải để Mở…, Tạo mới hoặc Đóng ứng dụng.
- **Trang chủ**: logo, khẩu hiệu, sáu thẻ ứng dụng (Mới… / Mở…), tệp gần đây, kéo-thả tệp.
- **Mở tệp đúng ứng dụng**: Krasis chọn ứng dụng phù hợp theo phần mở rộng; khi nhiều ứng dụng mở
  được (ví dụ `.png`, `.svg`, `.pdf`, `.psd`), Krasis hỏi “Mở bằng” và có thể ghi nhớ lựa chọn.
  Mở từ Explorer, dòng lệnh, kéo-thả, hoặc khi Krasis đang chạy (tệp được chuyển cho cửa sổ đang mở).
- **Mượt như Edge**: trình chiếu VSync với hai khung hình trong hàng đợi, cuộn và thu phóng bằng bánh
  xe chuột có gia tốc êm (chỉnh được thời gian trong Cài đặt), hiệu ứng chuyển cảnh khi đổi ứng dụng,
  hoạt ảnh cho thanh ứng dụng và thẻ. Chế độ “Độ trễ thấp nhất” dành cho bút vẽ.
- **Tiếng Anh và tiếng Việt**: đổi trong Cài đặt › Ngôn ngữ; cả Krasis và sáu ứng dụng cùng đổi.
  DesignCraft được bổ sung bản dịch tiếng Việt đầy đủ (2.125 chuỗi).
- **Ứng dụng ngủ như tab ngủ của Edge**: chỉ ứng dụng ở phía trước chạy; các ứng dụng khác giữ
  nguyên tài liệu và trạng thái. Mỗi ứng dụng giữ giao diện riêng (chủ đề, phông chữ, thu phóng).
- **Đóng an toàn**: khi đóng cửa sổ, lần lượt từng ứng dụng đang chạy hỏi về thay đổi chưa lưu.
- **Một bản cập nhật cho cả bộ**: các ứng dụng con không tự cập nhật riêng (tránh ghi đè `krasis.exe`).
- **Thanh ứng dụng tự ẩn** (1.1): khi đang mở một ứng dụng, thanh bên trái thu gọn; đưa chuột tới
  mép trái để hiện. Bật/tắt bằng nút ở cuối thanh hoặc Cài đặt › Thanh ứng dụng.
- **Một chủ đề cho cả bộ** (1.1): Sáng/Tối/Theo hệ thống trong Cài đặt áp dụng cho Krasis và cả sáu
  ứng dụng.
- **Tiết kiệm bộ nhớ** (1.1): ứng dụng chỉ khởi động khi dùng; ứng dụng không mở tệp nào tự đóng
  sau 1 phút chạy nền; khi Krasis thu nhỏ hoặc chạy nền, bộ nhớ không dùng được trả lại Windows;
  GPU dùng chế độ tiết kiệm bộ nhớ; không nạp sẵn ảnh mẫu hay quét toàn bộ phông chữ lúc khởi động.

## Dựng cho Windows 11

Cần: [Rust](https://rustup.rs) 1.95 trở lên (bộ công cụ MSVC) và Visual Studio 2022 Build Tools với
“Desktop development with C++”. Tùy chọn: [Inno Setup 6](https://jrsoftware.org/isinfo.php) để tạo
bộ cài.

```powershell
git clone <kho của bạn> krasis
cd krasis
powershell -ExecutionPolicy Bypass -File packaging\windows\build.ps1   # thêm -WithFonts để nhúng phông CJK
```

Kết quả: `target\release\krasis.exe` (không cần Visual C++ Redistributable) và, nếu có Inno Setup,
`dist\Krasis-Setup-1.1.0.exe`. Bộ cài thêm Krasis vào “Open with” và Settings › Apps › Default apps
cho 98 loại tệp. Bản dựng đầu tiên mất khá lâu (khoảng 3,2 triệu dòng Rust).

Chạy thử khi đang phát triển: `cargo run -p krasis` (hồ sơ dev đã tối ưu thư viện ở mức 2).

### Dòng lệnh

```
krasis [--module photo|light|vector|design|fig|cad] [--new-instance] [tệp…]
```

Biến môi trường: `KRASIS_LOG=debug` (nhật ký trong thư mục dữ liệu `Krasis\data\logs`),
`WGPU_BACKEND=dx12|vulkan|gl` (chọn API đồ họa), `WGPU_POWER_PREF=low|high` (chọn GPU). Các biến
riêng của từng ứng dụng (`PHOTOCRAFT_CONTROL_PORT`, `CADCRAFT_CONTROL_PORT`, …) vẫn có tác dụng.

## Cấu trúc mã nguồn

```
apps/krasis/            ứng dụng Krasis: thanh ứng dụng, trang chủ, định tuyến tệp, cài đặt
crates/krasis-host/     trait Module và kho lưu cài đặt cho từng ứng dụng
upstream/<app>/         mã nguồn gốc của sáu ứng dụng (giữ nguyên)
upstream/<app>/apps/<app>-embed/
                        ứng dụng desktop của mỗi app, đổi từ chương trình thành thư viện
vendor/egui/            nhánh egui dùng chung, thêm 3 điểm mở rộng cho Krasis
packaging/windows/      build.ps1 và bộ cài Inno Setup
```

---

## English

Krasis (v1.1.0) puts six open-source Rust creative apps (photo editing, a RAW photo library,
vector illustration, page layout, interface design and CAD; see NOTICE.md) in one Windows 11 app
and window. Every app keeps all of its features and its
own interface; Krasis adds the app rail, a home screen, file routing with an Open With chooser,
Edge-like smoothness (vsync pacing, eased wheel scrolling and zooming, transitions), English and
Vietnamese, a single instance that receives files from Explorer, and an unsaved-work check across
apps on close.

Build on Windows: `powershell -ExecutionPolicy Bypass -File packaging\windows\build.ps1`
(Rust 1.95+, MSVC build tools; Inno Setup 6 for the installer).

Licence: MIT OR Apache-2.0, as the apps it hosts. See [NOTICE.md](NOTICE.md).
