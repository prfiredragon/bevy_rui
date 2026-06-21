import os

def process(file):
    with open(file, "r") as f: content = f.read()
    orig = content
    
    # We already have `ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, ..ImageNode::... }`
    # Let's just insert `image_mode: bevy::ui::widget::NodeImageMode::Stretch, `
    content = content.replace("bevy::ui::VisualBox::BorderBox, ..ImageNode::", "bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::")
    
    # Also in theme.rs where it sets image_mode = NodeImageMode::Auto;
    if "theme.rs" in file:
        content = content.replace("image_node.image_mode = NodeImageMode::Auto;", "image_node.image_mode = NodeImageMode::Stretch;")
        content = content.replace("image_node.image_mode = bevy::ui::widget::NodeImageMode::Auto;", "image_node.image_mode = bevy::ui::widget::NodeImageMode::Stretch;")
        
    if content != orig:
        with open(file, "w") as f: f.write(content)

for root, _, files in os.walk('src'):
    for file in files:
        if file.endswith('.rs'):
            process(os.path.join(root, file))
