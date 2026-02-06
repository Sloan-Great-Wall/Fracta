import SwiftUI

/// Shows all managed data sources (locations) with clear status
/// Users can see exactly what Fracta is accessing
struct DataSourcesView: View {
    @EnvironmentObject var appState: AppState
    @State private var showingAddLocation = false

    var body: some View {
        VStack(alignment: .leading, spacing: Spacing.lg) {
            // Header
            HStack {
                VStack(alignment: .leading, spacing: Spacing.xs) {
                    Text("Data Sources")
                        .font(.title2.bold())

                    Text("Fracta only accesses folders you explicitly add here")
                        .font(.subheadline)
                        .foregroundStyle(.secondary)
                }

                Spacer()

                Button {
                    showingAddLocation = true
                } label: {
                    Label("Add Location", systemImage: "plus")
                }
                .buttonStyle(.borderedProminent)
            }

            Divider()

            // Location list
            if let location = appState.currentLocation {
                ManagedLocationCard(location: location) {
                    appState.closeLocation()
                }
            } else {
                emptyState
            }

            Spacer()

            // Privacy footer
            privacyFooter
        }
        .padding(Spacing.xl)
        .fileImporter(
            isPresented: $showingAddLocation,
            allowedContentTypes: [.folder],
            allowsMultipleSelection: false
        ) { result in
            if case .success(let urls) = result, let url = urls.first {
                if url.startAccessingSecurityScopedResource() {
                    appState.openLocation(at: url)
                } else {
                    appState.openLocation(at: url)
                }
            }
        }
    }

    private var emptyState: some View {
        VStack(spacing: Spacing.lg) {
            Image(systemName: "folder.badge.plus")
                .font(.system(size: 48))
                .foregroundStyle(.secondary)

            Text("No Locations Added")
                .font(.headline)

            Text("Add a folder to start organizing your files with Fracta")
                .font(.subheadline)
                .foregroundStyle(.secondary)
                .multilineTextAlignment(.center)

            Button {
                showingAddLocation = true
            } label: {
                Label("Add Your First Location", systemImage: "plus.circle")
            }
            .buttonStyle(.borderedProminent)
        }
        .frame(maxWidth: .infinity)
        .padding(Spacing.xxl)
        .background(Color.secondary.opacity(0.05))
        .clipShape(RoundedRectangle(cornerRadius: 12))
    }

    private var privacyFooter: some View {
        HStack(spacing: Spacing.md) {
            Image(systemName: "lock.shield.fill")
                .font(.title2)
                .foregroundStyle(.green)

            VStack(alignment: .leading, spacing: 2) {
                Text("Your Privacy is Protected")
                    .font(.headline)

                Text("Fracta only reads files in the locations above. Your data never leaves your device.")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
        .padding()
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(Color.green.opacity(0.1))
        .clipShape(RoundedRectangle(cornerRadius: 12))
    }
}

/// Card showing a managed location with status and controls
struct ManagedLocationCard: View {
    let location: LocationState
    let onRemove: () -> Void

    @State private var isHovering = false
    @State private var fileCount: UInt32 = 0
    @State private var indexedCount: UInt32 = 0

    var body: some View {
        VStack(alignment: .leading, spacing: Spacing.md) {
            // Header
            HStack {
                Image(systemName: "folder.fill")
                    .font(.title)
                    .foregroundStyle(Color.accentColor)

                VStack(alignment: .leading, spacing: 2) {
                    Text(location.label)
                        .font(.headline)

                    Text(location.rootPath)
                        .font(.caption)
                        .foregroundStyle(.secondary)
                        .lineLimit(1)
                        .truncationMode(.middle)
                }

                Spacer()

                // Status badge
                HStack(spacing: 4) {
                    Circle()
                        .fill(location.isManaged ? Color.green : Color.orange)
                        .frame(width: 8, height: 8)

                    Text(location.isManaged ? "Managed" : "Unmanaged")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
                .padding(.horizontal, 8)
                .padding(.vertical, 4)
                .background(Color.secondary.opacity(0.1))
                .clipShape(Capsule())
            }

            Divider()

            // Stats
            HStack(spacing: Spacing.xl) {
                StatItem(icon: "doc.fill", label: "Files", value: "\(fileCount)")
                StatItem(icon: "text.magnifyingglass", label: "Indexed", value: "\(indexedCount)")
                StatItem(icon: "clock.fill", label: "Last Scan", value: "Just now")
            }

            Divider()

            // Actions
            HStack {
                Button {
                    rebuildIndex()
                } label: {
                    Label("Rebuild Index", systemImage: "arrow.clockwise")
                }
                .buttonStyle(.bordered)

                Button {
                    revealInFinder()
                } label: {
                    Label("Show in Finder", systemImage: "folder")
                }
                .buttonStyle(.bordered)

                Spacer()

                Button(role: .destructive) {
                    onRemove()
                } label: {
                    Label("Remove", systemImage: "trash")
                }
                .buttonStyle(.bordered)
            }
        }
        .padding(Spacing.lg)
        .background(isHovering ? Color.secondary.opacity(0.05) : Color.clear)
        .background(.ultraThinMaterial)
        .clipShape(RoundedRectangle(cornerRadius: 12))
        .overlay(
            RoundedRectangle(cornerRadius: 12)
                .stroke(Color.secondary.opacity(0.2), lineWidth: 1)
        )
        .onHover { hovering in
            isHovering = hovering
        }
        .onAppear {
            loadStats()
        }
    }

    private func loadStats() {
        Task {
            do {
                fileCount = try FractaBridge.shared.indexFileCount(locationPath: location.rootPath)
                // indexedCount would come from a separate FFI call
            } catch {
                // Ignore errors for stats
            }
        }
    }

    private func rebuildIndex() {
        Task {
            do {
                let stats = try FractaBridge.shared.buildFullIndex(locationPath: location.rootPath)
                fileCount = stats.filesScanned
                indexedCount = stats.markdownIndexed
            } catch {
                // Handle error
            }
        }
    }

    private func revealInFinder() {
        #if os(macOS)
        NSWorkspace.shared.selectFile(nil, inFileViewerRootedAtPath: location.rootPath)
        #endif
    }
}

struct StatItem: View {
    let icon: String
    let label: String
    let value: String

    var body: some View {
        HStack(spacing: Spacing.sm) {
            Image(systemName: icon)
                .foregroundStyle(.secondary)

            VStack(alignment: .leading, spacing: 0) {
                Text(value)
                    .font(.headline)
                Text(label)
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
    }
}

// MARK: - Preview

#Preview {
    DataSourcesView()
        .environmentObject(AppState())
        .frame(width: 600, height: 500)
}
