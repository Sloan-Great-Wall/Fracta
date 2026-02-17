import SwiftUI

#if os(macOS)
import AppKit

/// Native NSTextView wrapper for efficient large text display and editing
struct NativeTextView: NSViewRepresentable {
    let text: String
    let font: NSFont
    var isEditable: Bool
    var onTextChange: ((String) -> Void)?

    init(
        _ text: String,
        font: NSFont = .monospacedSystemFont(ofSize: 13, weight: .regular),
        isEditable: Bool = false,
        onTextChange: ((String) -> Void)? = nil
    ) {
        self.text = text
        self.font = font
        self.isEditable = isEditable
        self.onTextChange = onTextChange
    }

    func makeCoordinator() -> Coordinator {
        Coordinator(onTextChange: onTextChange)
    }

    func makeNSView(context: Context) -> NSScrollView {
        let scrollView = NSTextView.scrollableTextView()
        let textView = scrollView.documentView as! NSTextView

        // Configure for performance
        textView.isEditable = isEditable
        textView.isSelectable = true
        textView.font = font
        textView.textColor = .labelColor
        textView.backgroundColor = .clear
        textView.drawsBackground = false

        // Disable expensive features for large files
        textView.isAutomaticQuoteSubstitutionEnabled = false
        textView.isAutomaticDashSubstitutionEnabled = false
        textView.isAutomaticTextReplacementEnabled = false
        textView.isAutomaticSpellingCorrectionEnabled = false
        textView.isContinuousSpellCheckingEnabled = false
        textView.isGrammarCheckingEnabled = false

        // Use layer-backed view for better performance
        textView.wantsLayer = true
        scrollView.wantsLayer = true

        // Set delegate for edit callbacks
        textView.delegate = context.coordinator

        return scrollView
    }

    func updateNSView(_ scrollView: NSScrollView, context: Context) {
        guard let textView = scrollView.documentView as? NSTextView else { return }

        context.coordinator.onTextChange = onTextChange
        textView.isEditable = isEditable

        // Only update if text changed externally (not from user typing)
        if textView.string != text && !context.coordinator.isUpdating {
            textView.string = text
        }
    }

    class Coordinator: NSObject, NSTextViewDelegate {
        var onTextChange: ((String) -> Void)?
        var isUpdating = false

        init(onTextChange: ((String) -> Void)?) {
            self.onTextChange = onTextChange
        }

        func textDidChange(_ notification: Notification) {
            guard let textView = notification.object as? NSTextView else { return }
            isUpdating = true
            onTextChange?(textView.string)
            isUpdating = false
        }
    }
}

#else
import UIKit

/// Native UITextView wrapper for efficient large text display and editing
struct NativeTextView: UIViewRepresentable {
    let text: String
    let font: UIFont
    var isEditable: Bool
    var onTextChange: ((String) -> Void)?

    init(
        _ text: String,
        font: UIFont = .monospacedSystemFont(ofSize: 14, weight: .regular),
        isEditable: Bool = false,
        onTextChange: ((String) -> Void)? = nil
    ) {
        self.text = text
        self.font = font
        self.isEditable = isEditable
        self.onTextChange = onTextChange
    }

    func makeCoordinator() -> Coordinator {
        Coordinator(onTextChange: onTextChange)
    }

    func makeUIView(context: Context) -> UITextView {
        let textView = UITextView()

        // Configure for performance
        textView.isEditable = isEditable
        textView.isSelectable = true
        textView.font = font
        textView.textColor = .label
        textView.backgroundColor = .clear

        // Disable expensive features
        textView.autocorrectionType = .no
        textView.spellCheckingType = .no

        // Set delegate for edit callbacks
        textView.delegate = context.coordinator

        return textView
    }

    func updateUIView(_ textView: UITextView, context: Context) {
        context.coordinator.onTextChange = onTextChange
        textView.isEditable = isEditable

        // Only update if text changed externally (not from user typing)
        if textView.text != text && !context.coordinator.isUpdating {
            textView.text = text
        }
    }

    class Coordinator: NSObject, UITextViewDelegate {
        var onTextChange: ((String) -> Void)?
        var isUpdating = false

        init(onTextChange: ((String) -> Void)?) {
            self.onTextChange = onTextChange
        }

        func textViewDidChange(_ textView: UITextView) {
            isUpdating = true
            onTextChange?(textView.text)
            isUpdating = false
        }
    }
}
#endif

// MARK: - Quick Look Preview for various file types

#if os(macOS)
import QuickLook
import Quartz

/// Quick Look preview for any file type (images, PDFs, documents, etc.)
struct QuickLookPreview: NSViewRepresentable {
    let url: URL

    func makeNSView(context: Context) -> QLPreviewView {
        let preview = QLPreviewView(frame: .zero, style: .normal)!
        preview.autostarts = true
        return preview
    }

    func updateNSView(_ preview: QLPreviewView, context: Context) {
        preview.previewItem = url as QLPreviewItem
    }
}
#else
import QuickLook

/// Quick Look preview controller wrapper for iOS
struct QuickLookPreview: UIViewControllerRepresentable {
    let url: URL

    func makeUIViewController(context: Context) -> QLPreviewController {
        let controller = QLPreviewController()
        controller.dataSource = context.coordinator
        return controller
    }

    func updateUIViewController(_ controller: QLPreviewController, context: Context) {
        controller.reloadData()
    }

    func makeCoordinator() -> Coordinator {
        Coordinator(url: url)
    }

    class Coordinator: NSObject, QLPreviewControllerDataSource {
        let url: URL

        init(url: URL) {
            self.url = url
        }

        func numberOfPreviewItems(in controller: QLPreviewController) -> Int { 1 }

        func previewController(_ controller: QLPreviewController, previewItemAt index: Int) -> QLPreviewItem {
            url as QLPreviewItem
        }
    }
}
#endif

// MARK: - File Preview Router

/// Routes files to appropriate preview based on type
struct FilePreviewRouter: View {
    let file: FileItem
    @State private var textContent: String?
    @State private var isLoading = true
    @State private var error: String?

    var body: some View {
        Group {
            if isLoading {
                ProgressView("Loading...")
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
            } else if let error = error {
                VStack(spacing: 12) {
                    Image(systemName: "exclamationmark.triangle")
                        .font(.largeTitle)
                        .foregroundStyle(.orange)
                    Text(error)
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
                .frame(maxWidth: .infinity, maxHeight: .infinity)
            } else {
                contentView
            }
        }
        .task(id: file.id) {
            await loadContent()
        }
    }

    @ViewBuilder
    private var contentView: some View {
        switch file.previewType {
        case .text:
            if let content = textContent {
                NativeTextView(content)
            }
        case .quickLook:
            QuickLookPreview(url: URL(fileURLWithPath: file.path))
        case .unsupported:
            VStack(spacing: 12) {
                Image(systemName: file.icon)
                    .font(.system(size: 48))
                    .foregroundStyle(.secondary)
                Text("Preview not available")
                    .font(.headline)
                    .foregroundStyle(.secondary)
                Text(file.name)
                    .font(.caption)
                    .foregroundStyle(.tertiary)
            }
            .frame(maxWidth: .infinity, maxHeight: .infinity)
        }
    }

    private func loadContent() async {
        isLoading = true
        error = nil
        textContent = nil

        // Small delay to show loading state (prevents flash)
        try? await Task.sleep(nanoseconds: 50_000_000) // 50ms

        switch file.previewType {
        case .text:
            // Load text on background thread
            let result = await Task.detached(priority: .userInitiated) { [path = file.path] in
                do {
                    let content = try String(contentsOfFile: path, encoding: .utf8)
                    return Result<String, Error>.success(content)
                } catch {
                    return Result<String, Error>.failure(error)
                }
            }.value

            await MainActor.run {
                switch result {
                case .success(let content):
                    textContent = content
                case .failure(let err):
                    error = err.localizedDescription
                }
                isLoading = false
            }

        case .quickLook, .unsupported:
            // Quick Look handles its own loading
            await MainActor.run {
                isLoading = false
            }
        }
    }
}

// MARK: - Preview Type Extension

extension FileItem {
    enum PreviewType {
        case text       // Use NativeTextView
        case quickLook  // Use Quick Look (images, PDFs, videos, etc.)
        case unsupported
    }

    var previewType: PreviewType {
        guard let ext = fileExtension?.lowercased() else {
            return isFolder ? .unsupported : .quickLook
        }

        // Text files - use native text view
        let textExtensions = ["md", "markdown", "txt", "json", "yaml", "yml",
                              "xml", "html", "css", "js", "ts", "swift", "rs",
                              "py", "rb", "go", "java", "c", "cpp", "h", "hpp",
                              "sh", "bash", "zsh", "fish", "toml", "ini", "conf",
                              "log", "csv", "tsv"]
        if textExtensions.contains(ext) {
            return .text
        }

        // Quick Look handles these well
        let quickLookExtensions = ["pdf", "png", "jpg", "jpeg", "gif", "webp",
                                   "heic", "tiff", "bmp", "svg", "mp4", "mov",
                                   "m4v", "mp3", "m4a", "wav", "aiff", "doc",
                                   "docx", "xls", "xlsx", "ppt", "pptx", "rtf",
                                   "pages", "numbers", "keynote", "zip", "epub"]
        if quickLookExtensions.contains(ext) {
            return .quickLook
        }

        // Default to Quick Look for unknown types (it often works)
        return .quickLook
    }
}
