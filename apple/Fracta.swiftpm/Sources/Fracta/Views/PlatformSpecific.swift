import SwiftUI

// MARK: - Platform-Specific Views
//
// This file contains views that adapt to different Apple platforms:
// - macOS: Menu bar, window chrome
// - iOS/iPadOS: Touch-optimized, compact layouts
// - visionOS: Spatial UI, ornaments, volumes

// MARK: - visionOS Ornaments

#if os(visionOS)
import RealityKit

/// visionOS-specific ornament for quick actions
struct QuickActionsOrnament: View {
    @EnvironmentObject var appState: AppState

    var body: some View {
        HStack(spacing: 20) {
            Button {
                appState.isSearching = true
            } label: {
                Label("Search", systemImage: "magnifyingglass")
            }
            .buttonStyle(.bordered)

            Button {
                // Quick capture
            } label: {
                Label("Capture", systemImage: "plus.circle")
            }
            .buttonStyle(.bordered)

            Button {
                // Voice input
            } label: {
                Label("Voice", systemImage: "mic")
            }
            .buttonStyle(.bordered)
        }
        .padding()
        .glassBackgroundEffect()
    }
}

/// Spatial file card for visionOS
struct SpatialFileCard: View {
    let file: FileItem
    var depth: CGFloat = 20

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            FileIconView(file: file)
                .frame(width: 60, height: 60)

            Text(file.name)
                .font(.headline)
                .lineLimit(2)

            Text(file.formattedDate)
                .font(.caption)
                .foregroundStyle(.secondary)
        }
        .padding(20)
        .frame(width: 200, height: 200)
        .background(.regularMaterial)
        .clipShape(RoundedRectangle(cornerRadius: 20))
        .hoverEffect(.lift)
    }
}
#endif

// MARK: - macOS Specific

#if os(macOS)
/// macOS toolbar with window controls
struct MacToolbar: ToolbarContent {
    @EnvironmentObject var appState: AppState

    var body: some ToolbarContent {
        ToolbarItem(placement: .navigation) {
            Button {
                // Toggle sidebar
            } label: {
                Image(systemName: "sidebar.left")
            }
        }

        ToolbarItemGroup(placement: .primaryAction) {
            Spacer()

            Button {
                appState.isSearching = true
            } label: {
                Image(systemName: "magnifyingglass")
            }
            .keyboardShortcut("f", modifiers: .command)

            Button {
                // New file
            } label: {
                Image(systemName: "doc.badge.plus")
            }
            .keyboardShortcut("n", modifiers: .command)
        }
    }
}
#endif

// MARK: - iOS/iPadOS Specific

#if os(iOS)
/// iOS-optimized file grid for tablets
struct iPadFileGrid: View {
    let files: [FileItem]
    @Binding var selectedFile: FileItem?

    private let columns = [
        GridItem(.adaptive(minimum: 160, maximum: 200), spacing: 16)
    ]

    var body: some View {
        LazyVGrid(columns: columns, spacing: 16) {
            ForEach(files) { file in
                FileGridCell(file: file, isSelected: selectedFile?.id == file.id)
                    .onTapGesture {
                        selectedFile = file
                    }
            }
        }
        .padding()
    }
}

/// Grid cell for iPad file view
struct FileGridCell: View {
    let file: FileItem
    var isSelected: Bool = false

    var body: some View {
        VStack(spacing: 12) {
            FileIconView(file: file)
                .frame(width: 64, height: 64)

            VStack(spacing: 4) {
                Text(file.name)
                    .font(.subheadline)
                    .lineLimit(2)
                    .multilineTextAlignment(.center)

                Text(file.formattedDate)
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            }
        }
        .frame(maxWidth: .infinity)
        .padding()
        .glassCard(isSelected: isSelected)
    }
}
#endif

// MARK: - Adaptive Layout

/// Chooses between list and grid based on platform and size class
struct AdaptiveFileView: View {
    let files: [FileItem]
    @Binding var selectedFile: FileItem?
    @Environment(\.horizontalSizeClass) var horizontalSizeClass

    var body: some View {
        #if os(iOS)
        if horizontalSizeClass == .regular {
            iPadFileGrid(files: files, selectedFile: $selectedFile)
        } else {
            FileListView(files: files, selectedFile: $selectedFile)
        }
        #elseif os(visionOS)
        // visionOS uses spatial layout
        SpatialFileGrid(files: files, selectedFile: $selectedFile)
        #else
        // macOS uses list
        FileListView(files: files, selectedFile: $selectedFile)
        #endif
    }
}

/// Standard file list view
struct FileListView: View {
    let files: [FileItem]
    @Binding var selectedFile: FileItem?
    @State private var focusedIndex: Int = 0
    @FocusState private var isFocused: Bool

    var body: some View {
        ScrollView {
            LazyVStack(spacing: 8) {
                ForEach(Array(files.enumerated()), id: \.element.id) { index, file in
                    FileRowView(
                        file: file,
                        isSelected: selectedFile?.id == file.id,
                        isFocused: focusedIndex == index && isFocused
                    )
                    .onTapGesture {
                        selectedFile = file
                        focusedIndex = index
                    }
                }
            }
            .padding()
        }
        .focused($isFocused)
        .onMoveCommand { direction in
            switch direction {
            case .up:
                focusedIndex = max(0, focusedIndex - 1)
            case .down:
                focusedIndex = min(files.count - 1, focusedIndex + 1)
            default:
                break
            }
        }
    }
}

#if os(visionOS)
/// Spatial grid for visionOS
struct SpatialFileGrid: View {
    let files: [FileItem]
    @Binding var selectedFile: FileItem?

    var body: some View {
        ScrollView {
            LazyVGrid(columns: [GridItem(.adaptive(minimum: 200))], spacing: 24) {
                ForEach(files) { file in
                    SpatialFileCard(file: file)
                        .onTapGesture {
                            selectedFile = file
                        }
                }
            }
            .padding(40)
        }
    }
}
#endif
