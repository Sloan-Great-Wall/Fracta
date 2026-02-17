import SwiftUI

/// AI chat interface — sends prompts to the Fracta AI engine
///
/// Currently uses the echo provider (development mode). When a real
/// provider is configured (OpenAI, Anthropic, local model), this same
/// interface will produce actual AI responses.
struct AiChatView: View {
    @EnvironmentObject var appState: AppState
    @State private var prompt = ""
    @State private var messages: [ChatBubble] = []
    @State private var isProcessing = false
    @State private var errorMessage: String?
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationStack {
            VStack(spacing: 0) {
                // Messages list
                ScrollViewReader { proxy in
                    ScrollView {
                        LazyVStack(alignment: .leading, spacing: Spacing.md) {
                            if messages.isEmpty {
                                emptyState
                            }

                            ForEach(messages) { message in
                                chatBubble(message)
                                    .id(message.id)
                            }
                        }
                        .padding(Spacing.lg)
                    }
                    .onChange(of: messages.count) { _, _ in
                        if let last = messages.last {
                            withAnimation {
                                proxy.scrollTo(last.id, anchor: .bottom)
                            }
                        }
                    }
                }

                Divider()

                // Error banner
                if let error = errorMessage {
                    HStack {
                        Image(systemName: "exclamationmark.triangle")
                            .foregroundStyle(.orange)
                        Text(error)
                            .font(.caption)
                            .foregroundStyle(.secondary)
                        Spacer()
                        Button {
                            errorMessage = nil
                        } label: {
                            Image(systemName: "xmark.circle")
                                .foregroundStyle(.secondary)
                        }
                        .buttonStyle(.plain)
                    }
                    .padding(.horizontal, Spacing.lg)
                    .padding(.vertical, Spacing.sm)
                    .background(.orange.opacity(0.1))
                }

                // Input bar
                HStack(spacing: Spacing.sm) {
                    TextField("Ask Fracta AI...", text: $prompt, axis: .vertical)
                        .textFieldStyle(.plain)
                        .lineLimit(1...5)
                        .onSubmit {
                            sendMessage()
                        }

                    Button {
                        sendMessage()
                    } label: {
                        Image(systemName: isProcessing ? "stop.circle" : "arrow.up.circle.fill")
                            .font(.title2)
                            .foregroundStyle(prompt.isEmpty && !isProcessing ? Color.secondary : Color.accentColor)
                    }
                    .disabled(prompt.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty || isProcessing)
                }
                .padding(Spacing.md)
            }
            .navigationTitle("Fracta AI")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Done") {
                        dismiss()
                    }
                }

                ToolbarItem(placement: .automatic) {
                    Text(FractaBridge.shared.aiModelName)
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
            }
        }
        #if os(macOS)
        .frame(minWidth: 480, idealWidth: 560, minHeight: 400, idealHeight: 600)
        #endif
    }

    // MARK: - Empty State

    private var emptyState: some View {
        VStack(spacing: Spacing.lg) {
            Spacer()

            Image(systemName: "sparkles")
                .font(.system(size: 48))
                .foregroundStyle(.secondary)

            Text("Ask Fracta AI")
                .font(.glassTitle)
                .foregroundStyle(.secondary)

            Text("Ask questions about your files, get help organizing notes, or request summaries of your content.")
                .font(.glassCaption)
                .foregroundStyle(.tertiary)
                .multilineTextAlignment(.center)
                .frame(maxWidth: 320)

            // Quick action chips
            VStack(spacing: Spacing.sm) {
                quickAction("Summarize my recent notes")
                quickAction("What topics appear most in my files?")
                quickAction("Help me organize this folder")
            }

            Spacer()
        }
        .frame(maxWidth: .infinity)
    }

    private func quickAction(_ text: String) -> some View {
        Button {
            prompt = text
            sendMessage()
        } label: {
            Text(text)
                .font(.caption)
                .padding(.horizontal, Spacing.md)
                .padding(.vertical, Spacing.sm)
                .background(.secondary.opacity(0.1))
                .clipShape(Capsule())
        }
        .buttonStyle(.plain)
    }

    // MARK: - Chat Bubble

    private func chatBubble(_ message: ChatBubble) -> some View {
        HStack {
            if message.isUser { Spacer(minLength: 60) }

            VStack(alignment: message.isUser ? .trailing : .leading, spacing: 4) {
                Text(message.content)
                    .font(.body)
                    .textSelection(.enabled)

                if let model = message.model {
                    Text("\(model) · \(message.tokensUsed) tokens")
                        .font(.caption2)
                        .foregroundStyle(.tertiary)
                }
            }
            .padding(Spacing.md)
            .background(message.isUser ? Color.accentColor.opacity(0.15) : Color.secondary.opacity(0.1))
            .clipShape(RoundedRectangle(cornerRadius: 12))

            if !message.isUser { Spacer(minLength: 60) }
        }
    }

    // MARK: - Send

    private func sendMessage() {
        let text = prompt.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !text.isEmpty, !isProcessing else { return }

        // Add user message
        messages.append(ChatBubble(role: .user, content: text))
        prompt = ""
        isProcessing = true
        errorMessage = nil

        Task {
            do {
                let response = try FractaBridge.shared.askAI(prompt: text)
                messages.append(ChatBubble(
                    role: .assistant,
                    content: response.content,
                    model: response.model,
                    tokensUsed: response.tokensUsed
                ))
            } catch {
                errorMessage = error.localizedDescription
            }
            isProcessing = false
        }
    }
}

// MARK: - Chat Bubble Model

private struct ChatBubble: Identifiable {
    let id = UUID()
    let role: AiRole
    let content: String
    var model: String? = nil
    var tokensUsed: UInt32 = 0

    var isUser: Bool { role == .user }
}

// MARK: - Preview

#Preview {
    AiChatView()
        .environmentObject(AppState())
}
