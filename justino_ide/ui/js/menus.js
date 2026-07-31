document.addEventListener('DOMContentLoaded', () => {
    // Command Palette Logic
    const cmdOverlay = document.getElementById('command-palette-overlay');
    const cmdInput = document.getElementById('command-input');
    const cmdList = document.getElementById('command-list');

    const commands = [
        { label: 'File: New File', action: () => alert('New File') },
        { label: 'File: Save', action: () => alert('Save') },
        { label: 'View: Toggle Terminal', action: () => alert('Toggle Terminal') },
        { label: 'Editor: Format Document', action: () => alert('Format') },
        { label: 'Justino: Build Executable', action: () => alert('Build .exe') }
    ];

    function toggleCommandPalette() {
        if (cmdOverlay.style.display === 'none') {
            cmdOverlay.style.display = 'flex';
            cmdInput.value = '';
            renderCommands(commands);
            cmdInput.focus();
        } else {
            cmdOverlay.style.display = 'none';
        }
    }

    function renderCommands(filtered) {
        cmdList.innerHTML = '';
        filtered.forEach(cmd => {
            const div = document.createElement('div');
            div.textContent = cmd.label;
            div.style.padding = '8px 20px';
            div.style.cursor = 'pointer';
            div.style.color = 'var(--c-text-primary)';
            div.style.fontSize = '13px';
            
            div.addEventListener('mouseover', () => div.style.backgroundColor = 'var(--c-electric)');
            div.addEventListener('mouseout', () => div.style.backgroundColor = 'transparent');
            
            div.addEventListener('click', () => {
                cmd.action();
                toggleCommandPalette();
            });
            cmdList.appendChild(div);
        });
    }

    cmdInput.addEventListener('input', (e) => {
        const val = e.target.value.toLowerCase();
        const filtered = commands.filter(c => c.label.toLowerCase().includes(val));
        renderCommands(filtered);
    });

    document.addEventListener('keydown', (e) => {
        // Ctrl+Shift+P
        if (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === 'p') {
            e.preventDefault();
            toggleCommandPalette();
        }
        // Escape closes modal
        if (e.key === 'Escape' && cmdOverlay.style.display === 'flex') {
            toggleCommandPalette();
        }
    });
    
    // Close modal on click outside
    cmdOverlay.addEventListener('click', (e) => {
        if (e.target === cmdOverlay) toggleCommandPalette();
    });

    // Menus Dropdown Logic Placeholder
    const menuItems = document.querySelectorAll('.menu-item');
    menuItems.forEach(item => {
        item.addEventListener('click', () => {
            alert(`Menu ${item.dataset.menu} clicked. (Dropdown rendering logic here)`);
        });
    });
});
