import fs from 'node:fs';
import path from 'node:path';
import { execSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const PACKAGE_NAME = 'pai-engine';
const CRATE_NAME = 'engine';
// Put rustdoc files in a subdirectory to avoid conflicts with Astro routes
const OUTPUT_DIR = path.resolve(__dirname, '../public/reference/rustdoc/html');

function ensureDir(dir) {
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
}

function isCargoAvailable() {
  try {
    execSync('cargo --version', { stdio: 'pipe' });
    return true;
  } catch {
    return false;
  }
}

function generate() {
  // Check if cargo is available (it won't be in Vercel/CI environments)
  if (!isCargoAvailable()) {
    const isCI = process.env.CI || process.env.VERCEL || process.env.NETLIFY;
    if (isCI) {
      console.log('⚠️  Cargo not available in CI environment. Skipping rustdoc HTML generation.');
      console.log('   Pre-generated docs should be committed to the repository.');
      process.exit(0); // Exit successfully - this is expected in CI
    } else {
      console.error('❌ Cargo is not installed or not in PATH.');
      console.error('   Please install Rust: https://rustup.rs/');
      process.exit(1);
    }
  }

  console.log('📚 Generating standard rustdoc HTML...');

  // Generate rustdoc HTML
  execSync(`cargo doc --no-deps`, {
    stdio: 'inherit',
    cwd: path.resolve(__dirname, '../../engine')
  });

  // Copy to public directory
  const targetDir = OUTPUT_DIR;
  const targetDocDir = path.resolve(__dirname, '../../engine/target/doc');

  if (!fs.existsSync(targetDocDir)) {
    console.error(`❌ Error: rustdoc output not found at ${targetDocDir}`);
    process.exit(1);
  }

  // Find the actual engine documentation folder (could be 'engine', 'pai_engine', etc.)
  const folders = fs.readdirSync(targetDocDir).filter(f =>
    fs.statSync(path.join(targetDocDir, f)).isDirectory() &&
    (f.includes('engine') || f === CRATE_NAME)
  );

  if (folders.length === 0) {
    console.error(`❌ Error: Could not find engine documentation folder in ${targetDocDir}`);
    process.exit(1);
  }

  console.log(`📦 Copying rustdoc HTML to ${targetDir}...`);

  // Clean target directory
  if (fs.existsSync(targetDir)) {
    fs.rmSync(targetDir, { recursive: true });
  }
  ensureDir(targetDir);

  // Copy all base files
  execSync(`cp -r ${targetDocDir}/* ${targetDir}/`, { stdio: 'inherit' });

  // IMPORTANT: Ensure the folder is named 'engine' in our public directory for consistent URLs
  // even if Cargo named it 'pai_engine'
  const actualFolder = folders[0];
  if (actualFolder !== CRATE_NAME) {
    console.log(`rename: ${actualFolder} -> ${CRATE_NAME}`);
    fs.renameSync(path.join(targetDir, actualFolder), path.join(targetDir, CRATE_NAME));
  }

  // Note: The path /reference/rustdoc/engine/index (without .html) will not work
  // because Astro/Starlight doesn't serve files without extensions from public/
  // Users should use /reference/rustdoc/engine/index.html directly
  // All links in the documentation are configured to use the .html extension

  console.log('✅ Standard rustdoc HTML generated!');
  console.log(`💡 Access it at /reference/rustdoc/html/engine/index.html`);
  console.log(`💡 Also works at /reference/rustdoc/engine/index (redirects via Astro route)`);
}

generate();

