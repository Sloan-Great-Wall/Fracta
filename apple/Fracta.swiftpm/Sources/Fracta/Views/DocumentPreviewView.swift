import SwiftUI

/// Document preview with Liquid Glass styling
///
/// Shows Markdown documents with:
/// - Title and metadata header
/// - Plain text content (pending full Markdown rendering)
/// - Tags and area information
struct DocumentPreviewView: View {
    let file: FileItem
    @EnvironmentObject var appState: AppState
    @State private var document: DocumentContent?
    @State private var isLoading = true
    @State private var errorMessage: String?

    // Edit mode state
    @State private var isEditing = false
    @State private var editContent = ""
    @State private var hasUnsavedChanges = false
    @State private var isSaving = false
    @State private var isLoadingFullContent = false
    @State private var showDiscardAlert = false

    var body: some View {
        Group {
            if isEditing {
                // Edit mode: full-screen editable text view
                NativeTextView(editContent, isEditable: true) { newText in
                    editContent = newText
                    hasUnsavedChanges = true
                }
                .background(.ultraThinMaterial)
            } else {
                // Preview mode: scrollable document preview
                ScrollView {
                    VStack(alignment: .leading, spacing: Spacing.lg) {
                        // Header
                        headerSection

                        Divider()
                            .background(.white.opacity(0.1))

                        // Content
                        if let doc = document {
                            contentSection(doc)
                        } else if isLoading {
                            loadingSection
                        } else if let error = errorMessage {
                            errorSection(error)
                        } else {
                            emptySection
                        }
                    }
                    .padding(Spacing.xl)
                }
                .background(.ultraThinMaterial)
            }
        }
        .navigationTitle(isEditing ? "Editing \(file.name)" : file.name)
        #if os(iOS)
        .navigationBarTitleDisplayMode(.inline)
        #endif
        .toolbar {
            ToolbarItemGroup {
                if isEditing {
                    editToolbar
                } else {
                    previewToolbar
                }
            }
        }
        .task(id: file.id) {
            // Reset state and load new document when file changes
            document = nil
            errorMessage = nil
            isLoading = true
            isEditing = false
            hasUnsavedChanges = false
            editContent = ""
            await loadDocument()
        }
        .alert("Unsaved Changes", isPresented: $showDiscardAlert) {
            Button("Discard", role: .destructive) {
                discardChanges()
            }
            Button("Cancel", role: .cancel) { }
        } message: {
            Text("You have unsaved changes. Do you want to discard them?")
        }
    }

    // MARK: - Toolbars

    @ViewBuilder
    private var previewToolbar: some View {
        if file.isMarkdown {
            Button {
                enterEditMode()
            } label: {
                Image(systemName: "pencil")
            }
        }

        Button {
            // TODO: Share action
        } label: {
            Image(systemName: "square.and.arrow.up")
        }

        Menu {
            Button("Copy Path") { }
            Button("Reveal in Finder") { }
            Divider()
            Button("Delete", role: .destructive) { }
        } label: {
            Image(systemName: "ellipsis.circle")
        }
    }

    @ViewBuilder
    private var editToolbar: some View {
        if isSaving {
            ProgressView()
                .controlSize(.small)
        }

        Button {
            saveDocument()
        } label: {
            Image(systemName: "square.and.arrow.down")
        }
        .disabled(!hasUnsavedChanges || isSaving)
        .keyboardShortcut("s", modifiers: .command)

        Button {
            cancelEditing()
        } label: {
            Text("Done")
        }
    }

    // MARK: - Sections

    private var headerSection: some View {
        VStack(alignment: .leading, spacing: Spacing.md) {
            // File icon and title
            HStack(spacing: Spacing.md) {
                FileIconView(file: file)
                    .frame(width: 56, height: 56)

                VStack(alignment: .leading, spacing: Spacing.xs) {
                    if let doc = document, let title = doc.title {
                        Text(title)
                            .font(.glassTitle)
                    } else {
                        Text(file.name)
                            .font(.glassTitle)
                    }

                    Text(file.path)
                        .font(.glassCaption)
                        .foregroundStyle(.secondary)
                        .lineLimit(1)
                }
            }

            // Metadata badges
            if let doc = document {
                HStack(spacing: Spacing.sm) {
                    if let area = doc.area {
                        MetadataBadge(label: area, icon: "folder", color: .blue)
                    }

                    ForEach(doc.tags.prefix(5), id: \.self) { tag in
                        MetadataBadge(label: tag, icon: "tag", color: .purple)
                    }

                    if doc.tags.count > 5 {
                        MetadataBadge(label: "+\(doc.tags.count - 5)", icon: nil, color: .secondary)
                    }
                }
            }

            // File info
            HStack(spacing: Spacing.lg) {
                Label(file.formattedSize, systemImage: "doc")
                    .font(.glassCaption)
                    .foregroundStyle(.secondary)

                if let modified = file.modified {
                    Label(modified.formatted(date: .abbreviated, time: .shortened), systemImage: "clock")
                        .font(.glassCaption)
                        .foregroundStyle(.secondary)
                }

                if let doc = document {
                    Label("\(doc.blockCount) blocks", systemImage: "square.stack")
                        .font(.glassCaption)
                        .foregroundStyle(.secondary)
                }
            }
        }
    }

    private func contentSection(_ doc: DocumentContent) -> some View {
        VStack(alignment: .leading, spacing: Spacing.md) {
            HStack {
                Text("Content")
                    .font(.glassHeadline)
                    .foregroundStyle(.secondary)

                Spacer()

                Text("\(doc.fullCharacterCount.formatted()) bytes")
                    .font(.caption)
                    .foregroundStyle(.tertiary)
            }

            // Use native text view for efficient rendering of any size
            NativeTextView(doc.plainText)
                .frame(minHeight: 200, maxHeight: 600)
                .glassCard(cornerRadius: 12, padding: Spacing.sm)

            if doc.isTruncated {
                HStack {
                    Image(systemName: "info.circle")
                        .foregroundStyle(.secondary)
                    Text("Large file - showing preview. Open in editor for full content.")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
                .padding(.top, Spacing.sm)
            }
        }
    }

    private var loadingSection: some View {
        VStack(spacing: Spacing.md) {
            ProgressView()
            Text("Loading document...")
                .font(.glassCaption)
                .foregroundStyle(.secondary)
        }
        .frame(maxWidth: .infinity, minHeight: 200)
    }

    private func errorSection(_ error: String) -> some View {
        VStack(spacing: Spacing.md) {
            Image(systemName: "exclamationmark.triangle")
                .font(.largeTitle)
                .foregroundStyle(.orange)

            Text("Error loading document")
                .font(.glassHeadline)
                .foregroundStyle(.secondary)

            Text(error)
                .font(.glassCaption)
                .foregroundStyle(.tertiary)
                .multilineTextAlignment(.center)
        }
        .frame(maxWidth: .infinity, minHeight: 200)
    }

    private var emptySection: some View {
        VStack(spacing: Spacing.md) {
            Image(systemName: "doc.questionmark")
                .font(.largeTitle)
                .foregroundStyle(.secondary)

            Text("Unable to load document")
                .font(.glassHeadline)
                .foregroundStyle(.secondary)
        }
        .frame(maxWidth: .infinity, minHeight: 200)
    }

    // MARK: - Loading

    private func loadDocument() async {
        guard file.isMarkdown else {
            // Non-markdown files: show basic info
            isLoading = false
            return
        }

        // Read and parse on background thread
        let result = await Task.detached(priority: .userInitiated) { [path = file.path] in
            do {
                // Read file
                let content = try String(contentsOfFile: path, encoding: .utf8)

                // Fast parse - only frontmatter + first heading
                let parsed = Self.parseFast(content)
                return Result<DocumentContent, Error>.success(parsed)
            } catch {
                return Result<DocumentContent, Error>.failure(error)
            }
        }.value

        await MainActor.run {
            switch result {
            case .success(let doc):
                document = doc
            case .failure(let error):
                errorMessage = error.localizedDescription
            }
            isLoading = false
        }
    }

    // MARK: - Edit Mode

    private func enterEditMode() {
        isLoadingFullContent = true

        Task {
            let result = await Task.detached(priority: .userInitiated) { [path = file.path] in
                do {
                    let content = try String(contentsOfFile: path, encoding: .utf8)
                    return Result<String, Error>.success(content)
                } catch {
                    return Result<String, Error>.failure(error)
                }
            }.value

            await MainActor.run {
                switch result {
                case .success(let content):
                    editContent = content
                    isEditing = true
                    hasUnsavedChanges = false
                case .failure(let error):
                    errorMessage = "Failed to load file for editing: \(error.localizedDescription)"
                }
                isLoadingFullContent = false
            }
        }
    }

    private func saveDocument() {
        guard let location = appState.activeLocation else { return }

        isSaving = true
        let path = file.path
        let content = editContent
        let locationPath = location.rootPath

        Task {
            do {
                try FractaBridge.shared.writeFile(
                    locationPath: locationPath,
                    filePath: path,
                    content: content
                )
                hasUnsavedChanges = false
                isEditing = false
                // Reload the preview to reflect saved changes
                document = nil
                isLoading = true
                await loadDocument()
            } catch {
                errorMessage = "Save failed: \(error.localizedDescription)"
            }
            isSaving = false
        }
    }

    private func cancelEditing() {
        if hasUnsavedChanges {
            showDiscardAlert = true
        } else {
            isEditing = false
        }
    }

    private func discardChanges() {
        hasUnsavedChanges = false
        editContent = ""
        isEditing = false
    }

    /// Ultra-fast Markdown parsing - only extract metadata, don't process full content
    private nonisolated static func parseFast(_ content: String) -> DocumentContent {
        var title: String? = nil
        var tags: [String] = []
        var area: String? = nil
        var hasFrontMatter = false
        var contentStartIndex = content.startIndex

        // Only check first 2KB for frontmatter (frontmatter is always at the start)
        let headerLimit = min(2048, content.count)
        let headerRange = content.startIndex..<content.index(content.startIndex, offsetBy: headerLimit)
        let header = String(content[headerRange])

        // Check for YAML frontmatter
        if header.hasPrefix("---\n") || header.hasPrefix("---\r\n") {
            // Find closing ---
            if let endRange = header.range(of: "\n---\n") ?? header.range(of: "\r\n---\r\n") {
                hasFrontMatter = true
                let frontMatter = String(header[header.index(header.startIndex, offsetBy: 4)..<endRange.lowerBound])

                // Parse frontmatter line by line
                for line in frontMatter.split(separator: "\n", omittingEmptySubsequences: false) {
                    let trimmed = line.trimmingCharacters(in: .whitespaces)
                    if trimmed.hasPrefix("title:") {
                        title = String(trimmed.dropFirst(6)).trimmingCharacters(in: .whitespaces).trimmingCharacters(in: CharacterSet(charactersIn: "\"'"))
                    } else if trimmed.hasPrefix("area:") {
                        area = String(trimmed.dropFirst(5)).trimmingCharacters(in: .whitespaces).trimmingCharacters(in: CharacterSet(charactersIn: "\"'"))
                    } else if trimmed.hasPrefix("tags:") {
                        let tagsPart = String(trimmed.dropFirst(5)).trimmingCharacters(in: .whitespaces)
                        if tagsPart.hasPrefix("[") && tagsPart.hasSuffix("]") {
                            let inner = tagsPart.dropFirst().dropLast()
                            tags = inner.split(separator: ",").map {
                                String($0).trimmingCharacters(in: .whitespaces).trimmingCharacters(in: CharacterSet(charactersIn: "\"'"))
                            }
                        }
                    }
                }

                // Find content start after frontmatter
                if let fullEndRange = content.range(of: "\n---\n") ?? content.range(of: "\r\n---\r\n") {
                    contentStartIndex = fullEndRange.upperBound
                }
            }
        }

        // Extract title from first heading if not in frontmatter (only check first 1KB of content)
        if title == nil {
            let contentStart = String(content[contentStartIndex...].prefix(1024))
            for line in contentStart.split(separator: "\n", omittingEmptySubsequences: false) {
                let trimmed = line.trimmingCharacters(in: .whitespaces)
                if trimmed.hasPrefix("# ") {
                    title = String(trimmed.dropFirst(2))
                    break
                }
                // Stop if we hit a non-empty, non-heading line
                if !trimmed.isEmpty && !trimmed.hasPrefix("#") {
                    break
                }
            }
        }

        // Get content after frontmatter
        let contentSubstring = content[contentStartIndex...]

        // Calculate full size using utf8 count (O(1) for substring, faster than .count)
        let fullSize = contentSubstring.utf8.count

        // Only store preview (first 8000 chars) to avoid slow SwiftUI rendering
        let previewLimit = 8000
        let preview: String
        if fullSize > previewLimit {
            // Take prefix efficiently
            preview = String(contentSubstring.prefix(previewLimit))
        } else {
            preview = String(contentSubstring)
        }

        return DocumentContent(
            title: title,
            plainText: preview,
            fullCharacterCount: fullSize,
            hasFrontMatter: hasFrontMatter,
            tags: tags,
            area: area,
            blockCount: 0
        )
    }
}

/// Metadata badge component
struct MetadataBadge: View {
    let label: String
    let icon: String?
    let color: Color

    var body: some View {
        HStack(spacing: 4) {
            if let icon {
                Image(systemName: icon)
                    .font(.caption2)
            }
            Text(label)
                .font(.caption)
        }
        .padding(.horizontal, 10)
        .padding(.vertical, 5)
        .background(color.opacity(0.15))
        .foregroundStyle(color)
        .clipShape(Capsule())
    }
}

// MARK: - Preview

#Preview {
    NavigationStack {
        // Create a sample file for preview
        let sampleFile = FileItem(
            id: "/demo/notes.md",
            path: "/demo/notes.md",
            name: "notes.md",
            kind: .file,
            size: 1024,
            modified: Date(),
            created: Date(),
            scope: .managed,
            fileExtension: "md"
        )
        DocumentPreviewView(file: sampleFile)
            .environmentObject(AppState())
    }
}
