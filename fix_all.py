import os, re

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
    
    # --- BackgroundColor fix ---
    content = re.sub(r'ImageNode::solid_color\((.*?)\)', r'BackgroundColor(\1)', content)
    
    if "button.rs" in file:
        content = content.replace("ImageNode::default(),", "BackgroundColor::default(),")
        content = content.replace("&mut ImageNode", "&mut BackgroundColor")
        content = content.replace("mut image_node", "mut bg")
        content = content.replace("image_node.color", "bg.0")
    if "tabs.rs" in file:
        content = content.replace("&mut ImageNode", "&mut BackgroundColor")
        content = content.replace("mut image_node", "mut bg")
        content = content.replace("bg.color", "bg.0")
        content = content.replace("image_node.color", "bg.0")
        # Ensure we also replace `fn tab(&mut self, label: &str, closable: bool, modifier: impl FnOnce(&mut Node, &mut ImageNode) + 'a`
        content = content.replace("&mut Node, &mut BackgroundColor", "&mut Node, &mut ImageNode") # Wait! The tabs API might pass ImageNode
    if "windows.rs" in file:
        content = content.replace("ImageNode::default(),", "BackgroundColor::default(),")
    if "file_dialog.rs" in file:
        content = content.replace("ImageNode::default(),", "BackgroundColor::default(),")
    
    if content != orig:
        with open(file, "w") as f: f.write(content)

for root, _, files in os.walk('src'):
    for file in files:
        if file.endswith('.rs'):
            process(os.path.join(root, file))

