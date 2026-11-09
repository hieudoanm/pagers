/* pagers graph page: a dependency-free force-directed canvas graph.
   Reads window.__GRAPH_DATA__ = { nodes: [{id,label,path,url,degree}],
   edges: [{from,to}] } and renders an interactive graph with drag,
   click-for-details and light/dark theming via CSS custom properties. */
(function () {
  'use strict';

  var DATA = window.__GRAPH_DATA__ || { nodes: [], edges: [] };
  var canvas = document.getElementById('graph-canvas');
  var details = document.getElementById('graph-details');
  var STORAGE_KEY = 'pagers-theme';

  var nodes = (DATA.nodes || []).map(function (n, i) {
    var angle = (i / Math.max(DATA.nodes.length, 1)) * Math.PI * 2;
    var radius = 40 + Math.sqrt(i + 1) * 18;
    return {
      id: String(n.id),
      label: n.label || n.id,
      path: n.path || n.id,
      url: n.url || null,
      degree: n.degree || 0,
      x: Math.cos(angle) * radius,
      y: Math.sin(angle) * radius,
      vx: 0,
      vy: 0,
      r: 5 + Math.min(n.degree || 0, 12),
    };
  });
  var byId = {};
  nodes.forEach(function (n) { byId[n.id] = n; });

  var links = (DATA.edges || [])
    .map(function (e) {
      var s = byId[String(e.from)];
      var t = byId[String(e.to)];
      return s && t ? { s: s, t: t } : null;
    })
    .filter(Boolean);

  var width = 0;
  var height = 0;
  var dpr = window.devicePixelRatio || 1;
  var alpha = 1;
  var selected = null;
  var drag = null;
  var hover = null;
  var colors = {};

  function readColors() {
    var cs = getComputedStyle(document.documentElement);
    colors = {
      edge: cs.getPropertyValue('--g-edge').trim(),
      edgeHot: cs.getPropertyValue('--g-edge-hot').trim(),
      node: cs.getPropertyValue('--g-node').trim(),
      ring: cs.getPropertyValue('--g-node-ring').trim(),
      fg: cs.getPropertyValue('--g-fg').trim(),
      muted: cs.getPropertyValue('--g-muted').trim(),
    };
  }

  function resize() {
    var rect = canvas.parentElement.getBoundingClientRect();
    width = rect.width;
    height = rect.height;
    canvas.width = Math.round(width * dpr);
    canvas.height = Math.round(height * dpr);
  }

  function tick() {
    alpha = Math.max(alpha * 0.985, 0.02);
    var cx = width / 2;
    var cy = height / 2;

    // Repulsion between all pairs.
    for (var i = 0; i < nodes.length; i++) {
      var a = nodes[i];
      for (var j = i + 1; j < nodes.length; j++) {
        var b = nodes[j];
        var dx = a.x - b.x;
        var dy = a.y - b.y;
        var d2 = dx * dx + dy * dy + 40;
        var d = Math.sqrt(d2);
        var f = (900 * alpha) / d2;
        var fx = (dx / d) * f;
        var fy = (dy / d) * f;
        a.vx += fx; a.vy += fy;
        b.vx -= fx; b.vy -= fy;
      }
    }

    // Springs along edges.
    links.forEach(function (l) {
      var dx = l.t.x - l.s.x;
      var dy = l.t.y - l.s.y;
      var d = Math.sqrt(dx * dx + dy * dy) + 0.01;
      var rest = 90;
      var f = ((d - rest) / d) * 0.04 * alpha;
      var fx = dx * f;
      var fy = dy * f;
      l.s.vx += fx; l.s.vy += fy;
      l.t.vx -= fx; l.t.vy -= fy;
    });

    // Gravity to centre + damping + integrate.
    nodes.forEach(function (n) {
      if (n === drag) return;
      n.vx += (cx - n.x) * 0.002 * alpha;
      n.vy += (cy - n.y) * 0.002 * alpha;
      n.vx *= 0.82;
      n.vy *= 0.82;
      n.x += n.vx;
      n.y += n.vy;
    });

    draw();
    if (alpha > 0.021 || drag) {
      requestAnimationFrame(tick);
    }
  }

  function draw() {
    var ctx = canvas.getContext('2d');
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.clearRect(0, 0, width, height);

    ctx.lineWidth = 1;
    links.forEach(function (l) {
      var hot = selected && (l.s === selected || l.t === selected);
      var hov = hover && (l.s === hover || l.t === hover);
      ctx.strokeStyle = hot || hov ? colors.edgeHot : colors.edge;
      ctx.globalAlpha = hot || hov ? 0.9 : 0.55;
      ctx.beginPath();
      ctx.moveTo(l.s.x, l.s.y);
      ctx.lineTo(l.t.x, l.t.y);
      ctx.stroke();
    });
    ctx.globalAlpha = 1;

    nodes.forEach(function (n) {
      var active = n === selected || n === hover;
      ctx.beginPath();
      ctx.arc(n.x, n.y, n.r + (active ? 3 : 0), 0, Math.PI * 2);
      ctx.fillStyle = colors.node;
      ctx.fill();
      ctx.lineWidth = 2;
      ctx.strokeStyle = colors.ring;
      ctx.stroke();

      if (active || n.degree > 2) {
        ctx.font = '11px ui-sans-serif, system-ui, sans-serif';
        ctx.fillStyle = colors.muted;
        ctx.textAlign = 'center';
        ctx.fillText(n.label, n.x, n.y - n.r - 6);
      }
    });
  }

  function nodeAt(x, y) {
    for (var i = nodes.length - 1; i >= 0; i--) {
      var n = nodes[i];
      var dx = n.x - x;
      var dy = n.y - y;
      if (dx * dx + dy * dy <= (n.r + 4) * (n.r + 4)) return n;
    }
    return null;
  }

  function eventPos(e) {
    var rect = canvas.getBoundingClientRect();
    return { x: e.clientX - rect.left, y: e.clientY - rect.top };
  }

  canvas.addEventListener('mousedown', function (e) {
    var p = eventPos(e);
    var n = nodeAt(p.x, p.y);
    if (n) {
      drag = n;
      canvas.classList.add('dragging');
      canvas.setPointerCapture(e.pointerId);
    }
  });
  canvas.addEventListener('mousemove', function (e) {
    var p = eventPos(e);
    if (drag) {
      drag.x = p.x;
      drag.y = p.y;
      drag.vx = 0;
      drag.vy = 0;
      alpha = Math.max(alpha, 0.3);
      requestAnimationFrame(tick);
      return;
    }
    var n = nodeAt(p.x, p.y);
    if (n !== hover) {
      hover = n;
      canvas.style.cursor = n ? 'pointer' : 'grab';
    }
  });
  canvas.addEventListener('mouseup', function (e) {
    if (!drag) return;
    var p = eventPos(e);
    var moved = Math.abs(drag.x - p.x) + Math.abs(drag.y - p.y);
    var n = nodeAt(p.x, p.y);
    if (n === drag && moved < 4) {
      select(n);
    }
    drag = null;
    canvas.classList.remove('dragging');
  });
  canvas.addEventListener('mouseleave', function () {
    hover = null;
  });

  function select(n) {
    selected = n === selected ? null : n;
    if (!selected) {
      details.hidden = true;
      return;
    }
    details.hidden = false;
    details.querySelector('[data-detail-name]').textContent = n.label;
    details.querySelector('[data-detail-path]').textContent = n.path;
    details.querySelector('[data-detail-degree]').textContent = n.degree;
    var open = details.querySelector('[data-detail-open]');
    if (n.url) {
      open.href = n.url;
      open.hidden = false;
    } else {
      open.hidden = true;
    }
  }

  document.getElementById('graph-details-close').addEventListener('click', function () {
    selected = null;
    details.hidden = true;
  });

  function applyTheme(theme) {
    document.documentElement.dataset.theme = theme;
    readColors();
  }

  function initTheme() {
    var saved = null;
    try { saved = localStorage.getItem(STORAGE_KEY); } catch (e) {}
    if (saved === 'light' || saved === 'dark') applyTheme(saved);
    else if (window.matchMedia('(prefers-color-scheme: dark)').matches) applyTheme('dark');
    document.querySelectorAll('[data-theme-toggle]').forEach(function (btn) {
      btn.addEventListener('click', function () {
        var next = document.documentElement.dataset.theme === 'dark' ? 'light' : 'dark';
        applyTheme(next);
        try { localStorage.setItem(STORAGE_KEY, next); } catch (e) {}
      });
    });
    new MutationObserver(readColors).observe(document.documentElement, {
      attributes: true,
      attributeFilter: ['data-theme'],
    });
  }

  window.addEventListener('resize', resize);
  resize();
  readColors();
  initTheme();
  if (nodes.length) tick();
})();
