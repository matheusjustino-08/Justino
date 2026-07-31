document.addEventListener('DOMContentLoaded', () => {
    const termInput = document.getElementById('terminal-input');
    const termOutput = document.getElementById('terminal-output');

    if (termInput && termOutput) {
        termInput.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                const cmd = termInput.value.trim();
                if (cmd) {
                    termInput.value = '';
                    termOutput.style.color = 'var(--c-text-primary)';
                    termOutput.textContent = 'Running: ' + cmd + '...';
                    
                    // Send command to Rust Native Backend via IPC
                    if (window.ipc && window.ipc.postMessage) {
                        if (cmd === 'justino run' || cmd === 'run') {
                            // Extract code from Monaco editor to run
                            const code = window.editor ? window.editor.getValue() : "";
                            window.ipc.postMessage(JSON.stringify({
                                action: "run_code",
                                payload: code
                            }));
                        } else {
                            termOutput.style.color = 'var(--c-text-primary)';
                            termOutput.textContent = `Command not recognized. Try 'justino run'`;
                        }
                    } else {
                        // Web fallback if running in browser
                        setTimeout(() => {
                            termOutput.style.color = 'var(--c-mint)';
                            termOutput.textContent = `[Web Mode] Simulated execution of ${cmd}`;
                        }, 800);
                    }
                }
            }
        });
    }

    // Global listener for Rust IPC responses
    window.receiveIpcResponse = function(action, payload) {
        if (action === 'run_code') {
            termOutput.style.color = payload.startsWith("Error:") ? '#ff4d4d' : 'var(--c-mint)';
            termOutput.textContent = payload;
            
            // Auto scroll console if needed (not implemented yet, but keeping text short for now)
            console.log("Rust Response: ", payload);
        }
    };
});
