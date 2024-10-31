import os
import yaml
from PIL import Image
from tinytag import TinyTag
import pyheif

def get_image_metadata(file_path):
    _, ext = os.path.splitext(file_path)
    if ext.lower() in ['.heic']:
        heif_file = pyheif.read(file_path)
        return heif_file.metadata
    else:
        with Image.open(file_path) as img:
            return img._getexif()

def get_audio_metadata(file_path):
    tag = TinyTag.get(file_path)
    return { 'artist': tag.artist, 'album': tag.album, 'title': tag.title }

def save_metadata_to_markdown(file_path, metadata):
    with open(file_path, 'w') as f:
        f.write('---\n')
        f.write('origin_metadata:\n')
        f.write(yaml.dump(metadata))
        f.write('---\n')

def process_file(file_path):
    _, ext = os.path.splitext(file_path)
    if ext.lower() in ['.jpg', '.jpeg', '.png', '.gif', '.bmp', '.heic']:
        metadata = get_image_metadata(file_path)
    elif ext.lower() in ['.mp3', '.wav', '.flac', '.m4a']:
        metadata = get_audio_metadata(file_path)
    else:
        print(f'Unsupported file type: {ext}')
        return
    if metadata:
        save_metadata_to_markdown(file_path + '.metadata.md', metadata)
    print(f"Processing file: {file_path}")

def process_directory(directory_path):
    for root, dirs, files in os.walk(directory_path):
        for file in files:
            file_path = os.path.join(root, file)
            process_file(file_path)

# test the function
process_directory('./')