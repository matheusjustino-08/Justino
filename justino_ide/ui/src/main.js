import { OnboardingWizard } from './onboarding/wizard.js';
import { ThemeEngine } from './editor/theme_engine.js';
import { AiChatPanel } from './ai_panel/chat.js';
import { LivePreviewEngine } from './preview/live_view.js';
import { translations } from './i18n/dictionary.js';

document.addEventListener('DOMContentLoaded', () => {
    console.log('Justino Studio IDE (.jucode) - Initialized');

    // Apply default theme
    ThemeEngine.applyTheme('theme.dark_studio');

    // Initialize Onboarding Wizard if first launch
    const wizard = new OnboardingWizard((config) => {
        ThemeEngine.applyTheme(config.theme);
        console.log('Wizard Completed. Active AI Provider:', config.aiProvider);
    });
    document.body.insertAdjacentHTML('beforeend', wizard.render());
    wizard.setupEventListeners();

    // Initialize AI Chat Panel
    const aiPanel = new AiChatPanel('ai-panel-container');
    aiPanel.render();

    // Initialize Live Preview Engine
    const previewEngine = new LivePreviewEngine('preview-iframe');
    previewEngine.updatePreview('window {}', 'window { background: #1e1e2e; }');
});
