import os, re

def replace_solid_color(content):
    idx = 0
    while True:
        idx = content.find("ImageNode::solid_color(", idx)
        if idx == -1: break
        start = idx + len("ImageNode::solid_color(")
        paren_count = 1
        i = start
        while i < len(content) and paren_count > 0:
            if content[i] == '(': paren_count += 1
            elif content[i] == ')': paren_count -= 1
            i += 1
        # content[idx:i] is ImageNode::solid_color(...)
        original = content[idx:i]
        replacement = f"ImageNode {{ visual_box: bevy::ui::widget::VisualBox::PaddingBox, ..{original} }}"
        content = content[:idx] + replacement + content[i:]
        idx = idx + len(replacement)
    return content

def process(file):
    with open(file, "r") as f: content = f.read()
    orig = content
    
    # 1. InputFocus
    content = content.replace("use bevy::input_focus::InputFocus;", "use bevy::input_focus::{InputFocus, FocusCause};")
    content = content.replace("use bevy::{input_focus::InputFocus, prelude::*};", "use bevy::{input_focus::{InputFocus, FocusCause}, prelude::*};")
    content = content.replace("input_focus.0 !=", "input_focus.get() !=")
    content = content.replace("input_focus.0 ==", "input_focus.get() ==")
    content = content.replace("input_focus.0.is_none()", "input_focus.get().is_none()")
    content = content.replace("focus.0 !=", "focus.get() !=")
    content = content.replace("focus.0 ==", "focus.get() ==")
    content = content.replace("= focus.0 else", "= focus.get() else")
    content = content.replace("= input_focus.0 else", "= input_focus.get() else")
    
    content = content.replace("input_focus.set(entity);", "input_focus.set(entity, FocusCause::Navigated);")
    content = content.replace("focus.set(entity);", "focus.set(entity, FocusCause::Navigated);")
    content = content.replace("navigator.manual_directional_navigation.focus.set(entity)", "navigator.manual_directional_navigation.focus.set(entity, FocusCause::Navigated)")
    
    # TextLayout
    content = content.replace("TextLayout::new_with_justify(Justify::Left).with_no_wrap()", "TextLayout::justify(Justify::Left).with_no_wrap()")
    
    # FontSource
    content = content.replace("text_font.font == Handle::default()", "text_font.font == bevy::prelude::FontSource::Handle(Handle::default())")
    content = content.replace("text_font.font = font_res.0.clone();", "text_font.font = bevy::prelude::FontSource::Handle(font_res.0.clone());")
    content = content.replace("text_font.font = f;", "text_font.font = bevy::prelude::FontSource::Handle(f);")
    content = content.replace("font.font = theme.font.clone();", "font.font = bevy::prelude::FontSource::Handle(theme.font.clone());")
    
    # init_non_send
    content = content.replace("init_non_send_resource", "init_non_send")
    
    # FontSize in struct literals
    content = re.sub(r'font_size:\s*([0-9]+\.[0-9]+),', r'font_size: bevy::prelude::FontSize::Px(\1),', content)
    
    # font_size in closures
    content = content.replace("font.font_size = 14.0;", "font.font_size = bevy::prelude::FontSize::Px(14.0);")
    content = content.replace("font.font_size = 18.0;", "font.font_size = bevy::prelude::FontSize::Px(18.0);")
    
    # textbox math
    content = content.replace("font_opt.map_or(18.0, |f| f.font_size)", "font_opt.map_or(18.0, |f| match f.font_size { bevy::prelude::FontSize::Px(v) | bevy::prelude::FontSize::Vw(v) | bevy::prelude::FontSize::Vh(v) | bevy::prelude::FontSize::VMin(v) | bevy::prelude::FontSize::VMax(v) | bevy::prelude::FontSize::Rem(v) => v })")
    
    # VisualBox replacements
    content = replace_solid_color(content)
    
    if "theme.rs" in file:
        content = content.replace("image_node.color = Color::WHITE;", "image_node.color = Color::WHITE;\n                    image_node.visual_box = bevy::ui::widget::VisualBox::PaddingBox;")
        content = content.replace("image_node.color = theme.color_panel;", "image_node.color = theme.color_panel;\n                    image_node.visual_box = bevy::ui::widget::VisualBox::PaddingBox;")
        content = content.replace("image_node.color = theme.color_button_normal;", "image_node.color = theme.color_button_normal;\n                    image_node.visual_box = bevy::ui::widget::VisualBox::PaddingBox;")
        content = content.replace("image_node.color = theme.color_dropdown;", "image_node.color = theme.color_dropdown;\n                    image_node.visual_box = bevy::ui::widget::VisualBox::PaddingBox;")

    if "windows.rs" in file or "button.rs" in file or "textbox.rs" in file or "file_dialog.rs" in file or "color_picker.rs" in file:
        content = content.replace("ImageNode::default(),", "ImageNode { visual_box: bevy::ui::widget::VisualBox::PaddingBox, ..ImageNode::default() },")

    if content != orig:
        with open(file, "w") as f: f.write(content)

for root, _, files in os.walk('src'):
    for file in files:
        if file.endswith('.rs'):
            process(os.path.join(root, file))

