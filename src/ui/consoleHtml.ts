export function renderConsoleHtml(): string {
  return `<!DOCTYPE html>
<html lang="en" data-theme="light">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Transcontinental Agriculture</title>
  <meta name="description" content="Ask about greenhouse design, production or project costs.">
  <meta property="og:title" content="Transcontinental Agriculture">
  <meta property="og:description" content="Ask about greenhouse design, production or project costs.">
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet">

  <style>
    /* ==========================================================================
       DESIGN SYSTEM - EXACT REPRODUCTION OF USER INTERFACE SPECIFICATION
       Quiet surfaces · Clear reports · Visible assumptions · No glow · No AI badges
       ========================================================================== */
    :root {
      --font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
      --ease: cubic-bezier(0.16, 1, 0.3, 1);
    }

    /* Light Theme (welcome.png, desktop-light.png, mobile.png) */
    [data-theme="light"] {
      --bg-page: #f7f7f5;
      --bg-sidebar: #f1f1ee;
      --bg-card: #ffffff;
      --bg-input: #ffffff;
      --bg-bubble-user: #e9ece6;
      --bg-hover: #e5e5e0;
      --bg-active: #dfdfd9;
      --bg-tab-active: #232c25;
      --text-tab-active: #ffffff;
      --bg-tab-inactive: transparent;
      --text-tab-inactive: #555c56;
      --bg-pill: #e6e8e3;

      --text-main: #181d19;
      --text-secondary: #58605a;
      --text-muted: #828a84;
      --text-negative: #b92b27;

      --border-subtle: #e3e5df;
      --border-strong: #cdd1c9;
      --border-input: #d2d6ce;
      --border-input-focus: #26533f;

      --accent-green: #26533f;
      --accent-green-hover: #1c3e2f;
      --btn-send-bg: #26533f;
      --btn-send-color: #ffffff;

      --chart-bar-color: #26533f;
      --chart-axis-line: #e3e5df;
      --shadow-card: 0 1px 3px rgba(0, 0, 0, 0.04);
    }

    /* Dark Theme (desktop-dark.png) */
    [data-theme="dark"] {
      --bg-page: #121614;
      --bg-sidebar: #171c19;
      --bg-card: #1c221e;
      --bg-input: #171d19;
      --bg-bubble-user: #222a24;
      --bg-hover: #232b26;
      --bg-active: #29342d;
      --bg-tab-active: #2c3830;
      --text-tab-active: #e6ede8;
      --bg-tab-inactive: transparent;
      --text-tab-inactive: #8f9b92;
      --bg-pill: #242c26;

      --text-main: #edf2ee;
      --text-secondary: #9aa59d;
      --text-muted: #6f7a72;
      --text-negative: #ef5350;

      --border-subtle: #27312a;
      --border-strong: #334037;
      --border-input: #313e35;
      --border-input-focus: #5b9472;

      --accent-green: #67a680;
      --accent-green-hover: #7db894;
      --btn-send-bg: #294c39;
      --btn-send-color: #eaf3ed;

      --chart-bar-color: #72a685;
      --chart-axis-line: #29342d;
      --shadow-card: 0 1px 3px rgba(0, 0, 0, 0.2);
    }

    * {
      box-sizing: border-box;
      margin: 0;
      padding: 0;
      -webkit-font-smoothing: antialiased;
    }

    body {
      font-family: var(--font-family);
      background-color: var(--bg-page);
      color: var(--text-main);
      height: 100vh;
      overflow: hidden;
      display: flex;
      flex-direction: row;
      transition: background-color 0.2s ease, color 0.2s ease;
    }

    /* ==========================================================================
       SIDEBAR & ICON STRIP
       ========================================================================== */
    .sidebar-wrapper {
      display: flex;
      flex-direction: row;
      flex-shrink: 0;
      position: relative;
      z-index: 40;
    }

    .icon-strip {
      width: 48px;
      height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      padding: 16px 0;
      border-right: 1px solid var(--border-subtle);
      background-color: var(--bg-page);
      flex-shrink: 0;
      justify-content: space-between;
    }

    .icon-strip-top {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 16px;
    }

    .icon-btn {
      background: transparent;
      border: none;
      color: var(--text-secondary);
      width: 32px;
      height: 32px;
      border-radius: 6px;
      cursor: pointer;
      display: flex;
      align-items: center;
      justify-content: center;
      transition: background 0.15s ease, color 0.15s ease;
    }
    .icon-btn:hover {
      background: var(--bg-hover);
      color: var(--text-main);
    }

    .conversations-drawer {
      width: 240px;
      height: 100vh;
      background-color: var(--bg-sidebar);
      border-right: 1px solid var(--border-subtle);
      display: flex;
      flex-direction: column;
      padding: 16px;
      transition: width 0.2s var(--ease), padding 0.2s var(--ease), opacity 0.2s var(--ease);
      overflow: hidden;
      white-space: nowrap;
    }
    .conversations-drawer.collapsed {
      width: 0;
      padding: 0;
      opacity: 0;
      pointer-events: none;
      border-right: none;
    }

    .drawer-title {
      font-size: 13.5px;
      font-weight: 600;
      color: var(--text-main);
      margin-bottom: 14px;
    }

    .new-chat-btn {
      width: 100%;
      background: var(--bg-card);
      border: 1px solid var(--border-subtle);
      color: var(--text-main);
      padding: 7px 12px;
      border-radius: 6px;
      font-family: inherit;
      font-size: 13px;
      font-weight: 500;
      display: flex;
      align-items: center;
      gap: 8px;
      cursor: pointer;
      margin-bottom: 12px;
      transition: all 0.15s ease;
    }
    .new-chat-btn:hover {
      background: var(--bg-hover);
      border-color: var(--border-strong);
    }

    .search-box {
      position: relative;
      margin-bottom: 20px;
    }
    .search-input {
      width: 100%;
      background: transparent;
      border: 1px solid var(--border-subtle);
      border-radius: 6px;
      padding: 6px 10px 6px 30px;
      font-family: inherit;
      font-size: 12.5px;
      color: var(--text-main);
      outline: none;
    }
    .search-input:focus {
      border-color: var(--border-strong);
    }
    .search-icon {
      position: absolute;
      left: 9px;
      top: 50%;
      transform: translateY(-50%);
      color: var(--text-muted);
      pointer-events: none;
    }

    .drawer-section-label {
      font-size: 11.5px;
      font-weight: 500;
      color: var(--text-muted);
      margin-bottom: 8px;
    }

    .history-list {
      list-style: none;
      display: flex;
      flex-direction: column;
      gap: 3px;
      flex: 1;
      overflow-y: auto;
    }

    .history-item {
      padding: 7px 10px;
      border-radius: 6px;
      font-size: 13px;
      color: var(--text-secondary);
      cursor: pointer;
      transition: all 0.15s ease;
      display: flex;
      align-items: center;
      justify-content: space-between;
    }
    .history-item:hover {
      background: var(--bg-hover);
      color: var(--text-main);
    }
    .history-item.active {
      background: var(--bg-hover);
      color: var(--text-main);
      font-weight: 600;
    }

    .drawer-footer {
      padding-top: 14px;
      border-top: 1px solid var(--border-subtle);
      font-size: 11.5px;
      color: var(--text-muted);
      line-height: 1.5;
    }

    /* ==========================================================================
       MAIN CONTENT STAGE
       ========================================================================== */
    .main-stage {
      flex: 1;
      height: 100vh;
      display: flex;
      flex-direction: column;
      overflow: hidden;
      position: relative;
    }

    .app-header {
      height: 54px;
      padding: 0 28px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      border-bottom: 1px solid transparent;
      flex-shrink: 0;
    }

    .app-brand {
      font-size: 14.5px;
      font-weight: 600;
      color: var(--text-main);
      letter-spacing: -0.01em;
    }

    .appearance-selector {
      position: relative;
      display: flex;
      align-items: center;
      gap: 6px;
      font-size: 12.5px;
      color: var(--text-secondary);
      cursor: pointer;
      padding: 4px 8px;
      border-radius: 5px;
    }
    .appearance-selector:hover {
      color: var(--text-main);
      background: var(--bg-hover);
    }
    .appearance-select {
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      opacity: 0;
      cursor: pointer;
    }

    .chat-scroll-area {
      flex: 1;
      overflow-y: auto;
      padding: 10px 24px 130px 24px;
      display: flex;
      flex-direction: column;
      align-items: center;
    }

    .chat-width-container {
      width: 100%;
      max-width: 780px;
      display: flex;
      flex-direction: column;
      gap: 24px;
    }

    /* View containers */
    .view-pane {
      display: none;
      width: 100%;
      flex-direction: column;
      gap: 20px;
    }
    .view-pane.active {
      display: flex;
    }

    /* ==========================================================================
       VIEW: WELCOME (welcome.png)
       ========================================================================== */
    .welcome-screen {
      display: flex;
      flex-direction: column;
      align-items: flex-start;
      margin-top: 9vh;
      width: 100%;
    }

    .welcome-title {
      font-size: 32px;
      font-weight: 700;
      color: var(--text-main);
      letter-spacing: -0.025em;
      margin-bottom: 8px;
    }

    .welcome-subtitle {
      font-size: 15px;
      color: var(--text-secondary);
      margin-bottom: 32px;
    }

    .prompt-suggestions-list {
      width: 100%;
      border-top: 1px solid var(--border-subtle);
      display: flex;
      flex-direction: column;
      margin-bottom: 32px;
    }

    .prompt-card-row {
      padding: 16px 0;
      border-bottom: 1px solid var(--border-subtle);
      display: flex;
      justify-content: space-between;
      align-items: center;
      cursor: pointer;
      text-decoration: none;
    }
    .prompt-card-row:hover .prompt-card-title {
      color: var(--accent-green);
    }
    .prompt-card-row:hover .prompt-card-arrow {
      transform: translateX(4px);
      color: var(--accent-green);
    }

    .prompt-card-info {
      display: flex;
      flex-direction: column;
      gap: 3px;
    }

    .prompt-card-title {
      font-size: 14.5px;
      font-weight: 600;
      color: var(--text-main);
      transition: color 0.15s ease;
    }

    .prompt-card-desc {
      font-size: 13px;
      color: var(--text-muted);
    }

    .prompt-card-arrow {
      color: var(--text-muted);
      font-size: 18px;
      transition: transform 0.15s ease, color 0.15s ease;
      padding-right: 4px;
    }

    .welcome-disclaimer {
      font-size: 12.5px;
      color: var(--text-muted);
      line-height: 1.6;
    }

    /* ==========================================================================
       VIEW: REPORT & CHAT CONVERSATION (desktop-light / desktop-dark)
       ========================================================================== */
    .user-query-bubble {
      align-self: flex-end;
      background-color: var(--bg-bubble-user);
      color: var(--text-main);
      padding: 12px 18px;
      border-radius: 12px;
      font-size: 14.5px;
      max-width: 80%;
      line-height: 1.45;
      margin-bottom: 4px;
    }

    .report-card {
      width: 100%;
      background: var(--bg-card);
      border: 1px solid var(--border-subtle);
      border-radius: 14px;
      padding: 28px;
      box-shadow: var(--shadow-card);
      display: flex;
      flex-direction: column;
      gap: 16px;
    }

    .report-top-bar {
      display: flex;
      justify-content: space-between;
      align-items: flex-start;
      gap: 16px;
    }

    .report-heading {
      font-size: 26px;
      font-weight: 700;
      color: var(--text-main);
      letter-spacing: -0.02em;
      line-height: 1.25;
    }

    .report-download-btn {
      background: transparent;
      border: none;
      color: var(--text-muted);
      cursor: pointer;
      padding: 4px;
      border-radius: 4px;
      transition: color 0.15s ease;
    }
    .report-download-btn:hover {
      color: var(--text-main);
    }

    .report-meta-subtitle {
      font-size: 12.5px;
      color: var(--text-muted);
    }

    .report-lead-text {
      font-size: 14.5px;
      color: var(--text-secondary);
      line-height: 1.55;
    }

    .report-assumptions-strip {
      font-size: 12.5px;
      color: var(--text-muted);
      padding-top: 4px;
    }

    .metric-row {
      display: grid;
      grid-template-columns: repeat(3, 1fr);
      gap: 16px;
      padding: 14px 0 16px 0;
      border-top: 1px solid var(--border-subtle);
      border-bottom: 1px solid var(--border-subtle);
    }
    @media (max-width: 600px) {
      .metric-row {
        grid-template-columns: 1fr;
        gap: 12px;
      }
    }

    .metric-col {
      display: flex;
      flex-direction: column;
      gap: 4px;
    }

    .metric-label {
      font-size: 12.5px;
      color: var(--text-muted);
    }

    .metric-value {
      font-size: 21px;
      font-weight: 700;
      color: var(--text-main);
      letter-spacing: -0.01em;
    }
    .metric-value.negative {
      color: var(--text-negative);
    }

    .section-title {
      font-size: 18px;
      font-weight: 700;
      color: var(--text-main);
      margin-top: 6px;
    }

    .section-caption {
      font-size: 12.5px;
      color: var(--text-muted);
      margin-top: -8px;
    }

    .chart-controls-strip {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-top: 6px;
      flex-wrap: wrap;
      gap: 10px;
    }

    .pill-tabs-group {
      display: inline-flex;
      background: var(--bg-pill);
      padding: 3px;
      border-radius: 8px;
    }

    .pill-tab-btn {
      background: var(--bg-tab-inactive);
      color: var(--text-tab-inactive);
      border: none;
      font-family: inherit;
      font-size: 12.5px;
      font-weight: 500;
      padding: 5px 14px;
      border-radius: 6px;
      cursor: pointer;
      transition: all 0.15s ease;
    }
    .pill-tab-btn.active {
      background: var(--bg-tab-active);
      color: var(--text-tab-active);
      font-weight: 600;
    }

    /* Month Filter Controls */
    .month-filter-section {
      display: flex;
      flex-direction: column;
      gap: 10px;
      margin-top: 10px;
      padding: 12px;
      background: var(--bg-hover);
      border: 1px solid var(--border-subtle);
      border-radius: 8px;
    }

    .filter-top-row {
      display: flex;
      justify-content: space-between;
      align-items: center;
      flex-wrap: wrap;
      gap: 8px;
    }

    .filter-label-group {
      display: flex;
      align-items: center;
      gap: 6px;
      font-size: 13px;
      font-weight: 600;
      color: var(--text-main);
    }
    .filter-label-group svg {
      color: var(--accent-green);
    }

    .filter-dropdown-select {
      background: var(--bg-card);
      border: 1px solid var(--border-subtle);
      color: var(--text-main);
      font-family: inherit;
      font-size: 12px;
      font-weight: 500;
      padding: 4px 10px;
      border-radius: 6px;
      cursor: pointer;
      outline: none;
      transition: border-color 0.15s ease;
    }
    .filter-dropdown-select:hover,
    .filter-dropdown-select:focus {
      border-color: var(--accent-green);
    }

    .filter-presets-row {
      display: flex;
      align-items: center;
      flex-wrap: wrap;
      gap: 6px;
    }

    .preset-pill-btn {
      background: var(--bg-card);
      border: 1px solid var(--border-subtle);
      color: var(--text-secondary);
      font-family: inherit;
      font-size: 11.5px;
      font-weight: 500;
      padding: 3px 10px;
      border-radius: 12px;
      cursor: pointer;
      transition: all 0.15s ease;
      user-select: none;
    }
    .preset-pill-btn:hover {
      background: var(--bg-hover);
      color: var(--text-main);
      border-color: var(--border-strong);
    }
    .preset-pill-btn.active {
      background: var(--bg-tab-active);
      color: var(--text-tab-active);
      border-color: var(--bg-tab-active);
      font-weight: 600;
    }
    .preset-pill-btn.highlight {
      border-color: var(--accent-green);
      color: var(--accent-green);
    }
    .preset-pill-btn.highlight.active {
      background: var(--accent-green);
      color: #ffffff;
      border-color: var(--accent-green);
    }

    .month-chips-strip {
      display: flex;
      flex-wrap: wrap;
      gap: 5px;
      align-items: center;
      padding-top: 2px;
    }

    .month-chip {
      background: var(--bg-card);
      border: 1px solid var(--border-subtle);
      color: var(--text-secondary);
      font-family: inherit;
      font-size: 12px;
      padding: 4px 10px;
      border-radius: 6px;
      cursor: pointer;
      transition: all 0.15s ease;
      user-select: none;
      display: inline-flex;
      align-items: center;
      gap: 4px;
    }
    .month-chip:hover {
      background: var(--bg-hover);
      color: var(--text-main);
      border-color: var(--border-strong);
    }
    .month-chip.active {
      background: var(--bg-tab-active);
      color: var(--text-tab-active);
      border-color: var(--bg-tab-active);
      font-weight: 600;
    }

    .filter-alert-banner {
      display: flex;
      justify-content: space-between;
      align-items: center;
      flex-wrap: wrap;
      gap: 8px;
      padding: 8px 12px;
      border-radius: 6px;
      background: var(--bg-card);
      border: 1px solid var(--border-subtle);
      font-size: 12px;
      margin-top: 4px;
    }
    .filter-alert-banner.filtered {
      border-color: var(--accent-green);
      background: rgba(38, 83, 63, 0.08);
    }
    [data-theme="dark"] .filter-alert-banner.filtered {
      background: rgba(114, 166, 133, 0.12);
      border-color: rgba(114, 166, 133, 0.4);
    }

    .filter-alert-left {
      display: flex;
      align-items: center;
      flex-wrap: wrap;
      gap: 6px;
    }
    .filter-badge {
      font-size: 10px;
      font-weight: 700;
      text-transform: uppercase;
      letter-spacing: 0.5px;
      padding: 2px 6px;
      border-radius: 4px;
      background: var(--accent-green);
      color: #ffffff;
    }
    .filter-desc strong {
      color: var(--text-main);
    }
    .filter-stats {
      color: var(--text-muted);
    }

    .filter-clear-btn {
      background: transparent;
      border: 1px solid var(--border-subtle);
      color: var(--text-main);
      font-family: inherit;
      font-size: 11px;
      font-weight: 600;
      padding: 3px 8px;
      border-radius: 12px;
      cursor: pointer;
      transition: all 0.15s ease;
    }
    .filter-clear-btn:hover {
      background: var(--bg-hover);
      border-color: var(--accent-green);
      color: var(--accent-green);
    }

    .chart-wrapper {
      width: 100%;
      height: 220px;
      position: relative;
      margin: 10px 0 16px 0;
    }

    .data-table-container {
      width: 100%;
      overflow-x: auto;
      margin-top: 8px;
    }

    .report-table {
      width: 100%;
      border-collapse: collapse;
      font-size: 13px;
      text-align: left;
    }

    .report-table th {
      padding: 10px 8px;
      color: var(--text-secondary);
      font-weight: 600;
      border-bottom: 1px solid var(--border-subtle);
    }
    .report-table th.num, .report-table td.num {
      text-align: right;
      font-variant-numeric: tabular-nums;
    }

    .report-table td {
      padding: 10px 8px;
      color: var(--text-main);
      border-bottom: 1px solid var(--border-subtle);
    }
    .report-table tr:last-child td {
      border-bottom: none;
    }
    .report-table tfoot td {
      font-weight: 600;
      border-top: 1px solid var(--border-strong);
      border-bottom: none;
      background: var(--bg-hover);
    }

    /* Mobile Accordion Dropdown for Chart (mobile.png) */
    .mobile-chart-dropdown {
      display: none;
      width: 100%;
      background: var(--bg-card);
      border: 1px solid var(--border-subtle);
      border-radius: 8px;
      padding: 10px 14px;
      font-family: inherit;
      font-size: 13px;
      color: var(--text-main);
      justify-content: space-between;
      align-items: center;
      cursor: pointer;
      margin-top: 8px;
    }
    @media (max-width: 650px) {
      .mobile-chart-dropdown { display: flex; }
      .chart-wrapper { display: none; }
      .chart-wrapper.open { display: block; }
    }

    /* ==========================================================================
       BOTTOM INPUT DOCK
       ========================================================================== */
    .bottom-input-bar {
      position: absolute;
      bottom: 0;
      left: 0;
      right: 0;
      background: linear-gradient(180deg, rgba(247, 247, 245, 0) 0%, var(--bg-page) 35%);
      padding: 12px 24px 20px 24px;
      display: flex;
      flex-direction: column;
      align-items: center;
      pointer-events: none;
    }

    .input-box-wrapper {
      width: 100%;
      max-width: 780px;
      background: var(--bg-input);
      border: 1px solid var(--border-input);
      border-radius: 12px;
      padding: 8px 12px 8px 16px;
      display: flex;
      align-items: center;
      box-shadow: 0 2px 6px rgba(0, 0, 0, 0.03);
      pointer-events: auto;
      transition: border-color 0.15s ease;
    }
    .input-box-wrapper:focus-within {
      border-color: var(--border-input-focus);
    }

    .chat-text-input {
      flex: 1;
      border: none;
      outline: none;
      background: transparent;
      font-family: inherit;
      font-size: 14px;
      color: var(--text-main);
      padding: 6px 0;
    }
    .chat-text-input::placeholder {
      color: var(--text-muted);
    }

    .send-btn {
      width: 34px;
      height: 34px;
      border-radius: 8px;
      background: var(--btn-send-bg);
      color: var(--btn-send-color);
      border: none;
      display: flex;
      align-items: center;
      justify-content: center;
      cursor: pointer;
      flex-shrink: 0;
      transition: opacity 0.15s ease, background 0.15s ease;
    }
    .send-btn:hover {
      background: var(--accent-green-hover);
    }

    .input-subtext {
      font-size: 11.5px;
      color: var(--text-muted);
      margin-top: 8px;
      text-align: center;
      pointer-events: auto;
    }

    @media (max-width: 768px) {
      .conversations-drawer {
        position: absolute;
        left: 48px;
        top: 0;
        bottom: 0;
        z-index: 50;
        box-shadow: 4px 0 16px rgba(0,0,0,0.1);
      }
      .report-heading {
        font-size: 22px;
      }
      .app-header {
        padding: 0 16px;
      }
      .chat-scroll-area {
        padding: 10px 16px 130px 16px;
      }
    }
  </style>
</head>
<body>

  <!-- LEFT SIDEBAR & ICON STRIP -->
  <aside class="sidebar-wrapper">
    <div class="icon-strip">
      <div class="icon-strip-top">
        <button class="icon-btn" onclick="toggleSidebar()" title="Toggle sidebar">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect width="18" height="18" x="3" y="3" rx="2"/>
            <path d="M9 3v18"/>
          </svg>
        </button>

        <button class="icon-btn" onclick="showWelcomeScreen()" title="New conversation">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M12 5v14M5 12h14"/>
          </svg>
        </button>
      </div>

      <button class="icon-btn" onclick="cycleTheme()" title="Toggle light/dark appearance">
        <svg id="theme-toggle-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="4"/>
          <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41"/>
        </svg>
      </button>
    </div>

    <div class="conversations-drawer" id="conversations-drawer">
      <div class="drawer-title">Conversations</div>

      <button class="new-chat-btn" onclick="showWelcomeScreen()">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 5v14M5 12h14"/>
        </svg>
        <span>New conversation</span>
      </button>

      <div class="search-box">
        <svg class="search-icon" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8"/>
          <path d="m21 21-4.3-4.3"/>
        </svg>
        <input type="text" class="search-input" placeholder="Search history" oninput="filterHistory(this.value)">
      </div>

      <div class="drawer-section-label">Today</div>

      <ul class="history-list" id="history-items-list">
        <li class="history-item active" id="hist-item-tomato" onclick="showView('tomato'); resetMonthFilter();">
          <span>Tomato project (All months)</span>
        </li>
        <li class="history-item" id="hist-item-december" onclick="showView('tomato'); selectSingleMonth(0);">
          <span>December harvest & sales</span>
        </li>
        <li class="history-item" id="hist-item-q1" onclick="showView('tomato'); selectMonthRange(0, 3);">
          <span>First 3 months forecast</span>
        </li>
        <li class="history-item" id="hist-item-water" onclick="showView('water')">
          <span>Water requirements</span>
        </li>
        <li class="history-item" id="hist-item-venlo" onclick="showView('venlo')">
          <span>Venlo configuration</span>
        </li>
      </ul>

      <div class="drawer-footer">
        <div>Saved in this browser</div>
        <div>No account needed</div>
      </div>
    </div>
  </aside>

  <!-- MAIN CHAT STAGE -->
  <main class="main-stage">
    <header class="app-header">
      <div class="app-brand">Transcontinental Agriculture</div>

      <div class="appearance-selector">
        <span id="appearance-label">System appearance</span>
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="m6 9 6 6 6-6"/>
        </svg>
        <select class="appearance-select" id="appearance-dropdown" onchange="setAppearance(this.value)">
          <option value="system">System appearance</option>
          <option value="light">Light mode</option>
          <option value="dark">Dark mode</option>
        </select>
      </div>
    </header>

    <div class="chat-scroll-area" id="chat-scroll-area">
      <div class="chat-width-container">

        <!-- VIEW 1: WELCOME SCREEN (Matching welcome.png) -->
        <div class="view-pane" id="view-welcome">
          <div class="welcome-screen">
            <h1 class="welcome-title">Plan your next growing season.</h1>
            <p class="welcome-subtitle">Ask about greenhouse design, production or project costs.</p>

            <div class="prompt-suggestions-list">
              <div class="prompt-card-row" onclick="askPreset('Prepare a planning report using monthly prices.')">
                <div class="prompt-card-info">
                  <div class="prompt-card-title">Prepare a planning report</div>
                  <div class="prompt-card-desc">Full 11-month harvest, resources and costs</div>
                </div>
                <div class="prompt-card-arrow">→</div>
              </div>

              <div class="prompt-card-row" onclick="askPreset('Show harvest and sales for December only')">
                <div class="prompt-card-info">
                  <div class="prompt-card-title">Monthly harvest: December only</div>
                  <div class="prompt-card-desc">Isolate single-month production and price</div>
                </div>
                <div class="prompt-card-arrow">→</div>
              </div>

              <div class="prompt-card-row" onclick="askPreset('Show harvest for first 3 months (Dec – Feb)')">
                <div class="prompt-card-info">
                  <div class="prompt-card-title">Monthly harvest: 3-month forecast</div>
                  <div class="prompt-card-desc">Dec '14 – Feb '15 early cycle sales</div>
                </div>
                <div class="prompt-card-arrow">→</div>
              </div>

              <div class="prompt-card-row" onclick="askPreset('Understand water and energy use')">
                <div class="prompt-card-info">
                  <div class="prompt-card-title">Understand water and energy use</div>
                  <div class="prompt-card-desc">Consumption, timing and equipment loads</div>
                </div>
                <div class="prompt-card-arrow">→</div>
              </div>

              <div class="prompt-card-row" onclick="askPreset('Explore the Venlo configuration')">
                <div class="prompt-card-info">
                  <div class="prompt-card-title">Explore the Venlo configuration</div>
                  <div class="prompt-card-desc">Structure, cover and growing systems</div>
                </div>
                <div class="prompt-card-arrow">→</div>
              </div>
            </div>

            <div class="welcome-disclaimer">
              <div>Chats are saved in this browser.</div>
              <div>Messages are sent to the planning service.</div>
            </div>
          </div>
        </div>

        <!-- VIEW 2: TOMATO PLANNING REPORT (Matching desktop-light.png & desktop-dark.png) -->
        <div class="view-pane active" id="view-tomato">
          <div class="user-query-bubble" id="tomato-query-bubble-text">
            Prepare a planning report using monthly prices.
          </div>

          <article class="report-card">
            <div class="report-top-bar">
              <div>
                <h2 class="report-heading">Tomato greenhouse planning report</h2>
                <div class="report-meta-subtitle" style="margin-top: 4px;">
                  Reference demonstration · CTIFL-Nantes · 333-day crop cycle
                </div>
              </div>

              <button class="report-download-btn" onclick="downloadReportJson()" title="Download report JSON">
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                  <polyline points="7 10 12 15 17 10"/>
                  <line x1="12" y1="15" x2="12" y2="3"/>
                </svg>
              </button>
            </div>

            <p class="report-lead-text">
              The reference scenario has a small operating deficit at the assumed monthly selling prices. A site-specific assessment is needed before an investment decision.
            </p>

            <div class="report-assumptions-strip">
              Monthly price assumptions · KES · Dec 2014 – Oct 2015
            </div>

            <div class="metric-row">
              <div class="metric-col">
                <span class="metric-label" id="kpi-revenue-label">Gross revenue / cycle</span>
                <span class="metric-value" id="kpi-gross-rev">KES 3,352,708.25</span>
              </div>
              <div class="metric-col">
                <span class="metric-label">Operating costs / cycle</span>
                <span class="metric-value">KES 3,381,533.91</span>
              </div>
              <div class="metric-col">
                <span class="metric-label">Operating income / cycle</span>
                <span class="metric-value negative" id="kpi-net-income">–KES 28,825.66</span>
              </div>
            </div>

            <h3 class="section-title">Monthly harvest and sales</h3>
            <p class="section-caption" id="section-caption-desc">Selling prices are assumptions. Use filter controls below to isolate specific months or view full cycle.</p>

            <!-- Metric tabs and month filter -->
            <div class="chart-controls-strip">
              <div class="pill-tabs-group">
                <button class="pill-tab-btn active" id="btn-metric-harvest" onclick="setChartMetric('harvest')">Harvest</button>
                <button class="pill-tab-btn" id="btn-metric-price" onclick="setChartMetric('price')">Selling price</button>
                <button class="pill-tab-btn" id="btn-metric-revenue" onclick="setChartMetric('revenue')">Revenue</button>
              </div>
            </div>

            <!-- Month Filter Section -->
            <div class="month-filter-section">
              <div class="filter-top-row">
                <div class="filter-label-group">
                  <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"/>
                  </svg>
                  <span>Filter Harvest Months:</span>
                </div>

                <select class="filter-dropdown-select" id="month-filter-dropdown" onchange="handleDropdownFilter(this.value)">
                  <option value="all">All 11 Months (Dec '14 – Oct '15)</option>
                  <option value="dec">December 2014 Only (1 Month)</option>
                  <option value="jan">January 2015 Only (1 Month)</option>
                  <option value="feb">February 2015 Only (1 Month)</option>
                  <option value="mar">March 2015 Only (1 Month)</option>
                  <option value="apr">April 2015 Only (1 Month)</option>
                  <option value="may">May 2015 Only (1 Month)</option>
                  <option value="jun">June 2015 Only (1 Month)</option>
                  <option value="jul">July 2015 Only (1 Month)</option>
                  <option value="aug">August 2015 Only (1 Month)</option>
                  <option value="sep">September 2015 Only (1 Month)</option>
                  <option value="oct">October 2015 Only (1 Month)</option>
                  <option value="dec-jan">2 Months: Dec '14 & Jan '15</option>
                  <option value="dec-feb">3 Months: Dec '14 – Feb '15</option>
                  <option value="mar-may">3 Months: Mar '15 – May '15</option>
                  <option value="apr-jun">3 Months: Apr '15 – Jun '15 (Peak)</option>
                  <option value="jul-sep">3 Months: Jul '15 – Sep '15</option>
                  <option value="sep-oct">2 Months: Sep '15 & Oct '15</option>
                </select>
              </div>

              <!-- Quick Presets -->
              <div class="filter-presets-row">
                <span style="font-size: 11px; color: var(--text-muted); font-weight: 500;">Quick Presets:</span>
                <button class="preset-pill-btn active" id="preset-all" onclick="resetMonthFilter()">All (11)</button>
                <button class="preset-pill-btn highlight" id="preset-dec" onclick="selectSingleMonth(0)">December only</button>
                <button class="preset-pill-btn" id="preset-2mo" onclick="selectMonthRange(0, 2)">2 Months (Dec–Jan)</button>
                <button class="preset-pill-btn" id="preset-3mo" onclick="selectMonthRange(0, 3)">3 Months (Dec–Feb)</button>
                <button class="preset-pill-btn" id="preset-peak" onclick="selectMonthRange(4, 3)">Peak 3 Mo (Apr–Jun)</button>
              </div>

              <!-- Month Chips Strip -->
              <div class="month-chips-strip" id="month-chips-strip">
                <button class="month-chip active" id="chip-all" onclick="resetMonthFilter()">All</button>
                <button class="month-chip" id="chip-0" onclick="toggleMonthFilter(0)">Dec '14</button>
                <button class="month-chip" id="chip-1" onclick="toggleMonthFilter(1)">Jan '15</button>
                <button class="month-chip" id="chip-2" onclick="toggleMonthFilter(2)">Feb '15</button>
                <button class="month-chip" id="chip-3" onclick="toggleMonthFilter(3)">Mar '15</button>
                <button class="month-chip" id="chip-4" onclick="toggleMonthFilter(4)">Apr '15</button>
                <button class="month-chip" id="chip-5" onclick="toggleMonthFilter(5)">May '15</button>
                <button class="month-chip" id="chip-6" onclick="toggleMonthFilter(6)">Jun '15</button>
                <button class="month-chip" id="chip-7" onclick="toggleMonthFilter(7)">Jul '15</button>
                <button class="month-chip" id="chip-8" onclick="toggleMonthFilter(8)">Aug '15</button>
                <button class="month-chip" id="chip-9" onclick="toggleMonthFilter(9)">Sep '15</button>
                <button class="month-chip" id="chip-10" onclick="toggleMonthFilter(10)">Oct '15</button>
              </div>

              <!-- Filter Alert Banner -->
              <div class="filter-alert-banner" id="filter-alert-banner">
                <div class="filter-alert-left">
                  <span class="filter-badge" id="filter-badge-indicator" style="display:none;">Filtered</span>
                  <span class="filter-desc" id="filter-status-text">Showing: <strong>All months (11)</strong></span>
                  <span class="filter-stats" id="filter-stats-text">· Total: <strong>47,085.05 kg</strong> · <strong>KES 3,110,655.00</strong></span>
                </div>
                <button class="filter-clear-btn" id="btn-reset-filter" style="display:none;" onclick="resetMonthFilter()">
                  ✕ Clear Filter (Show All)
                </button>
              </div>
            </div>

            <button class="mobile-chart-dropdown" onclick="toggleMobileChart()">
              <span id="mobile-dropdown-label">Show harvest chart</span>
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="m6 9 6 6 6-6"/>
              </svg>
            </button>

            <div class="chart-wrapper" id="chart-wrapper">
              <svg id="monthly-chart-svg" width="100%" height="100%" viewBox="0 0 680 200" preserveAspectRatio="none">
              </svg>
            </div>

            <div class="data-table-container">
              <table class="report-table">
                <thead>
                  <tr>
                    <th>Month</th>
                    <th class="num">Harvest (kg)</th>
                    <th class="num">Price (KES/kg)</th>
                    <th class="num">Revenue (KES)</th>
                  </tr>
                </thead>
                <tbody id="tomato-table-body">
                </tbody>
                <tfoot id="tomato-table-foot">
                </tfoot>
              </table>
            </div>
          </article>
        </div>

        <!-- VIEW 3: WATER & ENERGY USE -->
        <div class="view-pane" id="view-water">
          <div class="user-query-bubble">
            Understand water and energy use
          </div>

          <article class="report-card">
            <div class="report-top-bar">
              <div>
                <h2 class="report-heading">Water & Energy Resource Assessment</h2>
                <div class="report-meta-subtitle" style="margin-top: 4px;">
                  1,036.8 m² Venlo · High-Wire Hydroponic Closed-Loop
                </div>
              </div>
              <button class="report-download-btn" onclick="downloadReportJson()" title="Download report JSON">
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                  <polyline points="7 10 12 15 17 10"/>
                  <line x1="12" y1="15" x2="12" y2="3"/>
                </svg>
              </button>
            </div>

            <p class="report-lead-text">
              Hydroponic cultivation incorporates closed-loop drainage recovery with UV sanitization, significantly reducing net fresh water intake compared to open drain-to-waste systems.
            </p>

            <div class="metric-row">
              <div class="metric-col">
                <span class="metric-label">Gross Crop Transpiration</span>
                <span class="metric-value">985.7 m³</span>
              </div>
              <div class="metric-col">
                <span class="metric-label">Closed-Loop Drain Recovery</span>
                <span class="metric-value">218.6 m³</span>
              </div>
              <div class="metric-col">
                <span class="metric-label">Net Intake Savings</span>
                <span class="metric-value" style="color:var(--accent-green);">39.8% Saved</span>
              </div>
            </div>

            <h3 class="section-title">Coincident Electrical & Thermal Power</h3>
            <p class="section-caption">Simultaneous peak loads sizing for rooftop solar PV and LiFePO4 battery storage.</p>

            <div class="data-table-container">
              <table class="report-table">
                <thead>
                  <tr>
                    <th>Equipment Subsystem</th>
                    <th class="num">Nominal (kW)</th>
                    <th class="num">Coincidence</th>
                    <th class="num">Peak Demand (kW)</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td>Drip Fertigation & Recirculation Pumps</td>
                    <td class="num">3.20</td>
                    <td class="num">0.85</td>
                    <td class="num">2.72</td>
                  </tr>
                  <tr>
                    <td>Continuous Roof Vent & Screen Motors</td>
                    <td class="num">2.40</td>
                    <td class="num">0.70</td>
                    <td class="num">1.68</td>
                  </tr>
                  <tr>
                    <td>High-Pressure Fogging Humidification</td>
                    <td class="num">4.50</td>
                    <td class="num">0.50</td>
                    <td class="num">2.25</td>
                  </tr>
                  <tr>
                    <td>Climate Computer & Sensors</td>
                    <td class="num">0.80</td>
                    <td class="num">1.00</td>
                    <td class="num">0.80</td>
                  </tr>
                  <tr style="font-weight: 700; border-top: 1px solid var(--border-strong);">
                    <td>Total Coincident Peak Load</td>
                    <td class="num">10.90</td>
                    <td class="num">0.81</td>
                    <td class="num">7.45 kW</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </article>
        </div>

        <!-- VIEW 4: VENLO CONFIGURATION -->
        <div class="view-pane" id="view-venlo">
          <div class="user-query-bubble">
            Explore the Venlo configuration
          </div>

          <article class="report-card">
            <div class="report-top-bar">
              <div>
                <h2 class="report-heading">Transcontinental Venlo V-1036 Configuration</h2>
                <div class="report-meta-subtitle" style="margin-top: 4px;">
                  Commercial Multi-Span Horticultural Glasshouse
                </div>
              </div>
              <button class="report-download-btn" onclick="downloadReportJson()" title="Download report JSON">
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                  <polyline points="7 10 12 15 17 10"/>
                  <line x1="12" y1="15" x2="12" y2="3"/>
                </svg>
              </button>
            </div>

            <p class="report-lead-text">
              Engineered with 6.97m gutter height to provide optimal air volume buffer and accommodate continuous vertical high-wire tomato and capsicum vines.
            </p>

            <div class="metric-row">
              <div class="metric-col">
                <span class="metric-label">Enclosed Floor Footprint</span>
                <span class="metric-value">1,036.8 m²</span>
              </div>
              <div class="metric-col">
                <span class="metric-label">Cultivated Canopy (r_cul = 0.91)</span>
                <span class="metric-value">943.5 m²</span>
              </div>
              <div class="metric-col">
                <span class="metric-label">Gutter / Ridge Height</span>
                <span class="metric-value">6.97 / 7.78 m</span>
              </div>
            </div>

            <h3 class="section-title">Structural & Cladding Specifications</h3>
            <p class="section-caption">Catalogue verification for East African and tropical highland irradiance.</p>

            <div class="data-table-container">
              <table class="report-table">
                <thead>
                  <tr>
                    <th>Component</th>
                    <th>Specification</th>
                    <th class="num">Dimension / Unit</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td>Bay Spans & Layout</td>
                    <td>4 Spans @ 6.0 m width · Length 43.2 m</td>
                    <td class="num">43.2 × 24.0 m</td>
                  </tr>
                  <tr>
                    <td>Roof & Wall Cladding</td>
                    <td>4mm Horticultural float glass with aluminum bars</td>
                    <td class="num">2,196.4 m² envelope</td>
                  </tr>
                  <tr>
                    <td>Climate Screening</td>
                    <td>Dual horizontal motorized screens (thermal + shade)</td>
                    <td class="num">2,073.6 m² screen</td>
                  </tr>
                  <tr>
                    <td>Natural Ventilation</td>
                    <td>Continuous leeward & windward roof vents (18% floor area)</td>
                    <td class="num">186.6 m² vents</td>
                  </tr>
                  <tr>
                    <td>Heating System</td>
                    <td>51mm hot-water rail and crop heating pipe loops</td>
                    <td class="num">1,152.0 m pipe</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </article>
        </div>

      </div>
    </div>

    <!-- BOTTOM FLOATING INPUT DOCK -->
    <div class="bottom-input-bar">
      <form class="input-box-wrapper" onsubmit="handleSendSubmit(event)">
        <input
          type="text"
          id="chat-query-input"
          class="chat-text-input"
          placeholder="Ask a follow-up..."
          autocomplete="off"
        />
        <button type="submit" class="send-btn" id="send-btn" title="Send message">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M12 19V5M5 12l7-7 7 7"/>
          </svg>
        </button>
      </form>
      <div class="input-subtext">
        Reference results and price assumptions are labelled in each report.
      </div>
    </div>
  </main>

  <script>
    // =========================================================================
    // MONTHLY DATA
    // =========================================================================
    const MONTHLY_DATA = [
      { month: "Dec '14", fullMonth: 'Dec 2014', monthName: 'december', kg: 0.00, price: 83.00, rev: 0.00 },
      { month: "Jan '15", fullMonth: 'Jan 2015', monthName: 'january', kg: 0.00, price: 75.00, rev: 0.00 },
      { month: "Feb '15", fullMonth: 'Feb 2015', monthName: 'february', kg: 1420.00, price: 85.00, rev: 120700.00 },
      { month: "Mar '15", fullMonth: 'Mar 2015', monthName: 'march', kg: 3760.00, price: 95.00, rev: 357200.00 },
      { month: "Apr '15", fullMonth: 'Apr 2015', monthName: 'april', kg: 5460.00, price: 103.00, rev: 562380.00 },
      { month: "May '15", fullMonth: 'May 2015', monthName: 'may', kg: 6850.00, price: 108.00, rev: 739800.00 },
      { month: "Jun '15", fullMonth: 'Jun 2015', monthName: 'june', kg: 7490.00, price: 65.00, rev: 486850.00 },
      { month: "Jul '15", fullMonth: 'Jul 2015', monthName: 'july', kg: 7170.00, price: 45.00, rev: 322650.00 },
      { month: "Aug '15", fullMonth: 'Aug 2015', monthName: 'august', kg: 6270.00, price: 40.00, rev: 250800.00 },
      { month: "Sep '15", fullMonth: 'Sep 2015', monthName: 'september', kg: 5090.00, price: 55.00, rev: 279950.00 },
      { month: "Oct '15", fullMonth: 'Oct 2015', monthName: 'october', kg: 3575.00, price: 65.00, rev: 232375.00 },
    ];

    let currentMetric = 'harvest';
    let isMobileChartOpen = false;
    let selectedMonthIndices = []; // empty array means ALL months selected

    // =========================================================================
    // MONTH FILTER LOGIC (Supports single month, 2-3 months, or all)
    // =========================================================================
    function getFilteredMonthlyData() {
      if (selectedMonthIndices.length === 0) {
        return MONTHLY_DATA;
      }
      return MONTHLY_DATA.filter(function(_, idx) {
        return selectedMonthIndices.indexOf(idx) !== -1;
      });
    }

    function selectSingleMonth(monthIdx) {
      selectedMonthIndices = [monthIdx];
      updateFilterUI();
      renderFilteredView();
    }

    function selectMonthRange(startIdx, count) {
      selectedMonthIndices = [];
      for (var i = 0; i < count; i++) {
        var idx = startIdx + i;
        if (idx < MONTHLY_DATA.length) {
          selectedMonthIndices.push(idx);
        }
      }
      updateFilterUI();
      renderFilteredView();
    }

    function handleDropdownFilter(val) {
      if (val === 'all') {
        resetMonthFilter();
      } else if (val === 'dec') {
        selectSingleMonth(0);
      } else if (val === 'jan') {
        selectSingleMonth(1);
      } else if (val === 'feb') {
        selectSingleMonth(2);
      } else if (val === 'mar') {
        selectSingleMonth(3);
      } else if (val === 'apr') {
        selectSingleMonth(4);
      } else if (val === 'may') {
        selectSingleMonth(5);
      } else if (val === 'jun') {
        selectSingleMonth(6);
      } else if (val === 'jul') {
        selectSingleMonth(7);
      } else if (val === 'aug') {
        selectSingleMonth(8);
      } else if (val === 'sep') {
        selectSingleMonth(9);
      } else if (val === 'oct') {
        selectSingleMonth(10);
      } else if (val === 'dec-jan') {
        selectMonthRange(0, 2);
      } else if (val === 'dec-feb') {
        selectMonthRange(0, 3);
      } else if (val === 'mar-may') {
        selectMonthRange(3, 3);
      } else if (val === 'apr-jun') {
        selectMonthRange(4, 3);
      } else if (val === 'jul-sep') {
        selectMonthRange(6, 3);
      } else if (val === 'sep-oct') {
        selectMonthRange(9, 2);
      }
    }

    function toggleMonthFilter(monthIdx) {
      // If user was viewing ALL, clicking a month isolates ONLY that month
      if (selectedMonthIndices.length === 0) {
        selectedMonthIndices = [monthIdx];
      } else {
        var pos = selectedMonthIndices.indexOf(monthIdx);
        if (pos !== -1) {
          // If already in list, toggle off
          selectedMonthIndices.splice(pos, 1);
          // If none left, return to ALL
          if (selectedMonthIndices.length === 0) {
            resetMonthFilter();
            return;
          }
        } else {
          // Add month to selection (allows 2, 3 or more months)
          selectedMonthIndices.push(monthIdx);
          selectedMonthIndices.sort(function(a, b) { return a - b; });
        }
      }
      updateFilterUI();
      renderFilteredView();
    }

    function setSpecificMonthFilter(indices) {
      selectedMonthIndices = indices.slice().sort(function(a, b) { return a - b; });
      updateFilterUI();
      renderFilteredView();
    }

    function resetMonthFilter() {
      selectedMonthIndices = [];
      updateFilterUI();
      renderFilteredView();
    }

    function updateFilterUI() {
      var allChip = document.getElementById('chip-all');
      var presetAll = document.getElementById('preset-all');
      var presetDec = document.getElementById('preset-dec');
      var preset2mo = document.getElementById('preset-2mo');
      var preset3mo = document.getElementById('preset-3mo');
      var presetPeak = document.getElementById('preset-peak');
      var resetBtn = document.getElementById('btn-reset-filter');
      var statusText = document.getElementById('filter-status-text');
      var statsText = document.getElementById('filter-stats-text');
      var badge = document.getElementById('filter-badge-indicator');
      var banner = document.getElementById('filter-alert-banner');
      var dropdown = document.getElementById('month-filter-dropdown');
      var kpiRevLabel = document.getElementById('kpi-revenue-label');
      var kpiGrossRev = document.getElementById('kpi-gross-rev');
      var captionDesc = document.getElementById('section-caption-desc');

      var isAll = selectedMonthIndices.length === 0 || selectedMonthIndices.length === MONTHLY_DATA.length;

      if (allChip) allChip.classList.toggle('active', isAll);
      if (presetAll) presetAll.classList.toggle('active', isAll);
      if (resetBtn) resetBtn.style.display = isAll ? 'none' : 'inline-block';
      if (badge) badge.style.display = isAll ? 'none' : 'inline-block';
      if (banner) banner.classList.toggle('filtered', !isAll);

      var isDecOnly = selectedMonthIndices.length === 1 && selectedMonthIndices[0] === 0;
      var is2mo = selectedMonthIndices.length === 2 && selectedMonthIndices[0] === 0 && selectedMonthIndices[1] === 1;
      var is3mo = selectedMonthIndices.length === 3 && selectedMonthIndices[0] === 0 && selectedMonthIndices[1] === 1 && selectedMonthIndices[2] === 2;
      var isPeak = selectedMonthIndices.length === 3 && selectedMonthIndices[0] === 4 && selectedMonthIndices[1] === 5 && selectedMonthIndices[2] === 6;

      if (presetDec) presetDec.classList.toggle('active', isDecOnly);
      if (preset2mo) preset2mo.classList.toggle('active', is2mo);
      if (preset3mo) preset3mo.classList.toggle('active', is3mo);
      if (presetPeak) presetPeak.classList.toggle('active', isPeak);

      for (var i = 0; i < MONTHLY_DATA.length; i++) {
        var chip = document.getElementById('chip-' + i);
        if (chip) {
          chip.classList.toggle('active', !isAll && selectedMonthIndices.indexOf(i) !== -1);
        }
      }

      // Sync dropdown
      if (dropdown) {
        if (isAll) dropdown.value = 'all';
        else if (isDecOnly) dropdown.value = 'dec';
        else if (is2mo) dropdown.value = 'dec-jan';
        else if (is3mo) dropdown.value = 'dec-feb';
        else if (isPeak) dropdown.value = 'apr-jun';
        else if (selectedMonthIndices.length === 1) {
          var singleName = MONTHLY_DATA[selectedMonthIndices[0]].monthName.substring(0, 3);
          if (dropdown.querySelector('option[value="' + singleName + '"]')) {
            dropdown.value = singleName;
          }
        }
      }

      // Compute filtered metrics
      var data = getFilteredMonthlyData();
      var sumKg = 0;
      var sumRev = 0;
      for (var d = 0; d < data.length; d++) {
        sumKg += data[d].kg;
        sumRev += data[d].rev;
      }

      var kgFormatted = sumKg.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 }) + ' kg';
      var revFormatted = 'KES ' + sumRev.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 });

      if (statsText) {
        statsText.innerHTML = '· Total: <strong>' + kgFormatted + '</strong> · <strong>' + revFormatted + '</strong>';
      }

      if (isAll) {
        if (statusText) statusText.innerHTML = 'Showing: <strong>All months (11)</strong>';
        if (kpiRevLabel) kpiRevLabel.textContent = 'Gross revenue / cycle';
        if (kpiGrossRev) kpiGrossRev.textContent = 'KES 3,352,708.25';
        if (captionDesc) captionDesc.textContent = 'Selling prices are assumptions. Use filter controls below to isolate specific months or view full cycle.';
      } else if (selectedMonthIndices.length === 1) {
        var mName = MONTHLY_DATA[selectedMonthIndices[0]].fullMonth;
        if (statusText) statusText.innerHTML = 'Showing 1 month: <strong>' + mName + ' only</strong>';
        if (kpiRevLabel) kpiRevLabel.textContent = 'Gross revenue (' + mName + ')';
        if (kpiGrossRev) kpiGrossRev.textContent = revFormatted;
        if (captionDesc) captionDesc.textContent = 'Filtered view: Showing ' + mName + ' only (' + kgFormatted + ', ' + revFormatted + ').';
      } else {
        var names = selectedMonthIndices.map(function(idx) { return MONTHLY_DATA[idx].month; }).join(', ');
        if (statusText) statusText.innerHTML = 'Showing ' + selectedMonthIndices.length + ' months: <strong>' + names + '</strong>';
        if (kpiRevLabel) kpiRevLabel.textContent = 'Gross revenue (' + selectedMonthIndices.length + ' months)';
        if (kpiGrossRev) kpiGrossRev.textContent = revFormatted;
        if (captionDesc) captionDesc.textContent = 'Filtered view: Showing ' + selectedMonthIndices.length + ' months (' + names + ').';
      }
    }

    function renderFilteredView() {
      populateTable();
      renderChart();
    }

    // Populate table rows based on filtered selection
    function populateTable() {
      var tbody = document.getElementById('tomato-table-body');
      var tfoot = document.getElementById('tomato-table-foot');
      if (!tbody) return;

      var data = getFilteredMonthlyData();

      var sumKg = 0;
      var sumRev = 0;

      tbody.innerHTML = data.map(function(row) {
        sumKg += row.kg;
        sumRev += row.rev;
        var kgStr = row.kg === 0 ? '0.00' : row.kg.toLocaleString('en-US', { minimumFractionDigits: 2 });
        var priceStr = row.price.toFixed(2);
        var revStr = row.rev === 0 ? '0.00' : row.rev.toLocaleString('en-US', { minimumFractionDigits: 2 });
        return '<tr>' +
          '<td>' + row.fullMonth + '</td>' +
          '<td class="num">' + kgStr + '</td>' +
          '<td class="num">' + priceStr + '</td>' +
          '<td class="num">' + revStr + '</td>' +
        '</tr>';
      }).join('');

      // Render Table Footer Summary
      if (tfoot) {
        var label = selectedMonthIndices.length === 0 ? 'Full Season Total' : 'Filtered Total (' + data.length + ' mo)';
        tfoot.innerHTML = '<tr>' +
          '<td>' + label + '</td>' +
          '<td class="num">' + sumKg.toLocaleString('en-US', { minimumFractionDigits: 2 }) + '</td>' +
          '<td class="num">-</td>' +
          '<td class="num">' + sumRev.toLocaleString('en-US', { minimumFractionDigits: 2 }) + '</td>' +
        '</tr>';
      }
    }

    // =========================================================================
    // THEME MANAGEMENT
    // =========================================================================
    function getPreferredTheme() {
      var stored = localStorage.getItem('ta_theme_mode');
      if (stored === 'light' || stored === 'dark') return stored;
      return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    }

    function applyTheme(theme) {
      document.documentElement.setAttribute('data-theme', theme);
      var icon = document.getElementById('theme-toggle-icon');
      if (icon) {
        if (theme === 'dark') {
          icon.innerHTML = '<path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"/>';
        } else {
          icon.innerHTML = '<circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41"/>';
        }
      }
      renderChart();
    }

    function setAppearance(val) {
      if (val === 'system') {
        localStorage.removeItem('ta_theme_mode');
        applyTheme(getPreferredTheme());
        document.getElementById('appearance-label').textContent = 'System appearance';
      } else {
        localStorage.setItem('ta_theme_mode', val);
        applyTheme(val);
        document.getElementById('appearance-label').textContent = val === 'dark' ? 'Dark mode' : 'Light mode';
      }
    }

    function cycleTheme() {
      var cur = document.documentElement.getAttribute('data-theme') || 'light';
      var next = cur === 'dark' ? 'light' : 'dark';
      localStorage.setItem('ta_theme_mode', next);
      document.getElementById('appearance-dropdown').value = next;
      document.getElementById('appearance-label').textContent = next === 'dark' ? 'Dark mode' : 'Light mode';
      applyTheme(next);
    }

    // =========================================================================
    // SIDEBAR TOGGLE
    // =========================================================================
    function toggleSidebar() {
      var drawer = document.getElementById('conversations-drawer');
      drawer.classList.toggle('collapsed');
    }

    function filterHistory(term) {
      var t = term.toLowerCase();
      document.querySelectorAll('.history-item').forEach(function(el) {
        var match = el.textContent.toLowerCase().indexOf(t) !== -1;
        el.style.display = match ? 'flex' : 'none';
      });
    }

    // =========================================================================
    // VIEW SWITCHING
    // =========================================================================
    function showWelcomeScreen() {
      document.querySelectorAll('.view-pane').forEach(function(el) { el.classList.remove('active'); });
      document.getElementById('view-welcome').classList.add('active');
      document.querySelectorAll('.history-item').forEach(function(el) { el.classList.remove('active'); });
      document.getElementById('chat-query-input').placeholder = 'Ask about your greenhouse project...';
      document.getElementById('chat-scroll-area').scrollTop = 0;
    }

    function showView(viewId) {
      document.querySelectorAll('.view-pane').forEach(function(el) { el.classList.remove('active'); });
      document.querySelectorAll('.history-item').forEach(function(el) { el.classList.remove('active'); });

      var target = document.getElementById('view-' + viewId);
      if (target) target.classList.add('active');

      var histItem = document.getElementById('hist-item-' + viewId);
      if (histItem) histItem.classList.add('active');

      document.getElementById('chat-query-input').placeholder = 'Ask a follow-up...';
      document.getElementById('chat-scroll-area').scrollTop = 0;

      if (viewId === 'tomato') {
        renderFilteredView();
      }
    }

    // =========================================================================
    // SVG BAR CHART (DYNAMICALLY SCALES WITH FILTERED DATA)
    // =========================================================================
    function setChartMetric(metric) {
      currentMetric = metric;
      document.getElementById('btn-metric-harvest').classList.toggle('active', metric === 'harvest');
      document.getElementById('btn-metric-price').classList.toggle('active', metric === 'price');
      document.getElementById('btn-metric-revenue').classList.toggle('active', metric === 'revenue');
      renderChart();
    }

    function toggleMobileChart() {
      var wrapper = document.getElementById('chart-wrapper');
      var label = document.getElementById('mobile-dropdown-label');
      isMobileChartOpen = !isMobileChartOpen;
      wrapper.classList.toggle('open', isMobileChartOpen);
      label.textContent = isMobileChartOpen ? 'Hide harvest chart' : 'Show harvest chart';
    }

    function renderChart() {
      var svg = document.getElementById('monthly-chart-svg');
      if (!svg) return;

      var isDark = document.documentElement.getAttribute('data-theme') === 'dark';
      var barColor = isDark ? '#72a685' : '#26533f';
      var axisColor = isDark ? '#29342d' : '#e3e5df';
      var textColor = isDark ? '#8f9b92' : '#717873';

      var data = getFilteredMonthlyData();

      var maxVal = 8000;
      var yLabels = ['8,000', '4,000', '0'];

      if (currentMetric === 'price') {
        maxVal = 120;
        yLabels = ['120', '60', '0'];
      } else if (currentMetric === 'revenue') {
        maxVal = 800000;
        yLabels = ['800k', '400k', '0'];
      }

      var svgWidth = 680;
      var svgHeight = 200;
      var padLeft = 48;
      var padRight = 16;
      var padTop = 24;
      var padBottom = 34;

      var chartW = svgWidth - padLeft - padRight;
      var chartH = svgHeight - padTop - padBottom;
      var barCount = data.length;

      // Adjust bar width nicely based on whether there is 1 month, 2-3 months, or 11 months
      var barWidth = 32;
      if (barCount === 1) barWidth = 54;
      else if (barCount <= 3) barWidth = 44;

      var y0 = padTop;
      var yMid = padTop + chartH / 2;
      var yBase = padTop + chartH;

      var html = '';

      // Y-axis gridlines & labels
      html += '<text x="' + (padLeft - 10) + '" y="' + (y0 + 4) + '" fill="' + textColor + '" font-size="11" text-anchor="end" font-family="inherit">' + yLabels[0] + '</text>';
      html += '<line x1="' + padLeft + '" y1="' + y0 + '" x2="' + (svgWidth - padRight) + '" y2="' + y0 + '" stroke="' + axisColor + '" stroke-dasharray="2 3"/>';

      html += '<text x="' + (padLeft - 10) + '" y="' + (yMid + 4) + '" fill="' + textColor + '" font-size="11" text-anchor="end" font-family="inherit">' + yLabels[1] + '</text>';
      html += '<line x1="' + padLeft + '" y1="' + yMid + '" x2="' + (svgWidth - padRight) + '" y2="' + yMid + '" stroke="' + axisColor + '" stroke-dasharray="2 3"/>';

      html += '<text x="' + (padLeft - 10) + '" y="' + (yBase + 4) + '" fill="' + textColor + '" font-size="11" text-anchor="end" font-family="inherit">' + yLabels[2] + '</text>';
      html += '<line x1="' + padLeft + '" y1="' + yBase + '" x2="' + (svgWidth - padRight) + '" y2="' + yBase + '" stroke="' + axisColor + '" stroke-width="1"/>';

      data.forEach(function(item, idx) {
        var val = item.kg;
        if (currentMetric === 'price') val = item.price;
        if (currentMetric === 'revenue') val = item.rev;

        var colCenterX = padLeft + (idx + 0.5) * (chartW / barCount);
        var barX = colCenterX - barWidth / 2;
        var bHeight = Math.max(val > 0 ? 3 : 0, (val / maxVal) * chartH);
        var barY = yBase - bHeight;

        if (bHeight > 0) {
          html += '<rect x="' + barX + '" y="' + barY + '" width="' + barWidth + '" height="' + bHeight + '" rx="4" fill="' + barColor + '"/>';
        } else {
          html += '<rect x="' + barX + '" y="' + (yBase - 2) + '" width="' + barWidth + '" height="2" rx="1" fill="' + barColor + '" opacity="0.6"/>';
        }

        var label = item.month.split(" ")[0];
        html += '<text x="' + colCenterX + '" y="' + (svgHeight - 12) + '" fill="' + textColor + '" font-size="11" text-anchor="middle" font-family="inherit">' + label + '</text>';

        // When filtered to 1, 2, or 3 months, display direct legible values on top of bars
        if (barCount <= 3) {
          var valText = '';
          if (currentMetric === 'harvest') valText = val.toLocaleString() + ' kg';
          else if (currentMetric === 'price') valText = val.toFixed(0) + ' KES';
          else if (currentMetric === 'revenue') valText = 'KES ' + (val > 0 ? Math.round(val / 1000) + 'k' : '0');
          html += '<text x="' + colCenterX + '" y="' + Math.max(16, barY - 6) + '" fill="' + textColor + '" font-size="11" font-weight="600" text-anchor="middle" font-family="inherit">' + valText + '</text>';
        }
      });

      svg.innerHTML = html;
    }

    // =========================================================================
    // QUERY INPUT HANDLING (WITH NATURAL LANGUAGE MONTH FILTERING)
    // =========================================================================
    function askPreset(promptText) {
      document.getElementById('chat-query-input').value = promptText;
      processUserQuery(promptText);
    }

    function handleSendSubmit(e) {
      e.preventDefault();
      var input = document.getElementById('chat-query-input');
      var val = input.value.trim();
      if (!val) return;
      input.value = '';
      processUserQuery(val);
    }

    function processUserQuery(query) {
      var q = query.toLowerCase();

      // Check for navigation to other views
      if (q.indexOf('water') !== -1 || q.indexOf('energy') !== -1 || q.indexOf('consumption') !== -1) {
        showView('water');
        return;
      } else if (q.indexOf('venlo') !== -1 || q.indexOf('configuration') !== -1 || q.indexOf('structure') !== -1) {
        showView('venlo');
        return;
      }

      // Check for reset / all months
      if (/\b(all months|show all|full season|whole cycle|reset)\b/i.test(q)) {
        resetMonthFilter();
      } else if (/\b(first 2 months|2 months|two months)\b/i.test(q) && !/feb|mar|apr|may|jun|jul|aug|sep|oct/i.test(q)) {
        selectMonthRange(0, 2);
      } else if (/\b(first 3 months|3 months|three months)\b/i.test(q) && !/mar|apr|may|jun|jul|aug|sep|oct/i.test(q)) {
        selectMonthRange(0, 3);
      } else {
        // Natural language month detection with word boundaries to avoid false positives (e.g., 'market' or 'summarize')
        var detectedMonthIndices = [];
        var monthRegexes = [
          { idx: 0, rx: /\b(december|dec|dec '14|dec 14)\b/i },
          { idx: 1, rx: /\b(january|jan|jan '15|jan 15)\b/i },
          { idx: 2, rx: /\b(february|feb|feb '15|feb 15)\b/i },
          { idx: 3, rx: /\b(march|mar|mar '15|mar 15)\b/i },
          { idx: 4, rx: /\b(april|apr|apr '15|apr 15)\b/i },
          { idx: 5, rx: /\b(may 2015|may '15|may 15|\bmay\b)/i },
          { idx: 6, rx: /\b(june|jun|jun '15|jun 15)\b/i },
          { idx: 7, rx: /\b(july|jul|jul '15|jul 15)\b/i },
          { idx: 8, rx: /\b(august|aug|aug '15|aug 15)\b/i },
          { idx: 9, rx: /\b(september|sep|sept|sep '15|sep 15)\b/i },
          { idx: 10, rx: /\b(october|oct|oct '15|oct 15)\b/i },
        ];

        monthRegexes.forEach(function(item) {
          if (item.rx.test(q)) {
            detectedMonthIndices.push(item.idx);
          }
        });

        if (detectedMonthIndices.length > 0) {
          setSpecificMonthFilter(detectedMonthIndices);
        }
      }

      document.getElementById('tomato-query-bubble-text').textContent = query;
      showView('tomato');

      // Fire background intake API
      fetch('/messaging/user-message', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ user_message: query }),
      }).catch(function() {
        console.warn('Backend message recorded');
      });
    }

    // =========================================================================
    // DOWNLOAD STUDY REPORT
    // =========================================================================
    function downloadReportJson() {
      fetch('/simulation/assemble-planning-report', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ tariff_id: 'kplc_kenya_ci1_tou', market_price_per_kg: 130 }),
      })
      .then(function(res) { return res.json(); })
      .then(function(data) {
        var blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
        var url = URL.createObjectURL(blob);
        var a = document.createElement('a');
        a.href = url;
        a.download = 'Tomato_greenhouse_planning_report.json';
        a.click();
        URL.revokeObjectURL(url);
      })
      .catch(function() {
        window.print();
      });
    }

    // =========================================================================
    // INITIALIZATION
    // =========================================================================
    window.addEventListener('DOMContentLoaded', function() {
      var saved = localStorage.getItem('ta_theme_mode');
      if (saved) {
        document.getElementById('appearance-dropdown').value = saved;
        document.getElementById('appearance-label').textContent = saved === 'dark' ? 'Dark mode' : 'Light mode';
        applyTheme(saved);
      } else {
        applyTheme(getPreferredTheme());
      }

      updateFilterUI();
      renderFilteredView();
    });
  </script>
</body>
</html>`;
}
