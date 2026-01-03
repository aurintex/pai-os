import fs from 'node:fs';
import path from 'node:path';
import { execSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const PACKAGE_NAME = 'pai-engine';
const CRATE_NAME = 'engine'; 
const JSON_NAME = 'pai_engine';
const OUTPUT_DIR = path.resolve(__dirname, '../src/content/docs/reference/rust');
const ENGINE_PATH = path.resolve(__dirname, '../../engine');

function ensureDir(dir) {
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
}

function generateJson(target) {
  console.log(`🏗️ Generating INTERNAL Rustdoc JSON for ${target}...`);
  const flag = target === 'lib' ? '--lib' : `--bin ${target}`;
  // Added --document-private-items for full internal documentation
  execSync(`cargo +nightly rustdoc ${flag} -- --output-format json -Z unstable-options --document-private-items`, { 
    stdio: 'inherit', 
    cwd: ENGINE_PATH 
  });
  
  // Cargo converts hyphens to underscores in JSON filenames
  // So 'pai-engine' becomes 'pai_engine.json' in the output
  const normalizedTarget = target.replace(/-/g, '_');
  const expectedJsonName = target === 'lib' ? JSON_NAME : normalizedTarget;
  const src = path.join(ENGINE_PATH, `target/doc/${expectedJsonName}.json`);
  
  // Verify the file exists before renaming
  if (!fs.existsSync(src)) {
    console.error(`❌ Error: Expected JSON file not found at ${src}`);
    console.error(`   Tried to find: ${expectedJsonName}.json`);
    console.error(`   Available files in target/doc/:`);
    const docDir = path.join(ENGINE_PATH, 'target/doc');
    if (fs.existsSync(docDir)) {
      const files = fs.readdirSync(docDir).filter(f => f.endsWith('.json'));
      console.error(`   ${files.join(', ')}`);
    }
    process.exit(1);
  }
  
  const dest = path.join(ENGINE_PATH, `target/doc/${JSON_NAME}_${target.replace(/-/g, '_')}.json`);
  fs.renameSync(src, dest);
  return dest;
}

function processJson(jsonPath, isRuntime = false) {
  if (!fs.existsSync(jsonPath)) {
    console.error(`❌ Error: JSON file not found at ${jsonPath}`);
    throw new Error(`JSON file not found: ${jsonPath}`);
  }

  const data = JSON.parse(fs.readFileSync(jsonPath, 'utf-8'));
  const index = data.index;
  const rootId = data.root;

  const processed = new Set();
  
  function processModule(id, parentPath = []) {
    if (processed.has(id)) return;
    processed.add(id);

    const item = index[id];
    if (!item || !item.inner.module) return;

    const currentPath = item.name === JSON_NAME ? [CRATE_NAME] : [...parentPath, item.name];
    const targetDir = path.join(OUTPUT_DIR, ...(item.name === JSON_NAME ? [] : parentPath));
    ensureDir(targetDir);

    // File naming
    let fileName;
    let pageTitle;
    let sidebarOrder;

    const isRoot = item.inner.module.is_crate;

    if (isRuntime && isRoot) {
      fileName = 'runtime.mdx';
      pageTitle = 'Engine Runtime (CLI)';
      sidebarOrder = 2;
    } else if (isRoot) {
      fileName = 'crate.mdx';
      pageTitle = 'engine Library';
      sidebarOrder = 1;
    } else {
      fileName = `${item.name}.mdx`;
      pageTitle = currentPath.join('::');
      sidebarOrder = parentPath.length + 10;
    }

    const filePath = path.join(targetDir, fileName);

    // Clean up doc comments: 
    // 1. Demote headers (H1 -> H2)
    // 2. Add frame="none" to bash/sh/zsh blocks to avoid "weird dots" (only if not already present)
    let cleanedDocs = item.docs ? item.docs.replace(/^# /gm, '## ') : null;
    if (cleanedDocs) {
      // Only add frame="none" if it's not already present in the code block line
      cleanedDocs = cleanedDocs.replace(/```(bash|sh|zsh|shell|console)([^\n]*)/g, (match, lang, attrs) => {
        if (attrs.includes('frame="none"')) {
          return match; // Already has frame="none", don't modify
        }
        return `\`\`\`${lang}${attrs} frame="none"`;
      });
    }

    let content = `---
title: "${pageTitle}"
description: "Rust API documentation for ${isRuntime && isRoot ? 'Engine Runtime' : pageTitle}"
sidebar:
  order: ${sidebarOrder}
---

${cleanedDocs || 'No description available.'}

`;

    const moduleItems = item.inner.module.items;
    const groups = {
      modules: [], structs: [], enums: [], traits: [],
      functions: [], constants: [], types: []
    };

    for (const itemId of moduleItems) {
      const it = index[itemId];
      if (!it) continue;
      if (it.inner.module) groups.modules.push(it);
      else if (it.inner.struct) groups.structs.push(it);
      else if (it.inner.enum) groups.enums.push(it);
      else if (it.inner.trait) groups.traits.push(it);
      else if (it.inner.function) groups.functions.push(it);
      else if (it.inner.constant) groups.constants.push(it);
      else if (it.inner.typedef) groups.types.push(it);
    }

    const hasContent = Object.values(groups).some(arr => arr.length > 0);
    if (!hasContent && !isRoot) return;

    // Render sections
    if (groups.functions.length > 0) {
      content += `## Functions\n\n`;
      for (const f of groups.functions) {
        const sig = f.inner.function.sig;
        const inputs = sig.inputs ? sig.inputs.map(([name, type]) => {
          const typeStr = formatType(type);
          return name ? `${name}: ${typeStr}` : typeStr;
        }).join(', ') : '';
        // Function output types may be wrapped in different structures
        // Try multiple possible structures
        let outputType = null;
        if (sig.output) {
          // Handle different output type structures:
          // - sig.output.type (direct type property)
          // - sig.output (output is the type itself)
          // - sig.output.type.type (nested type wrapper)
          if (sig.output.type) {
            outputType = sig.output.type;
          } else if (sig.output.resolved_path || sig.output.qualified_path || sig.output.primitive) {
            // Output is the type object itself
            outputType = sig.output;
          } else {
            // Fallback: use the output object itself
            outputType = sig.output;
          }
        }
        const output = outputType ? ` -> ${formatType(outputType)}` : '';
        content += `### \`${f.name}\`\n\n\`\`\`rust\nfn ${f.name}(${inputs})${output}\n\`\`\`\n\n`;
        content += f.docs ? `${f.docs.replace(/^# /gm, '#### ')}\n\n` : '*No documentation available.*\n\n';
        content += `---\n\n`;
      }
    }

    if (groups.structs.length > 0) {
      content += `## Structs\n\n`;
      for (const s of groups.structs) {
        content += `### \`${s.name}\`\n\n`;
        content += s.docs ? `${s.docs.replace(/^# /gm, '#### ')}\n\n` : '*No documentation available.*\n\n';
        let fields = '';
        const kind = s.inner.struct.kind;
        if (kind.plain) {
          const fieldIds = Array.isArray(kind.plain) ? kind.plain : kind.plain.fields;
          fields = fieldIds.map(fid => {
            const field = index[fid];
            if (!field.inner.struct_field) {
              return `  pub ${field.name}: (),`;
            }
            // struct_field might be the type directly, or it might have a type property
            const fieldType = field.inner.struct_field.type || field.inner.struct_field;
            return `  pub ${field.name}: ${formatType(fieldType)},`;
          }).join('\n');
        }
        content += `\`\`\`rust\nstruct ${s.name} {\n${fields}\n}\n\`\`\`\n\n---\n\n`;
      }
    }

    if (groups.enums.length > 0) {
      content += `## Enums\n\n`;
      for (const e of groups.enums) {
        content += `### \`${e.name}\`\n\n`;
        content += e.docs ? `${e.docs.replace(/^# /gm, '#### ')}\n\n` : '*No documentation available.*\n\n';
        const variants = e.inner.enum.variants.map(vid => `  ${index[vid].name},`).join('\n');
        content += `\`\`\`rust\nenum ${e.name} {\n${variants}\n}\n\`\`\`\n\n---\n\n`;
      }
    }

    // Write file
    fs.writeFileSync(filePath, content);

    // Submodules
    for (const subMod of groups.modules) {
      processModule(subMod.id, currentPath);
    }
  }

  processModule(rootId);
}

function formatType(type) {
  // Always return a string, never undefined
  if (!type || typeof type !== 'object') {
    return '()';
  }
  
  // Handle wrapped types (e.g., function output types, nested type wrappers)
  // Recursively unwrap until we find the actual type structure
  if (type.type && typeof type.type === 'object') {
    return formatType(type.type);
  }
  
  // Primitive types (i32, u64, bool, etc.)
  if (type.primitive) {
    return type.primitive;
  }
  
  // resolved_path is the most common way generic types are stored
  // e.g., Result<()> is stored as resolved_path with name "Result" and args
  if (type.resolved_path) {
    const path = type.resolved_path;
    // The path can be stored as:
    // - path.name (if present)
    // - path.path (string, e.g., "Option", "String", "Result")
    // - path.path.segments (array of path segments, less common)
    let name = path.name;
    if (!name && path.path) {
      if (typeof path.path === 'string') {
        // path.path is a direct string (most common case)
        name = path.path;
      } else if (Array.isArray(path.path)) {
        // path.path is an array of segments
        if (path.path.length > 0) {
          const lastSegment = path.path[path.path.length - 1];
          name = lastSegment.name || lastSegment || 'Unknown';
        }
      } else if (path.path.segments && Array.isArray(path.path.segments)) {
        // path.path is an object with segments array
        const segments = path.path.segments;
        if (segments.length > 0) {
          const lastSegment = segments[segments.length - 1];
          name = lastSegment.name || lastSegment || 'Unknown';
        }
      }
    }
    if (!name) {
      name = 'Unknown';
    }
    if (path.args && path.args.angle_bracketed) {
      const args = path.args.angle_bracketed.args.map(a => {
        if (a.type) return formatType(a.type);
        // Handle type bindings and other argument types
        if (a.binding) {
          const bound = a.binding.bound ? formatType(a.binding.bound) : '?';
          return `${a.binding.name || '?'}: ${bound}`;
        }
        return '?';
      }).join(', ');
      name += `<${args}>`;
    }
    return name;
  }
  
  // Qualified paths (e.g., std::result::Result)
  if (type.qualified_path) {
    const qpath = type.qualified_path;
    // Try to get name from qpath.name, or extract from path
    let name = qpath.name;
    if (!name && qpath.path) {
      if (typeof qpath.path === 'string') {
        name = qpath.path;
      } else if (Array.isArray(qpath.path)) {
        if (qpath.path.length > 0) {
          const lastSegment = qpath.path[qpath.path.length - 1];
          name = lastSegment.name || lastSegment || 'Unknown';
        }
      } else if (qpath.path.segments && Array.isArray(qpath.path.segments)) {
        const segments = qpath.path.segments;
        if (segments.length > 0) {
          const lastSegment = segments[segments.length - 1];
          name = lastSegment.name || lastSegment || 'Unknown';
        }
      }
    }
    if (!name) {
      name = 'Unknown';
    }
    if (qpath.args && qpath.args.angle_bracketed) {
      const args = qpath.args.angle_bracketed.args.map(a => {
        if (a.type) return formatType(a.type);
        if (a.binding) {
          const bound = a.binding.bound ? formatType(a.binding.bound) : '?';
          return `${a.binding.name || '?'}: ${bound}`;
        }
        return '?';
      }).join(', ');
      return `${name}<${args}>`;
    }
    return name;
  }
  
  // Function pointer types
  if (type.fn_pointer) {
    const fnPtr = type.fn_pointer;
    const inputs = fnPtr.inputs ? fnPtr.inputs.map(t => formatType(t)).join(', ') : '';
    const output = fnPtr.output ? ` -> ${formatType(fnPtr.output)}` : '';
    return `fn(${inputs})${output}`;
  }
  
  // Raw pointer types
  if (type.raw_pointer) {
    const mut = type.raw_pointer.mutable ? 'mut' : 'const';
    return `*${mut} ${formatType(type.raw_pointer.type)}`;
  }
  
  // Reference types
  if (type.borrowed_ref) {
    const lifetime = type.borrowed_ref.lifetime ? `'${type.borrowed_ref.lifetime} ` : '';
    const mut = type.borrowed_ref.mutable ? 'mut ' : '';
    return `&${lifetime}${mut}${formatType(type.borrowed_ref.type)}`;
  }
  
  // Tuple types
  if (type.tuple) {
    if (Array.isArray(type.tuple) && type.tuple.length === 0) {
      return '()';
    }
    const types = Array.isArray(type.tuple) 
      ? type.tuple.map(formatType).join(', ')
      : formatType(type.tuple);
    return `(${types})`;
  }
  
  // Slice types
  if (type.slice) {
    return `[${formatType(type.slice)}]`;
  }
  
  // Array types
  if (type.array) {
    return `[${formatType(type.array.type)}; ${type.array.len || '?'}]`;
  }
  
  // Generic types - these might be unresolved or need special handling
  if (type.generic) {
    // If it's an object with resolved_path, recurse
    if (typeof type.generic === 'object' && type.generic.resolved_path) {
      return formatType({ resolved_path: type.generic.resolved_path });
    }
    // If it's just a string identifier, return it
    if (typeof type.generic === 'string') {
      return type.generic;
    }
    // If it has a name property, use that
    if (type.generic.name) {
      return type.generic.name;
    }
  }
  
  // Never return undefined - always return a fallback string
  // Only warn for types that aren't common wrappers or empty objects
  const hasContent = Object.keys(type).length > 0 && 
    !type.type && // Not just a wrapper
    !type.resolved_path && 
    !type.qualified_path &&
    !type.primitive;
  if (hasContent) {
    console.warn('Unknown type structure:', JSON.stringify(type, null, 2));
  }
  return '/* unknown type */';
}

function isCargoAvailable() {
  try {
    execSync('cargo --version', { stdio: 'pipe' });
    return true;
  } catch {
    return false;
  }
}

function main() {
  try {
    // Check if cargo is available (it won't be in Vercel/CI environments)
    if (!isCargoAvailable()) {
      const isCI = process.env.CI || process.env.VERCEL || process.env.NETLIFY;
      if (isCI) {
        console.log('⚠️  Cargo not available in CI environment. Skipping Rustdoc generation.');
        console.log('   Pre-generated docs should be committed to the repository.');
        process.exit(0); // Exit successfully - this is expected in CI
      } else {
        console.error('❌ Cargo is not installed or not in PATH.');
        console.error('   Please install Rust: https://rustup.rs/');
        process.exit(1);
      }
    }

    ensureDir(OUTPUT_DIR);
    const libJson = generateJson('lib');
    const binJson = generateJson('pai-engine');
    
    console.log('📖 Parsing JSON files...');
    processJson(libJson, false);
    processJson(binJson, true);
    console.log('✅ MDX generation complete!');
  } catch (error) {
    console.error('❌ Error during documentation generation:', error.message);
    process.exit(1);
  }
}

main();
