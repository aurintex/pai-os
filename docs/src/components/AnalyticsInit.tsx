import { useEffect } from 'react';

/**
 * Runs Vercel Analytics init in the browser only.
 * Used so analytics is bundled (no bare specifier in head) and never runs on the server.
 */
export default function AnalyticsInit() {
	useEffect(() => {
		import('@vercel/analytics').then(({ inject, pageview, computeRoute }) => {
			inject({ framework: 'astro', disableAutoTrack: true });
			pageview({
				route: computeRoute(window.location.pathname, {}),
				path: window.location.pathname,
			});
		});
	}, []);
	return null;
}
