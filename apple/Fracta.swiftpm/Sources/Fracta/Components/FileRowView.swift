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

                    if file.scope == .ignored {
                        Label("Ignored", systemImage: "eye.slash")
                            .font(.glassCaption)
                            .foregroundStyle(.tertiary)
                    }
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
