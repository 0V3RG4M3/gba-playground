#!/usr/bin/env python3
"""
Main script to run the vision-enhanced multi-agent system.
"""

import sys
import loguru
from egjtools import VisionEnhancedMultiAgentSystem

def main():
    """Main function to test the vision-enhanced multi-agent system."""
    loguru.logger.add(sys.stderr, level="INFO", format="{time} {level} {message}", colorize=True)
    
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
