import os
import glob
import re

def process_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    orig_content = content

    # Replace ImageNode::solid_color(X) with BackgroundColor(X)
    content = re.sub(r'ImageNode::solid_color\((.*?)\)', r'BackgroundColor(\1)', content)
    
    # Replace ImageNode::default() with BackgroundColor(Color::WHITE) when it's part of a tuple spawn
    # This might be tricky, let's just do it manually for the known cases or a broad replace 
    # if it's not an icon. Let's look for ImageNode::default(),
    content = content.replace('ImageNode::default(),', 'BackgroundColor(Color::WHITE),')
    
    # Replace Query<(..., &mut ImageNode, ...)> with BackgroundColor
    # We must be careful because some queries might actually want ImageNode.
    # In button.rs, tabs.rs, color_picker.rs, theme.rs, etc.
    if 'button.rs' in filepath or 'tabs.rs' in filepath or 'theme.rs' in filepath or 'accordion.rs' in filepath or 'checkbox.rs' in filepath or 'resizer.rs' in filepath or 'scrollview.rs' in filepath or 'textbox.rs' in filepath or 'tooltip.rs' in filepath or 'windows.rs' in filepath or 'slider.rs' in filepath or 'dropdown.rs' in filepath or 'menu.rs' in filepath or 'file_dialog.rs' in filepath or 'color_picker.rs' in filepath:
        content = content.replace('&mut ImageNode', '&mut BackgroundColor')
        content = content.replace('Mut<ImageNode>', 'Mut<BackgroundColor>')
        content = content.replace('mut image_node', 'mut bg_node')
        content = content.replace('image_node.color', 'bg_node.0')
        # fix theme apply_to_image
        content = content.replace('apply_to_image', 'apply_to_bg')

    if content != orig_content:
        with open(filepath, 'w') as f:
            f.write(content)
        print(f"Updated {filepath}")

for root, _, files in os.walk('src'):
    for file in files:
        if file.endswith('.rs'):
            process_file(os.path.join(root, file))
