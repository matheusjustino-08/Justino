class ScreenManager {
  constructor() {
    this.screens = document.querySelectorAll('.screen');
  }

  showScreen(screenId) {
    this.screens.forEach(s => {
      if (s.id === screenId) {
        s.classList.add('active');
      } else {
        s.classList.remove('active');
      }
    });
  }
}

document.addEventListener('DOMContentLoaded', () => {
  window.app = new ScreenManager();

  // Screen 1: Login Actions
  const btnLogin = document.getElementById('btn-login-github');
  if (btnLogin) {
    btnLogin.addEventListener('click', () => {
      // Simulate OAuth flow
      btnLogin.innerHTML = '<svg width="24" height="24" viewBox="0 0 24 24"><path fill="currentColor" d="M12 2A10 10 0 1 0 22 12A10 10 0 0 0 12 2Zm0 18a8 8 0 1 1 8-8A8 8 0 0 1 12 20Z" opacity=".25"/><path fill="currentColor" d="M12 4a8 8 0 0 1 7.89 6.7 1.53 1.53 0 0 0 1.49 1.3 1.5 1.5 0 0 0 1.48-1.75 11 11 0 0 0-21.72 0A1.5 1.5 0 0 0 2.62 12a1.53 1.53 0 0 0 1.49-1.3A8 8 0 0 1 12 4Z"><animateTransform attributeName="transform" dur="0.75s" repeatCount="indefinite" type="rotate" values="0 12 12;360 12 12"/></path></svg> Authenticating...';
      
      setTimeout(() => {
        window.app.showScreen('screen-setup');
      }, 1500);
    });
  }

  // Screen 2: Setup Actions
  const btnFinishSetup = document.getElementById('btn-finish-setup');
  if (btnFinishSetup) {
    btnFinishSetup.addEventListener('click', () => {
      window.app.showScreen('screen-editor');
      // Later: Initialize Monaco Editor here when Screen 3 is ready
    });
  }

  // Options toggling logic
  document.querySelectorAll('.option-card').forEach(card => {
    card.addEventListener('click', (e) => {
      const grid = e.currentTarget.closest('.option-grid');
      grid.querySelectorAll('.option-card').forEach(c => c.classList.remove('selected'));
      e.currentTarget.classList.add('selected');
    });
  });
});
