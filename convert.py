from PIL import Image

img = Image.open("font-org.png")
pixels = img.load()

for x in range(img.size[0]):
    for y in range(img.size[1]):
        color = pixels[x, y]
        if color == (0, 0, 0, 255):
            pixels[x, y] = (255, 255, 255, 255)
        else:
            pixels[x, y] = (0, 0, 0, 0)


img.save("font.png")
