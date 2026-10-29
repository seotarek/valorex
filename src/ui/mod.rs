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

  /* --- Window Header (Top Row: Tabs + Drag Spacer + Window Controls) --- */
  .window-header {
    display: flex;
    align-items: stretch;
    height: 38px;
    background-color: var(--bg-base);
    user-select: none;
    -webkit-user-select: none;
    position: relative;
    padding: 0;
    margin: 0;
  }

  .tabs-container {
    display: flex;
    align-items: flex-end;
    height: 100%;
    padding: 0 0 0 6px;
    gap: 3px;
    max-width: calc(100% - 150px);
    overflow-x: auto;
    overflow-y: hidden;
  }

  .tabs-container::-webkit-scrollbar {
    display: none;
  }

  .tab {
    display: flex;
    align-items: center;
    max-width: 220px;
    min-width: 120px;
    height: 34px;
    padding: 0 10px;
    background-color: transparent;
    border-radius: var(--radius) var(--radius) 0 0;
    color: var(--text-secondary);
    font-size: 12px;
    cursor: pointer;
    transition: background-color 0.12s, color 0.12s;
    position: relative;
    border: 1px solid transparent;
    border-bottom: none;
    flex-shrink: 0;
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

  .tab.pinned {
    min-width: 40px;
    max-width: 40px;
    padding: 0 10px;
    justify-content: center;
  }

  .tab.pinned .tab-title, .tab.pinned .tab-close {
    display: none;
  }

  .tab.pinned .tab-icon {
    margin-right: 0;
  }

  .tab.drag-over {
    border-left: 2px solid var(--accent);
    transform: scale(1.01);
  }

  .tab-icon {
    width: 16px;
    height: 16px;
    margin-right: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    color: var(--text-secondary);
  }

  .tab.active .tab-icon {
    color: var(--accent);
  }

  .tab-title {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    font-size: 12px;
  }

  .tab-close {
    width: 18px;
    height: 18px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-left: 6px;
    color: var(--text-secondary);
    opacity: 0;
    transition: opacity 0.12s, background-color 0.12s, color 0.12s;
    flex-shrink: 0;
  }

  .tab:hover .tab-close, .tab.active .tab-close {
    opacity: 1;
  }

  .tab-close:hover {
    background-color: rgba(255, 255, 255, 0.2);
    color: #ffffff;
  }

  body.hide-tab-close .tab-close {
    display: none !important;
  }

  .new-tab-btn {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    transition: background 0.15s, color 0.15s;
    margin-left: 2px;
    margin-bottom: 3px;
    flex-shrink: 0;
  }

  .new-tab-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .tab-drag-spacer {
    flex: 1;
    height: 100%;
    min-width: 20px;
    cursor: default;
  }

  /* --- Window Caption Controls (Google Chrome / Brave CSD) --- */
  .window-controls {
    display: flex;
    align-items: stretch;
    height: 100%;
    margin-left: auto;
    flex-shrink: 0;
  }

  .win-btn {
    width: 46px;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    outline: none;
    color: #9ba1b0;
    cursor: pointer;
    padding: 0;
    margin: 0;
    transition: background-color 0.1s ease, color 0.1s ease;
  }

  .win-btn svg {
    pointer-events: none;
  }

  .win-btn:hover {
    background-color: rgba(255, 255, 255, 0.12);
    color: #ffffff;
  }

  .win-btn:active {
    background-color: rgba(255, 255, 255, 0.22);
  }

  .win-btn.win-close:hover {
    background-color: #e81123 !important;
    color: #ffffff !important;
  }

  .win-btn.win-close:active {
    background-color: #bf0f1d !important;
    color: #ffffff !important;
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

  .icon-btn.ai-btn {
    color: #c084fc;
    transition: color 0.15s, background 0.15s, transform 0.15s;
  }

  .icon-btn.ai-btn:hover {
    background: rgba(168, 85, 247, 0.22);
    color: #f3e8ff;
    transform: scale(1.05);
  }

  .icon-btn.profile-btn {
    color: #93c5fd;
    transition: color 0.15s, background 0.15s, transform 0.15s;
  }

  .icon-btn.profile-btn:hover {
    background: rgba(59, 130, 246, 0.22);
    color: #dbeafe;
    transform: scale(1.05);
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

</style>
</head>
<body>

  <!-- Top Tab Bar & Window Caption Header -->
  <div class="window-header">
    <div class="tabs-container" id="tabs-container">
      <!-- Tabs injected dynamically by JS -->
      <button class="new-tab-btn" id="new-tab-btn" title="New Tab (Ctrl+T)">
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
          <path d="M6 1.5v9M1.5 6h9"/>
        </svg>
      </button>
    </div>
    <div class="tab-drag-spacer" id="tab-drag-spacer"></div>
    <div class="window-controls" id="window-controls">
      <button class="win-btn win-min" id="win-min" title="Minimize">
        <svg width="10" height="1" viewBox="0 0 10 1"><rect width="10" height="1" fill="currentColor"/></svg>
      </button>
      <button class="win-btn win-max" id="win-max" title="Maximize">
        <svg class="icon-max" width="10" height="10" viewBox="0 0 10 10">
          <rect x="0.5" y="0.5" width="9" height="9" fill="none" stroke="currentColor" stroke-width="1"/>
        </svg>
        <svg class="icon-restore" width="10" height="10" viewBox="0 0 10 10" style="display:none;">
          <path d="M2.5 2.5V0.5h7v7h-2M0.5 2.5h7v7h-7z" fill="none" stroke="currentColor" stroke-width="1"/>
        </svg>
      </button>
      <button class="win-btn win-close" id="win-close" title="Close">
        <svg width="10" height="10" viewBox="0 0 10 10">
          <path d="M1 1l8 8M9 1L1 9" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
        </svg>
      </button>
    </div>
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
      <input type="text" class="address-input" id="address-input" placeholder="Search with Google or enter web address" autocomplete="off" spellcheck="false">
      <button class="bookmark-btn" id="bookmark-btn" title="Bookmark this tab">
        <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z"/></svg>
      </button>
    </div>

    <!-- Valorex Shield Button & Badge -->
    <button class="shield-btn" id="shield-btn" title="Valorex Privacy Shield (Click for options)">
      <svg class="shield-icon" viewBox="0 0 24 24" fill="currentColor"><path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm-2 16l-4-4 1.41-1.41L10 14.17l6.59-6.59L18 9l-8 8z"/></svg>
      <span id="shield-count">0</span>
    </button>

    <!-- Gemini AI Assistant Button -->
    <button class="icon-btn ai-btn" id="gemini-btn" title="Gemini AI Copilot (مساعد جمناي الذكي)">
      <svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 2l2.4 7.6L22 12l-7.6 2.4L12 22l-2.4-7.6L2 12l7.6-2.4z"/></svg>
    </button>

    <!-- Google Profile Button -->
    <button class="icon-btn profile-btn" id="profile-btn" title="Google Account & Gmail (حساب جوجل وجيميل)">
      <svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/></svg>
    </button>

    <!-- Menu Button (3 dots) -->
    <button class="icon-btn" id="menu-btn" title="Valorex Menu">
      <svg viewBox="0 0 24 24"><path d="M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z"/></svg>
    </button>
  </div>

<script>
  let tabsState = [
    { id: 1, title: 'New Tab', url: 'valorex://newtab', active: true, pinned: false }
  ];

  function post(msg) {
    const jsonStr = typeof msg === 'string' ? msg : JSON.stringify(msg);
    if (window.ipc && typeof window.ipc.postMessage === 'function') {
      window.ipc.postMessage(jsonStr);
    } else if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.ipc) {
      window.webkit.messageHandlers.ipc.postMessage(jsonStr);
    }
  }

  function updateMaximizeState(isMaximized) {
    const maxBtn = document.getElementById('win-max');
    if (!maxBtn) return;
    const iconMax = maxBtn.querySelector('.icon-max');
    const iconRestore = maxBtn.querySelector('.icon-restore');
    if (iconMax && iconRestore) {
      iconMax.style.display = isMaximized ? 'none' : 'block';
      iconRestore.style.display = isMaximized ? 'block' : 'none';
      maxBtn.title = isMaximized ? 'Restore' : 'Maximize';
    }
  }

  function renderTabs() {
    const tabsContainer = document.getElementById('tabs-container');
    const newBtn = document.getElementById('new-tab-btn');
    if (!tabsContainer || !newBtn) return;
    
    // Remove existing tabs
    const oldTabs = tabsContainer.querySelectorAll('.tab');
    oldTabs.forEach(t => t.remove());

    tabsState.forEach(tab => {
      const tabEl = document.createElement('div');
      tabEl.className = 'tab' + (tab.active ? ' active' : '') + (tab.pinned ? ' pinned' : '');
      tabEl.draggable = true;
      tabEl.dataset.id = tab.id;

      tabEl.onclick = (e) => {
        if (!e.target.closest('.tab-close')) {
          post({ action: 'switch_tab', id: tab.id });
        }
      };

      tabEl.ondblclick = (e) => {
        if (!e.target.closest('.tab-close')) {
          e.preventDefault();
          post({ action: 'maximize_window' });
        }
      };

      tabEl.onauxclick = (e) => {
        if (e.button === 1) {
          e.preventDefault();
          post({ action: 'close_tab', id: tab.id });
        }
      };

      tabEl.oncontextmenu = (e) => {
        e.preventDefault();
        e.stopPropagation();
        post({ action: 'open_tab_context', id: tab.id, x: Math.round(e.clientX), y: Math.round(e.clientY) });
      };

      // Drag and drop reordering
      tabEl.ondragstart = (e) => {
        e.dataTransfer.setData('text/plain', String(tab.id));
        e.dataTransfer.effectAllowed = 'move';
      };
      tabEl.ondragover = (e) => {
        e.preventDefault();
        tabEl.classList.add('drag-over');
      };
      tabEl.ondragleave = () => {
        tabEl.classList.remove('drag-over');
      };
      tabEl.ondrop = (e) => {
        e.preventDefault();
        tabEl.classList.remove('drag-over');
        const fromId = parseInt(e.dataTransfer.getData('text/plain'));
        if (fromId && fromId !== tab.id) {
          post({ action: 'reorder_tabs', from_id: fromId, to_id: tab.id });
        }
      };

      const iconSvg = tab.pinned 
        ? `<svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor"><path d="M16 12V4h1V2H7v2h1v8l-2 2v2h5v6l1 1 1-1v-6h5v-2l-2-2z"/></svg>`
        : `<svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>`;

      tabEl.innerHTML = `
        <div class="tab-icon">${iconSvg}</div>
        <div class="tab-title" title="${tab.title}">${tab.title}</div>
        <div class="tab-close" title="Close Tab (Ctrl+W)">
          <svg width="8" height="8" viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"><path d="M1.5 1.5l7 7M8.5 1.5l-7 7"/></svg>
        </div>
      `;

      tabEl.querySelector('.tab-close').onclick = (e) => {
        e.stopPropagation();
        post({ action: 'close_tab', id: tab.id });
      };

      tabsContainer.insertBefore(tabEl, newBtn);
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
  document.getElementById('home-btn').onclick = () => post({ action: 'navigate', url: 'valorex://newtab' });
  document.getElementById('new-tab-btn').onclick = () => post({ action: 'new_tab', url: 'valorex://newtab' });
  document.getElementById('bookmark-btn').onclick = () => post({ action: 'toggle_bookmark' });

  // Native Menus & Services
  document.getElementById('shield-btn').onclick = () => post({ action: 'open_shield_menu' });
  document.getElementById('gemini-btn').onclick = () => post({ action: 'open_gemini' });
  document.getElementById('profile-btn').onclick = () => post({ action: 'google_login' });
  document.getElementById('menu-btn').onclick = () => post({ action: 'open_menu' });

  // Window Controls
  document.getElementById('win-min').onclick = () => post({ action: 'minimize_window' });
  document.getElementById('win-max').onclick = () => post({ action: 'maximize_window' });
  document.getElementById('win-close').onclick = () => post({ action: 'close_window' });

  // Window Dragging & Maximize on Tab Bar with Threshold Tracking
  let dragStartX = 0, dragStartY = 0, isMouseDown = false;
  const DRAG_THRESHOLD = 5;

  const headerEl = document.querySelector('.window-header');
  const dragSpacer = document.getElementById('tab-drag-spacer');

  dragSpacer.addEventListener('mousedown', (e) => {
    if (e.button === 0) {
      isMouseDown = true;
      dragStartX = e.clientX;
      dragStartY = e.clientY;
    }
  });

  window.addEventListener('mousemove', (e) => {
    if (isMouseDown) {
      const dx = Math.abs(e.clientX - dragStartX);
      const dy = Math.abs(e.clientY - dragStartY);
      if (dx > DRAG_THRESHOLD || dy > DRAG_THRESHOLD) {
        isMouseDown = false;
        post({ action: 'drag_window' });
      }
    }
  });

  window.addEventListener('mouseup', () => {
    isMouseDown = false;
  });

  // Double click anywhere on empty header space or spacer to maximize/restore
  headerEl.addEventListener('dblclick', (e) => {
    if (e.target.closest('button') || e.target.closest('.tab-close') || e.target.closest('input')) return;
    post({ action: 'maximize_window' });
  });

  dragSpacer.addEventListener('contextmenu', (e) => {
    e.preventDefault();
    post({ action: 'open_tab_bar_context', x: Math.round(e.clientX), y: Math.round(e.clientY) });
  });

  // Keyboard navigation within chrome
  window.addEventListener('keydown', (e) => {
    if (e.key === 'F11') {
      e.preventDefault();
      post({ action: 'toggle_fullscreen' });
    } else if (e.ctrlKey && e.key === 'Tab') {
      e.preventDefault();
      if (e.shiftKey) {
        post({ action: 'prev_tab' });
      } else {
        post({ action: 'next_tab' });
      }
    } else if (e.ctrlKey && e.key >= '1' && e.key <= '9') {
      e.preventDefault();
      post({ action: 'switch_tab_index', index: parseInt(e.key) - 1 });
    }
  });

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
    }
    if (state.shields_enabled !== undefined) {
      document.getElementById('shield-btn').classList.toggle('disabled', !state.shields_enabled);
    }
    if (state.is_bookmarked !== undefined) {
      document.getElementById('bookmark-btn').classList.toggle('active', state.is_bookmarked);
    }
    if (state.decorations_enabled !== undefined) {
      const controls = document.getElementById('window-controls');
      if (controls) controls.style.display = (state.decorations_enabled || state.show_window_controls === false) ? 'none' : 'flex';
    }
    if (state.show_window_controls !== undefined) {
      const controls = document.getElementById('window-controls');
      if (controls) controls.style.display = (state.decorations_enabled || !state.show_window_controls) ? 'none' : 'flex';
      const tc = document.getElementById('tabs-container');
      if (tc) tc.style.maxWidth = state.show_window_controls ? 'calc(100% - 150px)' : '100%';
    }
    if (state.show_tab_close_buttons !== undefined) {
      document.body.classList.toggle('hide-tab-close', !state.show_tab_close_buttons);
    }
    if (state.is_maximized !== undefined) {
      updateMaximizeState(state.is_maximized);
    }
    if (state.language !== undefined) {
      const isAr = state.language === 'ar';
      document.documentElement.lang = isAr ? 'ar' : 'en';
      document.documentElement.dir = isAr ? 'rtl' : 'ltr';
      
      const newTabBtn = document.getElementById('new-tab-btn');
      if (newTabBtn) newTabBtn.title = isAr ? 'علامة تبويب جديدة (Ctrl+T)' : 'New Tab (Ctrl+T)';
      const winMin = document.getElementById('win-min');
      if (winMin) winMin.title = isAr ? 'تصغير النافذة' : 'Minimize';
      const winClose = document.getElementById('win-close');
      if (winClose) winClose.title = isAr ? 'إغلاق النافذة' : 'Close';
      const backBtn = document.getElementById('back-btn');
      if (backBtn) backBtn.title = isAr ? 'الرجوع للصفحة السابقة (Alt+Left)' : 'Click to go back (Alt+Left)';
      const forwardBtn = document.getElementById('forward-btn');
      if (forwardBtn) forwardBtn.title = isAr ? 'التقدم للصفحة التالية (Alt+Right)' : 'Click to go forward (Alt+Right)';
      const reloadBtn = document.getElementById('reload-btn');
      if (reloadBtn) reloadBtn.title = isAr ? 'إعادة تحميل الصفحة (Ctrl+R)' : 'Reload this page (Ctrl+R / F5)';
      const homeBtn = document.getElementById('home-btn');
      if (homeBtn) homeBtn.title = isAr ? 'الصفحة الرئيسية' : 'Open Home Page';
      const addressInput = document.getElementById('address-input');
      if (addressInput) addressInput.placeholder = isAr ? 'ابحث باستخدام Google أو أدخل رابط موقع...' : 'Search with Google or enter web address...';
      const bookmarkBtn = document.getElementById('bookmark-btn');
      if (bookmarkBtn) bookmarkBtn.title = isAr ? 'إضافة إلى المفضلة' : 'Bookmark this tab';
      const shieldBtn = document.getElementById('shield-btn');
      if (shieldBtn) shieldBtn.title = isAr ? 'درع Valorex لحجب الإعلانات والمتتبعات' : 'Valorex Privacy Shield';
      const geminiBtn = document.getElementById('gemini-btn');
      if (geminiBtn) geminiBtn.title = isAr ? 'مساعد Gemini الذكي' : 'Gemini AI Copilot';
      const profileBtn = document.getElementById('profile-btn');
      if (profileBtn) profileBtn.title = isAr ? 'حساب Google و Gmail' : 'Google Account & Gmail';
      const menuBtn = document.getElementById('menu-btn');
      if (menuBtn) menuBtn.title = isAr ? 'قائمة Valorex' : 'Valorex Menu';
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
