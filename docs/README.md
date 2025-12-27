# PAI Documentation

This directory contains the source for the PAI documentation site, built with [Starlight](https://starlight.astro.build/).

## Development

```bash frame="none"
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

The documentation site will be available at `http://localhost:4321` during development.

## Deployment

The documentation can be deployed to:

- **GitHub Pages**: Recommended for separate hosting from the main website
- **Vercel/Netlify**: Can be integrated with the main website (aurintex.com)

For GitHub Pages deployment, configure the repository settings to deploy from the `docs/` directory.

## Structure

- `src/content/docs/` - Documentation content (Markdown/MDX files)
- `public/` - Static assets (images, favicons, etc.)
- `astro.config.mjs` - Starlight configuration

## Content Organization

- `architecture/` - System design and architecture decisions
- `guides/` - Step-by-step tutorials and guides
- `reference/` - API documentation and technical reference
