import SwiftUI

/// App settings view
struct SettingsView: View {
    @EnvironmentObject var appState: AppState

    var body: some View {
        TabView {
            DataSourcesView()
                .environmentObject(appState)
                .tabItem {
                    Label("Data Sources", systemImage: "folder.fill")
                }

            PrivacySettingsView()
                .tabItem {
                    Label("Privacy", systemImage: "lock.shield.fill")
                }

            AboutView()
                .tabItem {
                    Label("About", systemImage: "info.circle.fill")
                }
        }
        .frame(minWidth: 500, minHeight: 400)
    }
}

/// Privacy settings
struct PrivacySettingsView: View {
    @State private var hasFullDiskAccess = false

    var body: some View {
        VStack(alignment: .leading, spacing: Spacing.lg) {
            Text("Privacy & Permissions")
                .font(.title2.bold())

            GroupBox {
                VStack(alignment: .leading, spacing: Spacing.md) {
                    HStack {
                        Image(systemName: "internaldrive")
                            .font(.title2)
                            .foregroundStyle(hasFullDiskAccess ? .green : .orange)

                        VStack(alignment: .leading) {
                            Text("Full Disk Access")
                                .font(.headline)
                            Text("Allows Fracta to browse all folders on your Mac")
                                .font(.caption)
                                .foregroundStyle(.secondary)
                        }

                        Spacer()

                        if hasFullDiskAccess {
                            Label("Granted", systemImage: "checkmark.circle.fill")
                                .foregroundStyle(.green)
                        } else {
                            Button("Open Settings") {
                                openFullDiskAccessSettings()
                            }
                        }
                    }
                }
                .padding(.vertical, Spacing.sm)
            }

            GroupBox {
                VStack(alignment: .leading, spacing: Spacing.md) {
                    Label("Data Collection", systemImage: "chart.bar.fill")
                        .font(.headline)

                    Text("Fracta does not collect any data. Everything stays on your device.")
                        .font(.subheadline)
                        .foregroundStyle(.secondary)

                    Divider()

                    HStack {
                        Image(systemName: "xmark.circle.fill")
                            .foregroundStyle(.red)
                        Text("No analytics or tracking")
                            .font(.subheadline)
                    }

                    HStack {
                        Image(systemName: "xmark.circle.fill")
                            .foregroundStyle(.red)
                        Text("No data uploaded to servers")
                            .font(.subheadline)
                    }

                    HStack {
                        Image(systemName: "checkmark.circle.fill")
                            .foregroundStyle(.green)
                        Text("100% local-first")
                            .font(.subheadline)
                    }
                }
                .padding(.vertical, Spacing.sm)
            }

            Spacer()
        }
        .padding(Spacing.xl)
        .onAppear {
            checkFullDiskAccess()
        }
    }

    private func checkFullDiskAccess() {
        let testPath = NSHomeDirectory() + "/Library/Mail"
        hasFullDiskAccess = FileManager.default.isReadableFile(atPath: testPath)
    }

    private func openFullDiskAccessSettings() {
        #if os(macOS)
        if let url = URL(string: "x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles") {
            NSWorkspace.shared.open(url)
        }
        #endif
    }
}

/// About view with version info
struct AboutView: View {
    var body: some View {
        VStack(spacing: Spacing.xl) {
            Spacer()

            Image(systemName: "sparkles")
                .font(.system(size: 64))
                .foregroundStyle(Color.accentColor)

            Text("Fracta")
                .font(.largeTitle.bold())

            // Keywords
            HStack(spacing: Spacing.sm) {
                KeywordBadge(text: "Local-first", color: .blue)
                KeywordBadge(text: "Private", color: .green)
                KeywordBadge(text: "Open", color: .purple)
            }

            Text("Version 0.1.0")
                .font(.caption)
                .foregroundStyle(.tertiary)

            Divider()
                .frame(maxWidth: 200)

            VStack(spacing: Spacing.sm) {
                Text("Built with")
                    .font(.caption)
                    .foregroundStyle(.secondary)

                HStack(spacing: Spacing.lg) {
                    TechBadge(name: "Swift", color: .orange)
                    TechBadge(name: "Rust", color: .red)
                    TechBadge(name: "SQLite", color: .blue)
                }
            }

            Spacer()

            Text("© 2024 Fracta Project")
                .font(.caption)
                .foregroundStyle(.tertiary)
        }
        .padding(Spacing.xl)
    }
}

struct TechBadge: View {
    let name: String
    let color: Color

    var body: some View {
        Text(name)
            .font(.caption.bold())
            .padding(.horizontal, 8)
            .padding(.vertical, 4)
            .background(color.opacity(0.2))
            .foregroundStyle(color)
            .clipShape(Capsule())
    }
}

// KeywordBadge is defined in OnboardingView.swift

// MARK: - Preview

#Preview {
    SettingsView()
        .environmentObject(AppState())
}
