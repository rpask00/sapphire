import concurrent
import json
import concurrent.futures

import requests
from PIL import Image

phase_list = json.loads(open('doppler_phases.json').read())


def compare_images(image_path1, image_path2):
    try:
        # Open the images
        img1 = Image.open(image_path1)
        img2 = Image.open(image_path2)

        # Ensure the images have the same dimensions
        if img1.size != img2.size:
            raise ValueError("Images must have the same dimensions for comparison.")

        # Convert images to RGB mode (if not already)
        img1 = img1.convert('RGB')
        img2 = img2.convert('RGB')

        # Get the pixel data
        pixels1 = list(img1.getdata())
        pixels2 = list(img2.getdata())

        # Compare pixel values
        common_pixels = sum(1 for p1, p2 in zip(pixels1, pixels2) if p1 == p2)

        # Calculate percentage of common pixels
        total_pixels = img1.size[0] * img1.size[1]
        percentage_common = (common_pixels / total_pixels) * 100

        # Print the result
        print(f"Percentage of common pixels: {percentage_common:.2f}%")
    except Exception as e:
        print(e)


# img_src_1 = "./phases/akamai" + ".png"
# img_src_2 = "./phases/cloudflare" + ".png"
#
# compare_images(img_src_1, img_src_2)


phases_list = []

for knife_name, phases in phase_list.items():
    for phase in phases:

        if not phase:
            continue

        phases_list.append(phase)


def download_image(_phase):
    print(_phase)

    url = f'https://community.cloudflare.steamstatic.com/economy/image/{_phase}/62fx62f'
    file = requests.get(url).content
    with open(f'./phases/{_phase}.png', 'wb') as f:
        f.write(file)


with concurrent.futures.ThreadPoolExecutor() as executor:
    executor.map(download_image, phases_list)
