# Multi-agent sprite generation system
# Agent 1: Sprite Generator - Creates sprites based on prompts
# Agent 2: Quality Evaluator - Assesses sprite quality and suggests improvements

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
            
        # Enhanced prompt for pixel art
        enhanced_prompt = f"pixel art style, 8-bit video game sprite, {prompt}, simple colors, retro gaming, clean pixels, sharp edges"
        
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

class QualityEvaluatorAgent:
    """Agent responsible for evaluating sprite quality and suggesting improvements."""
    
    def __init__(self):
        self.quality_criteria = {
            "color_variety": {"weight": 0.2, "min_score": 0.3},
            "edge_sharpness": {"weight": 0.25, "min_score": 0.4},
            "contrast": {"weight": 0.2, "min_score": 0.4},
            "pixel_art_style": {"weight": 0.25, "min_score": 0.5},
            "composition": {"weight": 0.1, "min_score": 0.3}
        }
        
    def evaluate_sprite(self, sprite: Image.Image, target_description: str) -> Dict:
        """Evaluate the quality of a generated sprite."""
        scores = {}
        
        # Convert to numpy array for analysis
        sprite_array = np.array(sprite)
        
        # 1. Color variety analysis
        scores["color_variety"] = self._analyze_color_variety(sprite_array)
        
        # 2. Edge sharpness analysis
        scores["edge_sharpness"] = self._analyze_edge_sharpness(sprite_array)
        
        # 3. Contrast analysis
        scores["contrast"] = self._analyze_contrast(sprite_array)
        
        # 4. Pixel art style analysis
        scores["pixel_art_style"] = self._analyze_pixel_art_style(sprite_array)
        
        # 5. Composition analysis
        scores["composition"] = self._analyze_composition(sprite_array)
        
        # Calculate overall score
        overall_score = sum(
            scores[criterion] * self.quality_criteria[criterion]["weight"]
            for criterion in scores
        )
        
        # Determine if sprite meets quality standards
        quality_met = all(
            scores[criterion] >= self.quality_criteria[criterion]["min_score"]
            for criterion in scores
        )
        
        # Convert numpy types to native Python types for JSON serialization
        scores_converted = {k: float(v) if hasattr(v, 'item') else v for k, v in scores.items()}
        
        return {
            "scores": scores_converted,
            "overall_score": float(overall_score),
            "quality_met": quality_met,
            "feedback": self._generate_feedback(scores, target_description)
        }
        
    def _analyze_color_variety(self, sprite_array: np.ndarray) -> float:
        """Analyze color variety in the sprite."""
        # Count unique colors
        if len(sprite_array.shape) == 3:
            colors = sprite_array.reshape(-1, sprite_array.shape[2])
            unique_colors = len(np.unique(colors.view(np.void), axis=0))
        else:
            unique_colors = len(np.unique(sprite_array))
            
        # Score based on color count (good pixel art has 4-16 colors)
        optimal_range = (4, 16)
        if optimal_range[0] <= unique_colors <= optimal_range[1]:
            return min(1.0, unique_colors / optimal_range[1])
        else:
            return max(0.1, 1.0 - abs(unique_colors - optimal_range[1]) / optimal_range[1])
            
    def _analyze_edge_sharpness(self, sprite_array: np.ndarray) -> float:
        """Analyze edge sharpness (pixel art should have sharp edges)."""
        # Convert to grayscale if needed
        if len(sprite_array.shape) == 3:
            gray = cv2.cvtColor(sprite_array, cv2.COLOR_RGB2GRAY)
        else:
            gray = sprite_array
            
        # Apply Laplacian edge detection
        laplacian = cv2.Laplacian(gray, cv2.CV_64F)
        sharpness = np.var(laplacian)
        
        # Normalize to 0-1 range (higher variance = sharper edges)
        return min(1.0, sharpness / 1000.0)
        
    def _analyze_contrast(self, sprite_array: np.ndarray) -> float:
        """Analyze contrast in the sprite."""
        if len(sprite_array.shape) == 3:
            gray = cv2.cvtColor(sprite_array, cv2.COLOR_RGB2GRAY)
        else:
            gray = sprite_array
            
        # Calculate contrast using standard deviation
        contrast = np.std(gray) / 255.0
        return min(1.0, contrast * 2)  # Scale up since pixel art can have high contrast
        
    def _analyze_pixel_art_style(self, sprite_array: np.ndarray) -> float:
        """Analyze if the image looks like pixel art."""
        # Check for repeated pixel patterns (blocky appearance)
        h, w = sprite_array.shape[:2]
        
        # Sample small regions and check for uniformity
        block_size = max(1, min(h, w) // 16)
        uniform_blocks = 0
        total_blocks = 0
        
        for y in range(0, h - block_size, block_size):
            for x in range(0, w - block_size, block_size):
                block = sprite_array[y:y+block_size, x:x+block_size]
                if len(block.shape) == 3:
                    # Check if block has uniform colors
                    std_dev = np.std(block, axis=(0, 1))
                    if np.mean(std_dev) < 10:  # Low variation = uniform block
                        uniform_blocks += 1
                total_blocks += 1
                
        if total_blocks == 0:
            return 0.5
        return uniform_blocks / total_blocks
        
    def _analyze_composition(self, sprite_array: np.ndarray) -> float:
        """Analyze composition (centered, not too empty, not too cluttered)."""
        h, w = sprite_array.shape[:2]
        
        # Convert to grayscale for analysis
        if len(sprite_array.shape) == 3:
            gray = cv2.cvtColor(sprite_array, cv2.COLOR_RGB2GRAY)
        else:
            gray = sprite_array
            
        # Check for content distribution
        # Calculate center of mass
        moments = cv2.moments(gray)
        if moments["m00"] != 0:
            cx = int(moments["m10"] / moments["m00"])
            cy = int(moments["m01"] / moments["m00"])
            
            # Score based on how centered the content is
            center_x, center_y = w // 2, h // 2
            distance_from_center = np.sqrt((cx - center_x)**2 + (cy - center_y)**2)
            max_distance = np.sqrt(center_x**2 + center_y**2)
            centering_score = 1.0 - (distance_from_center / max_distance)
            
            # Check for content density (not too empty, not too cluttered)
            non_zero_pixels = np.count_nonzero(gray)
            total_pixels = h * w
            density = non_zero_pixels / total_pixels
            
            # Optimal density is around 30-70%
            if 0.3 <= density <= 0.7:
                density_score = 1.0
            else:
                density_score = max(0.1, 1.0 - abs(density - 0.5) * 2)
                
            return (centering_score + density_score) / 2
        else:
            return 0.1  # Empty image
            
    def _generate_feedback(self, scores: Dict, target_description: str) -> List[str]:
        """Generate feedback for improving the sprite."""
        feedback = []
        
        if scores["color_variety"] < self.quality_criteria["color_variety"]["min_score"]:
            feedback.append("Increase color variety - use more distinct colors for different parts")
            
        if scores["edge_sharpness"] < self.quality_criteria["edge_sharpness"]["min_score"]:
            feedback.append("Make edges sharper - add 'crisp edges', 'no antialiasing' to prompt")
            
        if scores["contrast"] < self.quality_criteria["contrast"]["min_score"]:
            feedback.append("Increase contrast - use brighter highlights and darker shadows")
            
        if scores["pixel_art_style"] < self.quality_criteria["pixel_art_style"]["min_score"]:
            feedback.append("Make more pixelated - add '16-bit style', 'blocky pixels' to prompt")
            
        if scores["composition"] < self.quality_criteria["composition"]["min_score"]:
            feedback.append("Improve composition - center the subject better, adjust size")
            
        if not feedback:
            feedback.append("Sprite quality is good!")
            
        return feedback

class MultiAgentSpriteSystem:
    """Orchestrates the multi-agent sprite generation system."""
    
    def __init__(self, max_iterations: int = 5):
        self.generator = SpriteGeneratorAgent()
        self.evaluator = QualityEvaluatorAgent()
        self.max_iterations = max_iterations
        
    def generate_quality_sprite(self, 
                              initial_prompt: str, 
                              size: Tuple[int, int], 
                              output_path: str,
                              target_score: float = 0.7) -> Dict:
        """Generate a sprite with iterative quality improvement."""
        
        loguru.logger.info(f"Starting multi-agent sprite generation for: {initial_prompt}")
        
        current_prompt = initial_prompt
        best_sprite = None
        best_score = 0.0
        iteration_history = []
        
        for iteration in range(self.max_iterations):
            loguru.logger.info(f"Iteration {iteration + 1}/{self.max_iterations}")
            
            # Generate sprite
            sprite = self.generator.generate_sprite(current_prompt, size)
            
            # Evaluate quality
            evaluation = self.evaluator.evaluate_sprite(sprite, initial_prompt)
            
            loguru.logger.info(f"Quality scores: {evaluation['scores']}")
            loguru.logger.info(f"Overall score: {evaluation['overall_score']:.3f}")
            
            # Track iteration
            iteration_info = {
                "iteration": iteration + 1,
                "prompt": current_prompt,
                "scores": {k: float(v) if hasattr(v, 'item') else v for k, v in evaluation["scores"].items()},
                "overall_score": float(evaluation["overall_score"]),
                "feedback": evaluation["feedback"]
            }
            iteration_history.append(iteration_info)
            
            # Update best sprite if this one is better
            if evaluation["overall_score"] > best_score:
                best_sprite = sprite
                best_score = evaluation["overall_score"]
                
            # Check if quality target is met
            if evaluation["quality_met"] and evaluation["overall_score"] >= target_score:
                loguru.logger.info(f"Quality target achieved in {iteration + 1} iterations!")
                break
                
            # Generate improved prompt based on feedback
            if iteration < self.max_iterations - 1:
                current_prompt = self._improve_prompt(current_prompt, evaluation["feedback"])
                loguru.logger.info(f"Improved prompt: {current_prompt}")
                
        # Save the best sprite
        output_path = Path(output_path)
        output_path.parent.mkdir(parents=True, exist_ok=True)
        best_sprite.save(output_path)
        
        # Save generation report
        report_path = output_path.with_suffix('.json')
        report = {
            "initial_prompt": initial_prompt,
            "final_prompt": current_prompt,
            "best_score": float(best_score),
            "iterations": len(iteration_history),
            "target_achieved": bool(best_score >= target_score),
            "iteration_history": iteration_history
        }
        
        with open(report_path, 'w') as f:
            json.dump(report, f, indent=2)
            
        loguru.logger.info(f"Best sprite saved to: {output_path}")
        loguru.logger.info(f"Generation report saved to: {report_path}")
        
        return report
        
    def _improve_prompt(self, current_prompt: str, feedback: List[str]) -> str:
        """Improve the prompt based on evaluator feedback."""
        improvements = []
        
        for feedback_item in feedback:
            if "color variety" in feedback_item.lower():
                improvements.append("vibrant colors")
                improvements.append("multiple colors")
                
            elif "sharp" in feedback_item.lower() or "edge" in feedback_item.lower():
                improvements.append("crisp edges")
                improvements.append("no antialiasing")
                improvements.append("sharp pixels")
                
            elif "contrast" in feedback_item.lower():
                improvements.append("high contrast")
                improvements.append("bright highlights")
                improvements.append("dark shadows")
                
            elif "pixel" in feedback_item.lower() or "blocky" in feedback_item.lower():
                improvements.append("16-bit style")
                improvements.append("blocky pixels")
                improvements.append("retro game graphics")
                
            elif "composition" in feedback_item.lower() or "center" in feedback_item.lower():
                improvements.append("centered subject")
                improvements.append("well-composed")
                improvements.append("clear silhouette")
                
        # Add improvements to prompt
        if improvements:
            unique_improvements = list(set(improvements))
            improved_prompt = f"{current_prompt}, {', '.join(unique_improvements)}"
            return improved_prompt
            
        return current_prompt

def main():
    """Main function to test the multi-agent system."""
    system = MultiAgentSpriteSystem(max_iterations=3)
    
    # Test sprites
    test_sprites = [
        ("blue wizard with pointy hat and staff", (32, 32), "multi_agent_output/wizard_32x32.png"),
        ("red dragon breathing fire", (32, 32), "multi_agent_output/dragon_32x32.png"),
        ("green orc warrior with axe", (32, 32), "multi_agent_output/orc_32x32.png"),
        ("small health potion bottle", (16, 16), "multi_agent_output/potion_16x16.png"),
    ]
    
    for prompt, size, output_path in test_sprites:
        try:
            report = system.generate_quality_sprite(prompt, size, output_path)
            loguru.logger.info(f"Completed {prompt}: Final score {report['best_score']:.3f}")
        except Exception as e:
            loguru.logger.error(f"Failed to generate {prompt}: {e}")

if __name__ == "__main__":
    main()
