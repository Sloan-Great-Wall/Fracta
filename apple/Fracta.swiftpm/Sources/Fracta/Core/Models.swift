import Foundation

// MARK: - Location State

/// Represents a managed location (directory tree)
struct LocationState: Identifiable, Equatable, Codable, Hashable {
    let id: UUID
    let label: String
    let rootPath: String
    var isManaged: Bool

    /// Check if this location contains or is contained by another path
    func overlaps(with path: String) -> Bool {
        let normalizedSelf = rootPath.hasSuffix("/") ? rootPath : rootPath + "/"
        let normalizedOther = path.hasSuffix("/") ? path : path + "/"
        return normalizedSelf.hasPrefix(normalizedOther) || normalizedOther.hasPrefix(normalizedSelf)
    }
}

/// Quick access bookmark
struct QuickAccessItem: Identifiable, Codable, Hashable {
    let id: UUID
    let label: String
    let path: String
    let icon: String

    init(id: UUID = UUID(), label: String, path: String, icon: String = "folder.fill") {
        self.id = id
        self.label = label
        self.path = path
        self.icon = icon
    }
}

// MARK: - File Item

/// A file or folder entry
struct FileItem: Identifiable, Hashable {
    let id: String  // path as ID
    let path: String
    let name: String
    let kind: FileKind
    let size: UInt64
    let modified: Date?
    let created: Date?
    let scope: FileScope
    let fileExtension: String?

    var isFolder: Bool { kind == .folder }
    var isMarkdown: Bool { fileExtension == "md" || fileExtension == "markdown" }

    var icon: String {
        switch kind {
        case .folder:
            return "folder.fill"
        case .file:
            if isMarkdown { return "doc.richtext.fill" }
            switch fileExtension {
            case "json": return "curlybraces"
            case "yaml", "yml": return "list.bullet.rectangle"
            case "png", "jpg", "jpeg", "gif", "webp": return "photo.fill"
            case "pdf": return "doc.fill"
            case "swift", "rs", "py", "js", "ts": return "chevron.left.forwardslash.chevron.right"
            default: return "doc.fill"
            }
        }
    }

    var formattedSize: String {
        ByteCountFormatter.string(fromByteCount: Int64(size), countStyle: .file)
    }

    var formattedDate: String {
        guard let date = modified else { return "—" }
        let formatter = RelativeDateTimeFormatter()
        formatter.unitsStyle = .abbreviated
        return formatter.localizedString(for: date, relativeTo: Date())
    }

    /// Initialize from FFI entry
    init(from entry: FfiEntry) {
        self.id = entry.path
        self.path = entry.path
        self.name = entry.name
        self.kind = FileKind(from: entry.kind)
        self.size = entry.size
        self.modified = Self.parseISO8601(entry.modified)
        self.created = entry.created.flatMap { Self.parseISO8601($0) }
        self.scope = FileScope(from: entry.scope)
        self.fileExtension = entry.extension
    }

    /// Manual initializer for creating FileItem from search results
    init(
        id: String,
        path: String,
        name: String,
        kind: FileKind,
        size: UInt64,
        modified: Date?,
        created: Date?,
        scope: FileScope,
        fileExtension: String?
    ) {
        self.id = id
        self.path = path
        self.name = name
        self.kind = kind
        self.size = size
        self.modified = modified
        self.created = created
        self.scope = scope
        self.fileExtension = fileExtension
    }

    /// Parse ISO 8601 date string
    private static func parseISO8601(_ string: String) -> Date? {
        let formatter = ISO8601DateFormatter()
        formatter.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
        if let date = formatter.date(from: string) {
            return date
        }
        // Try without fractional seconds
        formatter.formatOptions = [.withInternetDateTime]
        return formatter.date(from: string)
    }
}

enum FileKind: String, Codable {
    case file
    case folder

    init(from ffiKind: FfiEntryKind) {
        switch ffiKind {
        case .file:
            self = .file
        case .folder:
            self = .folder
        }
    }
}

enum FileScope: String, Codable {
    case managed
    case ignored
    case plain

    init(from ffiScope: FfiScope) {
        switch ffiScope {
        case .managed:
            self = .managed
        case .ignored:
            self = .ignored
        case .plain:
            self = .plain
        }
    }
}

// MARK: - Document

/// A parsed Markdown document
struct DocumentContent: Equatable {
    let title: String?
    let plainText: String          // Preview only (max 8KB for display)
    let fullCharacterCount: Int    // Total character count (computed once)
    let hasFrontMatter: Bool
    let tags: [String]
    let area: String?
    let blockCount: Int

    /// Is this a large file with truncated preview?
    var isTruncated: Bool { fullCharacterCount > plainText.count }

    /// Initialize from FFI document
    init(from doc: FfiDocument) {
        let fullText = doc.plainText()
        self.title = doc.title()
        self.fullCharacterCount = fullText.count
        self.plainText = fullText.count > 8000 ? String(fullText.prefix(8000)) : fullText
        self.hasFrontMatter = doc.hasFrontMatter()
        self.tags = doc.frontMatterStringList(key: "tags") ?? []
        self.area = doc.frontMatterString(key: "area")
        self.blockCount = Int(doc.blockCount())
    }

    /// Manual initializer (plainText should already be truncated, fullCharacterCount is original size)
    init(
        title: String?,
        plainText: String,
        fullCharacterCount: Int,
        hasFrontMatter: Bool,
        tags: [String],
        area: String?,
        blockCount: Int
    ) {
        self.title = title
        self.plainText = plainText
        self.fullCharacterCount = fullCharacterCount
        self.hasFrontMatter = hasFrontMatter
        self.tags = tags
        self.area = area
        self.blockCount = blockCount
    }
}

// MARK: - Search

/// A search result hit
struct SearchHit: Identifiable {
    let id: String  // path
    let path: String
    let title: String?
    let score: Float

    /// Initialize from FFI search hit
    init(from hit: FfiSearchHit) {
        self.id = hit.path
        self.path = hit.path
        self.title = hit.title
        self.score = hit.score
    }

    /// Manual initializer
    init(id: String, path: String, title: String?, score: Float) {
        self.id = id
        self.path = path
        self.title = title
        self.score = score
    }
}

// MARK: - AI

/// Role in an AI conversation
enum AiRole {
    case system
    case user
    case assistant

    func toFfi() -> FfiChatRole {
        switch self {
        case .system: return .system
        case .user: return .user
        case .assistant: return .assistant
        }
    }
}

/// Response from the AI engine
struct AiResponse {
    let content: String
    let tokensUsed: UInt32
    let model: String
}
