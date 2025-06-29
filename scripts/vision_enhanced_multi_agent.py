# Vision-Enhanced Multi-agent sprite generation system
# Agent 1: Sprite Generator - Creates sprites based on prompts
# Agent 2: Vision Evaluator - Uses vision-language model to evaluate sprites
# Agent 3: Prompt Refiner - Refines prompts based on vision feedback

import os
import sys
from pathlib import Path
from PIL import Image, ImageStat
import numpy as np
import torch
from diffusers import StableDiffusionPipeline, LCMScheduler
from dotenv import load_dotenv
import loguru
from typing import Dict, List, Tuple, Optional
import json
import cv2
import base64
import io
from transformers import BlipProcessor, BlipForConditionalGeneration
import requests

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
        model_path = Path(__file__).parent.parent / "models" / "pixel-art-3.5L-v2_000000500.safetensors"
        
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

class VisionEvaluatorAgent:
    """Agent responsible for evaluating sprites using vision-language models."""
    
    def __init__(self):
        self.setup_vision_model()
        self.prompt_template = """
You are an assistant verifying game sprite images for correctness.

Given an image and a description:
1. Describe what you see in the image as accurately as possible.
2. Compare your description with the provided label.
   - Does the image match the description? Yes or no.
   - If not, explain the differences.
3. Check whether the background is fully transparent.
   - If any part of the background is not transparent (e.g. white, black, or any color), say so.
   - The background must be completely empty (alpha = 0) except where the sprite is drawn.

Respond in the following format:

---
Filename: {filename}  
Expected description: {description}  
Observed description: <your description>  
Description match: Yes / No  
Transparent background: Yes / No  
Comments:  
– <notes>
---
"""
        
    def setup_vision_model(self):
        """Setup vision-language model for image evaluation."""
        try:
            # Try to load BLIP model for image captioning
            loguru.logger.info("Loading BLIP vision model...")
            self.processor = BlipProcessor.from_pretrained("Salesforce/blip-image-captioning-base")
            self.model = BlipForConditionalGeneration.from_pretrained("Salesforce/blip-image-captioning-base")
            self.vision_available = True
            loguru.logger.info("BLIP vision model loaded successfully")
        except Exception as e:
            loguru.logger.warning(f"Failed to load vision model: {e}")
            self.vision_available = False
            
    def encode_image_to_base64(self, image: Image.Image) -> str:
        """Convert PIL Image to base64 string."""
        buffer = io.BytesIO()
        image.save(buffer, format="PNG")
        img_str = base64.b64encode(buffer.getvalue()).decode()
        return img_str
        
    def check_transparency(self, image: Image.Image) -> Tuple[bool, str]:
        """Check if image background is truly transparent."""
        if image.mode != 'RGBA':
            return False, "Image does not have alpha channel"
            
        # Convert to numpy array
        img_array = np.array(image)
        alpha_channel = img_array[:, :, 3]
        
        # Find non-transparent pixels
        non_transparent = np.where(alpha_channel > 0)
        
        if len(non_transparent[0]) == 0:
            return False, "Image is completely transparent"
            
        # Check if there are pixels that should be transparent but aren't
        # (This is a heuristic - we assume edges should be transparent)
        edge_alpha = np.concatenate([
            alpha_channel[0, :],  # top edge
            alpha_channel[-1, :],  # bottom edge
            alpha_channel[:, 0],  # left edge
            alpha_channel[:, -1]  # right edge
        ])
        
        non_transparent_edges = np.sum(edge_alpha > 0)
        total_edge_pixels = len(edge_alpha)
        
        if non_transparent_edges / total_edge_pixels > 0.3:
            return False, f"Background may not be transparent - {non_transparent_edges}/{total_edge_pixels} edge pixels are non-transparent"
            
        return True, "Background appears to be properly transparent"
        
    def describe_image(self, image: Image.Image) -> str:
        """Generate description of the image using vision model."""
        if not self.vision_available:
            return "Vision model not available - cannot generate description"
            
        try:
            inputs = self.processor(image, return_tensors="pt")
            out = self.model.generate(**inputs, max_length=50)
            description = self.processor.decode(out[0], skip_special_tokens=True)
            return description
        except Exception as e:
            return f"Error generating description: {e}"
            
    def evaluate_sprite_with_vision(self, sprite: Image.Image, expected_description: str, filename: str) -> Dict:
        """Evaluate sprite using vision-language model."""
        
        # Generate description
        observed_description = self.describe_image(sprite)
        
        # Check transparency
        is_transparent, transparency_note = self.check_transparency(sprite)
        
        # Simple matching heuristic (can be improved with more sophisticated NLP)
        description_match = self._check_description_match(expected_description, observed_description)
        
        # Format response
        evaluation_text = self.prompt_template.format(
            filename=filename,
            description=expected_description
        )
        
        evaluation_report = f"""---
Filename: {filename}  
Expected description: {expected_description}  
Observed description: {observed_description}  
Description match: {'Yes' if description_match else 'No'}  
Transparent background: {'Yes' if is_transparent else 'No'}  
Comments:  
– {transparency_note}
– Vision model confidence: {'High' if self.vision_available else 'Low (fallback)'}
---"""

        return {
            "filename": filename,
            "expected_description": expected_description,
            "observed_description": observed_description,
            "description_match": description_match,
            "transparent_background": is_transparent,
            "transparency_note": transparency_note,
            "evaluation_report": evaluation_report,
            "needs_refinement": not (description_match and is_transparent)
        }
        
    def _check_description_match(self, expected: str, observed: str) -> bool:
        """Simple heuristic to check if descriptions match."""
        expected_words = set(expected.lower().split())
        observed_words = set(observed.lower().split())
        
        # Remove common words
        common_words = {'a', 'an', 'the', 'is', 'are', 'with', 'and', 'or', 'of', 'in', 'on', 'at', 'pixel', 'background'}
        expected_words -= common_words
        observed_words -= common_words
        
        if len(expected_words) == 0:
            return True
            
        # Calculate overlap
        overlap = len(expected_words & observed_words)
        match_ratio = overlap / len(expected_words)
        
        # More lenient matching - accept if we see key terms
        key_terms = {'wizard', 'dragon', 'potion', 'bottle', 'hat', 'staff', 'fire', 'breathing', 'health'}
        key_overlap = len((expected_words | observed_words) & key_terms)
        
        # Accept if we have good ratio OR key terms are present
        return match_ratio >= 0.3 or key_overlap > 0

class PromptRefinerAgent:
    """Agent responsible for refining prompts based on vision feedback."""
    
    def __init__(self):
        self.refinement_instruction = """
You are a prompt refinement assistant for a pixel art generator.

You are given:
- An original generation prompt
- A report describing what was actually generated
- Notes about problems (e.g., incorrect content, missing transparency)

Your task:
- If the description in the image does not match the expected one, rephrase the prompt to be clearer and more specific.
- If the background is not transparent, explicitly instruct that the background must be fully transparent with alpha = 0.
- If both are fine, keep the prompt as is.

Return ONLY the corrected prompt, with no extra commentary.
"""
    
    def refine_prompt(self, original_prompt: str, evaluation_report: Dict, iteration: int = 1) -> str:
        """Refine prompt based on vision evaluation with progressive strategies."""
        
        # If everything is good, return original prompt
        if not evaluation_report["needs_refinement"]:
            return original_prompt
            
        refined_prompt = original_prompt
        
        # Progressive strategy based on iteration
        if iteration <= 5:
            # Early iterations: basic refinements
            if not evaluation_report["transparent_background"]:
                if "transparent background" not in refined_prompt.lower():
                    refined_prompt += ", transparent background, alpha channel, no background, isolated sprite"
                    
            # Refine description if it doesn't match
            if not evaluation_report["description_match"]:
                expected = evaluation_report["expected_description"]
                expected_words = expected.lower().split()
                key_words = [word for word in expected_words if len(word) > 3 and word not in refined_prompt.lower()]
                
                if key_words:
                    refined_prompt += f", {', '.join(key_words[:3])}"
                    
        elif iteration <= 10:
            # Mid iterations: more aggressive refinements
            refined_prompt += ", high quality, detailed pixel art, game character sprite"
            
            if "wizard" in original_prompt.lower():
                refined_prompt += ", fantasy wizard, blue robes, wizard hat, magic staff, RPG character"
            elif "dragon" in original_prompt.lower():
                refined_prompt += ", fantasy dragon, fire breathing, red scales, wings, mythical creature"
            elif "potion" in original_prompt.lower():
                refined_prompt += ", health potion, glass bottle, green liquid, healing item, game item"
                
        else:
            # Late iterations: completely different approaches
            if "wizard" in original_prompt.lower():
                refined_prompt = "16-bit pixel art style wizard character, blue wizard with pointed hat and wooden staff, fantasy RPG sprite, transparent background, no background, isolated character"
            elif "dragon" in original_prompt.lower():
                refined_prompt = "16-bit pixel art style red dragon, fire breathing dragon with wings, fantasy RPG monster sprite, transparent background, no background, isolated creature"
            elif "potion" in original_prompt.lower():
                refined_prompt = "16-bit pixel art style health potion, small glass bottle with green liquid, RPG healing item sprite, transparent background, no background, isolated item"
            else:
                refined_prompt = f"16-bit pixel art style {original_prompt}, game sprite, transparent background, no background, isolated sprite"
        
        # Always add quality enhancers
        quality_terms = ["sharp pixels", "clean edges", "retro game art", "pixel perfect"]
        for term in quality_terms:
            if term not in refined_prompt.lower():
                refined_prompt += f", {term}"
                break
                
        loguru.logger.info(f"Iteration {iteration} refined prompt: {original_prompt} -> {refined_prompt}")
        return refined_prompt

class PromptCondenserAgent:
    """Agent responsible for condensing prompts to fit within CLIP's 77 token limit."""
    
    def __init__(self):
        self.max_tokens = 75  # Leave some buffer
        
    def condense_prompt(self, prompt: str) -> str:
        """Condense a prompt to fit within CLIP token limits while preserving key information."""
        # Simple tokenization approximation (rough estimate)
        words = prompt.split()
        
        if len(words) <= self.max_tokens:
            return prompt
            
        loguru.logger.warning(f"Prompt has {len(words)} words, condensing to fit CLIP limit")
        
        # Priority keywords for sprite generation (keep these)
        priority_keywords = [
            'pixel art', 'sprite', 'transparent', 'background', 'isolated', 
            'health', 'potion', 'bottle', 'glass', 'liquid', 'green', 'red',
            'character', 'item', 'game', 'retro', '16-bit', '8-bit', 'sharp'
        ]
        
        # Remove duplicate words
        unique_words = []
        seen = set()
        for word in words:
            clean_word = word.lower().strip(',')
            if clean_word not in seen:
                unique_words.append(word)
                seen.add(clean_word)
        
        # If still too long, prioritize important terms
        if len(unique_words) > self.max_tokens:
            priority_words = []
            other_words = []
            
            for word in unique_words:
                if any(keyword in word.lower() for keyword in priority_keywords):
                    priority_words.append(word)
                else:
                    other_words.append(word)
            
            # Take priority words + as many others as fit
            condensed_words = priority_words[:self.max_tokens//2]
            remaining_space = self.max_tokens - len(condensed_words)
            condensed_words.extend(other_words[:remaining_space])
            
            condensed_prompt = ' '.join(condensed_words)
        else:
            condensed_prompt = ' '.join(unique_words)
            
        loguru.logger.info(f"Condensed prompt: {condensed_prompt}")
        return condensed_prompt

class TransparencyProcessorAgent:
    """Agent responsible for detecting and adding transparency to sprite images."""
    
    def __init__(self):
        self.setup_transparency_tools()
        
    def setup_transparency_tools(self):
        """Setup tools for transparency processing."""
        try:
            import rembg
            self.rembg_available = True
            loguru.logger.info("rembg library available for background removal")
        except ImportError:
            self.rembg_available = False
            loguru.logger.warning("rembg not available, using fallback transparency methods")
    
    def make_transparent(self, image: Image.Image) -> Image.Image:
        """Make image background transparent using multiple techniques."""
        
        # Convert to RGBA if not already
        if image.mode != 'RGBA':
            image = image.convert('RGBA')
        
        # Method 1: Try rembg if available
        if self.rembg_available:
            try:
                transparent_image = self._remove_background_rembg(image)
                if self._has_transparency(transparent_image):
                    loguru.logger.info("Successfully removed background using rembg")
                    return transparent_image
            except Exception as e:
                loguru.logger.warning(f"rembg failed: {e}")
        
        # Method 2: Color-based background removal
        transparent_image = self._remove_background_color_based(image)
        if self._has_transparency(transparent_image):
            loguru.logger.info("Successfully removed background using color-based method")
            return transparent_image
        
        # Method 3: Edge-based background detection
        transparent_image = self._remove_background_edge_based(image)
        if self._has_transparency(transparent_image):
            loguru.logger.info("Successfully removed background using edge-based method")
            return transparent_image
        
        loguru.logger.warning("Could not create transparent background, returning original")
        return image
    
    def _remove_background_rembg(self, image: Image.Image) -> Image.Image:
        """Remove background using rembg library."""
        from rembg import remove
        import io
        
        # Convert PIL image to bytes
        img_byte_arr = io.BytesIO()
        image.convert('RGB').save(img_byte_arr, format='PNG')
        img_byte_arr = img_byte_arr.getvalue()
        
        # Remove background
        result = remove(img_byte_arr)
        
        # Convert back to PIL image
        return Image.open(io.BytesIO(result)).convert('RGBA')
    
    def _remove_background_color_based(self, image: Image.Image) -> Image.Image:
        """Remove background based on dominant colors (corners)."""
        data = np.array(image)
        h, w = data.shape[:2]
        
        # Sample corner colors to determine background
        corners = [
            data[0, 0],      # top-left
            data[0, w-1],    # top-right  
            data[h-1, 0],    # bottom-left
            data[h-1, w-1]   # bottom-right
        ]
        
        # Find most common corner color
        from collections import Counter
        corner_colors = [tuple(corner[:3]) for corner in corners]
        bg_color = Counter(corner_colors).most_common(1)[0][0]
        
        # Create mask for background color (with tolerance)
        tolerance = 30
        mask = np.all(np.abs(data[:, :, :3] - bg_color) <= tolerance, axis=2)
        
        # Set background pixels to transparent
        data[:, :, 3] = np.where(mask, 0, 255)
        
        return Image.fromarray(data, 'RGBA')
    
    def _remove_background_edge_based(self, image: Image.Image) -> Image.Image:
        """Remove background using edge detection."""
        # Convert to numpy array
        data = np.array(image)
        
        # Convert to grayscale for edge detection
        gray = cv2.cvtColor(data[:, :, :3], cv2.COLOR_RGB2GRAY)
        
        # Find edges
        edges = cv2.Canny(gray, 50, 150)
        
        # Create contours
        contours, _ = cv2.findContours(edges, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
        
        if contours:
            # Find largest contour (assumed to be main sprite)
            largest_contour = max(contours, key=cv2.contourArea)
            
            # Create mask
            mask = np.zeros(gray.shape, dtype=np.uint8)
            cv2.fillPoly(mask, [largest_contour], 255)
            
            # Apply mask to alpha channel
            data[:, :, 3] = mask
            
            return Image.fromarray(data, 'RGBA')
        
        return image
    
    def _has_transparency(self, image: Image.Image) -> bool:
        """Check if image has meaningful transparency."""
        if image.mode != 'RGBA':
            return False
            
        alpha = np.array(image)[:, :, 3]
        transparent_pixels = np.sum(alpha < 255)
        total_pixels = alpha.size
        
        # Consider transparent if more than 10% of pixels are transparent
        transparency_ratio = transparent_pixels / total_pixels
        return transparency_ratio > 0.1
    
    def evaluate_transparency(self, image: Image.Image) -> Dict:
        """Evaluate transparency quality of an image."""
        if image.mode != 'RGBA':
            return {
                "has_alpha_channel": False,
                "transparency_ratio": 0.0,
                "background_removed": False
            }
        
        alpha = np.array(image)[:, :, 3]
        transparent_pixels = np.sum(alpha < 255)
        total_pixels = alpha.size
        transparency_ratio = transparent_pixels / total_pixels
        
        return {
            "has_alpha_channel": True,
            "transparency_ratio": transparency_ratio,
            "background_removed": transparency_ratio > 0.1
        }

class VisionEnhancedMultiAgentSystem:
    """Orchestrates the vision-enhanced multi-agent sprite generation system."""
    
    def __init__(self, max_iterations: int = 15):
        self.generator = SpriteGeneratorAgent()
        self.vision_evaluator = VisionEvaluatorAgent()
        self.prompt_refiner = PromptRefinerAgent()
        self.prompt_condenser = PromptCondenserAgent()
        self.transparency_processor = TransparencyProcessorAgent()
        self.max_iterations = max_iterations
        
    def generate_quality_sprite(self, 
                              initial_prompt: str, 
                              expected_description: str,
                              size: Tuple[int, int], 
                              output_path: str) -> Dict:
        """Generate a sprite with iterative quality improvement using vision evaluation."""
        
        loguru.logger.info(f"Starting vision-enhanced sprite generation for: {initial_prompt}")
        
        current_prompt = initial_prompt
        best_sprite = None
        best_evaluation = None
        iteration_history = []
        
        for iteration in range(self.max_iterations):
            loguru.logger.info(f"Iteration {iteration + 1}/{self.max_iterations}")
            
            # Condense prompt to fit CLIP token limits
            condensed_prompt = self.prompt_condenser.condense_prompt(current_prompt)
            
            # Generate sprite
            sprite = self.generator.generate_sprite(condensed_prompt, size)
            
            # Process for transparency if needed
            if not self._has_transparency_simple(sprite):
                loguru.logger.info("Attempting to add transparency to sprite")
                transparent_sprite = self.transparency_processor.make_transparent(sprite)
                transparency_info = self.transparency_processor.evaluate_transparency(transparent_sprite)
                
                if transparency_info["background_removed"]:
                    sprite = transparent_sprite
                    loguru.logger.info(f"Background removal successful: {transparency_info['transparency_ratio']:.2f} transparency ratio")
                else:
                    loguru.logger.warning("Background removal failed, using original sprite")
            
            # Evaluate with vision model
            filename = f"iteration_{iteration + 1}.png"
            evaluation = self.vision_evaluator.evaluate_sprite_with_vision(
                sprite, expected_description, filename
            )
            
            loguru.logger.info(f"Vision evaluation: Match={evaluation['description_match']}, Transparent={evaluation['transparent_background']}")
            loguru.logger.info(f"Observed: {evaluation['observed_description']}")
            
            # Track iteration
            iteration_info = {
                "iteration": iteration + 1,
                "prompt": current_prompt,
                "condensed_prompt": condensed_prompt,
                "evaluation": evaluation,
                "needs_refinement": evaluation["needs_refinement"]
            }
            iteration_history.append(iteration_info)
            
            # Update best sprite if this one is better (prioritize correct description and transparency)
            if best_evaluation is None or self._is_better_evaluation(evaluation, best_evaluation):
                best_sprite = sprite
                best_evaluation = evaluation
                
            # Check if quality target is met
            if not evaluation["needs_refinement"]:
                loguru.logger.info(f"Quality target achieved in {iteration + 1} iterations!")
                break
                
            # Refine prompt based on vision feedback
            if iteration < self.max_iterations - 1:
                current_prompt = self.prompt_refiner.refine_prompt(current_prompt, evaluation, iteration + 1)
                
        # Save the best sprite
        output_path = Path(output_path)
        output_path.parent.mkdir(parents=True, exist_ok=True)
        best_sprite.save(output_path)
        
        # Save generation report
        report_path = output_path.with_suffix('.json')
        report = {
            "initial_prompt": initial_prompt,
            "expected_description": expected_description,
            "final_prompt": current_prompt,
            "best_evaluation": best_evaluation,
            "iterations": len(iteration_history),
            "target_achieved": not best_evaluation["needs_refinement"],
            "iteration_history": iteration_history
        }
        
        with open(report_path, 'w') as f:
            json.dump(report, f, indent=2)
            
        loguru.logger.info(f"Best sprite saved to: {output_path}")
        loguru.logger.info(f"Generation report saved to: {report_path}")
        
        return report
        
    def _has_transparency_simple(self, image: Image.Image) -> bool:
        """Quick check if image has transparency."""
        return image.mode == 'RGBA' and np.array(image)[:, :, 3].min() < 255
    
    def _is_better_evaluation(self, eval1: Dict, eval2: Dict) -> bool:
        """Determine if eval1 is better than eval2."""
        # Priority: description match > transparency > overall quality
        if eval1["description_match"] and not eval2["description_match"]:
            return True
        elif eval2["description_match"] and not eval1["description_match"]:
            return False
        elif eval1["transparent_background"] and not eval2["transparent_background"]:
            return True
        elif eval2["transparent_background"] and not eval1["transparent_background"]:
            return False
        else:
            # If both have same description/transparency status, prefer the one that needs less refinement
            return not eval1["needs_refinement"] and eval2["needs_refinement"]

def main():
    """Main function to test the vision-enhanced multi-agent system."""
    system = VisionEnhancedMultiAgentSystem(max_iterations=15)
    
    # Test sprites with expected descriptions
    test_sprites = [
        {
            "prompt": "blue wizard with pointy hat and staff", 
            "description": "blue wizard character wearing a pointed hat and holding a magical staff",
            "size": (32, 32), 
            "output": "vision_output/wizard_32x32.png"
        },
        {
            "prompt": "red dragon breathing fire", 
            "description": "red dragon creature with wings breathing flames",
            "size": (32, 32), 
            "output": "vision_output/dragon_32x32.png"
        },
        {
            "prompt": "small health potion bottle", 
            "description": "small glass bottle containing red or green liquid health potion",
            "size": (16, 16), 
            "output": "vision_output/potion_16x16.png"
        },
    ]
    
    for sprite_config in test_sprites:
        try:
            report = system.generate_quality_sprite(
                sprite_config["prompt"], 
                sprite_config["description"],
                sprite_config["size"], 
                sprite_config["output"]
            )
            loguru.logger.info(f"Completed {sprite_config['prompt']}: Target achieved: {report['target_achieved']}")
        except Exception as e:
            loguru.logger.error(f"Failed to generate {sprite_config['prompt']}: {e}")

if __name__ == "__main__":
    main()
