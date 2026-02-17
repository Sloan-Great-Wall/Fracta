import Foundation
#if os(macOS)
import AppKit
typealias PlatformFont = NSFont
typealias PlatformColor = NSColor
#else
import UIKit
typealias PlatformFont = UIFont
typealias PlatformColor = UIColor
#endif

/// Converts raw Markdown text to styled NSAttributedString.
///
/// Handles headings, bold, italic, code spans/blocks, links, lists,
/// block quotes, and horizontal rules. Uses regex-free line-by-line
/// parsing for performance on large documents.
enum MarkdownRenderer {

    // MARK: - Public API

    /// Render Markdown text to an attributed string with rich formatting.
    static func render(_ markdown: String) -> NSAttributedString {
        let result = NSMutableAttributedString()
        let lines = markdown.components(separatedBy: "\n")
        var i = 0

        while i < lines.count {
            let line = lines[i]

            // Fenced code block
            if line.hasPrefix("```") {
                var codeLines: [String] = []
                i += 1
                while i < lines.count && !lines[i].hasPrefix("```") {
                    codeLines.append(lines[i])
                    i += 1
                }
                if i < lines.count { i += 1 } // skip closing ```
                appendCodeBlock(codeLines.joined(separator: "\n"), to: result)
                appendNewline(to: result)
                continue
            }

            // Horizontal rule
            let trimmed = line.trimmingCharacters(in: .whitespaces)
            if trimmed == "---" || trimmed == "***" || trimmed == "___" {
                appendHorizontalRule(to: result)
                i += 1
                continue
            }

            // Heading
            if let (level, text) = parseHeading(line) {
                appendHeading(text, level: level, to: result)
                appendNewline(to: result)
                i += 1
                continue
            }

            // Block quote
            if trimmed.hasPrefix("> ") || trimmed == ">" {
                let quoteText = String(trimmed.dropFirst(trimmed.hasPrefix("> ") ? 2 : 1))
                appendBlockQuote(quoteText, to: result)
                appendNewline(to: result)
                i += 1
                continue
            }

            // Unordered list
            if trimmed.hasPrefix("- ") || trimmed.hasPrefix("* ") || trimmed.hasPrefix("+ ") {
                let itemText = String(trimmed.dropFirst(2))
                appendListItem(itemText, ordered: false, number: 0, to: result)
                appendNewline(to: result)
                i += 1
                continue
            }

            // Ordered list
            if let (number, itemText) = parseOrderedListItem(trimmed) {
                appendListItem(itemText, ordered: true, number: number, to: result)
                appendNewline(to: result)
                i += 1
                continue
            }

            // Task list
            if trimmed.hasPrefix("- [ ] ") || trimmed.hasPrefix("- [x] ") || trimmed.hasPrefix("- [X] ") {
                let checked = trimmed.hasPrefix("- [x] ") || trimmed.hasPrefix("- [X] ")
                let itemText = String(trimmed.dropFirst(6))
                appendTaskItem(itemText, checked: checked, to: result)
                appendNewline(to: result)
                i += 1
                continue
            }

            // Empty line = paragraph break
            if trimmed.isEmpty {
                appendNewline(to: result)
                i += 1
                continue
            }

            // Regular paragraph
            appendInlineFormatted(line, to: result, baseFont: bodyFont)
            appendNewline(to: result)
            i += 1
        }

        return result
    }

    // MARK: - Fonts

    private static var bodyFont: PlatformFont {
        .systemFont(ofSize: 14, weight: .regular)
    }

    private static var boldFont: PlatformFont {
        .systemFont(ofSize: 14, weight: .bold)
    }

    private static var italicFont: PlatformFont {
        #if os(macOS)
        NSFontManager.shared.convert(bodyFont, toHaveTrait: .italicFontMask)
        #else
        let descriptor = bodyFont.fontDescriptor.withSymbolicTraits(.traitItalic) ?? bodyFont.fontDescriptor
        return UIFont(descriptor: descriptor, size: 14)
        #endif
    }

    private static var boldItalicFont: PlatformFont {
        #if os(macOS)
        NSFontManager.shared.convert(boldFont, toHaveTrait: .italicFontMask)
        #else
        let descriptor = boldFont.fontDescriptor.withSymbolicTraits([.traitBold, .traitItalic]) ?? boldFont.fontDescriptor
        return UIFont(descriptor: descriptor, size: 14)
        #endif
    }

    private static var codeFont: PlatformFont {
        .monospacedSystemFont(ofSize: 13, weight: .regular)
    }

    private static func headingFont(level: Int) -> PlatformFont {
        switch level {
        case 1: return .systemFont(ofSize: 24, weight: .bold)
        case 2: return .systemFont(ofSize: 20, weight: .bold)
        case 3: return .systemFont(ofSize: 17, weight: .semibold)
        case 4: return .systemFont(ofSize: 15, weight: .semibold)
        default: return .systemFont(ofSize: 14, weight: .semibold)
        }
    }

    // MARK: - Colors

    private static var bodyColor: PlatformColor { .labelColor }
    private static var secondaryColor: PlatformColor { .secondaryLabelColor }
    private static var linkColor: PlatformColor { .systemBlue }
    private static var codeBackgroundColor: PlatformColor {
        #if os(macOS)
        .quaternaryLabelColor
        #else
        .quaternarySystemFill
        #endif
    }

    // MARK: - Block Renderers

    private static func appendHeading(_ text: String, level: Int, to result: NSMutableAttributedString) {
        let font = headingFont(level: level)
        let headingStr = NSMutableAttributedString()
        appendInlineFormatted(text, to: headingStr, baseFont: font)

        // Override font for all ranges
        headingStr.addAttribute(.foregroundColor, value: bodyColor, range: NSRange(location: 0, length: headingStr.length))

        // Add paragraph spacing
        let para = NSMutableParagraphStyle()
        para.paragraphSpacingBefore = level == 1 ? 16 : 12
        para.paragraphSpacing = 4
        headingStr.addAttribute(.paragraphStyle, value: para, range: NSRange(location: 0, length: headingStr.length))

        result.append(headingStr)
    }

    private static func appendCodeBlock(_ code: String, to result: NSMutableAttributedString) {
        let para = NSMutableParagraphStyle()
        para.headIndent = 12
        para.firstLineHeadIndent = 12
        para.tailIndent = -12
        para.paragraphSpacingBefore = 8
        para.paragraphSpacing = 8

        let attrs: [NSAttributedString.Key: Any] = [
            .font: codeFont,
            .foregroundColor: bodyColor,
            .backgroundColor: codeBackgroundColor,
            .paragraphStyle: para,
        ]
        result.append(NSAttributedString(string: code, attributes: attrs))
    }

    private static func appendBlockQuote(_ text: String, to result: NSMutableAttributedString) {
        let para = NSMutableParagraphStyle()
        para.headIndent = 20
        para.firstLineHeadIndent = 20

        let quoteStr = NSMutableAttributedString(string: "│ ", attributes: [
            .font: bodyFont,
            .foregroundColor: secondaryColor,
        ])
        let textStr = NSMutableAttributedString()
        appendInlineFormatted(text, to: textStr, baseFont: bodyFont)
        textStr.addAttribute(.foregroundColor, value: secondaryColor, range: NSRange(location: 0, length: textStr.length))

        quoteStr.append(textStr)
        quoteStr.addAttribute(.paragraphStyle, value: para, range: NSRange(location: 0, length: quoteStr.length))
        result.append(quoteStr)
    }

    private static func appendListItem(_ text: String, ordered: Bool, number: Int, to result: NSMutableAttributedString) {
        let para = NSMutableParagraphStyle()
        para.headIndent = 24
        para.firstLineHeadIndent = 8

        let bullet = ordered ? "\(number). " : "  •  "
        let bulletStr = NSAttributedString(string: bullet, attributes: [
            .font: bodyFont,
            .foregroundColor: secondaryColor,
        ])

        let itemStr = NSMutableAttributedString()
        itemStr.append(bulletStr)
        appendInlineFormatted(text, to: itemStr, baseFont: bodyFont)
        itemStr.addAttribute(.paragraphStyle, value: para, range: NSRange(location: 0, length: itemStr.length))
        result.append(itemStr)
    }

    private static func appendTaskItem(_ text: String, checked: Bool, to result: NSMutableAttributedString) {
        let para = NSMutableParagraphStyle()
        para.headIndent = 24
        para.firstLineHeadIndent = 8

        let checkbox = checked ? "  ☑  " : "  ☐  "
        let checkStr = NSAttributedString(string: checkbox, attributes: [
            .font: bodyFont,
            .foregroundColor: checked ? PlatformColor.systemGreen : secondaryColor,
        ])

        let itemStr = NSMutableAttributedString()
        itemStr.append(checkStr)
        appendInlineFormatted(text, to: itemStr, baseFont: bodyFont)

        if checked {
            itemStr.addAttribute(.strikethroughStyle, value: NSUnderlineStyle.single.rawValue, range: NSRange(location: checkbox.count, length: itemStr.length - checkbox.count))
            itemStr.addAttribute(.foregroundColor, value: secondaryColor, range: NSRange(location: checkbox.count, length: itemStr.length - checkbox.count))
        }

        itemStr.addAttribute(.paragraphStyle, value: para, range: NSRange(location: 0, length: itemStr.length))
        result.append(itemStr)
    }

    private static func appendHorizontalRule(to result: NSMutableAttributedString) {
        let para = NSMutableParagraphStyle()
        para.paragraphSpacingBefore = 8
        para.paragraphSpacing = 8
        para.alignment = .center

        let rule = NSAttributedString(string: "─────────────────────────────────\n", attributes: [
            .font: PlatformFont.systemFont(ofSize: 8),
            .foregroundColor: secondaryColor,
            .paragraphStyle: para,
        ])
        result.append(rule)
    }

    private static func appendNewline(to result: NSMutableAttributedString) {
        result.append(NSAttributedString(string: "\n", attributes: [.font: bodyFont]))
    }

    // MARK: - Inline Formatting

    /// Parse inline formatting: **bold**, *italic*, `code`, [links](url)
    private static func appendInlineFormatted(_ text: String, to result: NSMutableAttributedString, baseFont: PlatformFont) {
        let scanner = InlineScanner(text)
        let defaultAttrs: [NSAttributedString.Key: Any] = [
            .font: baseFont,
            .foregroundColor: bodyColor,
        ]

        while !scanner.isAtEnd {
            // Code span: `code`
            if scanner.match("`") {
                if let code = scanner.scanUntil("`") {
                    let attrs: [NSAttributedString.Key: Any] = [
                        .font: codeFont,
                        .foregroundColor: bodyColor,
                        .backgroundColor: codeBackgroundColor,
                    ]
                    result.append(NSAttributedString(string: code, attributes: attrs))
                    continue
                }
            }

            // Bold italic: ***text*** or ___text___
            if scanner.match("***") || scanner.match("___") {
                let delim = scanner.lastMatch!
                if let content = scanner.scanUntil(delim) {
                    let attrs: [NSAttributedString.Key: Any] = [
                        .font: boldItalicFont,
                        .foregroundColor: bodyColor,
                    ]
                    result.append(NSAttributedString(string: content, attributes: attrs))
                    continue
                }
            }

            // Bold: **text** or __text__
            if scanner.match("**") || scanner.match("__") {
                let delim = scanner.lastMatch!
                if let content = scanner.scanUntil(delim) {
                    let attrs: [NSAttributedString.Key: Any] = [
                        .font: boldFont,
                        .foregroundColor: bodyColor,
                    ]
                    result.append(NSAttributedString(string: content, attributes: attrs))
                    continue
                }
            }

            // Italic: *text* or _text_
            if scanner.match("*") || scanner.match("_") {
                let delim = scanner.lastMatch!
                if let content = scanner.scanUntil(delim) {
                    let attrs: [NSAttributedString.Key: Any] = [
                        .font: italicFont,
                        .foregroundColor: bodyColor,
                    ]
                    result.append(NSAttributedString(string: content, attributes: attrs))
                    continue
                }
            }

            // Link: [text](url)
            if scanner.match("[") {
                if let linkText = scanner.scanUntil("]") {
                    if scanner.match("(") {
                        if let url = scanner.scanUntil(")") {
                            let attrs: [NSAttributedString.Key: Any] = [
                                .font: baseFont,
                                .foregroundColor: linkColor,
                                .underlineStyle: NSUnderlineStyle.single.rawValue,
                                .link: URL(string: url) as Any,
                            ]
                            result.append(NSAttributedString(string: linkText, attributes: attrs))
                            continue
                        }
                    }
                    // Not a valid link, emit as plain text
                    result.append(NSAttributedString(string: "[" + linkText + "]", attributes: defaultAttrs))
                    continue
                }
            }

            // Regular character
            let ch = scanner.advance()
            result.append(NSAttributedString(string: String(ch), attributes: defaultAttrs))
        }
    }

    // MARK: - Helpers

    private static func parseHeading(_ line: String) -> (Int, String)? {
        var level = 0
        var idx = line.startIndex
        while idx < line.endIndex && line[idx] == "#" && level < 6 {
            level += 1
            idx = line.index(after: idx)
        }
        guard level > 0, idx < line.endIndex, line[idx] == " " else { return nil }
        let text = String(line[line.index(after: idx)...])
        return (level, text)
    }

    private static func parseOrderedListItem(_ line: String) -> (Int, String)? {
        let parts = line.split(separator: ".", maxSplits: 1)
        guard parts.count == 2, let number = Int(parts[0]) else { return nil }
        let text = String(parts[1]).trimmingCharacters(in: .whitespaces)
        guard !text.isEmpty else { return nil }
        return (number, text)
    }
}

// MARK: - Inline Scanner

/// Simple character-by-character scanner for inline Markdown formatting.
private class InlineScanner {
    private let text: String
    private var index: String.Index
    private(set) var lastMatch: String?

    var isAtEnd: Bool { index >= text.endIndex }

    init(_ text: String) {
        self.text = text
        self.index = text.startIndex
    }

    /// Try to match a delimiter at the current position. If found, advance past it.
    func match(_ delimiter: String) -> Bool {
        let remaining = text[index...]
        if remaining.hasPrefix(delimiter) {
            lastMatch = delimiter
            index = text.index(index, offsetBy: delimiter.count)
            return true
        }
        return false
    }

    /// Scan until the delimiter is found. Returns content between current position and delimiter.
    /// Advances past the delimiter. Returns nil if delimiter not found.
    func scanUntil(_ delimiter: String) -> String? {
        let start = index
        while index < text.endIndex {
            if text[index...].hasPrefix(delimiter) {
                let content = String(text[start..<index])
                index = text.index(index, offsetBy: delimiter.count)
                return content.isEmpty ? nil : content
            }
            index = text.index(after: index)
        }
        // Delimiter not found — rewind
        index = start
        return nil
    }

    /// Advance one character and return it.
    func advance() -> Character {
        let ch = text[index]
        index = text.index(after: index)
        return ch
    }
}
