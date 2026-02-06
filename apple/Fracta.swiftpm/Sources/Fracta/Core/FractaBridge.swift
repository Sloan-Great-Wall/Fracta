import Foundation

// MARK: - Fracta FFI Bridge
//
// This file bridges the Rust FFI layer to Swift using the auto-generated UniFFI bindings.
// The bridge manages FfiLocation/FfiIndex/FfiDocument instances and converts FFI types
// to SwiftUI-friendly models.

// MARK: - Bridge Manager

/// Central bridge to Rust FFI layer
@MainActor
class FractaBridge: ObservableObject {
    static let shared = FractaBridge()

    @Published private(set) var isInitialized = false
    @Published var lastError: BridgeError?

    /// Cache of open locations (path -> FfiLocation)
    private var locations: [String: FfiLocation] = [:]

    /// Cache of open indexes (path -> FfiIndex)
    private var indexes: [String: FfiIndex] = [:]

    private init() {
        isInitialized = true
    }

    // MARK: - Location Operations

    /// Open an existing managed Location
    func openLocation(label: String, path: String) throws -> LocationState {
        if let cached = locations[path] {
            return LocationState(
                id: UUID(), // Generate new for each open
                label: cached.label(),
                rootPath: cached.root(),
                isManaged: cached.isManaged()
            )
        }

        do {
            let location = try FfiLocation.open(label: label, root: path)
            locations[path] = location
            return LocationState(
                id: UUID(),
                label: location.label(),
                rootPath: location.root(),
                isManaged: location.isManaged()
            )
        } catch let error as FfiError {
            throw BridgeError.from(error)
        }
    }

    /// Create and initialize a new Location
    func createLocation(label: String, path: String) throws -> LocationState {
        do {
            let location = FfiLocation(label: label, root: path)
            try location.`init`()
            locations[path] = location
            return LocationState(
                id: UUID(),
                label: location.label(),
                rootPath: location.root(),
                isManaged: location.isManaged()
            )
        } catch let error as FfiError {
            throw BridgeError.from(error)
        }
    }

    /// Get cached FfiLocation by path
    private func getLocation(path: String) -> FfiLocation? {
        return locations[path]
    }

    /// List directory contents
    func listDirectory(locationPath: String, directoryPath: String) throws -> [FileItem] {
        guard let location = locations[locationPath] else {
            throw BridgeError.notFound("Location not open: \(locationPath)")
        }

        do {
            let entries = try location.listDirectory(path: directoryPath)
            return entries.map { FileItem(from: $0) }
        } catch let error as FfiError {
            throw BridgeError.from(error)
        }
    }

    /// Read file content
    func readFile(locationPath: String, filePath: String) throws -> String {
        guard let location = locations[locationPath] else {
            throw BridgeError.notFound("Location not open: \(locationPath)")
        }

        do {
            return try location.readFile(path: filePath)
        } catch let error as FfiError {
            throw BridgeError.from(error)
        }
    }

    /// Write file content
    func writeFile(locationPath: String, filePath: String, content: String) throws {
        guard let location = locations[locationPath] else {
            throw BridgeError.notFound("Location not open: \(locationPath)")
        }

        do {
            try location.writeFile(path: filePath, content: content)
        } catch let error as FfiError {
            throw BridgeError.from(error)
        }
    }

    /// Create a new file
    func createFile(locationPath: String, filePath: String, content: String) throws {
        guard let location = locations[locationPath] else {
            throw BridgeError.notFound("Location not open: \(locationPath)")
        }

        do {
            try location.createFile(path: filePath, content: content)
        } catch let error as FfiError {
            throw BridgeError.from(error)
        }
    }

    /// Create a new folder
    func createFolder(locationPath: String, folderPath: String) throws {
        guard let location = locations[locationPath] else {
            throw BridgeError.notFound("Location not open: \(locationPath)")
        }

        do {
            try location.createFolder(path: folderPath)
        } catch let error as FfiError {
            throw BridgeError.from(error)
        }
    }

    /// Delete a file
    func deleteFile(locationPath: String, filePath: String) throws {
        guard let location = locations[locationPath] else {
            throw BridgeError.notFound("Location not open: \(locationPath)")
        }

        do {
            try location.deleteFile(path: filePath)
        } catch let error as FfiError {
            throw BridgeError.from(error)
        }
    }

    /// Delete a folder
    func deleteFolder(locationPath: String, folderPath: String) throws {
        guard let location = locations[locationPath] else {
            throw BridgeError.notFound("Location not open: \(locationPath)")
        }

        do {
            try location.deleteFolder(path: folderPath)
        } catch let error as FfiError {
            throw BridgeError.from(error)
        }
    }

    /// Rename a file or folder
    func rename(locationPath: String, itemPath: String, newName: String) throws -> String {
        guard let location = locations[locationPath] else {
            throw BridgeError.notFound("Location not open: \(locationPath)")
        }

        do {
            return try location.rename(path: itemPath, newName: newName)
        } catch let error as FfiError {
            throw BridgeError.from(error)
        }
    }

    // MARK: - Document Operations

    /// Parse a Markdown document
    func parseDocument(markdown: String) -> DocumentContent {
        let doc = FfiDocument.parse(markdown: markdown)
        return DocumentContent(from: doc)
    }

    /// Read and parse a Markdown file
    func readDocument(locationPath: String, filePath: String) throws -> DocumentContent {
        let content = try readFile(locationPath: locationPath, filePath: filePath)
        return parseDocument(markdown: content)
    }

    /// Read and parse a Markdown file by absolute path (for folder pages)
    func readDocumentAtPath(_ path: String) throws -> DocumentContent {
        let content = try String(contentsOfFile: path, encoding: .utf8)
        return parseDocument(markdown: content)
    }

    // MARK: - Index Operations

    /// Open or create an index for a location
    func openIndex(locationPath: String) throws -> FfiIndex {
        if let cached = indexes[locationPath] {
            return cached
        }

        guard let location = locations[locationPath] else {
            throw BridgeError.notFound("Location not open: \(locationPath)")
        }

        do {
            let index = try FfiIndex.open(location: location)
            indexes[locationPath] = index
            return index
        } catch let error as FfiError {
            throw BridgeError.from(error)
        }
    }

    /// Search for documents
    func search(locationPath: String, query: String, limit: UInt32 = 50) throws -> [SearchHit] {
        let index = try openIndex(locationPath: locationPath)

        do {
            let hits = try index.search(query: query, limit: limit)
            return hits.map { SearchHit(from: $0) }
        } catch let error as FfiError {
            throw BridgeError.from(error)
        }
    }

    /// Search by metadata
    func searchByMetadata(
        locationPath: String,
        area: String? = nil,
        tag: String? = nil,
        dateFrom: String? = nil,
        dateTo: String? = nil,
        limit: UInt32 = 50
    ) throws -> [String] {
        let index = try openIndex(locationPath: locationPath)

        do {
            return try index.searchByMetadata(
                area: area,
                tag: tag,
                dateFrom: dateFrom,
                dateTo: dateTo,
                limit: limit
            )
        } catch let error as FfiError {
            throw BridgeError.from(error)
        }
    }

    /// Build full index (runs on main thread - prefer buildFullIndexAsync for background)
    func buildFullIndex(locationPath: String) throws -> (filesScanned: UInt32, markdownIndexed: UInt32) {
        guard let location = locations[locationPath] else {
            throw BridgeError.notFound("Location not open: \(locationPath)")
        }

        let index = try openIndex(locationPath: locationPath)

        do {
            let stats = try index.buildFull(location: location)
            return (stats.filesScanned, stats.markdownIndexed)
        } catch let error as FfiError {
            throw BridgeError.from(error)
        }
    }

    /// Build full index asynchronously in background (non-blocking)
    /// This creates fresh FFI objects on a background thread to avoid blocking the main thread.
    static func buildFullIndexAsync(
        label: String,
        locationPath: String
    ) async throws -> (filesScanned: UInt32, markdownIndexed: UInt32) {
        // Run the heavy work on a background thread
        return try await Task.detached(priority: .userInitiated) {
            // Create fresh FFI objects for this background operation
            // This avoids any @MainActor isolation issues
            let location = try FfiLocation.open(label: label, root: locationPath)
            let index = try FfiIndex.open(location: location)
            let stats = try index.buildFull(location: location)
            return (stats.filesScanned, stats.markdownIndexed)
        }.value
    }

    /// Update index incrementally
    func updateIndex(locationPath: String) throws -> (filesScanned: UInt32, markdownIndexed: UInt32) {
        guard let location = locations[locationPath] else {
            throw BridgeError.notFound("Location not open: \(locationPath)")
        }

        let index = try openIndex(locationPath: locationPath)

        do {
            let stats = try index.updateIncremental(location: location)
            return (stats.filesScanned, stats.markdownIndexed)
        } catch let error as FfiError {
            throw BridgeError.from(error)
        }
    }

    /// Get index file count
    func indexFileCount(locationPath: String) throws -> UInt32 {
        let index = try openIndex(locationPath: locationPath)

        do {
            return try index.fileCount()
        } catch let error as FfiError {
            throw BridgeError.from(error)
        }
    }

    // MARK: - Version

    var version: String {
        // Call the global ffiVersion() function from the generated FFI module
        Fracta.ffiVersion()
    }

    // MARK: - Cleanup

    /// Close a location and its index
    func closeLocation(path: String) {
        locations.removeValue(forKey: path)
        indexes.removeValue(forKey: path)
    }

    /// Close all locations
    func closeAll() {
        locations.removeAll()
        indexes.removeAll()
    }
}

// MARK: - Bridge Errors

enum BridgeError: LocalizedError {
    case notInitialized
    case notFound(String)
    case outsideLocation(String)
    case permissionDenied(String)
    case alreadyExists(String)
    case ioError(String)
    case indexError(String)
    case invalidArgument(String)
    case internalError(String)

    static func from(_ error: FfiError) -> BridgeError {
        switch error {
        case .NotFound(let path):
            return .notFound(path)
        case .OutsideLocation(let path):
            return .outsideLocation(path)
        case .PermissionDenied(let path):
            return .permissionDenied(path)
        case .AlreadyExists(let path):
            return .alreadyExists(path)
        case .Io(let message):
            return .ioError(message)
        case .Index(let message):
            return .indexError(message)
        case .InvalidArgument(let message):
            return .invalidArgument(message)
        case .Internal(let message):
            return .internalError(message)
        }
    }

    var errorDescription: String? {
        switch self {
        case .notInitialized:
            return "FFI bridge is not initialized"
        case .notFound(let path):
            return "Not found: \(path)"
        case .outsideLocation(let path):
            return "Path outside managed location: \(path)"
        case .permissionDenied(let path):
            return "Permission denied: \(path)"
        case .alreadyExists(let path):
            return "Already exists: \(path)"
        case .ioError(let message):
            return "I/O error: \(message)"
        case .indexError(let message):
            return "Index error: \(message)"
        case .invalidArgument(let message):
            return "Invalid argument: \(message)"
        case .internalError(let message):
            return "Internal error: \(message)"
        }
    }
}
