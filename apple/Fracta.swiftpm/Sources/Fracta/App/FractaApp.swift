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
        }
        .windowStyle(.hiddenTitleBar)
        .defaultSize(width: 1200, height: 800)

        Settings {
            SettingsView()
                .environmentObject(appState)
        }
        #elseif os(visionOS)
        WindowGroup {
            ContentView()
                .environmentObject(appState)
        }
        .windowStyle(.volumetric)
        #else
        WindowGroup {
            ContentView()
                .environmentObject(appState)
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

    init() {
        // Initialize with demo location if no real location
        #if DEBUG
        self.currentLocation = LocationState.demo
        #endif
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
