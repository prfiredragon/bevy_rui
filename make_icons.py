from PIL import Image, ImageDraw

def make_icon(name, color, text):
    img = Image.new('RGBA', (32, 32), color=(0,0,0,0))
    d = ImageDraw.Draw(img)
    d.rounded_rectangle([(0,0), (31, 31)], radius=4, fill=color, outline="white", width=2)
    img.save(f"assets/icons/{name}.png")

make_icon("folder", (240, 200, 50, 255), "")
make_icon("file", (150, 150, 150, 255), "")
make_icon("code", (50, 150, 240, 255), "")
make_icon("image", (50, 240, 150, 255), "")
make_icon("audio", (240, 50, 150, 255), "")
