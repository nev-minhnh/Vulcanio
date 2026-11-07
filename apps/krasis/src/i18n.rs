//! Krasis' own interface text in English and Vietnamese. English is the key; a string missing
//! from the table shows in English.

use krasis_host::Lang;

/// `en` in the interface language.
pub fn tr(lang: Lang, en: &'static str) -> &'static str {
    match lang {
        Lang::En => en,
        Lang::Vi => vi(en).unwrap_or(en),
    }
}

/// `en` with `{name}` placeholders filled, in the interface language.
pub fn trf(lang: Lang, en: &'static str, args: &[(&str, &str)]) -> String {
    let mut s = tr(lang, en).to_string();
    for (k, v) in args {
        s = s.replace(&format!("{{{k}}}"), v);
    }
    s
}

fn vi(en: &str) -> Option<&'static str> {
    Some(match en {
        "Your creative oasis" => "Ốc đảo sáng tạo của bạn",
        "Home" => "Trang chủ",
        "Settings" => "Cài đặt",
        "About Krasis" => "Giới thiệu Krasis",
        "Good morning" => "Chào buổi sáng",
        "Good afternoon" => "Chào buổi chiều",
        "Good evening" => "Chào buổi tối",
        "What will you create today?" => "Hôm nay bạn sẽ sáng tạo gì?",
        "Open a file…" => "Mở tệp…",
        "Open" => "Mở",
        "Open…" => "Mở…",
        "Start" => "Bắt đầu",
        "Recent files" => "Tệp gần đây",
        "No recent files yet. Files you open in any app appear here." => {
            "Chưa có tệp gần đây. Các tệp bạn mở trong bất kỳ ứng dụng nào sẽ hiện ở đây."
        }
        "Apps" => "Ứng dụng",
        "Running" => "Đang chạy",
        "Close app" => "Đóng ứng dụng",
        "Switch to" => "Chuyển sang",
        "Open with" => "Mở bằng",
        "Open {file} with" => "Mở {file} bằng",
        "{n} files" => "{n} tệp",
        "Always use this app for .{ext} files" => "Luôn dùng ứng dụng này cho tệp .{ext}",
        "Cancel" => "Hủy",
        "Recommended" => "Đề xuất",
        "No app in Krasis opens .{ext} files." => "Không có ứng dụng nào trong Krasis mở được tệp .{ext}.",
        "Starting {app}…" => "Đang khởi động {app}…",
        "Closing Krasis: each app asks about unsaved work first." => "Đang đóng Krasis: mỗi ứng dụng sẽ hỏi về các thay đổi chưa lưu trước.",
        "Keep working" => "Tiếp tục làm việc",
        "Language" => "Ngôn ngữ",
        "Same as Windows" => "Theo hệ thống",
        "Appearance" => "Giao diện",
        "Theme" => "Chủ đề",
        "System" => "Hệ thống",
        "Light" => "Sáng",
        "Dark" => "Tối",
        "Motion and smoothness" => "Chuyển động và độ mượt",
        "Smooth scrolling and zooming" => "Cuộn và thu phóng mượt",
        "Ease time" => "Thời gian chuyển động",
        "Animated transitions" => "Hiệu ứng chuyển cảnh",
        "Presentation" => "Trình chiếu khung hình",
        "Smooth (VSync, like Microsoft Edge)" => "Mượt (VSync, như Microsoft Edge)",
        "Lowest latency (for pen drawing)" => "Độ trễ thấp nhất (cho bút vẽ)",
        "Takes effect the next time Krasis starts." => "Có hiệu lực ở lần khởi động Krasis tiếp theo.",
        "Files" => "Tệp",
        "Make Krasis the app for these file types" => "Đặt Krasis làm ứng dụng mở các loại tệp này",
        "Registered for the file types Krasis opens." => "Đã đăng ký các loại tệp Krasis mở được.",
        "Remembered choices" => "Lựa chọn đã ghi nhớ",
        "Forget" => "Quên",
        "None yet." => "Chưa có.",
        "Startup" => "Khởi động",
        "Reopen the last app on start" => "Mở lại ứng dụng cuối cùng khi khởi động",
        "Close" => "Đóng",
        "Version" => "Phiên bản",
        "Krasis brings six open-source creative apps together in one window." => {
            "Krasis gộp sáu ứng dụng sáng tạo mã nguồn mở vào một cửa sổ."
        }
        "Open-source components under MIT OR Apache-2.0; see NOTICE.md." => "Thành phần mã nguồn mở theo giấy phép MIT HOẶC Apache-2.0; xem NOTICE.md.",
        "Licensed MIT OR Apache-2.0, like the apps it is built on." => "Giấy phép MIT HOẶC Apache-2.0, như các ứng dụng gốc.",
        "Couldn't open {file}: {reason}" => "Không mở được {file}: {reason}",
        "the file does not exist" => "tệp không tồn tại",
        "All supported files" => "Mọi tệp được hỗ trợ",
        "All files" => "Mọi tệp",
        "Drop files to open them" => "Thả tệp vào đây để mở",
        "New" => "Mới",
        "Remove from list" => "Xóa khỏi danh sách",
        "Show in folder" => "Hiện trong thư mục",
        "Keep the app bar open" => "Luôn hiện thanh ứng dụng",
        "Hide the app bar while an app is open" => "Ẩn thanh ứng dụng khi đang mở một ứng dụng",
        "App bar" => "Thanh ứng dụng",
        "Hide while an app is open; show when the pointer reaches the left edge" => "Ẩn khi đang mở ứng dụng; hiện khi đưa chuột tới mép trái",
        "Memory" => "Bộ nhớ",
        "Close apps with nothing open after a minute in the background" => "Đóng các ứng dụng không mở tệp nào sau một phút chạy nền",
        _ => return None,
    })
}
