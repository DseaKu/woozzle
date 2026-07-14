import os
import re

for root, _, files in os.walk('src'):
    for file in files:
        if file.endswith('.rs'):
            path = os.path.join(root, file)
            with open(path, 'r') as f:
                content = f.read()
            
            modules = ['components', 'events', 'resources', 'systems', 'bundles']
            added_uses = []
            
            original_content = content
            
            for mod in modules:
                pattern = rf'super::{mod}::'
                # Check if it exists
                if re.search(pattern, content):
                    # We want to replace super::mod:: with nothing, UNLESS it's part of a use statement.
                    # 'use super::mod::...' or 'use super::mod::{...}'
                    # Let's just do a manual replacement line by line to be safe
                    
                    new_lines = []
                    needs_use = False
                    for line in content.split('\n'):
                        if 'use super::' + mod in line:
                            # keep it as is, or remove it? Let's keep it.
                            new_lines.append(line)
                        else:
                            if pattern in line:
                                line = line.replace(pattern, '')
                                needs_use = True
                            new_lines.append(line)
                    
                    content = '\n'.join(new_lines)
                    if needs_use:
                        added_uses.append(f"use super::{mod}::*;")
            
            if content != original_content:
                lines = content.split('\n')
                insert_idx = 0
                for i, line in enumerate(lines):
                    if line.startswith('use '):
                        insert_idx = i + 1
                
                for use_stmt in set(added_uses):
                    lines.insert(insert_idx, use_stmt)
                
                with open(path, 'w') as f:
                    f.write('\n'.join(lines))

print("Done")
