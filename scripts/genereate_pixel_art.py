# use stable diffusion to generate pixel art images from nerijs/pixel-art-3.5L huggingface.co
from diffusers import DiffusionPipeline, LCMScheduler, StableDiffusionPipeline
import torch    
import gc
from pathlib import Path
import os
from dotenv import load_dotenv
from PIL import Image
# configure loguru to print in the console
import sys
import loguru

def setup_logging():
    """Configure loguru logging to console."""
    loguru.logger.remove()
    loguru.logger.add(sys.stderr, level="INFO", format="{time} {level} {message}", colorize=True)

def load_environment_variables():
    """Load environment variables from .env file."""
    if load_dotenv():
        loguru.logger.info("Environment variables loaded successfully.")
    else:
        loguru.logger.info("Failed to load environment variables.")
        sys.exit(1)
    
    api_key = os.getenv("HUGGINGFACE_API_KEY")
    if not api_key:
        loguru.logger.error("HUGGINGFACE_API_KEY is not set in the environment variables.")
        sys.exit(1)
    
    return api_key

def get_model_path():
    """Get the path to the local model file."""
    script_dir = Path(__file__).parent
    model_path = script_dir.parent / "models" / "pixel-art-3.5L-v2_000000500.safetensors"
    
    if not model_path.exists():
        loguru.logger.error(f"Local model file not found: {model_path}")
        sys.exit(1)
    
    return model_path

def load_model_from_single_file(model_path):
    """Try to load model from single safetensors file."""
    try:
        loguru.logger.info("Loading model from single safetensors file...")
        pipe = StableDiffusionPipeline.from_single_file(
            str(model_path),
            torch_dtype=torch.float16,
            safety_checker=None,
            requires_safety_checker=False
        )
        loguru.logger.info("Model loaded successfully from single file")
        return pipe
    except Exception as e:
        loguru.logger.error(f"Failed to load from single file: {e}")
        return None

def load_base_model():
    """Load base Stable Diffusion model as fallback."""
    try:
        loguru.logger.info("Trying base model approach...")
        base_model = "runwayml/stable-diffusion-v1-5"
        pipe = StableDiffusionPipeline.from_pretrained(
            base_model,
            torch_dtype=torch.float16,
            safety_checker=None,
            requires_safety_checker=False
        )
        loguru.logger.info("Loaded base model (custom weights not applied)")
        return pipe
    except Exception as e:
        loguru.logger.error(f"Failed to load base model: {e}")
        sys.exit(1)

def setup_pipeline(pipe):
    """Configure the pipeline with scheduler and move to GPU."""
    pipe.scheduler = LCMScheduler.from_pretrained("runwayml/stable-diffusion-v1-5", subfolder="scheduler")
    pipe.to("cuda")
    loguru.logger.info("Model loaded successfully.")
    return pipe

def generate_pixel_art_image(pipe, prompt, negative_prompt, output_filename):
    """Generate a pixel art image using the pipeline."""
    loguru.logger.info(f"Generating image with prompt: {prompt}")
    
    image = pipe(
        prompt=prompt,
        negative_prompt=negative_prompt,
        num_inference_steps=20,
        guidance_scale=7.5,
        width=512,
        height=512
    ).images[0]
    
    # Resize to 32x32 for pixel art effect
    image = image.resize((32, 32), Image.NEAREST)
    image.save(output_filename)
    loguru.logger.info(f"Image saved as {output_filename}")
    
    return image

def generate_sprite(pipe, prompt, size, target_filename):
    """Generate a sprite with specified prompt, size, and filename.
    
    Args:
        pipe: The diffusion pipeline
        prompt: Text prompt for image generation
        size: Tuple of (width, height) for the final sprite size
        target_filename: Output filename for the generated sprite
    
    Returns:
        PIL Image object of the generated sprite
    """
    loguru.logger.info(f"Generating sprite: {target_filename} ({size[0]}x{size[1]})")
    loguru.logger.info(f"Prompt: {prompt}")
    
    # Enhanced prompt for pixel art
    enhanced_prompt = f"pixel art style, 8-bit video game sprite, {prompt}, simple colors, retro gaming, clean pixels"
    negative_prompt = "blurry, realistic, photograph, 3d render, smooth gradients, antialiasing"
    
    # Generate at higher resolution first
    image = pipe(
        prompt=enhanced_prompt,
        negative_prompt=negative_prompt,
        num_inference_steps=20,
        guidance_scale=7.5,
        width=512,
        height=512
    ).images[0]
    
    # Resize to target size using nearest neighbor for pixel art effect
    sprite = image.resize(size, Image.NEAREST)
    sprite.save(target_filename)
    loguru.logger.info(f"Sprite saved as {target_filename}")
    
    return sprite

def create_sprite(prompt, size=(32, 32), target_filename=None):
    """Standalone function to create a sprite from scratch.
    
    Args:
        prompt: Text description of the sprite to generate
        size: Tuple of (width, height) for the sprite size, default (32, 32)
        target_filename: Output filename, auto-generated if None
    
    Returns:
        PIL Image object of the generated sprite
    """
    if target_filename is None:
        # Auto-generate filename from prompt
        safe_prompt = "".join(c for c in prompt if c.isalnum() or c in (' ', '-', '_')).rstrip()
        safe_prompt = safe_prompt.replace(' ', '_')[:30]
        target_filename = f"{safe_prompt}_{size[0]}x{size[1]}.png"
    
    # Setup everything needed
    setup_logging()
    api_key = load_environment_variables()
    model_path = get_model_path()
    
    # Load model
    loguru.logger.info(f"Loading model for sprite generation...")
    pipe = load_model_from_single_file(model_path)
    
    if pipe is None:
        pipe = load_base_model()
    
    pipe = setup_pipeline(pipe)
    
    # Generate the sprite
    return generate_sprite(pipe, prompt, size, target_filename)

def main():
    """Main function to orchestrate the pixel art generation process."""
    # Setup
    setup_logging()
    api_key = load_environment_variables()
    model_path = get_model_path()
    
    # Load model
    loguru.logger.info(f"Loading local model from {model_path}...")
    pipe = load_model_from_single_file(model_path)
    
    if pipe is None:
        pipe = load_base_model()
    
    pipe = setup_pipeline(pipe)
    
    # Generate sprites with different prompts and sizes
    sprites = [
        ("blue wizard with pointy hat and staff", (32, 32), "wizard_32x32.png"),
        ("red dragon breathing fire", (64, 64), "dragon_64x64.png"),
        ("green orc warrior with axe", (32, 32), "orc_32x32.png"),
        ("Crazy baby", (32, 32), "baby1_32x32.png"),
        ("Speed baby", (32, 32), "baby2_32x32.png"),
        ("tiny rat with glowing eyes", (16, 16), "rat_16x16.png"),
        ("small health potion bottle", (16, 16), "potion_16x16.png"),
        ("gold coin with shine", (16, 16), "coin_16x16.png"),
        ("sleeping bat hanging upside down", (16, 16), "bat_16x16.png"),
        ("spiky turtle crawling", (16, 16), "turtle_16x16.png"),
        ("knight in shining armor with sword", (32, 32), "knight_32x32.png"),
        ("zombie with torn clothes", (32, 32), "zombie_32x32.png"),
        ("pirate with eyepatch and hook", (32, 32), "pirate_32x32.png"),
        ("fairy with glowing wings", (32, 32), "fairy_32x32.png"),
        ("ghost floating with arms up", (32, 32), "ghost_32x32.png"),
        ("blue slime with eyes", (16, 16), "slime_16x16.png"),
        ("tiny treasure chest", (16, 16), "chest_16x16.png"),
        ("scroll with magic symbols", (16, 16), "scroll_16x16.png"),
        ("flaming sword", (32, 32), "sword_32x32.png"),
        ("cat with glowing eyes", (16, 16), "cat_16x16.png")
    ]
    target_directory = Path("generated_sprites/new_sprites")
    target_directory.mkdir(parents=True, exist_ok=True)
    for prompt, size, filename in sprites:
        generate_sprite(pipe, prompt, size, target_directory / filename)

if __name__ == "__main__":
    main()