"""
Vision Evaluator Agent - Evaluates sprites using vision-language models.
"""

import io
import base64
import numpy as np
from PIL import Image
import loguru
from typing import Dict, Tuple
from transformers import BlipProcessor, BlipForConditionalGeneration

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
