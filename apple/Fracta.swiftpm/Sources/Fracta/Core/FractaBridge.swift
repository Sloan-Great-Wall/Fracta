import Foundation

// MARK: - Fracta FFI Bridge
//
// This file provides the Swift interface to the Rust FFI layer.
// In production, this will be replaced by the auto-generated UniFFI bindings.
//
// To generate real bindings:
// ```bash
// cargo run -p fracta-ffi --bin uniffi-bindgen generate \
//   --library target/debug/libfracta_ffi.dylib \
//   --language swift \
//   --out-dir apple/Fracta.swiftpm/Sources/Fracta/Generated
// ```

// MARK: - Protocol Definitions

/// Protocol for Location operations
protocol LocationProtocol {
    var id: UUID { get }
    var label: String { get }
    var root: String { get }
    var isManaged: Bool { get }

    func listDirectory(path: String) throws -> [FileItem]
    func readFile(path: String) throws -> String
    func writeFile(path: String, content: String) throws
    func createFile(path: String, content: String) throws
    func createFolder(path: String) throws
    func deleteFile(path: String) throws
    func deleteFolder(path: String) throws
}

/// Protocol for Document operations
protocol DocumentProtocol {
    var title: String? { get }
    var plainText: String { get }
    var hasFrontMatter: Bool { get }
    var blockCount: Int { get }

    func frontMatterString(key: String) -> String?
    func frontMatterStringList(key: String) -> [String]?
}

/// Protocol for Index operations
protocol IndexProtocol {
    func search(query: String, limit: Int) throws -> [SearchHit]
    func searchByMetadata(area: String?, tag: String?, dateFrom: String?, dateTo: String?, limit: Int) throws -> [String]
    func fileCount() throws -> Int
    func indexedCount() throws -> Int
}

// MARK: - Mock Implementations (for development)

#if DEBUG

/// Mock Location for development/preview
class MockLocation: LocationProtocol {
    let id = UUID()
    let label: String
    let root: String
    var isManaged = true

    init(label: String, root: String) {
        self.label = label
        self.root = root
    }

    func listDirectory(path: String) throws -> [FileItem] {
        return FileItem.demoFiles
    }

    func readFile(path: String) throws -> String {
        return """
        ---
        title: Mock Document
        tags: [demo, test]
        ---

        # Hello World

        This is a mock document for development.
        """
    }

    func writeFile(path: String, content: String) throws {
        print("Mock: writeFile(\(path))")
    }

    func createFile(path: String, content: String) throws {
        print("Mock: createFile(\(path))")
    }

    func createFolder(path: String) throws {
        print("Mock: createFolder(\(path))")
    }

    func deleteFile(path: String) throws {
        print("Mock: deleteFile(\(path))")
    }

    func deleteFolder(path: String) throws {
        print("Mock: deleteFolder(\(path))")
    }
}

/// Mock Document for development/preview
class MockDocument: DocumentProtocol {
    let title: String?
    let plainText: String
    let hasFrontMatter: Bool
    let blockCount: Int

    private let metadata: [String: Any]

    init(markdown: String) {
        // Simple mock parsing
        self.title = "Mock Title"
        self.plainText = markdown
        self.hasFrontMatter = markdown.hasPrefix("---")
        self.blockCount = markdown.components(separatedBy: "\n\n").count
        self.metadata = ["tags": ["demo", "test"], "area": "library"]
    }

    func frontMatterString(key: String) -> String? {
        return metadata[key] as? String
    }

    func frontMatterStringList(key: String) -> [String]? {
        return metadata[key] as? [String]
    }
}

/// Mock Index for development/preview
class MockIndex: IndexProtocol {
    func search(query: String, limit: Int) throws -> [SearchHit] {
        return [
            SearchHit(id: "/demo/notes.md", path: "/demo/notes.md", title: "Notes", score: 0.95),
            SearchHit(id: "/demo/inbox.md", path: "/demo/inbox.md", title: "Inbox", score: 0.8)
        ]
    }

    func searchByMetadata(area: String?, tag: String?, dateFrom: String?, dateTo: String?, limit: Int) throws -> [String] {
        return ["/demo/notes.md", "/demo/inbox.md"]
    }

    func fileCount() throws -> Int {
        return 5
    }

    func indexedCount() throws -> Int {
        return 3
    }
}

#endif

// MARK: - Bridge Manager

/// Central bridge to Rust FFI layer
@MainActor
class FractaBridge: ObservableObject {
    static let shared = FractaBridge()

    @Published var isInitialized = false
    @Published var error: BridgeError?

    private init() {
        initialize()
    }

    private func initialize() {
        // In production, this would initialize the Rust FFI layer
        // For now, mark as initialized for mock usage
        #if DEBUG
        isInitialized = true
        #else
        // TODO: Initialize real FFI bridge
        // try FractaFFI.initialize()
        isInitialized = true
        #endif
    }

    // MARK: - Location Operations

    func openLocation(label: String, path: String) throws -> any LocationProtocol {
        #if DEBUG
        return MockLocation(label: label, root: path)
        #else
        // TODO: Use real FFI
        // return try FfiLocation.open(label: label, root: path)
        throw BridgeError.notImplemented
        #endif
    }

    func createLocation(label: String, path: String) throws -> any LocationProtocol {
        #if DEBUG
        let location = MockLocation(label: label, root: path)
        return location
        #else
        // TODO: Use real FFI
        // let location = FfiLocation(label: label, root: path)
        // try location.init()
        // return location
        throw BridgeError.notImplemented
        #endif
    }

    // MARK: - Document Operations

    func parseDocument(markdown: String) -> any DocumentProtocol {
        #if DEBUG
        return MockDocument(markdown: markdown)
        #else
        // TODO: Use real FFI
        // return FfiDocument.parse(markdown: markdown)
        return MockDocument(markdown: markdown)
        #endif
    }

    // MARK: - Index Operations

    func openIndex(location: any LocationProtocol) throws -> any IndexProtocol {
        #if DEBUG
        return MockIndex()
        #else
        // TODO: Use real FFI
        // return try FfiIndex.open(location: location as! FfiLocation)
        throw BridgeError.notImplemented
        #endif
    }

    // MARK: - Version

    var ffiVersion: String {
        #if DEBUG
        return "0.1.0-mock"
        #else
        // TODO: Use real FFI
        // return ffiVersion()
        return "0.1.0"
        #endif
    }
}

// MARK: - Bridge Errors

enum BridgeError: LocalizedError {
    case notInitialized
    case notImplemented
    case ffiError(String)
    case notFound(String)
    case permissionDenied(String)

    var errorDescription: String? {
        switch self {
        case .notInitialized:
            return "FFI bridge is not initialized"
        case .notImplemented:
            return "This feature is not yet implemented"
        case .ffiError(let message):
            return "FFI error: \(message)"
        case .notFound(let path):
            return "Not found: \(path)"
        case .permissionDenied(let path):
            return "Permission denied: \(path)"
        }
    }
}
