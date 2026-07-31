// 3-Step Interactive Onboarding Wizard
export class OnboardingWizard {
    constructor(onComplete) {
        this.step = 1;
        this.selectedTheme = 'theme.dark_studio';
        this.selectedAiProvider = 'Claude';
        this.onComplete = onComplete;
    }

    render() {
        return `
            <div id="onboarding-modal" class="onboarding-overlay">
                <div class="wizard-card">
                    <div id="step-1" class="wizard-step">
                        <h2 class="wizard-title">Step 1: Justino Account Login</h2>
                        <p>Authenticate with your unified Justino account to sync settings and themes.</p>
                        <button id="btn-oauth" class="wizard-btn">Login with Justino Account</button>
                    </div>

                    <div id="step-2" class="wizard-step" style="display: none;">
                        <h2 class="wizard-title">Step 2: Choose Theme</h2>
                        <select id="theme-selector">
                            <option value="theme.dark_studio">Dark Studio</option>
                            <option value="theme.cyberpunk">Cyberpunk Neon</option>
                        </select>
                        <button id="btn-step2-next" class="wizard-btn">Next</button>
                    </div>

                    <div id="step-3" class="wizard-step" style="display: none;">
                        <h2 class="wizard-title">Step 3: Configure AI Assistant</h2>
                        <select id="ai-provider-selector">
                            <option value="Claude">Claude 3.5 Sonnet</option>
                            <option value="OpenAi">GPT-4o</option>
                            <option value="Gemini">Gemini 1.5 Pro</option>
                            <option value="OllamaLocal">Ollama Local (Offline)</option>
                        </select>
                        <button id="btn-wizard-finish" class="wizard-btn">Start Coding in Justino IDE</button>
                    </div>
                </div>
            </div>
        `;
    }

    setupEventListeners() {
        document.getElementById('btn-oauth')?.addEventListener('click', () => {
            document.getElementById('step-1').style.display = 'none';
            document.getElementById('step-2').style.display = 'flex';
        });

        document.getElementById('btn-step2-next')?.addEventListener('click', () => {
            const themeSelect = document.getElementById('theme-selector');
            this.selectedTheme = themeSelect ? themeSelect.value : 'theme.dark_studio';
            document.getElementById('step-2').style.display = 'none';
            document.getElementById('step-3').style.display = 'flex';
        });

        document.getElementById('btn-wizard-finish')?.addEventListener('click', () => {
            const aiSelect = document.getElementById('ai-provider-selector');
            this.selectedAiProvider = aiSelect ? aiSelect.value : 'Claude';
            document.getElementById('onboarding-modal')?.remove();
            if (this.onComplete) {
                this.onComplete({
                    theme: this.selectedTheme,
                    aiProvider: this.selectedAiProvider
                });
            }
        });
    }
}
