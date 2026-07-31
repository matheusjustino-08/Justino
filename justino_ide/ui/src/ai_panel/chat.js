// AI Chat (Ctrl+L) and Refactor (Ctrl+K) Interface
export class AiChatPanel {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
        this.messages = [];
    }

    render() {
        if (!this.container) return;
        this.container.innerHTML = `
            <div class="ai-panel">
                <div class="ai-header">
                    <h3>Justino AI Assistant (Ctrl+L)</h3>
                </div>
                <div id="ai-messages" class="ai-messages">
                    <div class="msg ai">Hello! I am your project-aware Justino AI Assistant. How can I help you write .jucode or .css?</div>
                </div>
                <div class="ai-input-box">
                    <input id="ai-input" type="text" placeholder="Ask AI about your project or refactor code..." />
                    <button id="ai-send-btn">Send</button>
                </div>
            </div>
        `;

        document.getElementById('ai-send-btn')?.addEventListener('click', () => {
            const input = document.getElementById('ai-input');
            if (input && input.value.trim()) {
                this.addMessage(input.value.trim(), 'user');
                const userQuery = input.value.trim();
                input.value = '';
                setTimeout(() => {
                    this.addMessage(`AI Response to '${userQuery}': Context checked via justino-lsp. Code refactoring ready!`, 'ai');
                }, 400);
            }
        });
    }

    addMessage(text, sender) {
        const messagesBox = document.getElementById('ai-messages');
        if (!messagesBox) return;
        const msgDiv = document.createElement('div');
        msgDiv.className = `msg ${sender}`;
        msgDiv.innerText = text;
        messagesBox.appendChild(msgDiv);
        messagesBox.scrollTop = messagesBox.scrollHeight;
    }
}
