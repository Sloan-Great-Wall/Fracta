import SwiftUI

// MARK: - Liquid Glass Design System
//
// Fracta's visual language inspired by iOS 26/macOS 26 Liquid Glass:
// - Translucent materials with depth
// - Soft, ambient lighting effects
// - Subtle animations and transitions
// - Focus-friendly large touch targets

// MARK: - Glass Card Modifier

struct GlassCardModifier: ViewModifier {
    var cornerRadius: CGFloat = 16
    var padding: CGFloat = 16
    var isSelected: Bool = false
    var isFocused: Bool = false

    func body(content: Content) -> some View {
        content
            .padding(padding)
            .background {
                RoundedRectangle(cornerRadius: cornerRadius)
                    .fill(.ultraThinMaterial)
                    .overlay {
                        RoundedRectangle(cornerRadius: cornerRadius)
                            .stroke(
                                isFocused ? Color.accentColor : Color.white.opacity(0.2),
                                lineWidth: isFocused ? 2 : 0.5
                            )
                    }
                    .shadow(
                        color: .black.opacity(isSelected ? 0.2 : 0.1),
                        radius: isSelected ? 12 : 6,
                        y: isSelected ? 6 : 3
                    )
            }
            .scaleEffect(isFocused ? 1.02 : 1.0)
            .animation(.spring(response: 0.3, dampingFraction: 0.7), value: isFocused)
            .animation(.spring(response: 0.3, dampingFraction: 0.7), value: isSelected)
    }
}

extension View {
    func glassCard(
        cornerRadius: CGFloat = 16,
        padding: CGFloat = 16,
        isSelected: Bool = false,
        isFocused: Bool = false
    ) -> some View {
        modifier(GlassCardModifier(
            cornerRadius: cornerRadius,
            padding: padding,
            isSelected: isSelected,
            isFocused: isFocused
        ))
    }
}

// MARK: - Glass Button Style

struct GlassButtonStyle: ButtonStyle {
    var size: ControlSize = .regular

    private var minHeight: CGFloat {
        switch size {
        case .mini: return 32
        case .small: return 40
        case .regular: return 48
        case .large: return 56
        case .extraLarge: return 64
        @unknown default: return 48
        }
    }

    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .font(size == .large || size == .extraLarge ? .headline : .body)
            .padding(.horizontal, 20)
            .frame(minHeight: minHeight)
            .background {
                Capsule()
                    .fill(.ultraThinMaterial)
                    .overlay {
                        Capsule()
                            .stroke(Color.white.opacity(0.3), lineWidth: 0.5)
                    }
            }
            .scaleEffect(configuration.isPressed ? 0.95 : 1.0)
            .opacity(configuration.isPressed ? 0.8 : 1.0)
            .animation(.spring(response: 0.2, dampingFraction: 0.7), value: configuration.isPressed)
    }
}

extension ButtonStyle where Self == GlassButtonStyle {
    static var glass: GlassButtonStyle { GlassButtonStyle() }
    static func glass(size: ControlSize) -> GlassButtonStyle {
        GlassButtonStyle(size: size)
    }
}

// MARK: - Glass Navigation Style

struct GlassSidebarStyle: ViewModifier {
    func body(content: Content) -> some View {
        content
            .background(.ultraThinMaterial)
            .scrollContentBackground(.hidden)
    }
}

extension View {
    func glassSidebar() -> some View {
        modifier(GlassSidebarStyle())
    }
}

// MARK: - Focus Ring Modifier

struct FocusRingModifier: ViewModifier {
    var isFocused: Bool
    var cornerRadius: CGFloat = 12

    func body(content: Content) -> some View {
        content
            .overlay {
                RoundedRectangle(cornerRadius: cornerRadius)
                    .stroke(Color.accentColor, lineWidth: isFocused ? 3 : 0)
                    .padding(-4)
            }
            .animation(.easeInOut(duration: 0.15), value: isFocused)
    }
}

extension View {
    func focusRing(_ isFocused: Bool, cornerRadius: CGFloat = 12) -> some View {
        modifier(FocusRingModifier(isFocused: isFocused, cornerRadius: cornerRadius))
    }
}

// MARK: - Color Palette

extension Color {
    static let glassBackground = Color(white: 0.1, opacity: 0.3)
    static let glassBorder = Color.white.opacity(0.2)
    static let glassHighlight = Color.white.opacity(0.1)

    // Semantic colors
    static let folderColor = Color.blue
    static let markdownColor = Color.purple
    static let codeColor = Color.orange
    static let imageColor = Color.green
}

// MARK: - Typography

extension Font {
    static let glassTitle = Font.system(.largeTitle, design: .rounded, weight: .bold)
    static let glassHeadline = Font.system(.headline, design: .rounded, weight: .semibold)
    static let glassBody = Font.system(.body, design: .default)
    static let glassCaption = Font.system(.caption, design: .default)
    static let glassMono = Font.system(.body, design: .monospaced)
}

// MARK: - Spacing Constants

enum Spacing {
    static let xs: CGFloat = 4
    static let sm: CGFloat = 8
    static let md: CGFloat = 16
    static let lg: CGFloat = 24
    static let xl: CGFloat = 32
    static let xxl: CGFloat = 48

    // Game controller friendly minimum touch target
    static let touchTarget: CGFloat = 44
    static let gamepadTarget: CGFloat = 60
}
