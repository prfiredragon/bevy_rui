import os

def process(file):
    with open(file, "r") as f: content = f.read()
    orig = content
    
    content = content.replace("bevy::ui::widget::VisualBox::PaddingBox", "bevy::ui::VisualBox::PaddingBox")
    
    if content != orig:
        with open(file, "w") as f: f.write(content)

for root, _, files in os.walk('src'):
    for file in files:
        if file.endswith('.rs'):
            process(os.path.join(root, file))
