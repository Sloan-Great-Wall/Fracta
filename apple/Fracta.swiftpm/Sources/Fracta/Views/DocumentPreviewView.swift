import SwiftUI

/// Document preview with Liquid Glass styling
///
/// Shows Markdown documents with:
/// - Title and metadata header
/// - Plain text content (pending full Markdown rendering)
/// - Tags and area information
struct DocumentPreviewView: View {
    let file: FileItem
    @State private var document: DocumentContent?
    @State private var isLoading = true

    #if DEBUG
    private var displayDocument: DocumentContent {
        document ?? FileItem.demoDocument
    }
    #else
    private var displayDocument: DocumentContent? { document }
    #endif

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: Spacing.lg) {
                // Header
                headerSection

                Divider()
                    .background(.white.opacity(0.1))

                // Content
                if let doc = displayDocument {
                    contentSection(doc)
                } else if isLoading {
                    loadingSection
                } else {
                    emptySection
                }
            }
            .padding(Spacing.xl)
        }
        .background(.ultraThinMaterial)
        .navigationTitle(file.name)
        #if os(iOS)
        .navigationBarTitleDisplayMode(.inline)
        #endif
        .toolbar {
            ToolbarItemGroup {
                Button {
                    // TODO: Edit action
                } label: {
                    Image(systemName: "pencil")
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
        }
        .task {
            await loadDocument()
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
                    if let doc = displayDocument, let title = doc.title {
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
            if let doc = displayDocument {
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

                if let doc = displayDocument {
                    Label("\(doc.blockCount) blocks", systemImage: "square.stack")
                        .font(.glassCaption)
                        .foregroundStyle(.secondary)
                }
            }
        }
    }

    private func contentSection(_ doc: DocumentContent) -> some View {
        VStack(alignment: .leading, spacing: Spacing.md) {
            Text("Content")
                .font(.glassHeadline)
                .foregroundStyle(.secondary)

            Text(doc.plainText)
                .font(.glassBody)
                .lineSpacing(6)
                .textSelection(.enabled)
                .glassCard(cornerRadius: 12, padding: Spacing.lg)
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
        // TODO: Use FFI to load real document
        // For now, simulate loading
        try? await Task.sleep(for: .milliseconds(300))
        isLoading = false

        #if DEBUG
        // Use demo document in debug
        document = FileItem.demoDocument
        #endif
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
        DocumentPreviewView(file: FileItem.demoFiles[2])
    }
}
