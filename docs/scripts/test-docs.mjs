import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const DOCS_ROOT = path.resolve(__dirname, '..');
const DIST_DIR = path.join(DOCS_ROOT, 'dist');

// Check if we're in a CI environment where rustdoc is not available
const isCI = process.env.CI || process.env.VERCEL || process.env.NETLIFY;

// Core paths that should always exist
const CORE_PATHS = [
  // Native Starlight Pages (always generated)
  'reference/rust/index.html',
  'reference/rust/runtime/index.html',

  // Protocol Buffer API Documentation (generated with fallback parser)
  'reference/api/index.html',
];

// Rustdoc-generated paths (only available when cargo/rustdoc is present)
const RUSTDOC_PATHS = [
  'reference/rust/crate/index.html',
  'reference/rustdoc/html/engine/index.html',
];

// In CI (Vercel/Netlify), rustdoc is skipped, so only check core paths
const CRITICAL_PATHS = isCI ? CORE_PATHS : [...CORE_PATHS, ...RUSTDOC_PATHS];

async function runTests() {
  console.log('🧪 Running Documentation Smoke Tests...');
  let failed = false;

  if (!fs.existsSync(DIST_DIR)) {
    console.error('❌ Error: dist/ directory not found. Run "npm run build" first.');
    process.exit(1);
  }

  for (const relPath of CRITICAL_PATHS) {
    const fullPath = path.join(DIST_DIR, relPath);
    if (fs.existsSync(fullPath)) {
      console.log(`✅ Found: ${relPath}`);
    } else {
      console.error(`❌ Missing: ${relPath}`);
      failed = true;
    }
  }

  if (failed) {
    console.error('\n💥 Documentation tests FAILED!');
    process.exit(1);
  } else {
    console.log('\n✨ All documentation paths are valid!');
  }
}

runTests();

