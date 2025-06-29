"""
Sprite Generator Agent - Generates sprites using Stable Diffusion models.
"""

import os
import sys
from pathlib import Path
from PIL import Image
import torch
from diffusers import StableDiffusionPipeline, LCMScheduler
from dotenv import load_dotenv
import loguru
from typing import Tuple

class SpriteGeneratorAgent:
    """Agent responsible for generating sprites based on prompts."""
    
    def __init__(self):
        self.setup_logging()
        self.load_environment()
        self.load_model()
        
    def setup_logging(self):
        """Configure logging."""
        loguru.logger.remove()
        loguru.logger.add(sys.stderr, level="INFO", format="{time} {level} {message}", colorize=True)
        
    def load_environment(self):
        """Load environment variables."""
        if not load_dotenv():
            loguru.logger.error("Failed to load environment variables.")
            sys.exit(1)
            
        self.api_key = os.getenv("HUGGINGFACE_API_KEY")
        if not self.api_key:
            loguru.logger.warning("HUGGINGFACE_API_KEY not set - proceeding without authentication")
            
    def load_model(self):
        """Load the diffusion model."""
        model_path = Path(__file__).parent.parent.parent / "models" / "pixel-art-3.5L-v2_000000500.safetensors"
        
        # Try custom model first, fallback to base model
        try:
            if model_path.exists():
                loguru.logger.info("Attempting to load custom model...")
                self.pipe = StableDiffusionPipeline.from_single_file(
                    str(model_path),
                    torch_dtype=torch.float16,
                    safety_checker=None,
                    requires_safety_checker=False
                )
                loguru.logger.info("Custom model loaded successfully")
            else:
                raise FileNotFoundError("Custom model not found")
        except Exception as e:
            loguru.logger.warning(f"Custom model failed: {e}")
            loguru.logger.info("Loading base model...")
            self.pipe = StableDiffusionPipeline.from_pretrained(
                "runwayml/stable-diffusion-v1-5",
                torch_dtype=torch.float16,
                safety_checker=None,
                requires_safety_checker=False
            )
            loguru.logger.info("Base model loaded successfully")
            
        # Configure pipeline
        self.pipe.scheduler = LCMScheduler.from_pretrained("runwayml/stable-diffusion-v1-5", subfolder="scheduler")
        self.pipe.to("cuda")
        
    def generate_sprite(self, prompt: str, size: Tuple[int, int], negative_prompt: str = None) -> Image.Image:
        """Generate a sprite based on the prompt."""
        if negative_prompt is None:
            negative_prompt = "blurry, realistic, photograph, 3d render, smooth gradients, antialiasing, text, watermark"
            
        # Check if prompt already has pixel art styling
        if "pixel art" not in prompt.lower():
            enhanced_prompt = f"pixel art style, 8-bit video game sprite, {prompt}, simple colors, retro gaming, clean pixels, sharp edges"
        else:
            enhanced_prompt = prompt
        
        loguru.logger.info(f"Generating sprite with prompt: {enhanced_prompt}")
        
        # Generate at higher resolution first
        image = self.pipe(
            prompt=enhanced_prompt,
            negative_prompt=negative_prompt,
            num_inference_steps=25,
            guidance_scale=7.5,
            width=512,
            height=512
        ).images[0]
        
        # Resize to target size using nearest neighbor for pixel art effect
        sprite = image.resize(size, Image.NEAREST)
        return sprite
