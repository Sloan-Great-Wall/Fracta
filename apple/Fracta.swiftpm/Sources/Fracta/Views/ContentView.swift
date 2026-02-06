import SwiftUI
import UniformTypeIdentifiers

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
        ZStack {
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

            // Loading overlay
            if appState.isLoading {
                LoadingOverlay(message: appState.loadingMessage)
            }
        }
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
        // Folder picker
        .fileImporter(
            isPresented: $appState.showingFolderPicker,
            allowedContentTypes: [.folder],
            allowsMultipleSelection: false
        ) { result in
            switch result {
            case .success(let urls):
                if let url = urls.first {
                    // Start accessing security-scoped resource
                    if url.startAccessingSecurityScopedResource() {
                        appState.openLocation(at: url)
                        // Note: stopAccessingSecurityScopedResource() should be called
                        // when the app is done with the folder, but we keep it open
                    } else {
                        appState.openLocation(at: url)
                    }
                }
            case .failure(let error):
                appState.showError(.locationOpenFailed(error.localizedDescription))
            }
        }
        // Error alert
        .alert(
            appState.currentError?.title ?? "Error",
            isPresented: $appState.showingError,
            presenting: appState.currentError
        ) { _ in
            Button("OK") {
                appState.dismissError()
            }
        } message: { error in
            VStack {
                Text(error.errorDescription ?? "An unknown error occurred")
                if let suggestion = error.recoverySuggestion {
                    Text(suggestion)
                        .font(.caption)
                }
            }
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

/// Loading overlay with blur background
struct LoadingOverlay: View {
    let message: String

    var body: some View {
        ZStack {
            Color.black.opacity(0.3)
                .ignoresSafeArea()

            VStack(spacing: Spacing.lg) {
                ProgressView()
                    .scaleEffect(1.5)
                    .tint(.white)

                if !message.isEmpty {
                    Text(message)
                        .font(.glassHeadline)
                        .foregroundStyle(.white)
                }
            }
            .padding(Spacing.xl)
            .background(.ultraThinMaterial, in: RoundedRectangle(cornerRadius: 16))
        }
    }
}

/// Sidebar with locations and quick access
struct SidebarView: View {
    @EnvironmentObject var appState: AppState
    @State private var showingRemoveConfirm: LocationState? = nil
    @State private var showingQuickAccessPicker: Bool = false

    var body: some View {
        List {
            // Locations section (Data Sources)
            Section("Locations") {
                ForEach(appState.locations) { location in
                    HStack {
                        Button {
                            appState.activateLocation(location)
                        } label: {
                            HStack {
                                Image(systemName: "folder.fill")
                                    .foregroundStyle(appState.activeLocation?.id == location.id ? Color.accentColor : .secondary)
                                Text(location.label)
                                    .foregroundStyle(appState.activeLocation?.id == location.id ? .primary : .secondary)
                            }
                        }
                        .buttonStyle(.plain)

                        Spacer()

                        if appState.activeLocation?.id == location.id {
                            Image(systemName: "checkmark")
                                .foregroundStyle(Color.accentColor)
                                .font(.caption)
                        }

                        Button {
                            showingRemoveConfirm = location
                        } label: {
                            Image(systemName: "xmark.circle.fill")
                                .foregroundStyle(.secondary)
                        }
                        .buttonStyle(.plain)
                    }
                }

                Button {
                    appState.showingFolderPicker = true
                } label: {
                    Label("Add Location", systemImage: "plus.circle")
                }
            }

            // Quick Access section
            Section("Quick Access") {
                ForEach(appState.quickAccessItems) { item in
                    Button {
                        appState.navigateTo(path: item.path)
                    } label: {
                        Label(item.label, systemImage: item.icon)
                    }
                    .buttonStyle(.plain)
                    .contextMenu {
                        Button(role: .destructive) {
                            appState.removeQuickAccess(item)
                        } label: {
                            Label("Remove", systemImage: "trash")
                        }
                    }
                }

                Button {
                    showingQuickAccessPicker = true
                } label: {
                    Label("Add Quick Access", systemImage: "plus.circle")
                }
            }

            // Areas section (PARA-style) - only show if active location exists
            if appState.activeLocation != nil {
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
        }
        .listStyle(.sidebar)
        .navigationTitle("Fracta")
        #if os(iOS)
        .navigationBarTitleDisplayMode(.large)
        #endif
        .confirmationDialog(
            "Remove Location",
            isPresented: Binding(
                get: { showingRemoveConfirm != nil },
                set: { if !$0 { showingRemoveConfirm = nil } }
            ),
            presenting: showingRemoveConfirm
        ) { location in
            Button("Remove", role: .destructive) {
                appState.removeLocation(location)
            }
            Button("Cancel", role: .cancel) {}
        } message: { location in
            Text("Remove '\(location.label)' from Fracta? Your files won't be deleted.")
        }
        .fileImporter(
            isPresented: $showingQuickAccessPicker,
            allowedContentTypes: [.folder],
            allowsMultipleSelection: false
        ) { result in
            if case .success(let urls) = result, let url = urls.first {
                appState.addQuickAccess(
                    path: url.path,
                    label: url.lastPathComponent,
                    icon: "folder.fill"
                )
            }
        }
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

    /// Current directory name for title
    private var currentDirectoryName: String {
        if appState.currentPath.isEmpty {
            return appState.activeLocation?.label ?? "Files"
        }
        return URL(fileURLWithPath: appState.currentPath).lastPathComponent
    }

    /// Can navigate back?
    private var canGoBack: Bool {
        guard let location = appState.activeLocation else { return false }
        return appState.currentPath != location.rootPath && !appState.currentPath.isEmpty
    }

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
            } else if appState.activeLocation == nil {
                emptyBrowserView
            } else if files.isEmpty {
                emptyFolderView
            } else {
                fileListView
            }
        }
        .navigationTitle(currentDirectoryName)
        .toolbar {
            ToolbarItemGroup(placement: .navigation) {
                if canGoBack {
                    Button {
                        navigateBack()
                    } label: {
                        Image(systemName: "chevron.left")
                    }
                }
            }

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
        .onChange(of: appState.activeLocation?.rootPath) { _, _ in loadFiles() }
        .onChange(of: appState.currentPath) { _, _ in loadFiles() }
    }

    private var emptyBrowserView: some View {
        VStack(spacing: Spacing.lg) {
            Image(systemName: "folder.badge.plus")
                .font(.system(size: 64))
                .foregroundStyle(.secondary)

            Text("No Location Selected")
                .font(.glassHeadline)
                .foregroundStyle(.secondary)

            Text("Add a folder to start browsing")
                .font(.glassCaption)
                .foregroundStyle(.tertiary)

            Button {
                appState.showingFolderPicker = true
            } label: {
                Label("Add Location", systemImage: "folder.badge.plus")
            }
            .buttonStyle(.borderedProminent)
            .keyboardShortcut("o", modifiers: .command)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }

    private var emptyFolderView: some View {
        VStack(spacing: Spacing.lg) {
            Image(systemName: "folder")
                .font(.system(size: 64))
                .foregroundStyle(.secondary)

            Text("Empty Folder")
                .font(.glassHeadline)
                .foregroundStyle(.secondary)

            if canGoBack {
                Button {
                    navigateBack()
                } label: {
                    Label("Go Back", systemImage: "chevron.left")
                }
                .buttonStyle(.bordered)
            }
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
                        .onTapGesture(count: 2) {
                            activateFile(file)
                        }
                        .contextMenu {
                            if file.isFolder {
                                Button {
                                    appState.addQuickAccess(
                                        path: file.path,
                                        label: file.name,
                                        icon: "folder.fill"
                                    )
                                } label: {
                                    Label("Add to Quick Access", systemImage: "star")
                                }
                            }

                            Divider()

                            Button(role: .destructive) {
                                // TODO: Implement delete
                            } label: {
                                Label("Delete", systemImage: "trash")
                            }
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
            .onKeyPress(.escape) {
                if canGoBack {
                    navigateBack()
                }
                return .handled
            }
        }
    }

    private func loadFiles() {
        guard let location = appState.activeLocation else {
            files = []
            return
        }

        // Use currentPath if set, otherwise use rootPath
        let pathToLoad = appState.currentPath.isEmpty ? location.rootPath : appState.currentPath

        isLoading = true
        errorMessage = nil

        Task {
            do {
                let items = try FractaBridge.shared.listDirectory(
                    locationPath: location.rootPath,
                    directoryPath: pathToLoad
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

    private func navigateBack() {
        guard let location = appState.activeLocation else { return }
        let parentPath = URL(fileURLWithPath: appState.currentPath).deletingLastPathComponent().path

        // Don't go above the location root
        if parentPath.hasPrefix(location.rootPath) || parentPath == location.rootPath {
            appState.navigateTo(path: parentPath)
        } else {
            appState.navigateTo(path: location.rootPath)
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
            // Navigate into folder
            appState.navigateTo(path: file.path)
        } else {
            appState.selectedFile = file
            appState.viewMode = .document
        }
    }
}

/// Detail view showing document preview or folder page
struct DetailView: View {
    @EnvironmentObject var appState: AppState

    var body: some View {
        Group {
            if let file = appState.selectedFile {
                DocumentPreviewView(file: file)
            } else if let folderPage = appState.folderPageContent {
                // Show folder page content
                FolderPageView(content: folderPage, folderPath: appState.currentPath)
            } else {
                EmptyDetailView()
            }
        }
    }
}

/// View for displaying folder page content (same-named .md file)
struct FolderPageView: View {
    let content: DocumentContent
    let folderPath: String

    var folderName: String {
        URL(fileURLWithPath: folderPath).lastPathComponent
    }

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: Spacing.lg) {
                // Header
                HStack {
                    Image(systemName: "folder.fill")
                        .font(.title)
                        .foregroundStyle(Color.accentColor)

                    Text(content.title ?? folderName)
                        .font(.largeTitle.bold())
                }
                .padding(.bottom, Spacing.sm)

                // Tags if present
                if !content.tags.isEmpty {
                    HStack {
                        ForEach(content.tags, id: \.self) { tag in
                            Text(tag)
                                .font(.caption)
                                .padding(.horizontal, 8)
                                .padding(.vertical, 4)
                                .background(Color.accentColor.opacity(0.2))
                                .clipShape(Capsule())
                        }
                    }
                }

                Divider()

                // Content
                Text(content.plainText)
                    .font(.body)
                    .textSelection(.enabled)

                Spacer()
            }
            .padding(Spacing.xl)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
        .background(.ultraThinMaterial)
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
