import os

replacements = {
    'VisibleTilesUpdated': 'VisibleUpdated',
    'TileDataUpdated': 'DataUpdated',
    'TileData': 'Data',
    'VisibleTiles': 'Visible',
    'WoozzleDataUpdated': 'DataUpdated',
    'VisibleWoozzleUpdated': 'VisibleUpdated',
    'WoozlesData': 'Data',
    'VisibleWoozzle': 'Visible'
}

for root, _, files in os.walk('src'):
    for file in files:
        if file.endswith('.rs'):
            path = os.path.join(root, file)
            with open(path, 'r') as f:
                content = f.read()
            
            new_content = content
            for old, new in replacements.items():
                new_content = new_content.replace(old, new)
                
            if new_content != content:
                with open(path, 'w') as f:
                    f.write(new_content)

print("Done")
