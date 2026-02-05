import Foundation

// MARK: - Location State

/// Represents a managed location (directory tree)
struct LocationState: Identifiable, Equatable {
    let id: UUID
    let label: String
    let rootPath: String
    var isManaged: Bool

    static let demo = LocationState(
        id: UUID(),
        label: "Demo Location",
        rootPath: "/Users/Demo/Documents",
        isManaged: true
    )
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
}

enum FileKind: String, Codable {
    case file
    case folder
}

enum FileScope: String, Codable {
    case managed
    case ignored
    case plain
}

// MARK: - Document

/// A parsed Markdown document
struct DocumentContent: Equatable {
    let title: String?
    let plainText: String
    let hasFrontMatter: Bool
    let tags: [String]
    let area: String?
    let blockCount: Int
}

// MARK: - Search

/// A search result hit
struct SearchHit: Identifiable {
    let id: String  // path
    let path: String
    let title: String?
    let score: Float
}

// MARK: - Demo Data

#if DEBUG
extension FileItem {
    static let demoFiles: [FileItem] = [
        FileItem(
            id: "/demo/Projects",
            path: "/demo/Projects",
            name: "Projects",
            kind: .folder,
            size: 0,
            modified: Date(),
            created: Date(),
            scope: .managed,
            fileExtension: nil
        ),
        FileItem(
            id: "/demo/Library",
            path: "/demo/Library",
            name: "Library",
            kind: .folder,
            size: 0,
            modified: Date().addingTimeInterval(-86400),
            created: Date().addingTimeInterval(-86400 * 30),
            scope: .managed,
            fileExtension: nil
        ),
        FileItem(
            id: "/demo/inbox.md",
            path: "/demo/inbox.md",
            name: "inbox.md",
            kind: .file,
            size: 2048,
            modified: Date().addingTimeInterval(-3600),
            created: Date().addingTimeInterval(-86400 * 7),
            scope: .managed,
            fileExtension: "md"
        ),
        FileItem(
            id: "/demo/notes.md",
            path: "/demo/notes.md",
            name: "notes.md",
            kind: .file,
            size: 4096,
            modified: Date().addingTimeInterval(-7200),
            created: Date().addingTimeInterval(-86400 * 14),
            scope: .managed,
            fileExtension: "md"
        ),
        FileItem(
            id: "/demo/config.json",
            path: "/demo/config.json",
            name: "config.json",
            kind: .file,
            size: 512,
            modified: Date().addingTimeInterval(-86400 * 3),
            created: Date().addingTimeInterval(-86400 * 30),
            scope: .managed,
            fileExtension: "json"
        )
    ]

    static let demoDocument = DocumentContent(
        title: "Welcome to Fracta",
        plainText: """
        Welcome to Fracta — your local-first life operating system.

        Fracta helps you organize your digital life with:
        - File browsing with smart indexing
        - Full-text search across all your documents
        - AI-assisted insights and summaries
        - Game controller navigation support

        Get started by adding a location (folder) to manage.
        """,
        hasFrontMatter: true,
        tags: ["welcome", "getting-started"],
        area: "library",
        blockCount: 5
    )
}
#endif
