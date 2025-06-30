"""
Prompt Refiner Agent - Refines prompts based on vision feedback.
"""

import loguru
from typing import Dict

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
