import SwiftUI

/// Fracta — Local-first life operating system
///
/// Multi-platform app supporting macOS, iOS, iPadOS, and visionOS.
/// Designed with Liquid Glass aesthetics and game controller support.
@main
struct FractaApp: App {
    @StateObject private var appState = AppState()

    var body: some Scene {
        #if os(macOS)
        Window("Fracta", id: "main") {
            ContentView()
                .environmentObject(appState)
                .frame(minWidth: 800, minHeight: 600)
                .sheet(isPresented: $appState.showingOnboarding) {
                    OnboardingView()
                        .environmentObject(appState)
                }
                .task {
                    // Auto-open home directory after view is ready
                    appState.openHomeDirectoryIfNeeded()
                }
        }
        .windowStyle(.hiddenTitleBar)
        .defaultSize(width: 1200, height: 800)
        .commands {
            CommandGroup(replacing: .newItem) {
                Button("Open Location...") {
                    appState.showingFolderPicker = true
                }
                .keyboardShortcut("o", modifiers: .command)
            }
        }

        Settings {
            SettingsView()
                .environmentObject(appState)
        }
        #elseif os(visionOS)
        WindowGroup {
            ContentView()
                .environmentObject(appState)
                .sheet(isPresented: $appState.showingOnboarding) {
                    OnboardingView()
                        .environmentObject(appState)
                }
                .task {
                    appState.openHomeDirectoryIfNeeded()
                }
        }
        .windowStyle(.volumetric)
        #else
        WindowGroup {
            ContentView()
                .environmentObject(appState)
                .sheet(isPresented: $appState.showingOnboarding) {
                    OnboardingView()
                        .environmentObject(appState)
                }
                .task {
                    appState.openHomeDirectoryIfNeeded()
                }
        }
        #endif
    }
}

/// Global application state
@MainActor
class AppState: ObservableObject {
    // MARK: - Locations (Multiple Data Sources)

    /// All managed locations
    @Published var locations: [LocationState] = [] {
        didSet { saveLocations() }
    }

    /// Currently active/browsing location
    @Published var activeLocation: LocationState?

    /// Quick access bookmarks
    @Published var quickAccessItems: [QuickAccessItem] = [] {
        didSet { saveQuickAccess() }
    }

    /// Navigation path for drill-down
    @Published var navigationPath: [PathItem] = []

    /// Current browsing path within active location
    @Published var currentPath: String = ""

    /// Search query
    @Published var searchQuery: String = ""

    /// Is search active
    @Published var isSearching: Bool = false

    /// Current view mode
    @Published var viewMode: ViewMode = .browser

    /// Selected file for preview
    @Published var selectedFile: FileItem?

    /// Folder page content (when folder has same-named .md file)
    @Published var folderPageContent: DocumentContent?

    // MARK: - Onboarding State

    /// Has user completed onboarding
    @AppStorage("hasCompletedOnboarding") var hasCompletedOnboarding: Bool = false

    /// Show onboarding view
    @Published var showingOnboarding: Bool = false

    // MARK: - Folder Picker State

    /// Show folder picker
    @Published var showingFolderPicker: Bool = false

    // MARK: - Error Handling State

    /// Current error to display
    @Published var currentError: AppError?

    /// Show error alert
    @Published var showingError: Bool = false

    // MARK: - AI State

    /// Show AI chat sheet
    @Published var showingAI: Bool = false

    // MARK: - Watcher / Indexing State

    /// Whether the filesystem watcher is active
    @Published var isWatching: Bool = false

    /// Whether an incremental index update is running
    @Published var isIndexing: Bool = false

    /// Timer that polls the watcher for events
    private var watcherTimer: Timer?

    // MARK: - Loading State

    /// Is performing a long operation
    @Published var isLoading: Bool = false

    /// Loading message
    @Published var loadingMessage: String = ""

    // MARK: - Persistence Keys
    private let locationsKey = "fracta.locations"
    private let quickAccessKey = "fracta.quickAccess"

    init() {
        // Load persisted data
        loadLocations()
        loadQuickAccess()

        // Show onboarding on first launch
        if !hasCompletedOnboarding {
            showingOnboarding = true
        }
    }

    /// Open home directory if onboarding is complete and no location is open
    func openHomeDirectoryIfNeeded() {
        guard hasCompletedOnboarding else { return }

        // If no locations exist, add home directory
        if locations.isEmpty {
            let homeURL = URL(fileURLWithPath: NSHomeDirectory())
            addLocation(at: homeURL)
        }

        // Activate the first location if none active
        if activeLocation == nil, let first = locations.first {
            activateLocation(first)
        }
    }

    // MARK: - Location Operations

    /// Add a new location (data source)
    func addLocation(at url: URL) {
        let path = url.path

        // Check for overlapping locations
        if let overlapping = locations.first(where: { $0.overlaps(with: path) }) {
            showError(.generic("This folder overlaps with '\(overlapping.label)'. Remove it first or choose a different folder."))
            return
        }

        // Create security-scoped bookmark for persistent access across app restarts
        let bookmark = try? url.bookmarkData(
            options: [.withSecurityScope],
            includingResourceValuesForKeys: nil,
            relativeTo: nil
        )

        isLoading = true
        loadingMessage = "Adding location..."

        Task {
            do {
                let label = url.lastPathComponent

                // Try to open existing managed location first
                var locationState: LocationState
                do {
                    locationState = try FractaBridge.shared.openLocation(
                        label: label,
                        path: path
                    )
                } catch BridgeError.notFound {
                    // Not managed yet, create new location
                    locationState = try FractaBridge.shared.createLocation(
                        label: label,
                        path: path
                    )
                }

                // Attach bookmark data for persistence
                locationState.bookmarkData = bookmark

                await MainActor.run {
                    // Add to locations if not already present
                    if !locations.contains(where: { $0.rootPath == path }) {
                        locations.append(locationState)
                    }
                    // Activate the new location
                    activateLocation(locationState)
                    isLoading = false
                    loadingMessage = ""
                }
            } catch {
                await MainActor.run {
                    showError(.locationOpenFailed(error.localizedDescription))
                    isLoading = false
                    loadingMessage = ""
                }
            }
        }
    }

    /// Activate a location for browsing
    func activateLocation(_ location: LocationState) {
        // Ensure the location is opened in the Rust backend
        Task {
            do {
                // Try to open in FractaBridge (may already be open, that's OK)
                _ = try FractaBridge.shared.openLocation(
                    label: location.label,
                    path: location.rootPath
                )
            } catch BridgeError.notFound {
                // Not managed yet, create it
                do {
                    _ = try FractaBridge.shared.createLocation(
                        label: location.label,
                        path: location.rootPath
                    )
                } catch {
                    await MainActor.run {
                        showError(.locationOpenFailed(error.localizedDescription))
                    }
                    return
                }
            } catch {
                await MainActor.run {
                    showError(.locationOpenFailed(error.localizedDescription))
                }
                return
            }

            await MainActor.run {
                activeLocation = location
                currentPath = location.rootPath
                selectedFile = nil
                folderPageContent = nil
                navigationPath = []
                // Load folder page for root if exists
                loadFolderPage(for: location.rootPath)
                // Start watching for filesystem changes
                startWatching()
            }
        }
    }

    // MARK: - Filesystem Watcher

    /// Start watching the active location for filesystem changes.
    /// Polls every 2 seconds and triggers incremental index updates.
    func startWatching() {
        guard let location = activeLocation else { return }
        stopWatching()

        do {
            try FractaBridge.shared.startWatching(locationPath: location.rootPath)
            isWatching = true

            // Poll every 2 seconds for filesystem events
            watcherTimer = Timer.scheduledTimer(withTimeInterval: 2.0, repeats: true) { [weak self] _ in
                Task { @MainActor [weak self] in
                    self?.pollWatcherEvents()
                }
            }
        } catch {
            // Watcher failure is non-fatal — just log and continue
            print("[Fracta] Watcher failed to start: \(error.localizedDescription)")
        }
    }

    /// Stop watching the active location.
    func stopWatching() {
        watcherTimer?.invalidate()
        watcherTimer = nil
        FractaBridge.shared.stopWatching()
        isWatching = false
    }

    /// Poll the watcher for events and trigger incremental index update.
    private func pollWatcherEvents() {
        let events = FractaBridge.shared.drainWatcherEvents()
        guard !events.isEmpty else { return }

        guard let location = activeLocation else { return }

        // Run incremental index update in background
        isIndexing = true
        Task {
            do {
                _ = try await FractaBridge.buildFullIndexAsync(
                    label: location.label,
                    locationPath: location.rootPath
                )
            } catch {
                print("[Fracta] Incremental index update failed: \(error.localizedDescription)")
            }
            await MainActor.run {
                self.isIndexing = false
            }
        }
    }

    /// Remove a location from managed locations
    func removeLocation(_ location: LocationState) {
        FractaBridge.shared.closeLocation(path: location.rootPath)
        locations.removeAll { $0.id == location.id }

        if activeLocation?.id == location.id {
            activeLocation = locations.first
            if let active = activeLocation {
                currentPath = active.rootPath
            }
        }
        selectedFile = nil
        folderPageContent = nil
    }

    // MARK: - Quick Access

    /// Add a quick access bookmark
    func addQuickAccess(path: String, label: String, icon: String = "folder.fill") {
        let item = QuickAccessItem(label: label, path: path, icon: icon)
        if !quickAccessItems.contains(where: { $0.path == path }) {
            quickAccessItems.append(item)
        }
    }

    /// Remove a quick access bookmark
    func removeQuickAccess(_ item: QuickAccessItem) {
        quickAccessItems.removeAll { $0.id == item.id }
    }

    // MARK: - Navigation

    /// Navigate to a path within the active location
    func navigateTo(path: String) {
        currentPath = path
        selectedFile = nil
        loadFolderPage(for: path)
    }

    /// Load folder page if exists (same-named .md file)
    private func loadFolderPage(for folderPath: String) {
        // Check for folder page: /path/to/Folder → /path/to/Folder.md
        let mdPath = folderPath + ".md"

        Task {
            do {
                if FileManager.default.fileExists(atPath: mdPath) {
                    let doc = try FractaBridge.shared.readDocumentAtPath(mdPath)
                    await MainActor.run {
                        folderPageContent = doc
                    }
                } else {
                    await MainActor.run {
                        folderPageContent = nil
                    }
                }
            } catch {
                await MainActor.run {
                    folderPageContent = nil
                }
            }
        }
    }

    // MARK: - Persistence

    private func saveLocations() {
        if let data = try? JSONEncoder().encode(locations) {
            UserDefaults.standard.set(data, forKey: locationsKey)
        }
    }

    private func loadLocations() {
        if let data = UserDefaults.standard.data(forKey: locationsKey),
           let saved = try? JSONDecoder().decode([LocationState].self, from: data) {
            locations = saved
            // Restore security-scoped access for each location with a bookmark
            restoreBookmarkAccess()
        }
    }

    /// Resolve security-scoped bookmarks and start accessing resources.
    ///
    /// Called on app launch to restore sandbox access to user-picked folders.
    /// If a bookmark is stale (folder moved/renamed), it's refreshed.
    /// If a bookmark can't be resolved at all, the location is kept but
    /// may fail when activated (user can re-add it).
    private func restoreBookmarkAccess() {
        var needsSave = false

        for i in locations.indices {
            guard let bookmarkData = locations[i].bookmarkData else { continue }

            var isStale = false
            do {
                let url = try URL(
                    resolvingBookmarkData: bookmarkData,
                    options: [.withSecurityScope],
                    relativeTo: nil,
                    bookmarkDataIsStale: &isStale
                )

                // Start accessing the security-scoped resource
                _ = url.startAccessingSecurityScopedResource()

                // If bookmark was stale, refresh it
                if isStale {
                    if let freshBookmark = try? url.bookmarkData(
                        options: [.withSecurityScope],
                        includingResourceValuesForKeys: nil,
                        relativeTo: nil
                    ) {
                        locations[i].bookmarkData = freshBookmark
                        needsSave = true
                    }
                }

                // Update path in case the folder was moved (bookmark tracks moves)
                let resolvedPath = url.path
                if resolvedPath != locations[i].rootPath {
                    locations[i] = LocationState(
                        id: locations[i].id,
                        label: url.lastPathComponent,
                        rootPath: resolvedPath,
                        isManaged: locations[i].isManaged,
                        bookmarkData: locations[i].bookmarkData
                    )
                    needsSave = true
                }
            } catch {
                print("[Fracta] Failed to resolve bookmark for '\(locations[i].label)': \(error.localizedDescription)")
            }
        }

        if needsSave {
            saveLocations()
        }
    }

    private func saveQuickAccess() {
        if let data = try? JSONEncoder().encode(quickAccessItems) {
            UserDefaults.standard.set(data, forKey: quickAccessKey)
        }
    }

    private func loadQuickAccess() {
        if let data = UserDefaults.standard.data(forKey: quickAccessKey),
           let saved = try? JSONDecoder().decode([QuickAccessItem].self, from: data) {
            quickAccessItems = saved
        }
    }

    // MARK: - Legacy Compatibility

    /// For backward compatibility with views using currentLocation
    var currentLocation: LocationState? {
        get { activeLocation }
        set { activeLocation = newValue }
    }

    /// Legacy: open location (now adds to locations list)
    func openLocation(at url: URL) {
        addLocation(at: url)
    }

    /// Legacy: close current location
    func closeLocation() {
        if let location = activeLocation {
            removeLocation(location)
        }
    }

    // MARK: - Error Handling

    /// Show an error to the user
    func showError(_ error: AppError) {
        currentError = error
        showingError = true
    }

    /// Dismiss current error
    func dismissError() {
        showingError = false
        currentError = nil
    }
}

/// View modes
enum ViewMode: String, CaseIterable {
    case browser = "Browser"
    case search = "Search"
    case document = "Document"
}

/// Navigation path item
enum PathItem: Hashable {
    case folder(path: String)
    case file(path: String)
}

/// App-level errors for user display
enum AppError: LocalizedError, Identifiable {
    case locationOpenFailed(String)
    case fileOperationFailed(String)
    case searchFailed(String)
    case indexBuildFailed(String)
    case generic(String)

    var id: String {
        switch self {
        case .locationOpenFailed(let msg): return "location:\(msg)"
        case .fileOperationFailed(let msg): return "file:\(msg)"
        case .searchFailed(let msg): return "search:\(msg)"
        case .indexBuildFailed(let msg): return "index:\(msg)"
        case .generic(let msg): return "generic:\(msg)"
        }
    }

    var errorDescription: String? {
        switch self {
        case .locationOpenFailed(let msg):
            return "Failed to open location: \(msg)"
        case .fileOperationFailed(let msg):
            return "File operation failed: \(msg)"
        case .searchFailed(let msg):
            return "Search failed: \(msg)"
        case .indexBuildFailed(let msg):
            return "Index build failed: \(msg)"
        case .generic(let msg):
            return msg
        }
    }

    var recoverySuggestion: String? {
        switch self {
        case .locationOpenFailed:
            return "Make sure the folder exists and you have permission to access it."
        case .fileOperationFailed:
            return "Check file permissions and try again."
        case .searchFailed:
            return "Try rebuilding the index from Settings."
        case .indexBuildFailed:
            return "Close and reopen the location, or check disk space."
        case .generic:
            return nil
        }
    }

    var title: String {
        switch self {
        case .locationOpenFailed: return "Location Error"
        case .fileOperationFailed: return "File Error"
        case .searchFailed: return "Search Error"
        case .indexBuildFailed: return "Index Error"
        case .generic: return "Error"
        }
    }
}
