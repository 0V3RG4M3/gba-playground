"""
Multi-agent System - Orchestrates all agents for quality sprite generation.
"""

import json
import numpy as np
from pathlib import Path
from PIL import Image
import loguru
from typing import Dict, Tuple

from .sprite_generator import SpriteGeneratorAgent
from .vision_evaluator import VisionEvaluatorAgent
from .prompt_refiner import PromptRefinerAgent
from .prompt_condenser import PromptCondenserAgent
from .transparency_processor import TransparencyProcessorAgent

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
