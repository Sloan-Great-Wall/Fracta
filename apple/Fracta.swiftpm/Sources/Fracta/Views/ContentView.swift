import SwiftUI

/// Main content view with adaptive layout for all platforms
struct ContentView: View {
    @EnvironmentObject var appState: AppState
    @State private var columnVisibility: NavigationSplitViewVisibility = .all
    @FocusState private var focusedSection: FocusSection?

    enum FocusSection: Hashable {
        case sidebar
        case browser
        case detail
    }

    var body: some View {
        NavigationSplitView(columnVisibility: $columnVisibility) {
            SidebarView()
                .focused($focusedSection, equals: .sidebar)
                .glassSidebar()
        } content: {
            BrowserView()
                .focused($focusedSection, equals: .browser)
        } detail: {
            DetailView()
                .focused($focusedSection, equals: .detail)
        }
        .navigationSplitViewStyle(.balanced)
        #if os(macOS)
        .frame(minWidth: 800, minHeight: 500)
        #endif
        .onAppear {
            // Set initial focus to browser
            focusedSection = .browser
        }
        .onMoveCommand { direction in
            handleMoveCommand(direction)
        }
    }

    /// Handle D-pad/arrow navigation between sections
    private func handleMoveCommand(_ direction: MoveCommandDirection) {
        switch direction {
        case .left:
            switch focusedSection {
            case .detail: focusedSection = .browser
            case .browser: focusedSection = .sidebar
            default: break
            }
        case .right:
            switch focusedSection {
            case .sidebar: focusedSection = .browser
            case .browser:
                if appState.selectedFile != nil {
                    focusedSection = .detail
                }
            default: break
            }
        default:
            break  // Up/down handled within each section
        }
    }
}

/// Sidebar with locations and quick access
struct SidebarView: View {
    @EnvironmentObject var appState: AppState

    var body: some View {
        List {
            // Locations section
            Section("Locations") {
                if let location = appState.currentLocation {
                    Label(location.label, systemImage: "folder.fill")
                        .tag(location.id)
                } else {
                    Button {
                        // TODO: Add location picker
                    } label: {
                        Label("Add Location", systemImage: "plus.circle")
                    }
                }
            }

            // Quick Access section
            Section("Quick Access") {
                NavigationLink(value: PathItem.folder(path: "inbox")) {
                    Label("Inbox", systemImage: "tray.fill")
                }

                NavigationLink(value: PathItem.folder(path: "recent")) {
                    Label("Recent", systemImage: "clock.fill")
                }

                NavigationLink(value: PathItem.folder(path: "favorites")) {
                    Label("Favorites", systemImage: "star.fill")
                }
            }

            // Areas section (PARA-style)
            Section("Areas") {
                NavigationLink(value: PathItem.folder(path: "library")) {
                    Label("Library", systemImage: "books.vertical.fill")
                }
                .tint(.blue)

                NavigationLink(value: PathItem.folder(path: "now")) {
                    Label("Now", systemImage: "flame.fill")
                }
                .tint(.orange)

                NavigationLink(value: PathItem.folder(path: "past")) {
                    Label("Past", systemImage: "clock.arrow.circlepath")
                }
                .tint(.purple)
            }
        }
        .listStyle(.sidebar)
        .navigationTitle("Fracta")
        #if os(iOS)
        .navigationBarTitleDisplayMode(.large)
        #endif
    }
}

/// Main file browser
struct BrowserView: View {
    @EnvironmentObject var appState: AppState
    @State private var files: [FileItem] = []
    @State private var focusedIndex: Int = 0
    @State private var isLoading: Bool = false
    @State private var errorMessage: String?
    @FocusState private var isFocused: Bool

    var body: some View {
        Group {
            if isLoading {
                ProgressView("Loading...")
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
            } else if let error = errorMessage {
                VStack(spacing: Spacing.lg) {
                    Image(systemName: "exclamationmark.triangle")
                        .font(.system(size: 48))
                        .foregroundStyle(.secondary)
                    Text(error)
                        .font(.glassBody)
                        .foregroundStyle(.secondary)
                    Button("Retry") { loadFiles() }
                }
                .frame(maxWidth: .infinity, maxHeight: .infinity)
            } else if files.isEmpty {
                emptyBrowserView
            } else {
                fileListView
            }
        }
        .navigationTitle(appState.currentLocation?.label ?? "Files")
        .toolbar {
            ToolbarItemGroup {
                Button {
                    appState.isSearching.toggle()
                } label: {
                    Image(systemName: "magnifyingglass")
                }
                .keyboardShortcut("f", modifiers: .command)

                Menu {
                    Button("Name") { }
                    Button("Date Modified") { }
                    Button("Size") { }
                } label: {
                    Image(systemName: "arrow.up.arrow.down")
                }
            }
        }
        .searchable(
            text: $appState.searchQuery,
            isPresented: $appState.isSearching,
            prompt: "Search files..."
        )
        .onAppear { loadFiles() }
        .onChange(of: appState.currentLocation?.rootPath) { _, _ in loadFiles() }
    }

    private var emptyBrowserView: some View {
        VStack(spacing: Spacing.lg) {
            Image(systemName: "folder.badge.plus")
                .font(.system(size: 64))
                .foregroundStyle(.secondary)

            Text("No Location Selected")
                .font(.glassHeadline)
                .foregroundStyle(.secondary)

            Text("Open a folder to start browsing")
                .font(.glassCaption)
                .foregroundStyle(.tertiary)

            Button {
                // TODO: Show folder picker
            } label: {
                Label("Open Location", systemImage: "folder")
            }
            .buttonStyle(.borderedProminent)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }

    private var fileListView: some View {
        ScrollViewReader { proxy in
            ScrollView {
                LazyVStack(spacing: Spacing.sm) {
                    ForEach(Array(files.enumerated()), id: \.element.id) { index, file in
                        FileRowView(
                            file: file,
                            isSelected: appState.selectedFile?.id == file.id,
                            isFocused: focusedIndex == index && isFocused
                        )
                        .id(index)
                        .onTapGesture {
                            selectFile(file)
                        }
                        .focusable()
                    }
                }
                .padding()
            }
            .focused($isFocused)
            .onMoveCommand { direction in
                handleMove(direction, proxy: proxy)
            }
            .onKeyPress(.return) {
                if let file = files[safe: focusedIndex] {
                    activateFile(file)
                }
                return .handled
            }
        }
    }

    private func loadFiles() {
        guard let location = appState.currentLocation else {
            files = []
            return
        }

        isLoading = true
        errorMessage = nil

        Task {
            do {
                let items = try FractaBridge.shared.listDirectory(
                    locationPath: location.rootPath,
                    directoryPath: location.rootPath
                )
                await MainActor.run {
                    files = items
                    isLoading = false
                }
            } catch {
                await MainActor.run {
                    errorMessage = error.localizedDescription
                    isLoading = false
                }
            }
        }
    }

    private func handleMove(_ direction: MoveCommandDirection, proxy: ScrollViewProxy) {
        switch direction {
        case .up:
            focusedIndex = max(0, focusedIndex - 1)
        case .down:
            focusedIndex = min(files.count - 1, focusedIndex + 1)
        default:
            return
        }
        withAnimation {
            proxy.scrollTo(focusedIndex, anchor: .center)
        }
    }

    private func selectFile(_ file: FileItem) {
        appState.selectedFile = file
        if let index = files.firstIndex(where: { $0.id == file.id }) {
            focusedIndex = index
        }
    }

    private func activateFile(_ file: FileItem) {
        if file.isFolder {
            appState.navigationPath.append(.folder(path: file.path))
        } else {
            appState.selectedFile = file
            appState.viewMode = .document
        }
    }
}

/// Detail view showing document preview
struct DetailView: View {
    @EnvironmentObject var appState: AppState

    var body: some View {
        Group {
            if let file = appState.selectedFile {
                DocumentPreviewView(file: file)
            } else {
                EmptyDetailView()
            }
        }
    }
}

/// Empty state for detail view
struct EmptyDetailView: View {
    var body: some View {
        VStack(spacing: Spacing.lg) {
            Image(systemName: "doc.text.magnifyingglass")
                .font(.system(size: 64))
                .foregroundStyle(.secondary)

            Text("Select a file to preview")
                .font(.glassHeadline)
                .foregroundStyle(.secondary)

            Text("Use arrow keys or D-pad to navigate")
                .font(.glassCaption)
                .foregroundStyle(.tertiary)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(.ultraThinMaterial)
    }
}

/// Settings view (macOS)
struct SettingsView: View {
    var body: some View {
        TabView {
            GeneralSettingsView()
                .tabItem {
                    Label("General", systemImage: "gear")
                }

            AppearanceSettingsView()
                .tabItem {
                    Label("Appearance", systemImage: "paintbrush")
                }
        }
        .frame(width: 500, height: 300)
    }
}

struct GeneralSettingsView: View {
    var body: some View {
        Form {
            Text("General settings coming soon...")
        }
        .padding()
    }
}

struct AppearanceSettingsView: View {
    var body: some View {
        Form {
            Text("Appearance settings coming soon...")
        }
        .padding()
    }
}

// MARK: - Array Extension

extension Array {
    subscript(safe index: Int) -> Element? {
        indices.contains(index) ? self[index] : nil
    }
}

#Preview {
    @Previewable @StateObject var appState = AppState()
    ContentView()
        .environmentObject(appState)
}
