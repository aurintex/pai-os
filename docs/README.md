# paiOS Documentation

This directory contains the source for the paiOS documentation site, built with [Starlight](https://starlight.astro.build/).

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
- `scripts/` - Documentation generation scripts
  - `rustdoc-gen.mjs` - Generates Rust API documentation (MDX)
  - `rustdoc-html.mjs` - Generates standard rustdoc HTML
  - `proto-doc-gen.mjs` - Generates Protocol Buffer API documentation (MDX)
- `astro.config.mjs` - Starlight configuration

## Content Organization

- `architecture/` - System design and architecture decisions
- `guides/` - Step-by-step tutorials and guides
- `reference/` - API documentation and technical reference
  - `api.mdx` - gRPC API documentation (generated from `.proto` files)
  - `rust/` - Rust API documentation (generated from Rust source)

## Documentation Generation

The documentation includes automatically generated content from source code:

- **Rust API**: Generated from Rust doc comments using `cargo rustdoc --output-format json`
- **Protocol Buffer API**: Generated from `.proto` files in `engine/proto/`

To regenerate all documentation:

```bash frame="none"
npm run build
```

Or generate individual components:

```bash frame="none"
npm run gen:rustdoc      # Rust MDX documentation
npm run gen:rustdoc:html # Rust HTML documentation
npm run gen:proto        # Protocol Buffer MDX documentation
```
