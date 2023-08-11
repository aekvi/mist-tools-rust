use std::fmt;

pub struct MimeType {
    tpe: &'static str,
    tail: &'static str,
}

impl MimeType {
    const fn new(tpe: &'static str, tail: &'static str) -> MimeType {
        MimeType { tpe, tail }
    }
}

impl fmt::Display for MimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.tpe, self.tail)
    }
}

pub const AAC: MimeType = MimeType::new("audio", "aac");
pub const ABW: MimeType = MimeType::new("application", "x-abiword");
pub const ARC: MimeType = MimeType::new("application", "x-freearc");
pub const AVIF: MimeType = MimeType::new("image", "avif");
pub const AVI: MimeType = MimeType::new("video", "x-msvideo");
pub const AZW: MimeType = MimeType::new("application", "vnd.amazon.ebook");
pub const BIN: MimeType = MimeType::new("application", "octet-stream");
pub const BMP: MimeType = MimeType::new("image", "bmp");
pub const BZ: MimeType = MimeType::new("application", "x-bzip");
pub const BZ2: MimeType = MimeType::new("application", "x-bzip2");
pub const CDA: MimeType = MimeType::new("application", "x-cdf");
pub const CSH: MimeType = MimeType::new("application", "x-csh");
pub const CSS: MimeType = MimeType::new("text", "css");
pub const CSV: MimeType = MimeType::new("text", "csv");
pub const DOC: MimeType = MimeType::new("application", "msword");
pub const DOCX: MimeType = MimeType::new(
    "application",
    "vnd.openxmlformats-officedocument.wordprocessingml.document",
);
pub const EOT: MimeType = MimeType::new("application", "vnd.ms-fontobject");
pub const EPUB: MimeType = MimeType::new("application", "epub+zip");
pub const GZ: MimeType = MimeType::new("application", "gzip");
pub const GIF: MimeType = MimeType::new("image", "gif");
pub const HTM: MimeType = MimeType::new("text", "html");
pub const HTML: MimeType = MimeType::new("text", "html");
pub const ICO: MimeType = MimeType::new("image", "vnd.microsoft.icon");
pub const ICS: MimeType = MimeType::new("text", "calendar");
pub const JAR: MimeType = MimeType::new("application", "java-archive");
pub const JPEG: MimeType = MimeType::new("image", "jpeg");
pub const JPG: MimeType = MimeType::new("image", "jpeg");
pub const JS: MimeType = MimeType::new("text", "javascript");
pub const JSON: MimeType = MimeType::new("application", "json");
pub const JSONLD: MimeType = MimeType::new("application", "ld+json");
pub const MID: MimeType = MimeType::new("audio", "midi");
pub const MIDI: MimeType = MimeType::new("audio", "midi");
pub const MJS: MimeType = MimeType::new("text", "javascript");
pub const MP3: MimeType = MimeType::new("audio", "mpeg");
pub const MP4: MimeType = MimeType::new("video", "mp4");
pub const MPEG: MimeType = MimeType::new("video", "mpeg");
pub const MPKG: MimeType = MimeType::new("application", "vnd.apple.installer+xml");
pub const ODP: MimeType = MimeType::new("application", "vnd.oasis.opendocument.presentation");
pub const ODS: MimeType = MimeType::new("application", "vnd.oasis.opendocument.spreadsheet");
pub const ODT: MimeType = MimeType::new("application", "vnd.oasis.opendocument.text");
pub const OGA: MimeType = MimeType::new("audio", "ogg");
pub const OGV: MimeType = MimeType::new("video", "ogg");
pub const OGX: MimeType = MimeType::new("application", "ogg");
pub const OPUS: MimeType = MimeType::new("audio", "opus");
pub const OTF: MimeType = MimeType::new("font", "otf");
pub const PNG: MimeType = MimeType::new("image", "png");
pub const PDF: MimeType = MimeType::new("application", "pdf");
pub const PHP: MimeType = MimeType::new("application", "x-httpd-php");
pub const PPT: MimeType = MimeType::new("application", "vnd.ms-powerpoint");
pub const PPTX: MimeType = MimeType::new(
    "application",
    "vnd.openxmlformats-officedocument.presentationml.presentation",
);
pub const RAR: MimeType = MimeType::new("application", "vnd.rar");
pub const RTF: MimeType = MimeType::new("application", "rtf");
pub const SH: MimeType = MimeType::new("application", "x-sh");
pub const SVG: MimeType = MimeType::new("image", "svg+xml");
pub const TAR: MimeType = MimeType::new("application", "x-tar");
pub const TIF: MimeType = MimeType::new("image", "tiff");
pub const TIFF: MimeType = MimeType::new("image", "tiff");
pub const TS: MimeType = MimeType::new("video", "mp2t");
pub const TTF: MimeType = MimeType::new("font", "ttf");
pub const TXT: MimeType = MimeType::new("text", "plain");
pub const VSD: MimeType = MimeType::new("application", "vnd.visio");
pub const WAV: MimeType = MimeType::new("audio", "wav");
pub const WEBA: MimeType = MimeType::new("audio", "webm");
pub const WEBM: MimeType = MimeType::new("video", "webm");
pub const WEBP: MimeType = MimeType::new("image", "webp");
pub const WOFF: MimeType = MimeType::new("font", "woff");
pub const WOFF2: MimeType = MimeType::new("font", "woff2");
pub const XHTML: MimeType = MimeType::new("application", "xhtml+xml");
pub const XLS: MimeType = MimeType::new("application", "vnd.ms-excel");
pub const XLSX: MimeType = MimeType::new(
    "application",
    "vnd.openxmlformats-officedocument.spreadsheetml.sheet",
);
pub const XML: MimeType = MimeType::new("application", "xml");
pub const XUL: MimeType = MimeType::new("application", "vnd.mozilla.xul+xml");
pub const ZIP: MimeType = MimeType::new("application", "zip");
pub const _3GP: MimeType = MimeType::new("video", "3gpp");
pub const _3G2: MimeType = MimeType::new("video", "3gpp2");
pub const _7Z: MimeType = MimeType::new("application", "x-7z-compressed");

pub fn ext2mime(ext: &str) -> Option<MimeType> {
    match ext {
        "aac" => Some(AAC),
        "abw" => Some(ABW),
        "arc" => Some(ARC),
        "avi" => Some(AVI),
        "azw" => Some(AZW),
        "bin" => Some(BIN),
        "bmp" => Some(BMP),
        "bz" => Some(BZ),
        "bz2" => Some(BZ2),
        "cda" => Some(CDA),
        "csh" => Some(CSH),
        "css" => Some(CSS),
        "csv" => Some(CSV),
        "doc" => Some(DOC),
        "docx" => Some(DOCX),
        "eot" => Some(EOT),
        "epub" => Some(EPUB),
        "gz" => Some(GZ),
        "gif" => Some(GIF),
        "htm" => Some(HTM),
        "html" => Some(HTML),
        "ico" => Some(ICO),
        "ics" => Some(ICS),
        "jar" => Some(JAR),
        "jpeg" => Some(JPEG),
        "jpg" => Some(JPG),
        "js" => Some(JS),
        "json" => Some(JSON),
        "jsonld" => Some(JSONLD),
        "mid" => Some(MID),
        "midi" => Some(MIDI),
        "mjs" => Some(MJS),
        "mp3" => Some(MP3),
        "mp4" => Some(MP4),
        "mpeg" => Some(MPEG),
        "mpkg" => Some(MPKG),
        "odp" => Some(ODP),
        "ods" => Some(ODS),
        "odt" => Some(ODT),
        "oga" => Some(OGA),
        "ogv" => Some(OGV),
        "ogx" => Some(OGX),
        "opus" => Some(OPUS),
        "otf" => Some(OTF),
        "png" => Some(PNG),
        "pdf" => Some(PDF),
        "php" => Some(PHP),
        "ppt" => Some(PPT),
        "pptx" => Some(PPTX),
        "rar" => Some(RAR),
        "rtf" => Some(RTF),
        "sh" => Some(SH),
        "svg" => Some(SVG),
        "tar" => Some(TAR),
        "tif" => Some(TIF),
        "tiff" => Some(TIFF),
        "ts" => Some(TS),
        "ttf" => Some(TTF),
        "txt" => Some(TXT),
        "vsd" => Some(VSD),
        "wav" => Some(WAV),
        "weba" => Some(WEBA),
        "webm" => Some(WEBM),
        "webp" => Some(WEBP),
        "woff" => Some(WOFF),
        "woff2" => Some(WOFF2),
        "xhtml" => Some(XHTML),
        "xls" => Some(XLS),
        "xlsx" => Some(XLSX),
        "xml" => Some(XML),
        "xul" => Some(XUL),
        "zip" => Some(ZIP),
        "_3gp" => Some(_3GP),
        "_3g2" => Some(_3G2),
        "_7z" => Some(_7Z),
        _ => None,
    }
}
