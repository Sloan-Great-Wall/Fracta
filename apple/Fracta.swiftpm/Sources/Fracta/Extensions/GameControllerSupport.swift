import SwiftUI
import GameController

// MARK: - Game Controller Support
//
// Fracta supports game controllers (MFi, DualShock, Xbox) for navigation:
// - D-pad: Navigate between items
// - A/X button: Select/Enter
// - B/O button: Back/Cancel
// - Menu button: Open menu
// - Shoulder buttons: Switch tabs/sections

/// Game controller manager for the app
@MainActor
class GameControllerManager: ObservableObject {
    @Published var isControllerConnected = false
    @Published var currentController: GCController?

    private var observers: [NSObjectProtocol] = []

    init() {
        setupNotifications()
        checkForConnectedControllers()
    }

    deinit {
        observers.forEach { NotificationCenter.default.removeObserver($0) }
    }

    private func setupNotifications() {
        let connectObserver = NotificationCenter.default.addObserver(
            forName: .GCControllerDidConnect,
            object: nil,
            queue: .main
        ) { [weak self] notification in
            if let controller = notification.object as? GCController {
                self?.controllerConnected(controller)
            }
        }

        let disconnectObserver = NotificationCenter.default.addObserver(
            forName: .GCControllerDidDisconnect,
            object: nil,
            queue: .main
        ) { [weak self] _ in
            self?.controllerDisconnected()
        }

        observers = [connectObserver, disconnectObserver]
    }

    private func checkForConnectedControllers() {
        if let controller = GCController.controllers().first {
            controllerConnected(controller)
        }
    }

    private func controllerConnected(_ controller: GCController) {
        currentController = controller
        isControllerConnected = true
        configureController(controller)
    }

    private func controllerDisconnected() {
        currentController = nil
        isControllerConnected = GCController.controllers().first != nil
    }

    private func configureController(_ controller: GCController) {
        // Enable micro gamepad for Siri Remote
        controller.microGamepad?.reportsAbsoluteDpadValues = true

        // Configure extended gamepad (standard controllers)
        if let gamepad = controller.extendedGamepad {
            configureExtendedGamepad(gamepad)
        }
    }

    private func configureExtendedGamepad(_ gamepad: GCExtendedGamepad) {
        // D-pad navigation is handled by SwiftUI's focus system
        // But we can add haptic feedback on iOS

        #if os(iOS)
        gamepad.buttonA.pressedChangedHandler = { [weak self] _, _, pressed in
            if pressed {
                self?.provideHapticFeedback(.selection)
            }
        }

        gamepad.buttonB.pressedChangedHandler = { [weak self] _, _, pressed in
            if pressed {
                self?.provideHapticFeedback(.light)
            }
        }
        #endif
    }

    #if os(iOS)
    private func provideHapticFeedback(_ style: UIImpactFeedbackGenerator.FeedbackStyle) {
        let generator = UIImpactFeedbackGenerator(style: style)
        generator.impactOccurred()
    }
    #endif
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
    @ObservedObject var controllerManager: GameControllerManager

    var body: some View {
        HStack(spacing: 8) {
            Image(systemName: controllerManager.isControllerConnected ? "gamecontroller.fill" : "gamecontroller")
                .foregroundStyle(controllerManager.isControllerConnected ? .green : .secondary)

            if controllerManager.isControllerConnected {
                Text("Controller Connected")
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

// MARK: - UIKit Bridge for Haptics

#if os(iOS)
import UIKit
#endif
