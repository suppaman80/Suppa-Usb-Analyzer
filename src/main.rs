// SPDX-License-Identifier: GPL-3.0-or-later
use eframe::egui::{
    self, Align, Color32, Layout, Pos2, Rect, RichText, Sense, Shape, Stroke, Vec2,
};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};

const BACKGROUND: Color32 = Color32::from_rgb(12, 13, 19);
const SURFACE: Color32 = Color32::from_rgb(24, 25, 34);
const SURFACE_DARK: Color32 = Color32::from_rgb(17, 18, 25);
const LINE: Color32 = Color32::from_rgb(43, 44, 55);
const TEXT: Color32 = Color32::from_rgb(239, 237, 244);
const MUTED: Color32 = Color32::from_rgb(139, 140, 153);
const PINK: Color32 = Color32::from_rgb(220, 78, 137);
const GREEN: Color32 = Color32::from_rgb(104, 214, 167);
const BLUE: Color32 = Color32::from_rgb(127, 181, 239);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AnalyzerIcon {
    Usb,
    Serial,
    Clock,
    Refresh,
    Search,
    Mouse,
    Keyboard,
    Chip,
    Ch910x,
    Ftdi,
    Pl2303,
    Esp32,
    Esp8266,
    Hub,
    Smartphone,
    Microphone,
    AudioInterface,
    ExternalHdd,
    ExternalSsd,
    FlashDrive,
    Webcam,
    Gamepad,
    Printer,
    WifiAdapter,
}

fn draw_icon(ui: &mut egui::Ui, icon: AnalyzerIcon, color: Color32, size: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    paint_icon(ui.painter(), rect, icon, color);
}

fn paint_icon(painter: &egui::Painter, rect: Rect, icon: AnalyzerIcon, color: Color32) {
    let side = rect.width().min(rect.height());
    let origin = rect.center() - Vec2::splat(side * 0.5);
    let point = |x: f32, y: f32| origin + Vec2::new(x / 24.0 * side, y / 24.0 * side);
    let stroke = Stroke::new((side * 1.6 / 24.0).max(1.0), color);
    let line = |points: &[(f32, f32)]| {
        painter.add(Shape::line(
            points.iter().map(|(x, y)| point(*x, *y)).collect(),
            stroke,
        ));
    };
    let segment = |from: (f32, f32), to: (f32, f32)| {
        painter.line_segment([point(from.0, from.1), point(to.0, to.1)], stroke);
    };

    match icon {
        AnalyzerIcon::Usb => {
            line(&[(12.0, 3.0), (12.0, 16.0)]);
            line(&[(12.0, 3.0), (15.0, 6.0)]);
            line(&[(12.0, 3.0), (9.0, 6.0)]);
            line(&[(12.0, 16.0), (8.0, 19.0), (5.0, 19.0)]);
            line(&[(12.0, 16.0), (16.0, 19.0), (19.0, 19.0)]);
            line(&[(12.0, 13.0), (15.0, 10.0), (18.0, 10.0), (18.0, 8.0)]);
            painter.circle_filled(point(12.0, 19.0), side * 1.5 / 24.0, color);
            painter.rect_filled(
                Rect::from_min_max(point(3.5, 17.5), point(6.5, 20.5)),
                0.8,
                color,
            );
            painter.rect_filled(
                Rect::from_min_max(point(17.0, 6.5), point(20.0, 9.5)),
                0.4,
                color,
            );
        }
        AnalyzerIcon::Serial => {
            line(&[
                (3.6, 12.0),
                (8.4, 12.0),
                (10.8, 7.2),
                (14.4, 16.8),
                (16.8, 12.0),
                (20.4, 12.0),
            ]);
        }
        AnalyzerIcon::Clock => {
            painter.circle_stroke(point(12.0, 12.0), side * 7.9 / 24.0, stroke);
            segment((12.0, 7.2), (12.0, 12.6));
            segment((12.0, 12.6), (15.6, 14.8));
        }
        AnalyzerIcon::Refresh => {
            line(&[
                (19.5, 11.0),
                (19.0, 8.5),
                (17.8, 6.4),
                (15.8, 4.8),
                (13.5, 4.0),
                (11.0, 4.0),
                (8.5, 4.8),
                (6.7, 6.0),
            ]);
            segment((5.0, 4.5), (5.0, 8.0));
            segment((5.0, 8.0), (8.5, 8.0));
            line(&[
                (4.5, 13.0),
                (5.0, 15.5),
                (6.2, 17.6),
                (8.2, 19.2),
                (10.5, 20.0),
                (13.0, 20.0),
                (15.5, 19.2),
                (17.3, 18.0),
            ]);
            segment((19.0, 19.5), (19.0, 16.0));
            segment((19.0, 16.0), (15.5, 16.0));
        }
        AnalyzerIcon::Search => {
            painter.circle_stroke(point(10.6, 10.6), side * 7.0 / 24.0, stroke);
            segment((15.7, 15.7), (20.5, 20.5));
        }
        AnalyzerIcon::Mouse => {
            line(&[
                (12.0, 3.0),
                (8.0, 4.0),
                (5.5, 7.0),
                (5.5, 15.0),
                (7.0, 19.0),
                (9.5, 21.0),
                (12.0, 22.0),
                (14.5, 21.0),
                (17.0, 19.0),
                (18.5, 15.0),
                (18.5, 7.0),
                (16.0, 4.0),
                (12.0, 3.0),
            ]);
            segment((12.0, 4.0), (12.0, 10.0));
            painter.circle_filled(point(12.0, 7.5), side * 0.7 / 24.0, color);
        }
        AnalyzerIcon::Keyboard => {
            line(&[
                (3.0, 6.0),
                (21.0, 6.0),
                (21.0, 18.0),
                (3.0, 18.0),
                (3.0, 6.0),
            ]);
            for y in [8.0, 11.0] {
                for x in [5.0, 9.0, 13.0, 17.0] {
                    line(&[
                        (x, y),
                        (x + 2.0, y),
                        (x + 2.0, y + 1.5),
                        (x, y + 1.5),
                        (x, y),
                    ]);
                }
            }
            line(&[
                (7.0, 14.0),
                (17.0, 14.0),
                (17.0, 16.0),
                (7.0, 16.0),
                (7.0, 14.0),
            ]);
        }
        AnalyzerIcon::Chip | AnalyzerIcon::Esp32 => {
            line(&[
                (8.0, 7.0),
                (16.0, 7.0),
                (16.0, 17.0),
                (8.0, 17.0),
                (8.0, 7.0),
            ]);
            for pin in [9.5, 12.0, 14.5] {
                segment((6.0, pin), (8.0, pin));
                segment((16.0, pin), (18.0, pin));
            }
            for pin in [10.0, 14.0] {
                segment((pin, 5.0), (pin, 7.0));
                segment((pin, 17.0), (pin, 19.0));
            }
            if matches!(icon, AnalyzerIcon::Esp32) {
                line(&[(10.0, 14.0), (10.0, 10.0), (14.0, 14.0), (14.0, 10.0)]);
            } else {
                painter.circle_filled(point(12.0, 12.0), side * 1.0 / 24.0, color);
            }
        }
        AnalyzerIcon::Hub => {
            line(&[(12.0, 20.0), (12.0, 15.0), (5.0, 15.0), (5.0, 10.0)]);
            line(&[(12.0, 15.0), (12.0, 10.0)]);
            line(&[(12.0, 15.0), (19.0, 15.0), (19.0, 10.0)]);
            for x in [5.0, 12.0, 19.0] {
                line(&[
                    (x - 2.0, 5.0),
                    (x + 2.0, 5.0),
                    (x + 2.0, 10.0),
                    (x - 2.0, 10.0),
                    (x - 2.0, 5.0),
                ]);
            }
            painter.circle_filled(point(12.0, 20.0), side * 1.0 / 24.0, color);
        }
        _ => {}
    }
}

#[derive(Debug, Clone)]
struct UsbDevice {
    usb_path: String,
    name: String,
    vendor_id: String,
    product_id: String,
    device_class: String,
    bus_number: Option<u32>,
    device_number: Option<u32>,
    serial_ports: Vec<String>,
    bridge: Option<&'static str>,
}

fn classify_device_icon(
    name: &str,
    vendor_id: &str,
    product_id: &str,
    device_class: &str,
    has_serial: bool,
) -> AnalyzerIcon {
    let name = name.to_ascii_lowercase();
    let vendor_id = vendor_id.to_ascii_lowercase();
    let product_id = product_id.to_ascii_lowercase();
    let device_class = device_class.trim_start_matches("0x");

    if ["esp8266", "lolin", "wemos", "nodemcu", "d1 mini", "d1-mini"]
        .iter()
        .any(|term| name.contains(term))
    {
        AnalyzerIcon::Esp8266
    } else if name.contains("esp32") || name.contains("espressif") || vendor_id == "303a" {
        AnalyzerIcon::Esp32
    } else if name.contains("ch910") || (has_serial && vendor_id == "1a86" && product_id == "55d4")
    {
        AnalyzerIcon::Ch910x
    } else if name.contains("ftdi") || name.contains("ft232") || (has_serial && vendor_id == "0403")
    {
        AnalyzerIcon::Ftdi
    } else if name.contains("pl2303")
        || name.contains("prolific")
        || (has_serial && vendor_id == "067b" && product_id == "2303")
    {
        AnalyzerIcon::Pl2303
    } else if name.contains("ch340")
        || name.contains("ch341")
        || (has_serial && vendor_id == "1a86" && matches!(product_id.as_str(), "7523" | "5523"))
    {
        AnalyzerIcon::Esp8266
    } else if [
        "smartphone",
        "android",
        "iphone",
        "ipad",
        "mobile phone",
        "mobile device",
        "mtp",
        "samsung",
        "xiaomi",
        "redmi",
        "poco",
        "huawei",
        "honor",
        "oppo",
        "vivo",
        "oneplus",
        "realme",
        "motorola",
        "google pixel",
        "pixel",
        "apple",
        "sony",
        "nokia",
        "hmd global",
        "asus",
        "rog phone",
        "zte",
        "nubia",
        "redmagic",
        "lenovo",
        "tecno",
        "infinix",
        "itel",
        "doogee",
        "ulefone",
        "blackview",
        "oukitel",
        "cubot",
        "umidigi",
        "agm",
        "kyocera",
        "sharp",
        "fairphone",
        "nothing",
        "meizu",
        "tcl",
        "alcatel",
        "lava",
        "micromax",
        "iqoo",
        "cat phone",
    ]
    .iter()
    .any(|term| name.contains(term))
    {
        AnalyzerIcon::Smartphone
    } else if device_class == "09"
        || device_class == "9"
        || name.contains("hub")
        || matches!(
            (vendor_id.as_str(), product_id.as_str()),
            ("05e3", "0610" | "0608") | ("2109", "2817") | ("1a40", "0101")
        )
    {
        AnalyzerIcon::Hub
    } else if device_class == "0e"
        || device_class == "e"
        || ["webcam", "web cam", "camera", "uvc video"]
            .iter()
            .any(|term| name.contains(term))
    {
        AnalyzerIcon::Webcam
    } else if device_class == "07"
        || device_class == "7"
        || name.contains("printer")
        || name.contains("printing")
    {
        AnalyzerIcon::Printer
    } else if [
        "gamepad",
        "game pad",
        "game controller",
        "joystick",
        "xbox",
        "playstation",
    ]
    .iter()
    .any(|term| name.contains(term))
    {
        AnalyzerIcon::Gamepad
    } else if ["microphone", "usb mic", "mic input", "microphone array"]
        .iter()
        .any(|term| name.contains(term))
    {
        AnalyzerIcon::Microphone
    } else if [
        "native instruments",
        "traktor",
        "komplete",
        "audio interface",
        "sound card",
        "soundcard",
        "usb audio",
        "focusrite",
        "scarlett",
        "behringer",
        "midi",
    ]
    .iter()
    .any(|term| name.contains(term))
        || device_class == "01"
        || device_class == "1"
    {
        AnalyzerIcon::AudioInterface
    } else if ["ssd", "solid state", "nvme"]
        .iter()
        .any(|term| name.contains(term))
    {
        AnalyzerIcon::ExternalSsd
    } else if [
        "hdd",
        "hard disk",
        "harddrive",
        "external disk",
        "external drive",
    ]
    .iter()
    .any(|term| name.contains(term))
    {
        AnalyzerIcon::ExternalHdd
    } else if ["wifi", "wi-fi", "wireless", "802.11", "wlan", "bluetooth"]
        .iter()
        .any(|term| name.contains(term))
    {
        AnalyzerIcon::WifiAdapter
    } else if device_class == "08"
        || device_class == "8"
        || name.contains("flash drive")
        || name.contains("thumb drive")
    {
        AnalyzerIcon::FlashDrive
    } else if ["keyboard", "key board", "tastiera"]
        .iter()
        .any(|term| name.contains(term))
    {
        AnalyzerIcon::Keyboard
    } else if [
        "mouse",
        "trackball",
        "trackpad",
        "touchpad",
        "pointing device",
    ]
    .iter()
    .any(|term| name.contains(term))
    {
        AnalyzerIcon::Mouse
    } else if has_serial {
        AnalyzerIcon::Chip
    } else {
        AnalyzerIcon::Usb
    }
}

fn device_icon(device: &UsbDevice) -> AnalyzerIcon {
    classify_device_icon(
        &device.name,
        &device.vendor_id,
        &device.product_id,
        &device.device_class,
        !device.serial_ports.is_empty() || device.bridge.is_some(),
    )
}

struct DeviceImages {
    mouse: egui::TextureHandle,
    keyboard: egui::TextureHandle,
    hub: egui::TextureHandle,
    cp210x: egui::TextureHandle,
    ch910x: egui::TextureHandle,
    ftdi: egui::TextureHandle,
    pl2303: egui::TextureHandle,
    esp32: egui::TextureHandle,
    esp8266: egui::TextureHandle,
    smartphone: egui::TextureHandle,
    microphone: egui::TextureHandle,
    audio_interface: egui::TextureHandle,
    external_hdd: egui::TextureHandle,
    external_ssd: egui::TextureHandle,
    flash_drive: egui::TextureHandle,
    webcam: egui::TextureHandle,
    gamepad: egui::TextureHandle,
    printer: egui::TextureHandle,
    wifi_adapter: egui::TextureHandle,
    usb: egui::TextureHandle,
}

impl DeviceImages {
    fn load(ctx: &egui::Context) -> Self {
        Self {
            mouse: load_device_image(
                ctx,
                "device-mouse",
                include_bytes!("../assets/devices/mouse.png"),
            ),
            keyboard: load_device_image(
                ctx,
                "device-keyboard",
                include_bytes!("../assets/devices/keyboard.png"),
            ),
            hub: load_device_image(
                ctx,
                "device-hub",
                include_bytes!("../assets/devices/hub.png"),
            ),
            cp210x: load_device_image(
                ctx,
                "device-cp210x",
                include_bytes!("../assets/devices/cp210x.png"),
            ),
            ch910x: load_device_image(
                ctx,
                "device-ch910x",
                include_bytes!("../assets/devices/ch910x.png"),
            ),
            ftdi: load_device_image(
                ctx,
                "device-ftdi",
                include_bytes!("../assets/devices/ftdi.png"),
            ),
            pl2303: load_device_image(
                ctx,
                "device-pl2303",
                include_bytes!("../assets/devices/pl2303.png"),
            ),
            esp32: load_device_image(
                ctx,
                "device-esp32",
                include_bytes!("../assets/devices/esp32.png"),
            ),
            esp8266: load_device_image(
                ctx,
                "device-esp8266",
                include_bytes!("../assets/devices/esp8266.png"),
            ),
            smartphone: load_device_image(
                ctx,
                "device-smartphone",
                include_bytes!("../assets/devices/smartphone.png"),
            ),
            microphone: load_device_image(
                ctx,
                "device-microphone",
                include_bytes!("../assets/devices/microphone.png"),
            ),
            audio_interface: load_device_image(
                ctx,
                "device-audio-interface",
                include_bytes!("../assets/devices/audio_interface.png"),
            ),
            external_hdd: load_device_image(
                ctx,
                "device-external-hdd",
                include_bytes!("../assets/devices/external_hdd.png"),
            ),
            external_ssd: load_device_image(
                ctx,
                "device-external-ssd",
                include_bytes!("../assets/devices/external_ssd.png"),
            ),
            flash_drive: load_device_image(
                ctx,
                "device-flash-drive",
                include_bytes!("../assets/devices/flash_drive.png"),
            ),
            webcam: load_device_image(
                ctx,
                "device-webcam",
                include_bytes!("../assets/devices/webcam.png"),
            ),
            gamepad: load_device_image(
                ctx,
                "device-gamepad",
                include_bytes!("../assets/devices/gamepad.png"),
            ),
            printer: load_device_image(
                ctx,
                "device-printer",
                include_bytes!("../assets/devices/printer.png"),
            ),
            wifi_adapter: load_device_image(
                ctx,
                "device-wifi-adapter",
                include_bytes!("../assets/devices/wifi_adapter.png"),
            ),
            usb: load_device_image(
                ctx,
                "device-usb",
                include_bytes!("../assets/devices/usb.png"),
            ),
        }
    }

    fn for_device(&self, device: &UsbDevice) -> &egui::TextureHandle {
        match device_icon(device) {
            AnalyzerIcon::Mouse => &self.mouse,
            AnalyzerIcon::Keyboard => &self.keyboard,
            AnalyzerIcon::Hub => &self.hub,
            AnalyzerIcon::Chip
                if device.vendor_id.eq_ignore_ascii_case("10c4")
                    || device.name.to_ascii_lowercase().contains("cp210") =>
            {
                &self.cp210x
            }
            AnalyzerIcon::Ch910x => &self.ch910x,
            AnalyzerIcon::Ftdi => &self.ftdi,
            AnalyzerIcon::Pl2303 => &self.pl2303,
            AnalyzerIcon::Esp32 => &self.esp32,
            AnalyzerIcon::Esp8266 => &self.esp8266,
            AnalyzerIcon::Smartphone => &self.smartphone,
            AnalyzerIcon::Microphone => &self.microphone,
            AnalyzerIcon::AudioInterface => &self.audio_interface,
            AnalyzerIcon::ExternalHdd => &self.external_hdd,
            AnalyzerIcon::ExternalSsd => &self.external_ssd,
            AnalyzerIcon::FlashDrive => &self.flash_drive,
            AnalyzerIcon::Webcam => &self.webcam,
            AnalyzerIcon::Gamepad => &self.gamepad,
            AnalyzerIcon::Printer => &self.printer,
            AnalyzerIcon::WifiAdapter => &self.wifi_adapter,
            _ => &self.usb,
        }
    }
}

fn load_device_image(ctx: &egui::Context, name: &str, bytes: &[u8]) -> egui::TextureHandle {
    let rgba = image::load_from_memory(bytes)
        .expect("invalid device image")
        .into_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let image = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
    ctx.load_texture(name, image, egui::TextureOptions::LINEAR)
}

struct SuppaUsbApp {
    devices: Vec<UsbDevice>,
    device_images: DeviceImages,
    filter_serial: bool,
    query: String,
    scanning: bool,
    scan_started: bool,
    error: Option<String>,
    last_scan: Option<Instant>,
    scan_receiver: Option<Receiver<Result<Vec<UsbDevice>, String>>>,
    info_open: bool,
}

impl SuppaUsbApp {
    fn new(ctx: &egui::Context) -> Self {
        Self {
            devices: Vec::new(),
            device_images: DeviceImages::load(ctx),
            filter_serial: false,
            query: String::new(),
            scanning: false,
            scan_started: false,
            error: None,
            last_scan: None,
            scan_receiver: None,
            info_open: false,
        }
    }

    fn start_scan(&mut self) {
        let (sender, receiver) = mpsc::channel();
        self.scan_receiver = Some(receiver);
        self.scanning = true;
        self.scan_started = true;
        self.error = None;
        thread::spawn(move || {
            let result = scan_usb_devices();
            let _ = sender.send(result);
        });
    }

    fn collect_scan_result(&mut self) {
        let result = self
            .scan_receiver
            .as_ref()
            .and_then(|receiver| receiver.try_recv().ok());
        if let Some(result) = result {
            match result {
                Ok(devices) => {
                    self.devices = devices;
                    self.last_scan = Some(Instant::now());
                }
                Err(error) => self.error = Some(error),
            }
            self.scanning = false;
            self.scan_receiver = None;
        }
    }

    fn filtered_devices(&self) -> Vec<&UsbDevice> {
        let query = self.query.trim().to_lowercase();
        self.devices
            .iter()
            .filter(|device| {
                (!self.filter_serial || !device.serial_ports.is_empty())
                    && (query.is_empty()
                        || format!(
                            "{} {} {} {} {} {}",
                            device.name,
                            device.usb_path,
                            device.vendor_id,
                            device.product_id,
                            device.serial_ports.join(" "),
                            device.bridge.unwrap_or_default()
                        )
                        .to_lowercase()
                        .contains(&query))
            })
            .collect()
    }
}

impl eframe::App for SuppaUsbApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.scan_started {
            self.start_scan();
        }
        self.collect_scan_result();
        if self.scanning {
            ctx.request_repaint_after(Duration::from_millis(120));
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(BACKGROUND).inner_margin(20.0))
            .show(ctx, |ui| {
                self.draw_header(ui);
                ui.add_space(18.0);
                self.draw_summary(ui);
                ui.add_space(16.0);
                self.draw_device_list(ui);
                ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                    ui.add_space(10.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Suppa-Usb-Analyzer").color(MUTED).size(11.0));
                        ui.label(RichText::new("·").color(PINK).size(11.0));
                        ui.label(
                            RichText::new("GPL-3.0-or-later License")
                                .color(MUTED)
                                .size(11.0),
                        );
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let info_button = ui.add(
                                egui::Button::new(
                                    RichText::new("About").color(TEXT).size(11.0).strong(),
                                )
                                .fill(SURFACE)
                                .stroke(Stroke::new(1.0_f32, LINE))
                                .corner_radius(6.0)
                                .min_size(Vec2::new(100.0, 25.0)),
                            );
                            if info_button.clicked() {
                                self.info_open = true;
                            }
                            ui.label(
                                RichText::new("Local scan · no root access")
                                    .color(MUTED)
                                    .size(11.0),
                            );
                        });
                    });
                });
            });
        self.draw_info_window(ctx);
    }
}

impl SuppaUsbApp {
    fn draw_info_window(&mut self, ctx: &egui::Context) {
        if !self.info_open {
            return;
        }

        egui::Window::new("About")
            .id(egui::Id::new("suppa-usb-info"))
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .collapsible(false)
            .resizable(false)
            .open(&mut self.info_open)
            .show(ctx, |ui| {
                ui.set_min_width(360.0);
                ui.label(
                    RichText::new("Suppa-Usb-Analyzer")
                        .color(TEXT)
                        .size(17.0)
                        .strong(),
                );
                ui.label(RichText::new("About this project").color(MUTED).size(12.0));
                ui.label(
                    RichText::new(format!("Version {}", env!("CARGO_PKG_VERSION")))
                        .color(MUTED)
                        .size(11.0),
                );
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(8.0);

                info_card(ui, "CREATED BY", "Suppaman80", PINK, None);
                ui.add_space(8.0);
                info_card(
                    ui,
                    "EMAIL",
                    "Suppaman80@proton.me",
                    BLUE,
                    Some("mailto:Suppaman80@proton.me"),
                );
                ui.add_space(8.0);
                info_card(
                    ui,
                    "GITHUB",
                    "Suppa-Usb-Analyzer",
                    BLUE,
                    Some("https://github.com/Suppaman80/Suppa-Usb-Analyzer"),
                );
            });
    }

    fn draw_header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(
                    RichText::new("Suppa-Usb-Analyzer")
                        .color(TEXT)
                        .size(17.0)
                        .strong(),
                );
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("USB DEVICE ANALYZER")
                            .color(Color32::from_rgb(213, 120, 161))
                            .size(11.0)
                            .strong(),
                    );
                    small_badge(ui, &format!("v{}", env!("CARGO_PKG_VERSION")), BLUE);
                });
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let label = if self.scanning {
                    "Scanning..."
                } else {
                    "Refresh"
                };
                let button = egui::Button::new(
                    RichText::new(label)
                        .color(Color32::WHITE)
                        .size(12.0)
                        .strong(),
                )
                .fill(PINK)
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(231, 96, 151)))
                .corner_radius(8.0)
                .min_size(Vec2::new(120.0, 34.0));
                let response = ui.add_enabled(!self.scanning, button);
                if !self.scanning {
                    paint_icon(
                        ui.painter(),
                        Rect::from_center_size(
                            Pos2::new(response.rect.left() + 16.0, response.rect.center().y),
                            Vec2::splat(15.0),
                        ),
                        AnalyzerIcon::Refresh,
                        Color32::WHITE,
                    );
                }
                if response.clicked() {
                    self.start_scan();
                }
            });
        });
        ui.separator();
    }

    fn draw_summary(&self, ui: &mut egui::Ui) {
        let serial_ports = self
            .devices
            .iter()
            .map(|device| device.serial_ports.len())
            .sum::<usize>();
        let serial_devices = self
            .devices
            .iter()
            .filter(|device| !device.serial_ports.is_empty())
            .count();
        let last_scan = self
            .last_scan
            .map(|time| {
                let minutes = time.elapsed().as_secs() / 60;
                if minutes == 0 {
                    "Now".to_string()
                } else {
                    format!("{minutes} min ago")
                }
            })
            .unwrap_or_else(|| "—".to_string());

        let device_count = if self.scanning && self.devices.is_empty() {
            "…".to_string()
        } else {
            self.devices.len().to_string()
        };
        let serial_caption = if serial_devices == 1 {
            "1 USB device".to_string()
        } else {
            format!("{serial_devices} USB devices")
        };
        ui.columns(3, |columns| {
            summary_card(
                &mut columns[0],
                "USB DEVICES",
                device_count,
                "Root hubs excluded",
                PINK,
                AnalyzerIcon::Usb,
            );
            summary_card(
                &mut columns[1],
                "SERIAL PORTS",
                serial_ports.to_string(),
                &serial_caption,
                BLUE,
                AnalyzerIcon::Serial,
            );
            summary_card(
                &mut columns[2],
                "LAST SCAN",
                last_scan,
                "Manual refresh",
                GREEN,
                AnalyzerIcon::Clock,
            );
        });
    }

    fn draw_device_list(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading(
                RichText::new("Connected devices")
                    .color(TEXT)
                    .size(17.0)
                    .strong(),
            );
            let shown = self.filtered_devices().len();
            small_badge(ui, &shown.to_string(), MUTED);
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if self.scanning {
                    ui.spinner();
                    ui.label(
                        RichText::new("Detecting devices...")
                            .color(MUTED)
                            .size(12.0),
                    );
                } else if self.last_scan.is_some() {
                    status_badge(ui, "SCAN COMPLETE", GREEN);
                }
            });
        });
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            let all_count = self.devices.len();
            let serial_count = self
                .devices
                .iter()
                .filter(|device| !device.serial_ports.is_empty())
                .count();
            let all_button = ui.selectable_label(
                !self.filter_serial,
                RichText::new(format!("All  {all_count}")).size(12.0),
            );
            if all_button.clicked() {
                self.filter_serial = false;
            }
            let serial_button = ui.selectable_label(
                self.filter_serial,
                RichText::new(format!("Serial  {serial_count}")).size(12.0),
            );
            if serial_button.clicked() {
                self.filter_serial = true;
            }
            ui.add_space(10.0);
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.query)
                        .hint_text("Search devices...")
                        .desired_width(220.0),
                );
                draw_icon(ui, AnalyzerIcon::Search, MUTED, 16.0);
            });
        });
        ui.add_space(8.0);

        if let Some(error) = &self.error {
            egui::Frame::new()
                .fill(Color32::from_rgb(50, 27, 35))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(117, 56, 71)))
                .corner_radius(8.0)
                .inner_margin(9.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("!").color(Color32::LIGHT_RED).strong());
                        ui.label(RichText::new("Scan failed").color(TEXT).strong());
                    });
                    ui.label(
                        RichText::new(error)
                            .color(Color32::from_rgb(220, 160, 168))
                            .size(12.0),
                    );
                });
            ui.add_space(8.0);
        }

        if self.scanning && self.devices.is_empty() {
            egui::Frame::new()
                .fill(SURFACE_DARK)
                .stroke(Stroke::new(1.0_f32, LINE))
                .corner_radius(10.0)
                .inner_margin(16.0)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add(egui::Spinner::new().color(PINK).size(22.0));
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new("Searching for USB devices")
                                .color(TEXT)
                                .strong(),
                        );
                        ui.label(
                            RichText::new("Reading devices connected to your system.")
                                .color(MUTED)
                                .size(12.0),
                        );
                    });
                });
            return;
        }

        let devices = self.filtered_devices();
        if devices.is_empty() {
            egui::Frame::new()
                .fill(SURFACE_DARK)
                .stroke(Stroke::new(1.0_f32, LINE))
                .corner_radius(10.0)
                .inner_margin(16.0)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        draw_icon(ui, AnalyzerIcon::Usb, PINK, 26.0);
                        ui.add_space(6.0);
                        ui.label(RichText::new("No devices to display").color(TEXT).strong());
                        ui.label(
                            RichText::new("Connect a device or change the search filter.")
                                .color(MUTED)
                                .size(12.0),
                        );
                    });
                });
            return;
        }

        let list_max_height = (ui.available_height() - 42.0).max(96.0);
        ui.scope(|ui| {
            ui.spacing_mut().scroll.bar_width = 9.0;
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .max_height(list_max_height)
                .show(ui, |ui| {
                    for device in &devices {
                        device_card(ui, device, &self.device_images);
                        ui.add_space(6.0);
                    }
                });
        });
    }
}

fn summary_card(
    ui: &mut egui::Ui,
    title: &str,
    value: String,
    caption: &str,
    accent: Color32,
    icon: AnalyzerIcon,
) {
    egui::Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0_f32, LINE))
        .corner_radius(8.0)
        .inner_margin(egui::Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.scope(|ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.horizontal(|ui| {
                    draw_icon(ui, icon, accent, 15.0);
                    ui.label(RichText::new(title).color(MUTED).size(11.0).strong());
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new(value).color(accent).size(20.0).strong());
                    });
                });
                ui.label(RichText::new(caption).color(MUTED).size(10.0));
            });
        });
}

fn info_card(ui: &mut egui::Ui, label: &str, value: &str, accent: Color32, link: Option<&str>) {
    egui::Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0_f32, LINE))
        .corner_radius(8.0)
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("●").color(accent).size(12.0));
                ui.vertical(|ui| {
                    ui.label(RichText::new(label).color(MUTED).size(10.0).strong());
                    if let Some(url) = link {
                        ui.hyperlink_to(RichText::new(value).color(TEXT).size(13.0), url);
                    } else {
                        ui.label(RichText::new(value).color(TEXT).size(13.0).strong());
                    }
                });
            });
        });
}

fn device_card(ui: &mut egui::Ui, device: &UsbDevice, images: &DeviceImages) {
    egui::Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0_f32, LINE))
        .corner_radius(9.0)
        .inner_margin(8.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let image = images.for_device(device);
                ui.allocate_ui(Vec2::splat(50.0), |ui| {
                    ui.with_layout(
                        Layout::centered_and_justified(egui::Direction::LeftToRight),
                        |ui| {
                            ui.add(egui::Image::new(image).fit_to_exact_size(Vec2::splat(48.0)));
                        },
                    );
                });
                ui.vertical(|ui| {
                    ui.label(RichText::new(&device.name).color(TEXT).size(13.0).strong());
                    ui.label(
                        RichText::new(format!("Port {}", device.usb_path))
                            .color(MUTED)
                            .size(11.0),
                    );
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.label(RichText::new("VID:PID").color(MUTED).size(10.0));
                    ui.label(
                        RichText::new(format!("{}:{}", device.vendor_id, device.product_id))
                            .color(Color32::from_rgb(192, 190, 201))
                            .monospace()
                            .size(12.0),
                    );
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.label(RichText::new("SERIAL PORTS").color(MUTED).size(10.0));
                    if device.serial_ports.is_empty() {
                        ui.label(RichText::new("—").color(MUTED).size(12.0));
                    } else {
                        ui.horizontal_wrapped(|ui| {
                            for port in &device.serial_ports {
                                small_badge(ui, port, GREEN);
                            }
                        });
                    }
                });
                if let Some(bridge) = device.bridge {
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.label(RichText::new("BRIDGE").color(MUTED).size(10.0));
                        ui.label(RichText::new(bridge).color(TEXT).size(12.0));
                    });
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let kind = if device.serial_ports.is_empty() {
                        "USB"
                    } else {
                        "SERIAL"
                    };
                    small_badge(
                        ui,
                        kind,
                        if device.serial_ports.is_empty() {
                            MUTED
                        } else {
                            GREEN
                        },
                    );
                    if let (Some(bus), Some(number)) = (device.bus_number, device.device_number) {
                        ui.label(
                            RichText::new(format!("BUS {bus:03} · DEV {number:03}"))
                                .color(MUTED)
                                .monospace()
                                .size(10.0),
                        );
                    }
                });
            });
        });
}

fn small_badge(ui: &mut egui::Ui, label: &str, color: Color32) {
    egui::Frame::new()
        .fill(Color32::from_rgba_unmultiplied(
            color.r(),
            color.g(),
            color.b(),
            22,
        ))
        .stroke(Stroke::new(
            1.0_f32,
            Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 70),
        ))
        .corner_radius(5.0)
        .inner_margin(egui::Margin::symmetric(4, 2))
        .show(ui, |ui| {
            ui.label(RichText::new(label).color(color).size(11.0).strong());
        });
}

fn status_badge(ui: &mut egui::Ui, label: &str, color: Color32) {
    egui::Frame::new()
        .fill(SURFACE_DARK)
        .stroke(Stroke::new(1.0_f32, LINE))
        .corner_radius(9.0)
        .inner_margin(egui::Margin::symmetric(6, 3))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(color, "●");
                ui.label(RichText::new(label).color(MUTED).size(11.0).strong());
            });
        });
}

fn main() -> eframe::Result {
    let icon = image::load_from_memory(include_bytes!("../assets/suppa-usb-analyzer.png"))
        .expect("failed to load application icon")
        .into_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    let icon = egui::IconData {
        rgba: icon.into_raw(),
        width: icon_width,
        height: icon_height,
    };

    eframe::run_native(
        "Suppa-Usb-Analyzer",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_app_id("suppa-usb-analyzer")
                .with_icon(icon)
                .with_inner_size([1080.0, 760.0])
                .with_min_inner_size([760.0, 580.0]),
            ..Default::default()
        },
        Box::new(|context| {
            context.egui_ctx.set_visuals(egui::Visuals::dark());
            let mut style = (*context.egui_ctx.style()).clone();
            style.visuals.panel_fill = BACKGROUND;
            style.visuals.window_fill = SURFACE;
            style.visuals.extreme_bg_color = SURFACE_DARK;
            style.visuals.faint_bg_color = SURFACE;
            style.visuals.override_text_color = Some(TEXT);
            style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, LINE);
            style.visuals.widgets.inactive.bg_fill = SURFACE;
            style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(49, 39, 51);
            style.visuals.widgets.active.bg_fill = Color32::from_rgb(75, 42, 63);
            style.spacing.item_spacing = Vec2::new(8.0, 8.0);
            context.egui_ctx.set_style(style);
            Ok(Box::new(SuppaUsbApp::new(&context.egui_ctx)))
        }),
    )
}

fn read_attribute(device: &Path, name: &str) -> Option<String> {
    fs::read_to_string(device.join(name))
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn parse_lsusb_names(output: &str) -> HashMap<(u32, u32), String> {
    output
        .lines()
        .filter_map(|line| {
            let fields = line.split_whitespace().collect::<Vec<_>>();
            if fields.len() < 7 || fields[0] != "Bus" || fields[2] != "Device" {
                return None;
            }
            let bus = fields[1].parse().ok()?;
            let device = fields[3].trim_end_matches(':').parse().ok()?;
            let name = fields[6..].join(" ");
            (!name.is_empty()).then_some(((bus, device), name))
        })
        .collect()
}

fn describe_bridge(value: &str) -> Option<&'static str> {
    let lower = value.to_ascii_lowercase();
    if lower.contains("ch340")
        || lower.contains("ch341")
        || lower.contains("1a86:7523")
        || lower.contains("1a86:5523")
    {
        Some("CH340/CH341, common on ESP8266 adapters")
    } else if lower.contains("ch910") || lower.contains("1a86:55d4") {
        Some("CH910x, USB-to-serial converter")
    } else if lower.contains("cp210")
        || lower.contains("silicon_labs")
        || lower.contains("silicon labs")
        || lower.contains("10c4:ea60")
    {
        Some("CP210x, USB-to-serial converter")
    } else if lower.contains("ftdi") || lower.contains("ft232") || lower.contains("0403:6001") {
        Some("FTDI, USB-to-serial converter")
    } else if lower.contains("pl2303") || lower.contains("prolific") || lower.contains("067b:2303")
    {
        Some("PL2303, USB-to-serial converter")
    } else if lower.contains("cdc_acm") || lower.contains("303a:") {
        Some("USB CDC serial device")
    } else {
        None
    }
}

fn is_root_hub(name: &str) -> bool {
    name.strip_prefix("usb")
        .is_some_and(|number| !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()))
}

fn find_usb_ancestor(mut path: PathBuf) -> Option<PathBuf> {
    loop {
        if path.join("idVendor").is_file() && path.join("idProduct").is_file() {
            return path.canonicalize().ok();
        }
        if !path.starts_with("/sys") {
            return None;
        }
        path = path.parent()?.to_path_buf();
    }
}

fn scan_usb_devices() -> Result<Vec<UsbDevice>, String> {
    if let Ok(mut settle) = Command::new("udevadm")
        .args(["settle", "--timeout=2"])
        .spawn()
    {
        let _ = settle.wait();
    }

    let lsusb_output = Command::new("lsusb")
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
        .unwrap_or_default();
    let lsusb_names = parse_lsusb_names(&lsusb_output);

    let usb_root = Path::new("/sys/bus/usb/devices");
    let usb_entries = fs::read_dir(usb_root)
        .map_err(|error| format!("Unable to read {}/: {error}", usb_root.display()))?;
    let mut devices = Vec::new();
    for entry in usb_entries.flatten() {
        let device = entry.path();
        let usb_path = entry.file_name().to_string_lossy().into_owned();
        if is_root_hub(&usb_path) {
            continue;
        }
        let Some(vendor_id) = read_attribute(&device, "idVendor") else {
            continue;
        };
        let Some(product_id) = read_attribute(&device, "idProduct") else {
            continue;
        };
        let device_class = read_attribute(&device, "bDeviceClass").unwrap_or_default();
        let bus_number = read_attribute(&device, "busnum").and_then(|value| value.parse().ok());
        let device_number = read_attribute(&device, "devnum").and_then(|value| value.parse().ok());
        let vendor = read_attribute(&device, "manufacturer").unwrap_or_default();
        let product = read_attribute(&device, "product").unwrap_or_default();
        let sysfs_name = match (vendor.is_empty(), product.is_empty()) {
            (false, false) => format!("{vendor} {product}"),
            (false, true) => vendor,
            (true, false) => product,
            (true, true) => String::new(),
        };
        let fallback_name = bus_number
            .zip(device_number)
            .and_then(|key| lsusb_names.get(&key).cloned());
        let name = if sysfs_name.is_empty() {
            fallback_name.unwrap_or_else(|| "USB device".to_string())
        } else {
            sysfs_name
        };
        devices.push((
            device.canonicalize().ok(),
            UsbDevice {
                usb_path,
                name,
                vendor_id,
                product_id,
                device_class,
                bus_number,
                device_number,
                serial_ports: Vec::new(),
                bridge: None,
            },
        ));
    }

    let mut serial_ports: HashMap<PathBuf, Vec<String>> = HashMap::new();
    if let Ok(tty_entries) = fs::read_dir("/sys/class/tty") {
        for tty in tty_entries.flatten() {
            let tty_name = tty.file_name().to_string_lossy().into_owned();
            if !tty_name.starts_with("ttyUSB") && !tty_name.starts_with("ttyACM") {
                continue;
            }
            let Ok(tty_device) = tty.path().join("device").canonicalize() else {
                continue;
            };
            if let Some(usb_device) = find_usb_ancestor(tty_device) {
                serial_ports
                    .entry(usb_device)
                    .or_default()
                    .push(format!("/dev/{tty_name}"));
            }
        }
    }
    for (canonical_path, device) in &mut devices {
        if let Some(ports) = canonical_path
            .as_ref()
            .and_then(|path| serial_ports.remove(path))
        {
            device.serial_ports = ports;
            device.serial_ports.sort();
        }
        if !device.serial_ports.is_empty() {
            device.bridge = describe_bridge(&format!(
                "{} {}:{}",
                device.name, device.vendor_id, device.product_id
            ));
        }
    }
    devices.sort_by(|left, right| left.1.usb_path.cmp(&right.1.usb_path));
    Ok(devices.into_iter().map(|(_, device)| device).collect())
}

#[cfg(test)]
mod tests {
    use super::{
        AnalyzerIcon, classify_device_icon, describe_bridge, is_root_hub, parse_lsusb_names,
        scan_usb_devices,
    };

    #[test]
    fn identifies_root_hubs_only() {
        assert!(is_root_hub("usb1"));
        assert!(!is_root_hub("1-3"));
        assert!(!is_root_hub("usb1-port2"));
    }

    #[test]
    fn parses_lsusb_device_names() {
        let names =
            parse_lsusb_names("Bus 001 Device 003: ID 10c4:ea60 Silicon Labs CP210x UART Bridge");
        assert_eq!(
            names.get(&(1, 3)).map(String::as_str),
            Some("Silicon Labs CP210x UART Bridge")
        );
    }

    #[test]
    fn recognizes_serial_bridges() {
        assert_eq!(
            describe_bridge("10c4:ea60"),
            Some("CP210x, USB-to-serial converter")
        );
        assert_eq!(
            describe_bridge("1a86:7523"),
            Some("CH340/CH341, common on ESP8266 adapters")
        );
        assert_eq!(
            describe_bridge("1a86:55d4"),
            Some("CH910x, USB-to-serial converter")
        );
        assert_eq!(
            describe_bridge("0403:6001"),
            Some("FTDI, USB-to-serial converter")
        );
        assert_eq!(
            describe_bridge("067b:2303"),
            Some("PL2303, USB-to-serial converter")
        );
        assert_eq!(describe_bridge("046d:c313"), None);
    }

    #[test]
    fn classifies_device_icons_from_usb_descriptors() {
        assert_eq!(
            classify_device_icon("USB Optical Mouse", "046d", "c077", "00", false),
            AnalyzerIcon::Mouse
        );
        assert_eq!(
            classify_device_icon("USB Keyboard", "046d", "c31c", "00", false),
            AnalyzerIcon::Keyboard
        );
        assert_eq!(
            classify_device_icon("USB JTAG/serial debug unit", "303a", "1001", "00", true),
            AnalyzerIcon::Esp32
        );
        assert_eq!(
            classify_device_icon("Generic USB device", "1234", "5678", "09", false),
            AnalyzerIcon::Hub
        );
        assert_eq!(
            classify_device_icon("CP210x USB to UART Bridge", "10c4", "ea60", "00", true),
            AnalyzerIcon::Chip
        );
        assert_eq!(
            classify_device_icon("USB2.0-Serial", "1a86", "7523", "00", true),
            AnalyzerIcon::Esp8266
        );
        assert_eq!(
            classify_device_icon("USB2.0-Serial", "1a86", "55d4", "00", true),
            AnalyzerIcon::Ch910x
        );
        assert_eq!(
            classify_device_icon("FT232R USB UART", "0403", "6001", "00", true),
            AnalyzerIcon::Ftdi
        );
        assert_eq!(
            classify_device_icon("USB-Serial Controller D", "067b", "2303", "00", true),
            AnalyzerIcon::Pl2303
        );
        assert_eq!(
            classify_device_icon("CH341 serial converter", "1234", "5678", "00", true),
            AnalyzerIcon::Esp8266
        );
        assert_eq!(
            classify_device_icon("USB device", "1234", "5678", "00", false),
            AnalyzerIcon::Usb
        );
    }

    #[test]
    fn classifies_additional_peripheral_categories() {
        assert_eq!(
            classify_device_icon("LOLIN(WEMOS) D1 mini", "1a86", "7523", "00", true),
            AnalyzerIcon::Esp8266
        );
        assert_eq!(
            classify_device_icon("Google Android MTP", "18d1", "4ee7", "06", false),
            AnalyzerIcon::Smartphone
        );
        assert_eq!(
            classify_device_icon(
                "realme SM8350-IDP _SN:578F7BF8",
                "0000",
                "0000",
                "00",
                false
            ),
            AnalyzerIcon::Smartphone
        );
        assert_eq!(
            classify_device_icon("USB Microphone", "1234", "5678", "01", false),
            AnalyzerIcon::Microphone
        );
        assert_eq!(
            classify_device_icon(
                "Native Instruments Komplete Audio",
                "17cc",
                "1234",
                "00",
                false
            ),
            AnalyzerIcon::AudioInterface
        );
        assert_eq!(
            classify_device_icon("Portable SSD", "1234", "5678", "08", false),
            AnalyzerIcon::ExternalSsd
        );
        assert_eq!(
            classify_device_icon("USB hard disk drive", "1234", "5678", "08", false),
            AnalyzerIcon::ExternalHdd
        );
        assert_eq!(
            classify_device_icon("USB mass storage", "1234", "5678", "08", false),
            AnalyzerIcon::FlashDrive
        );
        assert_eq!(
            classify_device_icon("UVC Camera", "1234", "5678", "0e", false),
            AnalyzerIcon::Webcam
        );
        assert_eq!(
            classify_device_icon("Wireless Gamepad", "1234", "5678", "00", false),
            AnalyzerIcon::Gamepad
        );
        assert_eq!(
            classify_device_icon("USB Printer", "1234", "5678", "07", false),
            AnalyzerIcon::Printer
        );
        assert_eq!(
            classify_device_icon("Realtek 802.11ac WLAN Adapter", "0bda", "c811", "00", false),
            AnalyzerIcon::WifiAdapter
        );
    }

    #[test]
    fn classifies_phone_manufacturers() {
        for brand in [
            "Samsung",
            "Xiaomi",
            "Redmi",
            "POCO",
            "Huawei",
            "Honor",
            "OPPO",
            "vivo",
            "OnePlus",
            "realme",
            "Motorola",
            "Google Pixel",
            "Apple",
            "Sony",
            "Nokia",
            "HMD Global",
            "ASUS",
            "ROG Phone",
            "ZTE",
            "Nubia",
            "RedMagic",
            "Lenovo",
            "TECNO",
            "Infinix",
            "itel",
            "DOOGEE",
            "Ulefone",
            "Blackview",
            "OUKITEL",
            "Cubot",
            "UMIDIGI",
            "AGM",
            "Kyocera",
            "Sharp",
            "Fairphone",
            "Nothing",
            "Meizu",
            "TCL",
            "Alcatel",
            "Lava",
            "Micromax",
            "iQOO",
            "CAT Phone",
        ] {
            let name = format!("USB {brand} device");
            assert_eq!(
                classify_device_icon(&name, "0000", "0000", "00", false),
                AnalyzerIcon::Smartphone,
                "brand not recognized: {brand}"
            );
        }
    }

    #[test]
    fn scans_sysfs_and_excludes_root_hubs() {
        let devices = scan_usb_devices().expect("USB sysfs scan should succeed");
        assert!(devices.iter().all(|device| !is_root_hub(&device.usb_path)));
        assert!(
            devices
                .windows(2)
                .all(|pair| pair[0].usb_path <= pair[1].usb_path)
        );
    }
}
