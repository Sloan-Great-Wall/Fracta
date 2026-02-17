import SwiftUI

/// A single file/folder row with Liquid Glass styling
///
/// Designed for game controller navigation with:
/// - Large touch targets (min 60pt height)
/// - Clear focus indicators
/// - Visual hierarchy for scanning
struct FileRowView: View {
    let file: FileItem
    var isSelected: Bool = false
    var isFocused: Bool = false

    var body: some View {
        HStack(spacing: Spacing.md) {
            // Icon
            FileIconView(file: file)
                .frame(width: 44, height: 44)

            // Info
            VStack(alignment: .leading, spacing: Spacing.xs) {
                Text(file.name)
                    .font(.glassHeadline)
                    .lineLimit(1)

                HStack(spacing: Spacing.sm) {
                    if file.isFolder {
                        Text("Folder")
                            .font(.glassCaption)
                            .foregroundStyle(.secondary)
                    } else {
                        Text(file.formattedSize)
                            .font(.glassCaption)
                            .foregroundStyle(.secondary)
                    }

                    Text("•")
                        .font(.glassCaption)
                        .foregroundStyle(.tertiary)

                    Text(file.formattedDate)
                        .font(.glassCaption)
                        .foregroundStyle(.secondary)

                    ScopeBadge(scope: file.scope)
                }
            }

            Spacer()

            // Chevron for folders, or indicators for files
            if file.isFolder {
                Image(systemName: "chevron.right")
                    .font(.caption)
                    .foregroundStyle(.tertiary)
            } else if file.isMarkdown {
                Image(systemName: "richtext.doc")
                    .font(.caption)
                    .foregroundStyle(.purple.opacity(0.7))
            }
        }
        .frame(minHeight: Spacing.gamepadTarget)
        .glassCard(
            cornerRadius: 12,
            padding: Spacing.md,
            isSelected: isSelected,
            isFocused: isFocused
        )
        .contentShape(Rectangle())
    }
}

/// File type icon with appropriate color
struct FileIconView: View {
    let file: FileItem

    var iconColor: Color {
        if file.isFolder { return .folderColor }
        if file.isMarkdown { return .markdownColor }

        switch file.fileExtension {
        case "json", "yaml", "yml": return .orange
        case "swift", "rs", "py", "js", "ts": return .codeColor
        case "png", "jpg", "jpeg", "gif", "webp": return .imageColor
        default: return .secondary
        }
    }

    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 10)
                .fill(iconColor.opacity(0.15))

            Image(systemName: file.icon)
                .font(.title2)
                .foregroundStyle(iconColor)
        }
    }
}

/// Scope badge with tap-to-explain popover
///
/// Shows the file's scope state (Managed/Ignored/Plain) and provides
/// user-facing explanations when tapped, satisfying the 0.1 acceptance
/// criterion: "UI explains scope states".
struct ScopeBadge: View {
    let scope: FileScope
    @State private var showingExplanation = false

    var body: some View {
        // Only show badge for non-managed scopes (managed is the default/expected state)
        if scope != .managed {
            Button {
                showingExplanation = true
            } label: {
                Label(scope.label, systemImage: scope.icon)
                    .font(.caption2)
                    .foregroundStyle(scope.color)
            }
            .buttonStyle(.plain)
            .popover(isPresented: $showingExplanation) {
                scopeExplanation
            }
        }
    }

    private var scopeExplanation: some View {
        VStack(alignment: .leading, spacing: Spacing.md) {
            // Current scope
            Label(scope.label, systemImage: scope.icon)
                .font(.headline)
                .foregroundStyle(scope.color)

            Text(scope.explanation)
                .font(.body)
                .foregroundStyle(.primary)

            Divider()

            // All scopes overview
            Text("Scope Levels")
                .font(.caption)
                .foregroundStyle(.secondary)
                .textCase(.uppercase)

            VStack(alignment: .leading, spacing: Spacing.sm) {
                scopeRow(.managed)
                scopeRow(.ignored)
                scopeRow(.plain)
            }
        }
        .padding(Spacing.lg)
        .frame(width: 300)
    }

    private func scopeRow(_ s: FileScope) -> some View {
        HStack(spacing: Spacing.sm) {
            Image(systemName: s.icon)
                .foregroundStyle(s.color)
                .frame(width: 20)

            VStack(alignment: .leading, spacing: 2) {
                Text(s.label)
                    .font(.caption)
                    .fontWeight(s == scope ? .bold : .regular)
                Text(s.shortDescription)
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            }
        }
    }
}

// MARK: - FileScope UI Properties

extension FileScope {
    var label: String {
        switch self {
        case .managed: return "Managed"
        case .ignored: return "Ignored"
        case .plain: return "Plain"
        }
    }

    var icon: String {
        switch self {
        case .managed: return "checkmark.shield"
        case .ignored: return "eye.slash"
        case .plain: return "doc"
        }
    }

    var color: Color {
        switch self {
        case .managed: return .green
        case .ignored: return .secondary
        case .plain: return .orange
        }
    }

    var shortDescription: String {
        switch self {
        case .managed: return "Indexed, searchable, AI-accessible"
        case .ignored: return "Visible but not indexed or searched"
        case .plain: return "Location not managed by Fracta"
        }
    }

    var explanation: String {
        switch self {
        case .managed:
            return "This file is managed by Fracta. It's indexed for full-text search, included in AI analysis, and its metadata is tracked. Changes are detected automatically."
        case .ignored:
            return "This file is visible in the browser but excluded from indexing and search. Fracta won't analyze or track it. You can change ignore rules in the .fracta/config/ directory."
        case .plain:
            return "This location hasn't been initialized as a Fracta-managed folder. Files are browsable but won't be indexed, searched, or analyzed until you enable management."
        }
    }
}

// MARK: - Preview

#Preview {
    let folderItem = FileItem(
        id: "/Projects",
        path: "/Projects",
        name: "Projects",
        kind: .folder,
        size: 0,
        modified: Date(),
        created: Date(),
        scope: .managed,
        fileExtension: nil
    )

    let markdownItem = FileItem(
        id: "/notes.md",
        path: "/notes.md",
        name: "notes.md",
        kind: .file,
        size: 2048,
        modified: Date().addingTimeInterval(-3600),
        created: Date(),
        scope: .managed,
        fileExtension: "md"
    )

    let jsonItem = FileItem(
        id: "/config.json",
        path: "/config.json",
        name: "config.json",
        kind: .file,
        size: 512,
        modified: Date().addingTimeInterval(-86400),
        created: Date(),
        scope: .managed,
        fileExtension: "json"
    )

    VStack(spacing: 8) {
        FileRowView(
            file: folderItem,
            isSelected: false,
            isFocused: true
        )

        FileRowView(
            file: markdownItem,
            isSelected: true,
            isFocused: false
        )

        FileRowView(
            file: jsonItem,
            isSelected: false,
            isFocused: false
        )
    }
    .padding()
    .background(Color.black.opacity(0.8))
}
