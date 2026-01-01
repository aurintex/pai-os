// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	site: 'https://docs.aurintex.com', // Base URL for documentation deployment
	integrations: [
		starlight({
			title: 'paiOS Docs',
			customCss: [
				'./src/styles/theme.css',
			],
			components: {
				Search: './src/components/Search.astro',
			},
			head: [
				{
					tag: 'script',
					attrs: { type: 'module' },
					content: `import { inject, pageview, computeRoute } from "@vercel/analytics";
inject({ framework: 'astro', disableAutoTrack: true });
const path = window.location.pathname;
pageview({ route: computeRoute(path, {}), path });`,
				},
			],
			social: [
				{ icon: 'github', label: 'GitHub', href: 'https://github.com/aurintex/pai-os' },
			],
			sidebar: [
				{
					label: 'Guides',
					items: [
						{ label: 'Getting Started', link: '/guides/getting-started/' },
						{
							label: 'Contributing',
							items: [
								{ label: 'Contributing to paiOS', link: '/guides/contributing/' },
								{ label: 'AI-Assisted Development', link: '/guides/contributing/ai-workflow/' },
								{ label: 'Contribution Workflow', link: '/guides/contributing/workflow/' },
								{ label: 'Development Standards', link: '/guides/contributing/standards/' },
								{ label: 'Project Roles & Maintainers', link: '/guides/contributing/maintainers/' },
								{ label: 'Developer Certificate of Origin (DCO)', link: '/guides/contributing/dco/' },
							],
						},
					],
				},
				{
					label: 'Architecture',
					autogenerate: { directory: 'architecture' },
				},
				{
					label: 'Reference',
					autogenerate: { directory: 'reference' },
				},
			],
		}),
	],
});
