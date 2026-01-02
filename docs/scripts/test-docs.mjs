import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const DOCS_ROOT = path.resolve(__dirname, '..');
const DIST_DIR = path.join(DOCS_ROOT, 'dist');

const CRITICAL_PATHS = [
  // Native Starlight Pages
  'reference/rust/index.html',
  'reference/rust/crate/index.html',
  'reference/rust/runtime/index.html',
  
  // Protocol Buffer API Documentation
  'reference/api/index.html',
  
  // Standard rustdoc HTML (Hybrid)
  'reference/rustdoc/html/engine/index.html',
  
  // Note: 'reference/rustdoc/engine/index.html' is a redirect route in astro.config.mjs
  // and does not exist as a physical file, so we don't test for it here
];

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

