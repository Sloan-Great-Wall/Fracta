import SwiftUI

/// First launch onboarding experience
/// Explains what Fracta does and requests necessary permissions
struct OnboardingView: View {
    @EnvironmentObject var appState: AppState
    @State private var currentStep: OnboardingStep = .welcome
    @State private var hasFullDiskAccess: Bool = false

    enum OnboardingStep: Int, CaseIterable {
        case welcome
        case permissions
        case dataSources
        case ready
    }

    var body: some View {
        VStack(spacing: 0) {
            // Progress indicator
            progressIndicator
                .padding(.top, Spacing.xl)

            // Content
            TabView(selection: $currentStep) {
                welcomeStep
                    .tag(OnboardingStep.welcome)

                permissionsStep
                    .tag(OnboardingStep.permissions)

                dataSourcesStep
                    .tag(OnboardingStep.dataSources)

                readyStep
                    .tag(OnboardingStep.ready)
            }
            #if os(macOS)
            .tabViewStyle(.automatic)
            #else
            .tabViewStyle(.page(indexDisplayMode: .never))
            #endif

            // Navigation buttons
            navigationButtons
                .padding(Spacing.xl)
        }
        .frame(minWidth: 600, minHeight: 500)
        .background(.ultraThinMaterial)
        .onAppear {
            checkFullDiskAccess()
        }
    }

    // MARK: - Progress Indicator

    private var progressIndicator: some View {
        HStack(spacing: Spacing.md) {
            ForEach(OnboardingStep.allCases, id: \.rawValue) { step in
                Circle()
                    .fill(step.rawValue <= currentStep.rawValue ? Color.accentColor : Color.secondary.opacity(0.3))
                    .frame(width: 8, height: 8)
            }
        }
    }

    // MARK: - Welcome Step

    private var welcomeStep: some View {
        VStack(spacing: Spacing.xl) {
            Spacer()

            Image(systemName: "sparkles")
                .font(.system(size: 64))
                .foregroundStyle(Color.accentColor)

            Text("Welcome to Fracta")
                .font(.largeTitle.bold())

            Text("Your local-first life operating system")
                .font(.title3)
                .foregroundStyle(.secondary)

            VStack(alignment: .leading, spacing: Spacing.md) {
                FeatureRow(icon: "folder.fill", title: "Organize", description: "Browse and manage your files with smart indexing")
                FeatureRow(icon: "magnifyingglass", title: "Search", description: "Full-text search across all your documents")
                FeatureRow(icon: "lock.shield.fill", title: "Private", description: "Your data stays on your device, always")
            }
            .padding(.top, Spacing.xl)

            Spacer()
        }
        .padding(Spacing.xl)
    }

    // MARK: - Permissions Step

    private var permissionsStep: some View {
        VStack(spacing: Spacing.xl) {
            Spacer()

            Image(systemName: "hand.raised.fill")
                .font(.system(size: 64))
                .foregroundStyle(.orange)

            Text("Permission Request")
                .font(.largeTitle.bold())

            Text("Fracta needs access to your files to help you organize them")
                .font(.title3)
                .foregroundStyle(.secondary)
                .multilineTextAlignment(.center)

            // Permission status
            VStack(spacing: Spacing.lg) {
                PermissionRow(
                    icon: "internaldrive",
                    title: "Full Disk Access",
                    description: "Required to browse and index all your files",
                    isGranted: hasFullDiskAccess,
                    action: openFullDiskAccessSettings
                )
            }
            .padding(.top, Spacing.lg)

            // Privacy note
            VStack(spacing: Spacing.sm) {
                Label("Your Privacy Matters", systemImage: "lock.fill")
                    .font(.headline)

                Text("Fracta never uploads your data. Everything stays on your device. We only read files you explicitly add to managed locations.")
                    .font(.caption)
                    .foregroundStyle(.secondary)
                    .multilineTextAlignment(.center)
            }
            .padding()
            .background(Color.green.opacity(0.1))
            .clipShape(RoundedRectangle(cornerRadius: 12))

            Spacer()
        }
        .padding(Spacing.xl)
    }

    // MARK: - Data Sources Step

    private var dataSourcesStep: some View {
        VStack(spacing: Spacing.xl) {
            Spacer()

            Image(systemName: "tray.2.fill")
                .font(.system(size: 64))
                .foregroundStyle(.blue)

            Text("Choose Your Data Sources")
                .font(.largeTitle.bold())

            Text("Select which folders Fracta should manage")
                .font(.title3)
                .foregroundStyle(.secondary)

            // Suggested locations
            VStack(alignment: .leading, spacing: Spacing.md) {
                Text("Suggested Locations")
                    .font(.headline)
                    .foregroundStyle(.secondary)

                DataSourceRow(
                    icon: "doc.text.fill",
                    title: "Documents",
                    path: "~/Documents",
                    isSelected: false
                )

                DataSourceRow(
                    icon: "desktopcomputer",
                    title: "Desktop",
                    path: "~/Desktop",
                    isSelected: false
                )

                DataSourceRow(
                    icon: "folder.fill",
                    title: "Custom Folder...",
                    path: "Choose any folder",
                    isSelected: false,
                    isCustom: true
                ) {
                    appState.showingFolderPicker = true
                }
            }
            .padding()
            .background(.ultraThinMaterial)
            .clipShape(RoundedRectangle(cornerRadius: 12))

            Text("You can always add or remove locations later in Settings")
                .font(.caption)
                .foregroundStyle(.secondary)

            Spacer()
        }
        .padding(Spacing.xl)
    }

    // MARK: - Ready Step

    private var readyStep: some View {
        VStack(spacing: Spacing.xl) {
            Spacer()

            Image(systemName: "checkmark.circle.fill")
                .font(.system(size: 64))
                .foregroundStyle(.green)

            Text("You're All Set!")
                .font(.largeTitle.bold())

            Text("Fracta is ready to help you organize your digital life")
                .font(.title3)
                .foregroundStyle(.secondary)

            VStack(alignment: .leading, spacing: Spacing.md) {
                TipRow(icon: "command", tip: "Press ⌘O to open a location anytime")
                TipRow(icon: "magnifyingglass", tip: "Press ⌘F to search your files")
                TipRow(icon: "gamecontroller", tip: "Game controllers work for navigation!")
            }
            .padding()
            .background(.ultraThinMaterial)
            .clipShape(RoundedRectangle(cornerRadius: 12))

            Spacer()
        }
        .padding(Spacing.xl)
    }

    // MARK: - Navigation Buttons

    private var navigationButtons: some View {
        HStack {
            if currentStep != .welcome {
                Button("Back") {
                    withAnimation {
                        currentStep = OnboardingStep(rawValue: currentStep.rawValue - 1) ?? .welcome
                    }
                }
                .buttonStyle(.bordered)
            }

            Spacer()

            if currentStep == .ready {
                Button("Get Started") {
                    completeOnboarding()
                }
                .buttonStyle(.borderedProminent)
            } else {
                Button("Continue") {
                    withAnimation {
                        currentStep = OnboardingStep(rawValue: currentStep.rawValue + 1) ?? .ready
                    }
                }
                .buttonStyle(.borderedProminent)
            }
        }
    }

    // MARK: - Actions

    private func checkFullDiskAccess() {
        // Check if we can access a protected directory
        let testPath = NSHomeDirectory() + "/Library/Mail"
        hasFullDiskAccess = FileManager.default.isReadableFile(atPath: testPath)
    }

    private func openFullDiskAccessSettings() {
        #if os(macOS)
        if let url = URL(string: "x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles") {
            NSWorkspace.shared.open(url)
        }
        #endif

        // Check again after a delay (user might have granted access)
        DispatchQueue.main.asyncAfter(deadline: .now() + 2) {
            checkFullDiskAccess()
        }
    }

    private func completeOnboarding() {
        appState.hasCompletedOnboarding = true
        appState.showingOnboarding = false
    }
}

// MARK: - Supporting Views

struct FeatureRow: View {
    let icon: String
    let title: String
    let description: String

    var body: some View {
        HStack(spacing: Spacing.md) {
            Image(systemName: icon)
                .font(.title2)
                .foregroundStyle(Color.accentColor)
                .frame(width: 32)

            VStack(alignment: .leading, spacing: 2) {
                Text(title)
                    .font(.headline)
                Text(description)
                    .font(.subheadline)
                    .foregroundStyle(.secondary)
            }

            Spacer()
        }
    }
}

struct PermissionRow: View {
    let icon: String
    let title: String
    let description: String
    let isGranted: Bool
    let action: () -> Void

    var body: some View {
        HStack(spacing: Spacing.md) {
            Image(systemName: icon)
                .font(.title)
                .foregroundStyle(isGranted ? .green : .orange)
                .frame(width: 40)

            VStack(alignment: .leading, spacing: 2) {
                Text(title)
                    .font(.headline)
                Text(description)
                    .font(.subheadline)
                    .foregroundStyle(.secondary)
            }

            Spacer()

            if isGranted {
                Image(systemName: "checkmark.circle.fill")
                    .foregroundStyle(.green)
            } else {
                Button("Grant Access") {
                    action()
                }
                .buttonStyle(.borderedProminent)
            }
        }
        .padding()
        .background(.ultraThinMaterial)
        .clipShape(RoundedRectangle(cornerRadius: 12))
    }
}

struct DataSourceRow: View {
    let icon: String
    let title: String
    let path: String
    var isSelected: Bool = false
    var isCustom: Bool = false
    var action: (() -> Void)? = nil

    var body: some View {
        Button {
            action?()
        } label: {
            HStack(spacing: Spacing.md) {
                Image(systemName: icon)
                    .font(.title2)
                    .foregroundStyle(Color.accentColor)
                    .frame(width: 32)

                VStack(alignment: .leading, spacing: 2) {
                    Text(title)
                        .font(.headline)
                    Text(path)
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }

                Spacer()

                if isCustom {
                    Image(systemName: "chevron.right")
                        .foregroundStyle(.secondary)
                } else {
                    Image(systemName: isSelected ? "checkmark.circle.fill" : "circle")
                        .foregroundStyle(isSelected ? Color.accentColor : .secondary)
                }
            }
        }
        .buttonStyle(.plain)
    }
}

struct TipRow: View {
    let icon: String
    let tip: String

    var body: some View {
        HStack(spacing: Spacing.md) {
            Image(systemName: icon)
                .font(.title3)
                .foregroundStyle(Color.accentColor)
                .frame(width: 24)

            Text(tip)
                .font(.subheadline)

            Spacer()
        }
    }
}

// MARK: - Preview

#Preview {
    OnboardingView()
        .environmentObject(AppState())
}
