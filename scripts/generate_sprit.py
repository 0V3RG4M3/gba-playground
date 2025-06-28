from epjtools.toml import load_asset_description
from dotenv import load_dotenv
from PIL import Image
import sys
import os
import loguru
import requests
import io
from pathlib import Path
import openai



#configure loguru to print in the console
loguru.logger.remove()
loguru.logger.add(sys.stderr, level="INFO", format="{time} {level} {message}", colorize=True)   

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
    load_dotenv()
    loguru.logger.info("Loading environment variables from .env file")
    toml_file = "asset_description.toml"
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
    
    openai_client = openai.OpenAI(api_key=os.getenv("OPENAI_API_KEY"))
    if not os.getenv("OPENAI_API_KEY"):
        loguru.logger.error("Error: OPENAI_API_KEY environment variable is not set.")
        sys.exit(1)

    loguru.logger.info("OpenAI API key set successfully.")

    for level in asset_description.get("level", []):
        loguru.logger.info(f"Generating sprite for level: {level.get('name')}")
        sprites = level.get("sprites")
        for sprite in sprites:
            name = sprite.get("name")
            description = sprite.get("description", "")
            size = sprite.get("size", "32x32px")
            target = sprite.get("target", "sprite")

            loguru.logger.info(f"Generating sprite: {name} with size {size}")
            loguru.logger.info(f"Sprite description: {description}")
            loguru.logger.info(f"Sprite target: {target}")
            # Récupération du prompt de base depuis le TOML
            base_prompt = asset_description.get("prompt", "")
            if not base_prompt:
                loguru.logger.error("Error: No base prompt found in asset description.")
                sys.exit(1)
            loguru.logger.info(f"Using prompt: {base_prompt}")

            instructions = f"{base_prompt}\n\r Draw a {name} {description} for {size} sprite"
            try:
                response = openai.Image.create(
                    prompt=instructions,
                    n=1,
                    size=size,
                    response_format="url"
                )
                image_url = response.data[0].url
                loguru.logger.info(f"Sprite generated successfully: {image_url}")
                
                # Optionnel: télécharger et sauvegarder l'image
                download_and_save_image(image_url, f"{name}_{size}.png")
                
            except Exception as e:
                loguru.logger.error(f"Error generating sprite '{name}': {e}")
                continue
            

    loguru.logger.info("Sprite generation completed.")

