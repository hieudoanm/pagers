/* pagers progressive enhancement: theme toggle, mobile navigation, code-copy
   buttons. Everything works without it — this only upgrades the controls. */
(function () {
  'use strict';

  var STORAGE_KEY = 'pagers-theme';

  function applyTheme(theme) {
    document.documentElement.dataset.theme = theme;
    document
      .querySelectorAll('.md')
      .forEach(function (el) {
        el.classList.toggle('md-dark', theme === 'dark');
      });
  }

  function storedTheme() {
    try {
      return localStorage.getItem(STORAGE_KEY);
    } catch (e) {
      return null;
    }
  }

  function initTheme() {
    var saved = storedTheme();
    if (saved === 'light' || saved === 'dark') {
      applyTheme(saved);
    } else if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      // Only override the configured default when the user has no explicit
      // choice persisted and the config asked for the default light theme.
      if (document.documentElement.dataset.theme === 'light') {
        applyTheme('dark');
      }
    }

    document.querySelectorAll('[data-theme-toggle]').forEach(function (btn) {
      btn.addEventListener('click', function () {
        var next = document.documentElement.dataset.theme === 'dark' ? 'light' : 'dark';
        applyTheme(next);
        try {
          localStorage.setItem(STORAGE_KEY, next);
        } catch (e) {}
      });
    });
  }

  function initNav() {
    var toggle = document.querySelector('[data-nav-toggle]');
    var sidebar = document.getElementById('sidebar');
    if (!toggle || !sidebar) return;
    toggle.addEventListener('click', function () {
      var open = sidebar.classList.toggle('open');
      toggle.setAttribute('aria-expanded', String(open));
    });
    sidebar.addEventListener('click', function (event) {
      if (event.target.closest('a')) {
        sidebar.classList.remove('open');
        toggle.setAttribute('aria-expanded', 'false');
      }
    });
  }

  function initCopyButtons() {
    document.querySelectorAll('.markdown pre').forEach(function (pre) {
      if (pre.querySelector('.code-copy')) return;
      var btn = document.createElement('button');
      btn.type = 'button';
      btn.className = 'code-copy';
      btn.textContent = 'Copy';
      btn.addEventListener('click', function () {
        var code = pre.querySelector('code');
        var text = (code || pre).innerText;
        navigator.clipboard
          .writeText(text)
          .then(function () {
            btn.textContent = 'Copied';
            setTimeout(function () {
              btn.textContent = 'Copy';
            }, 1500);
          })
          .catch(function () {
            btn.textContent = 'Failed';
          });
      });
      pre.appendChild(btn);
    });
  }

  document.addEventListener('DOMContentLoaded', function () {
    initTheme();
    initNav();
    initCopyButtons();
  });
})();
