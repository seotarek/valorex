pub const CHROME_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<style>
  :root {
    --bg-base: #121316;
    --bg-elevated: #1e2025;
    --bg-hover: #2a2d35;
    --bg-active: #343842;
    --accent: #ff5500;
    --accent-glow: rgba(255, 85, 0, 0.25);
    --shield-cyan: #00d2ff;
    --border: #282b33;
    --text-primary: #f0f2f5;
    --text-secondary: #9ba1b0;
    --radius: 8px;
    --font: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  }

  * {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
    user-select: none;
    -webkit-user-select: none;
  }

  body {
    background-color: var(--bg-base);
    color: var(--text-primary);
    font-family: var(--font);
    font-size: 13px;
    overflow: hidden;
    height: 82px;
    display: flex;
    flex-direction: column;
    border-bottom: 1px solid var(--border);
  }

  /* --- Tab Bar --- */
  .tab-bar {
    display: flex;
    align-items: center;
    height: 38px;
    padding: 4px 8px 0 8px;
    background-color: var(--bg-base);
    overflow-x: auto;
    overflow-y: hidden;
    gap: 4px;
  }

  .tab-bar::-webkit-scrollbar {
    display: none;
  }

  .tab {
    display: flex;
    align-items: center;
    max-width: 220px;
    min-width: 120px;
    height: 32px;
    padding: 0 10px;
    background-color: transparent;
    border-radius: var(--radius) var(--radius) 0 0;
    color: var(--text-secondary);
    font-size: 12px;
    cursor: pointer;
    transition: background-color 0.15s, color 0.15s;
    position: relative;
    border: 1px solid transparent;
    border-bottom: none;
  }

  .tab:hover {
    background-color: var(--bg-hover);
    color: var(--text-primary);
  }

  .tab.active {
    background-color: var(--bg-elevated);
    color: var(--text-primary);
    font-weight: 500;
    border-color: var(--border);
  }

  .tab-icon {
    width: 14px;
    height: 14px;
    margin-right: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    opacity: 0.8;
  }

  .tab-title {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .tab-close {
    width: 18px;
    height: 18px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-left: 6px;
    font-size: 14px;
    color: var(--text-secondary);
    opacity: 0;
    transition: opacity 0.15s, background-color 0.15s, color 0.15s;
  }

  .tab:hover .tab-close, .tab.active .tab-close {
    opacity: 1;
  }

  .tab-close:hover {
    background-color: rgba(255, 255, 255, 0.15);
    color: #fff;
  }

  .new-tab-btn {
    width: 28px;
    height: 28px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    transition: background 0.15s, color 0.15s;
    margin-left: 2px;
  }

  .new-tab-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  /* --- Navigation Toolbar --- */
  .nav-toolbar {
    display: flex;
    align-items: center;
    height: 44px;
    padding: 0 10px;
    background-color: var(--bg-elevated);
    gap: 8px;
  }

  .icon-btn {
    width: 32px;
    height: 32px;
    border-radius: 6px;
    background: transparent;
    border: none;
    color: var(--text-primary);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.15s, opacity 0.15s;
  }

  .icon-btn:hover:not(:disabled) {
    background: var(--bg-hover);
  }

  .icon-btn:disabled {
    opacity: 0.35;
    cursor: default;
  }

  .icon-btn svg {
    width: 16px;
    height: 16px;
    fill: currentColor;
  }

  /* --- Address Bar (Omnibox) --- */
  .address-bar-container {
    flex: 1;
    height: 34px;
    background: var(--bg-base);
    border: 1px solid var(--border);
    border-radius: 18px;
    display: flex;
    align-items: center;
    padding: 0 12px;
    position: relative;
    transition: border-color 0.2s, box-shadow 0.2s;
  }

  .address-bar-container:focus-within {
    border-color: var(--shield-cyan);
    box-shadow: 0 0 0 2px rgba(0, 210, 255, 0.2);
  }

  .address-lock {
    width: 14px;
    height: 14px;
    margin-right: 8px;
    color: #10b981;
    display: flex;
    align-items: center;
  }

  .address-input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    color: var(--text-primary);
    font-size: 13px;
    user-select: text;
    -webkit-user-select: text;
  }

  .address-input::placeholder {
    color: var(--text-secondary);
  }

  .bookmark-btn {
    width: 24px;
    height: 24px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    color: var(--text-secondary);
    transition: color 0.15s;
    background: transparent;
    border: none;
  }

  .bookmark-btn.active {
    color: #fbbf24;
  }

  .bookmark-btn:hover {
    color: #fbbf24;
  }

  /* --- Valorex Shield Badge --- */
  .shield-btn {
    height: 32px;
    padding: 0 10px;
    border-radius: 16px;
    background: rgba(0, 210, 255, 0.12);
    border: 1px solid rgba(0, 210, 255, 0.3);
    color: var(--shield-cyan);
    display: flex;
    align-items: center;
    gap: 6px;
    cursor: pointer;
    font-weight: 600;
    font-size: 12px;
    transition: background 0.2s, transform 0.1s;
    position: relative;
  }

  .shield-btn:hover {
    background: rgba(0, 210, 255, 0.2);
    transform: scale(1.02);
  }

  .shield-btn.disabled {
    background: rgba(255, 255, 255, 0.05);
    border-color: var(--border);
    color: var(--text-secondary);
  }

  .shield-icon {
    width: 15px;
    height: 15px;
  }

  /* --- Shield Dropdown Popup --- */
  .shield-popup {
    display: none;
    position: absolute;
    top: 48px;
    right: 50px;
    width: 260px;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 12px;
    box-shadow: 0 12px 30px rgba(0, 0, 0, 0.6);
    padding: 14px;
    z-index: 1000;
  }

  .shield-popup.open {
    display: block;
    animation: fadeIn 0.15s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(-4px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .shield-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
    padding-bottom: 10px;
    border-bottom: 1px solid var(--border);
  }

  .shield-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .toggle-switch {
    position: relative;
    width: 36px;
    height: 20px;
  }

  .toggle-switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    cursor: pointer;
    top: 0; left: 0; right: 0; bottom: 0;
    background-color: var(--bg-hover);
    transition: .2s;
    border-radius: 20px;
  }

  .slider:before {
    position: absolute;
    content: "";
    height: 14px;
    width: 14px;
    left: 3px;
    bottom: 3px;
    background-color: white;
    transition: .2s;
    border-radius: 50%;
  }

  input:checked + .slider {
    background-color: var(--shield-cyan);
  }

  input:checked + .slider:before {
    transform: translateX(16px);
  }

  .shield-stat-row {
    display: flex;
    justify-content: space-between;
    padding: 6px 0;
    color: var(--text-secondary);
    font-size: 12px;
  }

  .shield-stat-value {
    color: var(--text-primary);
    font-weight: bold;
  }

  /* --- Menu Dropdown --- */
  .menu-popup {
    display: none;
    position: absolute;
    top: 48px;
    right: 10px;
    width: 180px;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 10px;
    box-shadow: 0 12px 30px rgba(0,0,0,0.6);
    padding: 6px;
    z-index: 1000;
  }

  .menu-popup.open {
    display: block;
    animation: fadeIn 0.15s ease-out;
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    border-radius: 6px;
    cursor: pointer;
    color: var(--text-primary);
    font-size: 12px;
    transition: background 0.15s;
  }

  .menu-item:hover {
    background: var(--bg-hover);
  }
</style>
</head>
<body>

  <!-- Top Tab Bar -->
  <div class="tab-bar" id="tab-bar">
    <!-- Tabs injected dynamically by JS -->
    <button class="new-tab-btn" id="new-tab-btn" title="Open new tab (Ctrl+T)">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
    </button>
  </div>

  <!-- Navigation Toolbar -->
  <div class="nav-toolbar">
    <!-- Back Button -->
    <button class="icon-btn" id="back-btn" title="Click to go back (Alt+Left)">
      <svg viewBox="0 0 24 24"><path d="M20 11H7.83l5.59-5.59L12 4l-8 8 8 8 1.41-1.41L7.83 13H20v-2z"/></svg>
    </button>
    <!-- Forward Button -->
    <button class="icon-btn" id="forward-btn" title="Click to go forward (Alt+Right)">
      <svg viewBox="0 0 24 24"><path d="M12 4l-1.41 1.41L16.17 11H4v2h12.17l-5.58 5.59L12 20l8-8z"/></svg>
    </button>
    <!-- Reload Button -->
    <button class="icon-btn" id="reload-btn" title="Reload this page (Ctrl+R / F5)">
      <svg viewBox="0 0 24 24"><path d="M17.65 6.35A7.958 7.958 0 0012 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08A5.99 5.99 0 0112 18c-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/></svg>
    </button>
    <!-- Home Button -->
    <button class="icon-btn" id="home-btn" title="Open Home Page">
      <svg viewBox="0 0 24 24"><path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z"/></svg>
    </button>

    <!-- Address Bar (Omnibox) -->
    <div class="address-bar-container">
      <div class="address-lock" title="Connection is secure">
        <svg viewBox="0 0 24 24" width="14" height="14" fill="currentColor"><path d="M18 8h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zm-6 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm3.1-9H8.9V6c0-1.71 1.39-3.1 3.1-3.1 1.71 0 3.1 1.39 3.1 3.1v2z"/></svg>
      </div>
      <input type="text" class="address-input" id="address-input" placeholder="Search with DuckDuckGo or enter web address" autocomplete="off" spellcheck="false">
      <button class="bookmark-btn" id="bookmark-btn" title="Bookmark this tab">
        <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z"/></svg>
      </button>
    </div>

    <!-- Valorex Shield Button & Badge -->
    <button class="shield-btn" id="shield-btn" title="Valorex Privacy Shield">
      <svg class="shield-icon" viewBox="0 0 24 24" fill="currentColor"><path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm-2 16l-4-4 1.41-1.41L10 14.17l6.59-6.59L18 9l-8 8z"/></svg>
      <span id="shield-count">0</span>
    </button>

    <!-- Menu Button -->
    <button class="icon-btn" id="menu-btn" title="Valorex Menu">
      <svg viewBox="0 0 24 24"><path d="M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z"/></svg>
    </button>
  </div>

  <!-- Shield Stats Popup -->
  <div class="shield-popup" id="shield-popup">
    <div class="shield-header">
      <span class="shield-title">Valorex Shields</span>
      <label class="toggle-switch">
        <input type="checkbox" id="shield-toggle" checked>
        <span class="slider"></span>
      </label>
    </div>
    <div class="shield-stat-row">
      <span>Trackers & Ads Blocked:</span>
      <span class="shield-stat-value" id="popup-blocked-count">0</span>
    </div>
    <div class="shield-stat-row">
      <span>Cosmetic Elements Cleaned:</span>
      <span class="shield-stat-value">Active</span>
    </div>
    <div class="shield-stat-row">
      <span>Connection Protection:</span>
      <span class="shield-stat-value" style="color: #10b981;">HTTPS / Secure</span>
    </div>
  </div>

  <!-- Menu Popup -->
  <div class="menu-popup" id="menu-popup">
    <div class="menu-item" onclick="navigateInternal('valorex://newtab')">
      <span>🚀</span> New Tab Page
    </div>
    <div class="menu-item" onclick="navigateInternal('valorex://history')">
      <span>🕒</span> History (Ctrl+H)
    </div>
    <div class="menu-item" onclick="navigateInternal('valorex://bookmarks')">
      <span>⭐</span> Bookmarks (Ctrl+B)
    </div>
    <div class="menu-item" onclick="navigateInternal('valorex://settings')">
      <span>⚙️</span> Settings & Shields
    </div>
  </div>

<script>
  let tabsState = [
    { id: 1, title: 'New Tab', url: 'https://duckduckgo.com', active: true }
  ];

  function post(msg) {
    if (window.ipc) {
      window.ipc.postMessage(JSON.stringify(msg));
    }
  }

  function renderTabs() {
    const tabBar = document.getElementById('tab-bar');
    const newBtn = document.getElementById('new-tab-btn');
    
    // Remove existing tabs
    const oldTabs = tabBar.querySelectorAll('.tab');
    oldTabs.forEach(t => t.remove());

    tabsState.forEach(tab => {
      const tabEl = document.createElement('div');
      tabEl.className = 'tab' + (tab.active ? ' active' : '');
      tabEl.onclick = (e) => {
        if (!e.target.classList.contains('tab-close')) {
          post({ action: 'switch_tab', id: tab.id });
        }
      };

      tabEl.innerHTML = `
        <div class="tab-icon">🌐</div>
        <div class="tab-title" title="${tab.title}">${tab.title}</div>
        <div class="tab-close" title="Close Tab (Ctrl+W)">&times;</div>
      `;

      tabEl.querySelector('.tab-close').onclick = (e) => {
        e.stopPropagation();
        post({ action: 'close_tab', id: tab.id });
      };

      tabBar.insertBefore(tabEl, newBtn);
    });
  }

  // Address Bar Navigation
  const addressInput = document.getElementById('address-input');
  addressInput.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') {
      const raw = addressInput.value.trim();
      if (raw) {
        post({ action: 'navigate', url: raw });
        addressInput.blur();
      }
    }
  });

  // Action Buttons
  document.getElementById('back-btn').onclick = () => post({ action: 'go_back' });
  document.getElementById('forward-btn').onclick = () => post({ action: 'go_forward' });
  document.getElementById('reload-btn').onclick = () => post({ action: 'reload' });
  document.getElementById('home-btn').onclick = () => post({ action: 'navigate', url: 'https://duckduckgo.com' });
  document.getElementById('new-tab-btn').onclick = () => post({ action: 'new_tab', url: 'https://duckduckgo.com' });
  document.getElementById('bookmark-btn').onclick = () => post({ action: 'toggle_bookmark' });

  // Shield Dropdown
  const shieldBtn = document.getElementById('shield-btn');
  const shieldPopup = document.getElementById('shield-popup');
  const shieldToggle = document.getElementById('shield-toggle');
  
  shieldBtn.onclick = (e) => {
    e.stopPropagation();
    shieldPopup.classList.toggle('open');
    menuPopup.classList.remove('open');
  };

  shieldToggle.onchange = () => {
    post({ action: 'toggle_shield', enabled: shieldToggle.checked });
  };

  // Menu Dropdown
  const menuBtn = document.getElementById('menu-btn');
  const menuPopup = document.getElementById('menu-popup');
  menuBtn.onclick = (e) => {
    e.stopPropagation();
    menuPopup.classList.toggle('open');
    shieldPopup.classList.remove('open');
  };

  document.addEventListener('click', () => {
    shieldPopup.classList.remove('open');
    menuPopup.classList.remove('open');
  });

  function navigateInternal(url) {
    post({ action: 'navigate', url });
    menuPopup.classList.remove('open');
  }

  // Public update API called from Rust evaluate_script
  window.valorexUpdate = function(state) {
    if (state.url !== undefined && document.activeElement !== addressInput) {
      addressInput.value = state.url;
    }
    if (state.can_go_back !== undefined) {
      document.getElementById('back-btn').disabled = !state.can_go_back;
    }
    if (state.can_go_forward !== undefined) {
      document.getElementById('forward-btn').disabled = !state.can_go_forward;
    }
    if (state.blocked_count !== undefined) {
      document.getElementById('shield-count').textContent = state.blocked_count;
      document.getElementById('popup-blocked-count').textContent = state.blocked_count;
    }
    if (state.shields_enabled !== undefined) {
      shieldToggle.checked = state.shields_enabled;
      shieldBtn.classList.toggle('disabled', !state.shields_enabled);
    }
    if (state.is_bookmarked !== undefined) {
      document.getElementById('bookmark-btn').classList.toggle('active', state.is_bookmarked);
    }
    if (state.tabs) {
      tabsState = state.tabs;
      renderTabs();
    }
  };

  renderTabs();
</script>
</body>
</html>
"#;
