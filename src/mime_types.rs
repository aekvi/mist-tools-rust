pub struct MimeType {
    tpe: &'static str,
    tail: &'static str,
}

impl MimeType {
    const fn new(tpe: &'static str, tail: &'static str) -> MimeType {
        MimeType { tpe, tail }
    }
}

const AAC: MimeType = MimeType::new("audio", "aac");
const ABW: MimeType = MimeType::new("application", "x-abiword");
const ARC: MimeType = MimeType::new("application", "x-freearc");
const AVIF: MimeType = MimeType::new("image", "avif");
const AVI: MimeType = MimeType::new("video", "x-msvideo");
const AZW: MimeType = MimeType::new("application", "vnd.amazon.ebook");
const BIN: MimeType = MimeType::new("application", "octet-stream");
const BMP: MimeType = MimeType::new("image", "bmp");
const BZ: MimeType = MimeType::new("application", "x-bzip");
const BZ2: MimeType = MimeType::new("application", "x-bzip2");
const CDA: MimeType = MimeType::new("application", "x-cdf");
const CSH: MimeType = MimeType::new("application", "x-csh");
const CSS: MimeType = MimeType::new("text", "css");
const CSV: MimeType = MimeType::new("text", "csv");
const DOC: MimeType = MimeType::new("application", "msword");
const DOCX: MimeType = MimeType::new(
    "application",
    "vnd.openxmlformats-officedocument.wordprocessingml.document",
);
const EOT: MimeType = MimeType::new("application", "vnd.ms-fontobject");
const EPUB: MimeType = MimeType::new("application", "epub+zip");
const GZ: MimeType = MimeType::new("application", "gzip");
const GIF: MimeType = MimeType::new("image", "gif");
const HTM: MimeType = MimeType::new("text", "html");
const HTML: MimeType = MimeType::new("text", "html");
const ICO: MimeType = MimeType::new("image", "vnd.microsoft.icon");
const ICS: MimeType = MimeType::new("text", "calendar");
const JAR: MimeType = MimeType::new("application", "java-archive");
const JPEG: MimeType = MimeType::new("image", "jpeg");
const JPG: MimeType = MimeType::new("image", "jpeg");
const JS: MimeType = MimeType::new("text", "javascript");
const JSON: MimeType = MimeType::new("application", "json");
const JSONLD: MimeType = MimeType::new("application", "ld+json");
const MID: MimeType = MimeType::new("audio", "midi");
const MIDI: MimeType = MimeType::new("audio", "midi");
const MJS: MimeType = MimeType::new("text", "javascript");
const MP3: MimeType = MimeType::new("audio", "mpeg");
const MP4: MimeType = MimeType::new("video", "mp4");
const MPEG: MimeType = MimeType::new("video", "mpeg");
const MPKG: MimeType = MimeType::new("application", "vnd.apple.installer+xml");
const ODP: MimeType = MimeType::new("application", "vnd.oasis.opendocument.presentation");
const ODS: MimeType = MimeType::new("application", "vnd.oasis.opendocument.spreadsheet");
const ODT: MimeType = MimeType::new("application", "vnd.oasis.opendocument.text");
const OGA: MimeType = MimeType::new("audio", "ogg");
const OGV: MimeType = MimeType::new("video", "ogg");
const OGX: MimeType = MimeType::new("application", "ogg");
const OPUS: MimeType = MimeType::new("audio", "opus");
const OTF: MimeType = MimeType::new("font", "otf");
const PNG: MimeType = MimeType::new("image", "png");
const PDF: MimeType = MimeType::new("application", "pdf");
const PHP: MimeType = MimeType::new("application", "x-httpd-php");
const PPT: MimeType = MimeType::new("application", "vnd.ms-powerpoint");
const PPTX: MimeType = MimeType::new(
    "application",
    "vnd.openxmlformats-officedocument.presentationml.presentation",
);
const RAR: MimeType = MimeType::new("application", "vnd.rar");
const RTF: MimeType = MimeType::new("application", "rtf");
const SH: MimeType = MimeType::new("application", "x-sh");
const SVG: MimeType = MimeType::new("image", "svg+xml");
const TAR: MimeType = MimeType::new("application", "x-tar");
const TIF: MimeType = MimeType::new("image", "tiff");
const TIFF: MimeType = MimeType::new("image", "tiff");
const TS: MimeType = MimeType::new("video", "mp2t");
const TTF: MimeType = MimeType::new("font", "ttf");
const TXT: MimeType = MimeType::new("text", "plain");
const VSD: MimeType = MimeType::new("application", "vnd.visio");
const WAV: MimeType = MimeType::new("audio", "wav");
const WEBA: MimeType = MimeType::new("audio", "webm");
const WEBM: MimeType = MimeType::new("video", "webm");
const WEBP: MimeType = MimeType::new("image", "webp");
const WOFF: MimeType = MimeType::new("font", "woff");
const WOFF2: MimeType = MimeType::new("font", "woff2");
const XHTML: MimeType = MimeType::new("application", "xhtml+xml");
const XLS: MimeType = MimeType::new("application", "vnd.ms-excel");
const XLSX: MimeType = MimeType::new(
    "application",
    "vnd.openxmlformats-officedocument.spreadsheetml.sheet",
);
const XML: MimeType = MimeType::new("application", "xml");
const XUL: MimeType = MimeType::new("application", "vnd.mozilla.xul+xml");
const ZIP: MimeType = MimeType::new("application", "zip");
const _3GP: MimeType = MimeType::new("video", "3gpp");
const _3G2: MimeType = MimeType::new("video", "3gpp2");
const _7Z: MimeType = MimeType::new("application", "x-7z-compressed");

fn ext2mime(ext: &str) -> Option<MimeType> {
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
