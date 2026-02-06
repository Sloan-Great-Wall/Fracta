import SwiftUI
@preconcurrency import GameController

// MARK: - Game Controller Support
//
// Fracta supports game controllers (MFi, DualShock, Xbox) for navigation:
// - D-pad: Navigate between items
// - A/X button: Select/Enter
// - B/O button: Back/Cancel
// - Menu button: Open menu
// - Shoulder buttons: Switch tabs/sections

/// Game controller manager for the app
/// Uses @Observable for simpler state management with Swift 6 concurrency
@Observable
@MainActor
final class GameControllerManager {
    var isControllerConnected = false
    var controllerName: String?

    init() {
        // Check for already connected controllers
        updateControllerStatus()

        // Set up notifications using modern async approach
        Task {
            for await notification in NotificationCenter.default.notifications(named: .GCControllerDidConnect) {
                await MainActor.run {
                    self.updateControllerStatus()
                    if let controller = notification.object as? GCController {
                        self.controllerName = controller.vendorName
                    }
                }
            }
        }

        Task {
            for await _ in NotificationCenter.default.notifications(named: .GCControllerDidDisconnect) {
                await MainActor.run {
                    self.updateControllerStatus()
                }
            }
        }
    }

    private func updateControllerStatus() {
        let controllers = GCController.controllers()
        isControllerConnected = !controllers.isEmpty
        controllerName = controllers.first?.vendorName
    }
}

// MARK: - Focus Navigation Helpers

/// Direction of focus movement
enum FocusDirection {
    case up, down, left, right

    init?(_ direction: MoveCommandDirection) {
        switch direction {
        case .up: self = .up
        case .down: self = .down
        case .left: self = .left
        case .right: self = .right
        @unknown default: return nil
        }
    }
}

/// A view modifier that adds game controller navigation to a list
struct GamepadNavigableList<Item: Identifiable>: ViewModifier {
    let items: [Item]
    @Binding var focusedIndex: Int
    var onSelect: (Item) -> Void

    func body(content: Content) -> some View {
        content
            .onMoveCommand { direction in
                switch direction {
                case .up:
                    focusedIndex = max(0, focusedIndex - 1)
                case .down:
                    focusedIndex = min(items.count - 1, focusedIndex + 1)
                default:
                    break
                }
            }
            .onKeyPress(.return) {
                if let item = items[safe: focusedIndex] {
                    onSelect(item)
                }
                return .handled
            }
    }
}

extension View {
    func gamepadNavigable<Item: Identifiable>(
        items: [Item],
        focusedIndex: Binding<Int>,
        onSelect: @escaping (Item) -> Void
    ) -> some View {
        modifier(GamepadNavigableList(items: items, focusedIndex: focusedIndex, onSelect: onSelect))
    }
}

// MARK: - Controller Status View

/// Shows controller connection status
struct ControllerStatusView: View {
    var controllerManager: GameControllerManager

    var body: some View {
        HStack(spacing: 8) {
            Image(systemName: controllerManager.isControllerConnected ? "gamecontroller.fill" : "gamecontroller")
                .foregroundStyle(controllerManager.isControllerConnected ? .green : .secondary)

            if controllerManager.isControllerConnected {
                Text(controllerManager.controllerName ?? "Controller Connected")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
        .padding(.horizontal, 12)
        .padding(.vertical, 6)
        .background(.ultraThinMaterial)
        .clipShape(Capsule())
    }
}
