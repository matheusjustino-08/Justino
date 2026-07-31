// Theme Engine Injector
export class ThemeEngine {
    static applyTheme(themeId) {
        let themeFile = 'dark_theme.css';
        if (themeId === 'theme.cyberpunk') {
            themeFile = 'cyberpunk_theme.css';
        }

        let linkElem = document.getElementById('active-theme-link');
        if (!linkElem) {
            linkElem = document.createElement('link');
            linkElem.id = 'active-theme-link';
            linkElem.rel = 'stylesheet';
            document.head.appendChild(linkElem);
        }
        linkElem.href = `themes/${themeFile}`;
    }
}
