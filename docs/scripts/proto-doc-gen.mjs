import fs from 'node:fs';
import path from 'path';
import { fileURLToPath } from 'node:url';
import { execSync } from 'node:child_process';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const PROTO_DIR = path.resolve(__dirname, '../../engine/proto');
const OUTPUT_DIR = path.resolve(__dirname, '../src/content/docs/reference');
const ENGINE_PATH = path.resolve(__dirname, '../../engine');

function ensureDir(dir) {
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
}

/**
 * Check if protoc-gen-doc is available
 */
function hasProtocGenDoc() {
  try {
    execSync('which protoc-gen-doc', { stdio: 'ignore' });
    return true;
  } catch {
    return false;
  }
}

/**
 * Parse proto file using protoc-gen-doc (if available)
 */
function generateWithProtocGenDoc(protoFile) {
  console.log(`📖 Generating documentation with protoc-gen-doc for ${path.basename(protoFile)}...`);
  
  const outputFile = path.join(OUTPUT_DIR, 'temp_proto_docs.md');
  
  try {
    execSync(
      `protoc --doc_out=${OUTPUT_DIR} --doc_opt=markdown,temp_proto_docs.md ${protoFile}`,
      { stdio: 'inherit', cwd: path.dirname(protoFile) }
    );
    
    if (fs.existsSync(outputFile)) {
      return fs.readFileSync(outputFile, 'utf-8');
    }
  } catch (error) {
    console.warn(`⚠️  protoc-gen-doc failed: ${error.message}`);
  }
  
  return null;
}

/**
 * Simple proto file parser (fallback when protoc-gen-doc is not available)
 * Parses basic structure: services, messages, enums, rpcs
 */
function parseProtoFile(protoPath) {
  const content = fs.readFileSync(protoPath, 'utf-8');
  const lines = content.split('\n');
  
  const result = {
    package: null,
    syntax: null,
    services: [],
    messages: [],
    enums: [],
    imports: [],
    comments: []
  };
  
  let currentService = null;
  let currentMessage = null;
  let currentEnum = null;
  let inService = false;
  let inMessage = false;
  let inEnum = false;
  let braceCount = 0;
  let commentBuffer = [];
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();
    
    // Collect comments
    if (line.startsWith('//')) {
      commentBuffer.push(line.substring(2).trim());
      continue;
    }
    
    // Skip empty lines but preserve comment context
    if (!line) {
      if (commentBuffer.length > 0) {
        // Comments before definitions are associated with them
        commentBuffer = [];
      }
      continue;
    }
    
    // Parse syntax
    if (line.startsWith('syntax')) {
      const match = line.match(/syntax\s*=\s*"([^"]+)"/);
      if (match) result.syntax = match[1];
      continue;
    }
    
    // Parse package
    if (line.startsWith('package')) {
      const match = line.match(/package\s+(\w+(?:\.\w+)*)/);
      if (match) result.package = match[1];
      continue;
    }
    
    // Parse import
    if (line.startsWith('import')) {
      const match = line.match(/import\s+"([^"]+)"/);
      if (match) result.imports.push(match[1]);
      continue;
    }
    
    // Parse service
    if (line.startsWith('service')) {
      const match = line.match(/service\s+(\w+)\s*\{/);
      if (match) {
        currentService = {
          name: match[1],
          rpcs: [],
          comments: [...commentBuffer]
        };
        commentBuffer = [];
        inService = true;
        braceCount = 1;
      }
      continue;
    }
    
    // Parse message
    if (line.startsWith('message')) {
      const match = line.match(/message\s+(\w+)\s*\{/);
      if (match) {
        currentMessage = {
          name: match[1],
          fields: [],
          comments: [...commentBuffer]
        };
        commentBuffer = [];
        inMessage = true;
        braceCount = 1;
      }
      continue;
    }
    
    // Parse enum
    if (line.startsWith('enum')) {
      const match = line.match(/enum\s+(\w+)\s*\{/);
      if (match) {
        currentEnum = {
          name: match[1],
          values: [],
          comments: [...commentBuffer]
        };
        commentBuffer = [];
        inEnum = true;
        braceCount = 1;
      }
      continue;
    }
    
    // Parse RPC (inside service)
    if (inService && line.startsWith('rpc')) {
      const match = line.match(/rpc\s+(\w+)\s*\(([^)]+)\)\s*returns\s*\(([^)]+)\)/);
      if (match) {
        currentService.rpcs.push({
          name: match[1],
          request: match[2].trim(),
          response: match[3].trim(),
          comments: [...commentBuffer]
        });
        commentBuffer = [];
      }
      continue;
    }
    
    // Parse field (inside message)
    if (inMessage && /^\w+/.test(line)) {
      const fieldMatch = line.match(/(?:repeated\s+|optional\s+)?(\w+)\s+(\w+)\s*=\s*(\d+)(?:\s*\[.*\])?;/);
      if (fieldMatch) {
        currentMessage.fields.push({
          type: fieldMatch[1],
          name: fieldMatch[2],
          number: fieldMatch[3],
          repeated: line.includes('repeated'),
          optional: line.includes('optional'),
          comments: [...commentBuffer]
        });
        commentBuffer = [];
      }
      continue;
    }
    
    // Parse enum value
    if (inEnum && /^\w+/.test(line)) {
      const enumMatch = line.match(/(\w+)\s*=\s*(\d+)(?:\s*\[.*\])?;/);
      if (enumMatch) {
        currentEnum.values.push({
          name: enumMatch[1],
          value: enumMatch[2],
          comments: [...commentBuffer]
        });
        commentBuffer = [];
      }
      continue;
    }
    
    // Track braces
    if (line.includes('{')) {
      braceCount += (line.match(/\{/g) || []).length;
    }
    if (line.includes('}')) {
      braceCount -= (line.match(/\}/g) || []).length;
      
      if (braceCount === 0) {
        if (inService && currentService) {
          result.services.push(currentService);
          currentService = null;
          inService = false;
        }
        if (inMessage && currentMessage) {
          result.messages.push(currentMessage);
          currentMessage = null;
          inMessage = false;
        }
        if (inEnum && currentEnum) {
          result.enums.push(currentEnum);
          currentEnum = null;
          inEnum = false;
        }
      }
    }
  }
  
  return result;
}

/**
 * Generate MDX content from parsed proto data
 */
function generateMdxContent(protoData, protoFileName) {
  const serviceName = protoFileName.replace('.proto', '');
  const pageTitle = protoData.services.length > 0 
    ? `${protoData.services[0].name} Service` 
    : `${serviceName} API`;
  
  let content = `---
title: "${pageTitle}"
description: "gRPC API documentation for ${protoData.package || serviceName}"
sidebar:
  order: 1
---

`;

  // Package info
  if (protoData.package) {
    content += `## Package\n\n\`${protoData.package}\`\n\n`;
  }
  
  if (protoData.syntax) {
    content += `**Syntax:** \`${protoData.syntax}\`\n\n`;
  }
  
  // Imports
  if (protoData.imports.length > 0) {
    content += `## Imports\n\n`;
    for (const imp of protoData.imports) {
      content += `- \`${imp}\`\n`;
    }
    content += `\n`;
  }
  
  // Services
  if (protoData.services.length > 0) {
    content += `## Services\n\n`;
    
    for (const service of protoData.services) {
      content += `### \`${service.name}\`\n\n`;
      
      if (service.comments.length > 0) {
        content += `${service.comments.join('\n')}\n\n`;
      }
      
      if (service.rpcs.length > 0) {
        content += `#### RPC Methods\n\n`;
        
        for (const rpc of service.rpcs) {
          content += `##### \`${rpc.name}\`\n\n`;
          
          if (rpc.comments.length > 0) {
            content += `${rpc.comments.join('\n')}\n\n`;
          }
          
          content += `\`\`\`protobuf\nrpc ${rpc.name}(${rpc.request}) returns (${rpc.response})\n\`\`\`\n\n`;
          content += `---\n\n`;
        }
      }
    }
  }
  
  // Messages
  if (protoData.messages.length > 0) {
    content += `## Messages\n\n`;
    
    for (const message of protoData.messages) {
      content += `### \`${message.name}\`\n\n`;
      
      if (message.comments.length > 0) {
        content += `${message.comments.join('\n')}\n\n`;
      }
      
      if (message.fields.length > 0) {
        content += `#### Fields\n\n`;
        content += `| Field | Type | Number | Description |\n`;
        content += `|-------|------|--------|-------------|\n`;
        
        for (const field of message.fields) {
          const typePrefix = field.repeated ? 'repeated ' : field.optional ? 'optional ' : '';
          const comments = field.comments.length > 0 ? field.comments.join(' ') : '-';
          content += `| \`${field.name}\` | \`${typePrefix}${field.type}\` | ${field.number} | ${comments} |\n`;
        }
        
        content += `\n`;
      }
      
      content += `---\n\n`;
    }
  }
  
  // Enums
  if (protoData.enums.length > 0) {
    content += `## Enums\n\n`;
    
    for (const enum_ of protoData.enums) {
      content += `### \`${enum_.name}\`\n\n`;
      
      if (enum_.comments.length > 0) {
        content += `${enum_.comments.join('\n')}\n\n`;
      }
      
      if (enum_.values.length > 0) {
        content += `| Value | Number | Description |\n`;
        content += `|-------|--------|-------------|\n`;
        
        for (const value of enum_.values) {
          const comments = value.comments.length > 0 ? value.comments.join(' ') : '-';
          content += `| \`${value.name}\` | ${value.value} | ${comments} |\n`;
        }
        
        content += `\n`;
      }
      
      content += `---\n\n`;
    }
  }
  
  return content;
}

/**
 * Main function
 */
function main() {
  ensureDir(OUTPUT_DIR);
  
  // Check if proto directory exists
  if (!fs.existsSync(PROTO_DIR)) {
    console.log(`⚠️  Proto directory not found at ${PROTO_DIR}`);
    console.log(`   Creating placeholder documentation...`);
    
    // Create placeholder
    const placeholderContent = `---
title: "gRPC API Reference"
description: "Protocol Buffer definitions and gRPC service documentation"
sidebar:
  order: 1
---

PAI uses **gRPC** over **Unix Domain Sockets (UDS)** for inter-process communication. This provides maximum performance, type safety via Protobuf, and access control via Linux file permissions.

## Protocol Buffer Definitions

The API definitions will be located in \`engine/proto/service.proto\` once the proto files are created.

## Service Overview

*API documentation will be automatically generated from Protocol Buffer definitions once proto files are available.*

## IPC Strategy

- **Transport**: Unix Domain Sockets (UDS) - zero networking overhead
- **Protocol**: gRPC with Protocol Buffers
- **Access Control**: Linux file permissions (User/Group isolation)
- **Default**: No network ports open

## Future: High-Performance Extension

For future video streams or raw sensor fusion, we plan to implement a hybrid model:
- **Control Plane**: gRPC for control messages
- **Data Plane**: Shared Memory Ring Buffer for bulk data
`;
    
    const outputFile = path.join(OUTPUT_DIR, 'api.mdx');
    fs.writeFileSync(outputFile, placeholderContent);
    console.log(`✅ Created placeholder documentation at ${outputFile}`);
    return;
  }
  
  // Find all proto files
  const protoFiles = fs.readdirSync(PROTO_DIR)
    .filter(file => file.endsWith('.proto'))
    .map(file => path.join(PROTO_DIR, file));
  
  if (protoFiles.length === 0) {
    console.log(`⚠️  No .proto files found in ${PROTO_DIR}`);
    return;
  }
  
  console.log(`📦 Found ${protoFiles.length} proto file(s)`);
  
  // Try to use protoc-gen-doc first, fallback to parser
  const useProtocGenDoc = hasProtocGenDoc();
  
  if (useProtocGenDoc) {
    console.log(`✅ Using protoc-gen-doc for documentation generation`);
  } else {
    console.log(`⚠️  protoc-gen-doc not found, using built-in parser`);
    console.log(`   Install with: go install github.com/pseudomuto/protoc-gen-doc/cmd/protoc-gen-doc@latest`);
  }
  
  // Process each proto file
  for (const protoFile of protoFiles) {
    const protoFileName = path.basename(protoFile);
    console.log(`\n📖 Processing ${protoFileName}...`);
    
    // Derive output filename from proto filename (replace .proto with .mdx)
    // Special case: service.proto -> api.mdx for main API documentation
    let outputBaseName = protoFileName.replace(/\.proto$/, '.mdx');
    if (protoFileName === 'service.proto') {
      outputBaseName = 'api.mdx';
    }
    const outputFile = path.join(OUTPUT_DIR, outputBaseName);
    
    let protoData;
    let markdownContent = null;
    
    if (useProtocGenDoc) {
      markdownContent = generateWithProtocGenDoc(protoFile);
    }
    
    // If protoc-gen-doc failed or is not available, use built-in parser
    if (!markdownContent) {
      protoData = parseProtoFile(protoFile);
      markdownContent = generateMdxContent(protoData, protoFileName);
    } else {
      // Convert protoc-gen-doc markdown to MDX format
      // Add frontmatter if not already present
      if (!markdownContent.includes('---\n')) {
        const serviceName = protoFileName.replace(/\.proto$/, '');
        const pageTitle = `${serviceName} API`;
        markdownContent = `---
title: "${pageTitle}"
description: "Protocol Buffer definitions and gRPC service documentation for ${serviceName}"
sidebar:
  order: 1
---

${markdownContent}
`;
      }
    }
    
    // Write MDX file with unique filename per proto
    fs.writeFileSync(outputFile, markdownContent);
    console.log(`✅ Generated documentation at ${outputFile}`);
  }
  
  console.log(`\n✅ Proto documentation generation complete!`);
}

main();

