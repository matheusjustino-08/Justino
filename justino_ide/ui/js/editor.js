// Configuração e Inicialização do Monaco Editor

require.config({ paths: { 'vs': 'https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.41.0/min/vs' }});

require(['vs/editor/editor.main'], function() {
    // Registra a linguagem Jucode
    monaco.languages.register({ id: 'jucode' });
    monaco.languages.setMonarchTokensProvider('jucode', window.JucodeMonarch);
    
    // Tema Volcanic (Vercel/Linear Style)
    monaco.editor.defineTheme('justino-volcanic', {
        base: 'vs-dark',
        inherit: true,
        rules: [
            { token: 'keyword', foreground: '0066FF', fontStyle: 'bold' },
            { token: 'type', foreground: '00E699' },
            { token: 'identifier', foreground: 'F3F5F7' },
            { token: 'string', foreground: '00E699' },
            { token: 'number', foreground: 'FFB000' },
            { token: 'comment', foreground: '8A94A6', fontStyle: 'italic' }
        ],
        colors: {
            'editor.background': '#0D0F12',
            'editor.lineHighlightBackground': '#16191E',
            'editorLineNumber.foreground': '#8A94A6',
            'editorIndentGuide.background': '#22262e'
        }
    });

    const defaultCode = `import window
import http

fun main() {
    let app = window::create({
        title: "Meu Software",
        stylesheet: "./style.css"
    });
}
`;

    // Atrasar a criação até que a tela do editor esteja visível
    const btnFinishSetup = document.getElementById('btn-finish-setup');
    if (btnFinishSetup) {
        btnFinishSetup.addEventListener('click', () => {
            const container = document.getElementById('monaco-container');
            container.innerHTML = ''; // Limpa o mock
            
            window.editor = monaco.editor.create(container, {
                value: defaultCode,
                language: 'jucode',
                theme: 'justino-volcanic',
                automaticLayout: true,
                minimap: { enabled: false },
                fontSize: 13,
                fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
                renderWhitespace: 'none',
                scrollbar: {
                    verticalScrollbarSize: 10,
                    horizontalScrollbarSize: 10
                }
            });
        });
    }
});
