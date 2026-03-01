// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import react from '@astrojs/react';
import mermaid from 'astro-mermaid';

// https://astro.build/config
export default defineConfig({
	site: 'https://docs.aurintex.com', // Base URL for documentation deployment
	output: 'static', // Explicit static SSG (avoids adapter; required for Starlight)
	redirects: {
		// Rustdoc is now hosted on GitHub Pages (generated via .github/workflows/rustdoc.yml)
		'/reference/rustdoc/engine/index': 'https://aurintex.github.io/pai-os/rustdoc/engine/index.html',
		'/reference/rustdoc/engine': 'https://aurintex.github.io/pai-os/rustdoc/engine/index.html',

		// SEO: DCO to CLA migration - 301 redirect for old URLs (single entry avoids route collision)
		'/guides/contributing/dco': '/guides/contributing/cla/',
	},
	integrations: [
		react(),
		mermaid(),
		starlight({
			title: 'paiOS Docs',
			customCss: [
				'./src/styles/theme.css',
			],
			components: {
				Search: './src/components/Search.astro',
				Head: './src/components/Head.astro',
			},
			head: [
				{
					tag: 'script',
					attrs: { defer: true },
					content: `(function(){
  // Skip on mobile — Starlight already has a hamburger menu there
  if (window.innerWidth < 800) return;

  var KEY = 'sidebar-collapsed';
  var stored = localStorage.getItem(KEY);

  function collapse() { document.body.classList.add('sidebar-collapsed'); }
  function expand()   { document.body.classList.remove('sidebar-collapsed'); }
  function isCollapsed() { return document.body.classList.contains('sidebar-collapsed'); }

  // Immediately apply stored preference (before paint when possible)
  if (stored === 'true') {
    if (document.body) { collapse(); } else {
      document.addEventListener('DOMContentLoaded', collapse);
    }
  } else if (stored !== 'false') {
    var ready = function() {
      setTimeout(function() {
        if (!isCollapsed()) { collapse(); localStorage.setItem(KEY, 'true'); updateHandle(); }
      }, 5000);
    };
    if (document.readyState === 'loading') {
      document.addEventListener('DOMContentLoaded', ready);
    } else { ready(); }
  }

  var handle;
  function updateHandle() {
    if (!handle) return;
    handle.textContent = isCollapsed() ? '\u203A' : '\u2039';
    handle.setAttribute('aria-label', isCollapsed() ? 'Expand navigation' : 'Collapse navigation');
    handle.setAttribute('title', isCollapsed() ? 'Expand sidebar (Alt+S)' : 'Collapse sidebar (Alt+S)');
  }

  function init() {
    handle = document.createElement('button');
    handle.className = 'sidebar-expand-handle';
    handle.type = 'button';
    updateHandle();
    handle.addEventListener('click', function() {
      if (isCollapsed()) {
        expand(); localStorage.setItem(KEY, 'false');
      } else {
        collapse(); localStorage.setItem(KEY, 'true');
      }
      updateHandle();
    });
    document.body.appendChild(handle);

    document.addEventListener('keydown', function(e) {
      if (e.altKey && e.key === 's') { e.preventDefault(); handle.click(); }
    });
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else { init(); }
})();`,
				},
				{
					tag: 'script',
					attrs: { defer: true },
					content: `(function(){
  var HIDE_DELAY_MS = 1500;
  var timeoutId = 0;
  function getScrollTarget(el) {
    if (!el || el === document) return document.documentElement;
    if (el === document.documentElement) return document.documentElement;
    if (el.id === 'starlight__sidebar') return el;
    if (el.classList && el.classList.contains('sidebar-pane')) return el;
    if (el.classList && el.classList.contains('right-sidebar')) return el;
    return el.parentElement ? getScrollTarget(el.parentElement) : document.documentElement;
  }
  function onScroll(e) {
    var target = getScrollTarget(e.target);
    target.classList.add('scrollbar-visible');
    clearTimeout(timeoutId);
    timeoutId = setTimeout(function(){ target.classList.remove('scrollbar-visible'); }, HIDE_DELAY_MS);
  }
  document.addEventListener('scroll', onScroll, true);
})();
`,
				},
			],
			social: [
				{ icon: 'github', label: 'GitHub', href: 'https://github.com/aurintex/pai-os' },
			],
			sidebar: [
				{
					label: 'Guides',
					items: [
						{ label: 'Guides Overview', link: '/guides/' },
						{ label: 'Getting Started', link: '/guides/getting-started/' },
						{ label: 'Licensing', link: '/guides/licensing/' },
						{ label: 'Roadmap', link: '/roadmap/' },
						{
							label: 'Contributing',
							items: [
								{ label: 'Contributing to paiOS', link: '/guides/contributing/' },
								{ label: 'Contribution Workflow', link: '/guides/contributing/workflow/' },
								{ label: 'Development Standards', link: '/guides/contributing/standards/' },
								{ label: 'Rust Style and Best Practices', link: '/guides/contributing/rust-style/' },
								{ label: 'Documentation Guide', link: '/guides/contributing/documentation/' },
								{ label: 'Documentation Maintenance', link: '/guides/contributing/docs-maintenance/' },
								{ label: 'AI-Assisted Development', link: '/guides/contributing/ai-workflow/' },
								{ label: 'Model Integration Guide', link: '/guides/contributing/model-integration/' },
								{ label: 'Project Roles & Maintainers', link: '/guides/contributing/maintainers/' },
								{ label: 'Contributor License Agreement (CLA)', link: '/guides/contributing/cla/' },
							],
						},
					],
				},
				{
					label: 'Architecture',
					items: [
						{ label: 'Architecture Overview', link: '/architecture/' },
						{ label: 'C4 Architecture', link: '/architecture/c4-architecture/' },
						{ label: 'OS & Infrastructure', link: '/architecture/operating-system/' },
						{
							label: 'Engine Domains',
							items: [
								{ label: 'Domains Overview', link: '/architecture/modules/' },
							{ label: 'Common', link: '/architecture/modules/common/' },
							{ label: 'Core', link: '/architecture/modules/core/' },
							{ label: 'Inference', link: '/architecture/modules/inference/' },
							{ label: 'Audio', link: '/architecture/modules/audio/' },
							{ label: 'Vision', link: '/architecture/modules/vision/' },
							{ label: 'API', link: '/architecture/modules/api/' },
							{ label: 'Peripherals', link: '/architecture/modules/peripherals/' },
							],
						},
						{
							label: 'Cross-Cutting',
							items: [
								{ label: 'Workspace and Build', link: '/architecture/workspace-and-build/' },
								{ label: 'Composition Root (main.rs)', link: '/architecture/composition-root/' },
								{ label: 'Security Architecture', link: '/architecture/security/' },
							],
						},
						{
							label: 'Decision Records (ADRs)',
							items: [
								{ label: 'ADR Index', link: '/architecture/adr/' },
								{ label: 'ADR-001: Licensing Strategy', link: '/architecture/adr/001-licensing-strategy/' },
								{ label: 'ADR-002: OS Building Tool', link: '/architecture/adr/002-os-building-tool/' },
								{ label: 'ADR-003: Update System', link: '/architecture/adr/003-update-system/' },
								{ label: 'ADR-004: Engine Architecture', link: '/architecture/adr/004-system-architecture/' },
								{ label: 'ADR-005: Language Selection (Rust)', link: '/architecture/adr/005-runtime-language-selection/' },
								{ label: 'ADR-006: Extension Architecture', link: '/architecture/adr/006-extension-architecture/' },
								{ label: 'ADR-007: Project Management Strategy', link: '/architecture/adr/007-project-management-strategy/' },
								{ label: 'ADR-008: Workspace Layout', link: '/architecture/adr/008-workspace-architecture/' },
								{ label: 'ADR-009: AI Context Strategy', link: '/architecture/adr/009-ai-context-strategy/' },
							],
						},
					],
				},
				{
					label: 'API',
					items: [
						{ label: 'API Overview', link: '/reference/' },
						{ label: 'gRPC API', link: '/reference/api/' },
					],
				},
				{
					label: 'Reference',
					items: [
						{
							label: 'Rust API (Internal)',
							autogenerate: { directory: 'reference/rust' },
						},
						{
							label: 'Rust API (Standard rustdoc)',
							link: 'https://aurintex.github.io/pai-os/rustdoc/engine/index.html',
							attrs: { target: '_blank', rel: 'noopener' }
						},
					],
				},
				{
					label: 'Glossary',
					link: '/glossary/',
				},
			],
		}),
	],
	vite: {
		define: {
			"process.env.NODE_ENV": JSON.stringify(process.env.NODE_ENV || "development"),
		},
		build: {
			// Mermaid + Excalidraw produce large chunks; avoid chunk size warnings (optional code-split later).
			chunkSizeWarningLimit: 2000,
			rollupOptions: {
				output: {
					// Single React chunk so Excalidraw and app share one instance — avoids "Invalid hook call" / useState null.
					manualChunks: (id) => {
						if (id.includes('node_modules/react/') || id.includes('node_modules/react-dom/')) {
							return 'react-vendor';
						}
					},
				},
			},
		},
		// Single React instance so Excalidraw uses the app's React.
		resolve: {
			dedupe: ['react', 'react-dom'],
		},
		// Pre-bundle React and Excalidraw so one React instance is used in dev and dynamic imports resolve.
		optimizeDeps: {
			include: ['react', 'react-dom', '@excalidraw/excalidraw'],
		},
	},
});
