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
                    
                    // Mocking an execution delay
                    setTimeout(() => {
                        termOutput.style.color = 'var(--c-mint)';
                        termOutput.textContent = `[Executed ${cmd} in 1.2ms]`;
                    }, 800);
                }
            }
        });
    }
});
