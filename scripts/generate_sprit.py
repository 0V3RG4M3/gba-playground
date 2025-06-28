from epjtools.toml import load_asset_description
from dotenv import load_dotenv
from PIL import Image
import sys
import os
import loguru
import requests
import io
from pathlib import Path
from openai import OpenAI


#configure loguru to print in the console
loguru.logger.remove()
loguru.logger.add(sys.stderr, level="INFO", format="{time} {level} {message}", colorize=True)


if load_dotenv():
    loguru.logger.info("Environment variables loaded successfully.")
else:
    loguru.logger.info("Failed to load environment variables.")
    sys.exit(1)


api_key=os.getenv("OPENAI_API_KEY")
client = OpenAI(api_key=api_key)

def generate_prompt(asset):
    palette = "#0F0F0F, #3F3F3F, #6F6F6F, #9F9F9F, #D7D7D7, #FFFFFF, #FF0000, #00FF00, #0000FF, #FFFF00, #FF00FF, #00FFFF, #804000, #008040, #800080, #4040FF"
    if asset["target"] == "sprite":
        return f"""Pixel art sprite of a single {asset['description']} , centered and occupying the full height and width of the canvas, without any cropping or padding. Use only hard edges and flat colors. No gradients. No anti-aliasing. No color swatches. No multiple views. Transparent background only. Use only this 16-color GBA-safe palette: {palette}. Output must be a clean 64x64 pixel sprite ready for downscaling."""
    elif asset["target"] == "splash":
        return f"""Pixel-perfect splash screen for a Game Boy Advance game: {asset['description']}. Resolution must be exactly {asset['size']}. Retro 16-bit style using only flat colors and clean outlines — no gradients or shading. Use only this GBA-safe 16-color palette: {palette}. Background allowed. No text unless specified."""
    else:
        return "Unknown asset type."

def download_and_save_image(image_url, filename, output_dir="generated_sprites"):
    """Télécharge et sauvegarde une image depuis une URL"""
    try:
        # Créer le dossier de sortie s'il n'existe pas
        output_path = Path(output_dir)
        output_path.mkdir(exist_ok=True)

        # Télécharger l'image
        response = requests.get(image_url)
        response.raise_for_status()

        # Ouvrir et sauvegarder l'image
        image = Image.open(io.BytesIO(response.content))
        full_path = output_path / filename
        image.save(full_path)

        loguru.logger.info(f"Image saved to: {full_path}")
        return full_path
    except Exception as e:
        loguru.logger.error(f"Error downloading/saving image: {e}")
        return None

if __name__ == "__main__":
    loguru.logger.info("Loading environment variables from .env file")
    toml_file = "scripts/asset_description.toml"
    if len(sys.argv) > 1:
        toml_file = sys.argv[1]
    if not os.path.exists(toml_file):
        loguru.logger.error(f"Error: TOML file '{toml_file}' does not exist.")
        sys.exit(1)

    asset_description = load_asset_description(toml_file)
    if not asset_description:
        loguru.logger.error(f"Error: Failed to load asset description from '{toml_file}'.")
        sys.exit(1)
    loguru.logger.info(f"Asset description loaded from {toml_file}")
    loguru.logger.debug(f"Asset description: {asset_description}")
    if not api_key:
        loguru.logger.error("Error: OPENAI_API_KEY environment variable is not set.")
        sys.exit(1)

    loguru.logger.info("OpenAI API key set successfully.")


    for level in asset_description.get("level", []):
        loguru.logger.info(f"Generating sprite for level: {level.get('name')}")
        sprites = level.get("sprites")
        for sprite in sprites:
            asset = {}
            asset["name"] = sprite.get("name")
            asset["description"] = sprite.get("description", "")
            asset["size"] = sprite.get("size", "32x32px")
            asset["target"] = sprite.get("target", "sprite")

            loguru.logger.info(f"Generating sprite: {asset['name']} with size {asset['size']}")
            loguru.logger.info(f"Sprite description: {asset['description']}")
            loguru.logger.info(f"Sprite target: {asset['target']}")
            use_openai = True  # Assurez-vous que l'API OpenAI est utilisée
            instructions = generate_prompt(asset)
            if not instructions:
                loguru.logger.error("Error: No base prompt found in asset description.")
                sys.exit(1)
            loguru.logger.info(f"Using prompt: {instructions}")

            if not use_openai:
                loguru.logger.info("Using local image generation method (not OpenAI).")
                sys.exit(0)
            try:
                response = client.images.generate(
                    model="dall-e-2",  # Assurez-vous que le modèle est correct
                    prompt=instructions,
                    n=1,
                    size="512x512",  # ← DALL·E ne supporte pas les tailles personnalisées
                    response_format="url"  # ou "b64_json" si tu veux les données brutes
                )
                image_url = response.data[0].url
                loguru.logger.info(f"Sprite generated successfully: {image_url}")

                # Optionnel: télécharger et sauvegarder l'image
                download_and_save_image(image_url, f"{asset['name']}_{asset['size']}.png")

            except Exception as e:
                loguru.logger.error(f"Error generating sprite '{asset['name']}': {e}")
                continue


    loguru.logger.info("Sprite generation completed.")

