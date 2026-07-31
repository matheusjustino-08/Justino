// Extensions and Themes Marketplace Store UI
export class ExtensionsStore {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
    }

    render() {
        if (!this.container) return;
        this.container.innerHTML = `
            <div class="store-panel">
                <h2>Justino Community Marketplace</h2>
                <div class="store-grid">
                    <div class="store-card">
                        <h3>Dark Studio Theme</h3>
                        <p>Official sleek dark palette for Justino IDE.</p>
                        <button class="btn-install" data-theme="theme.dark_studio">Activate Theme</button>
                    </div>
                    <div class="store-card">
                        <h3>Cyberpunk Neon Theme</h3>
                        <p>Vibrant high-contrast neon theme.</p>
                        <button class="btn-install" data-theme="theme.cyberpunk">Activate Theme</button>
                    </div>
                </div>
            </div>
        `;
    }
}
