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
        }
        #endif
    }
}

/// Global application state
@MainActor
class AppState: ObservableObject {
    /// Currently selected location
    @Published var currentLocation: LocationState?

    /// Navigation path for drill-down
    @Published var navigationPath: [PathItem] = []

    /// Search query
    @Published var searchQuery: String = ""

    /// Is search active
    @Published var isSearching: Bool = false

    /// Current view mode
    @Published var viewMode: ViewMode = .browser

    /// Selected file for preview
    @Published var selectedFile: FileItem?

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

    // MARK: - Loading State

    /// Is performing a long operation
    @Published var isLoading: Bool = false

    /// Loading message
    @Published var loadingMessage: String = ""

    init() {
        // Show onboarding on first launch
        if !hasCompletedOnboarding {
            showingOnboarding = true
        }
    }

    // MARK: - Location Operations

    /// Open an existing managed location
    func openLocation(at url: URL) {
        isLoading = true
        loadingMessage = "Opening location..."

        Task {
            do {
                // Get the folder name as label
                let label = url.lastPathComponent

                // Try to open existing managed location first
                let locationState: LocationState
                do {
                    locationState = try FractaBridge.shared.openLocation(
                        label: label,
                        path: url.path
                    )
                } catch BridgeError.notFound {
                    // Not managed yet, create new location
                    locationState = try FractaBridge.shared.createLocation(
                        label: label,
                        path: url.path
                    )
                }

                await MainActor.run {
                    currentLocation = locationState
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

    /// Close current location
    func closeLocation() {
        if let location = currentLocation {
            FractaBridge.shared.closeLocation(path: location.rootPath)
        }
        currentLocation = nil
        selectedFile = nil
        navigationPath = []
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
