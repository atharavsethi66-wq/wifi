document.addEventListener('DOMContentLoaded', () => {
  const form = document.getElementById('login-form');
  const usernameInput = document.getElementById('username');
  const passwordInput = document.getElementById('password');
  const loginBtn = document.getElementById('login-btn');
  const forgetBtn = document.getElementById('forget-btn');
  const statusText = document.getElementById('status-text');

  // Helper to load credentials on startup
  async function loadCredentials() {
    if (!window.__TAURI__ || !window.__TAURI__.core) {
      return;
    }
    try {
      const { invoke } = window.__TAURI__.core;
      const creds = await invoke('get_saved_credentials');
      if (creds) {
        usernameInput.value = creds.username;
        passwordInput.value = creds.password;
        forgetBtn.style.display = 'block';
      }
    } catch (err) {
      statusText.textContent = `Secure Storage Error: ${err}`;
      statusText.style.color = "#d93025";
    }
  }

  // Initial load
  loadCredentials();

  // Forget credentials action
  forgetBtn.addEventListener('click', async () => {
    if (!window.__TAURI__ || !window.__TAURI__.core) {
      return;
    }
    try {
      const { invoke } = window.__TAURI__.core;
      await invoke('forget_credentials');
      usernameInput.value = '';
      passwordInput.value = '';
      forgetBtn.style.display = 'none';
      statusText.textContent = "Credentials forgotten.";
      statusText.style.color = "#666";
    } catch (err) {
      statusText.textContent = `Error forgetting credentials: ${err}`;
      statusText.style.color = "#d93025";
    }
  });

  form.addEventListener('submit', async (e) => {
    e.preventDefault();

    const username = usernameInput.value.trim();
    const password = passwordInput.value;

    if (!username || !password) {
      statusText.textContent = "Please fill in all fields.";
      statusText.style.color = "#d93025";
      return;
    }

    // Set UI to loading state
    statusText.textContent = "Connecting...";
    statusText.style.color = "#666";
    loginBtn.disabled = true;
    forgetBtn.disabled = true;
    usernameInput.disabled = true;
    passwordInput.disabled = true;

    // Check if running within Tauri
    if (!window.__TAURI__ || !window.__TAURI__.core) {
      statusText.textContent = "Error: Tauri API not found. Must run inside Tauri.";
      statusText.style.color = "#d93025";
      loginBtn.disabled = false;
      forgetBtn.disabled = false;
      usernameInput.disabled = false;
      passwordInput.disabled = false;
      return;
    }

    try {
      const { invoke } = window.__TAURI__.core;
      const result = await invoke('login_wifi', { username, password });
      statusText.textContent = result;
      statusText.style.color = "#188038"; // Green for success
      forgetBtn.style.display = 'block'; // Credentials are saved now
    } catch (err) {
      statusText.textContent = err;
      statusText.style.color = "#d93025"; // Red for failure
    } finally {
      // Restore UI state
      loginBtn.disabled = false;
      forgetBtn.disabled = false;
      usernameInput.disabled = false;
      passwordInput.disabled = false;
    }
  });
});
