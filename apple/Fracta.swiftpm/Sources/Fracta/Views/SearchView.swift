import SwiftUI

/// Full-text search interface with Liquid Glass styling
struct SearchView: View {
    @EnvironmentObject var appState: AppState
    @State private var results: [SearchHit] = []
    @State private var isSearching = false
    @State private var focusedIndex: Int = 0
    @FocusState private var isResultsFocused: Bool

    var body: some View {
        VStack(spacing: 0) {
            // Search header
            searchHeader

            Divider()
                .background(.white.opacity(0.1))

            // Results
            if isSearching {
                loadingView
            } else if results.isEmpty && !appState.searchQuery.isEmpty {
                emptyResultsView
            } else if results.isEmpty {
                searchPromptView
            } else {
                resultsListView
            }
        }
        .background(.ultraThinMaterial)
        .onChange(of: appState.searchQuery) { _, newValue in
            performSearch(query: newValue)
        }
    }

    // MARK: - Sections

    private var searchHeader: some View {
        HStack(spacing: Spacing.md) {
            Image(systemName: "magnifyingglass")
                .font(.title2)
                .foregroundStyle(.secondary)

            VStack(alignment: .leading, spacing: Spacing.xs) {
                Text("Search")
                    .font(.glassHeadline)

                if !results.isEmpty {
                    Text("\(results.count) results for \"\(appState.searchQuery)\"")
                        .font(.glassCaption)
                        .foregroundStyle(.secondary)
                }
            }

            Spacer()

            if !appState.searchQuery.isEmpty {
                Button {
                    appState.searchQuery = ""
                    results = []
                } label: {
                    Image(systemName: "xmark.circle.fill")
                        .foregroundStyle(.secondary)
                }
                .buttonStyle(.plain)
            }
        }
        .padding()
    }

    private var loadingView: some View {
        VStack(spacing: Spacing.lg) {
            ProgressView()
                .scaleEffect(1.5)

            Text("Searching...")
                .font(.glassBody)
                .foregroundStyle(.secondary)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }

    private var emptyResultsView: some View {
        VStack(spacing: Spacing.lg) {
            Image(systemName: "magnifyingglass")
                .font(.system(size: 48))
                .foregroundStyle(.secondary)

            Text("No results found")
                .font(.glassHeadline)

            Text("Try different keywords or check your spelling")
                .font(.glassCaption)
                .foregroundStyle(.secondary)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }

    private var searchPromptView: some View {
        VStack(spacing: Spacing.lg) {
            Image(systemName: "text.magnifyingglass")
                .font(.system(size: 48))
                .foregroundStyle(.secondary)

            Text("Search your files")
                .font(.glassHeadline)

            Text("Full-text search across all Markdown documents")
                .font(.glassCaption)
                .foregroundStyle(.secondary)

            // Search tips
            VStack(alignment: .leading, spacing: Spacing.sm) {
                SearchTip(icon: "doc.text", text: "Search document content")
                SearchTip(icon: "tag", text: "Search by tags: tag:rust")
                SearchTip(icon: "folder", text: "Search by area: area:library")
            }
            .padding(.top, Spacing.lg)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }

    private var resultsListView: some View {
        ScrollViewReader { proxy in
            ScrollView {
                LazyVStack(spacing: Spacing.sm) {
                    ForEach(Array(results.enumerated()), id: \.element.id) { index, hit in
                        SearchResultRow(
                            hit: hit,
                            isFocused: focusedIndex == index && isResultsFocused
                        )
                        .id(index)
                        .onTapGesture {
                            selectResult(hit)
                        }
                    }
                }
                .padding()
            }
            .focused($isResultsFocused)
            .onMoveCommand { direction in
                switch direction {
                case .up:
                    focusedIndex = max(0, focusedIndex - 1)
                case .down:
                    focusedIndex = min(results.count - 1, focusedIndex + 1)
                default:
                    break
                }
                withAnimation {
                    proxy.scrollTo(focusedIndex, anchor: .center)
                }
            }
            .onKeyPress(.return) {
                if let hit = results[safe: focusedIndex] {
                    selectResult(hit)
                }
                return .handled
            }
        }
    }

    // MARK: - Actions

    private func performSearch(query: String) {
        guard !query.isEmpty else {
            results = []
            return
        }

        isSearching = true

        // Debounce search
        Task {
            try? await Task.sleep(for: .milliseconds(300))

            // Check if query is still the same
            guard appState.searchQuery == query else { return }

            // TODO: Use FFI bridge for real search
            #if DEBUG
            // Mock results
            await MainActor.run {
                results = [
                    SearchHit(id: "/notes.md", path: "/notes.md", title: "Notes containing '\(query)'", score: 0.95),
                    SearchHit(id: "/projects/rust.md", path: "/projects/rust.md", title: "Rust Project", score: 0.8),
                    SearchHit(id: "/library/books.md", path: "/library/books.md", title: "Reading List", score: 0.6)
                ]
                isSearching = false
                focusedIndex = 0
            }
            #else
            // Real search - safely handle missing location
            guard let location = appState.currentLocation else {
                await MainActor.run {
                    results = []
                    isSearching = false
                }
                return
            }

            do {
                let index = try FractaBridge.shared.openIndex(location: location)
                let hits = try index.search(query: query, limit: 50)
                await MainActor.run {
                    results = hits
                    isSearching = false
                    focusedIndex = 0
                }
            } catch {
                await MainActor.run {
                    results = []
                    isSearching = false
                }
            }
            #endif
        }
    }

    private func selectResult(_ hit: SearchHit) {
        // Convert hit to FileItem and select
        let file = FileItem(
            id: hit.path,
            path: hit.path,
            name: hit.path.components(separatedBy: "/").last ?? hit.path,
            kind: .file,
            size: 0,
            modified: nil,
            created: nil,
            scope: .managed,
            fileExtension: "md"
        )
        appState.selectedFile = file
        appState.isSearching = false
    }
}

/// Search result row
struct SearchResultRow: View {
    let hit: SearchHit
    var isFocused: Bool = false

    var body: some View {
        HStack(spacing: Spacing.md) {
            // Relevance indicator
            ZStack {
                Circle()
                    .fill(relevanceColor.opacity(0.2))

                Text(String(format: "%.0f", hit.score * 100))
                    .font(.caption2.bold())
                    .foregroundStyle(relevanceColor)
            }
            .frame(width: 36, height: 36)

            // Info
            VStack(alignment: .leading, spacing: Spacing.xs) {
                Text(hit.title ?? hit.path)
                    .font(.glassHeadline)
                    .lineLimit(1)

                Text(hit.path)
                    .font(.glassCaption)
                    .foregroundStyle(.secondary)
                    .lineLimit(1)
            }

            Spacer()

            Image(systemName: "chevron.right")
                .font(.caption)
                .foregroundStyle(.tertiary)
        }
        .frame(minHeight: Spacing.gamepadTarget)
        .glassCard(cornerRadius: 12, padding: Spacing.md, isFocused: isFocused)
    }

    private var relevanceColor: Color {
        if hit.score > 0.8 { return .green }
        if hit.score > 0.5 { return .yellow }
        return .orange
    }
}

/// Search tip row
struct SearchTip: View {
    let icon: String
    let text: String

    var body: some View {
        HStack(spacing: Spacing.sm) {
            Image(systemName: icon)
                .frame(width: 20)
                .foregroundStyle(.secondary)

            Text(text)
                .font(.glassCaption)
                .foregroundStyle(.secondary)
        }
    }
}

#Preview {
    SearchView()
        .environmentObject(AppState())
        .frame(width: 400, height: 600)
}
