// Supabase Configuration (Placeholder URL/Key for the user to fill)
const SUPABASE_URL = 'https://YOUR_SUPABASE_PROJECT.supabase.co';
const SUPABASE_ANON_KEY = 'YOUR_SUPABASE_ANON_KEY';

let supabase = null;

document.addEventListener('DOMContentLoaded', () => {
    // Inicializa o cliente se os valores forem preenchidos
    if (SUPABASE_URL !== 'https://YOUR_SUPABASE_PROJECT.supabase.co') {
        supabase = window.supabase.createClient(SUPABASE_URL, SUPABASE_ANON_KEY);
    }

    const btnGithub = document.getElementById('btn-login-github');
    const btnGoogle = document.getElementById('btn-login-google');
    const formEmail = document.getElementById('email-auth-form');

    if (btnGithub) {
        btnGithub.addEventListener('click', async () => {
            if (!supabase) return alert('Configure a URL e Key do Supabase no auth.js');
            await supabase.auth.signInWithOAuth({ provider: 'github' });
        });
    }

    if (btnGoogle) {
        btnGoogle.addEventListener('click', async () => {
            if (!supabase) return alert('Configure a URL e Key do Supabase no auth.js');
            await supabase.auth.signInWithOAuth({ provider: 'google' });
        });
    }

    if (formEmail) {
        formEmail.addEventListener('submit', async (e) => {
            e.preventDefault();
            const email = document.getElementById('auth-email').value;
            if (!supabase) return alert('Configure a URL e Key do Supabase no auth.js');
            
            const { error } = await supabase.auth.signInWithOtp({ email });
            if (error) alert(error.message);
            else alert('Check your email for the magic link!');
        });
    }
});
