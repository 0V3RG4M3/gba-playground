#!/usr/bin/env python3
"""
Test script for the modular epjtools package.
"""

import sys
from pathlib import Path
sys.path.append(str(Path(__file__).parent))

import loguru
from epjtools import VisionEnhancedMultiAgentSystem

def test_modular_system():
    """Test the modular multi-agent system."""
    
    loguru.logger.add(sys.stderr, level="INFO", format="{time} {level} {message}", colorize=True)
    loguru.logger.info("Testing modular EPJTools system...")
    
    # Initialize system with lower max iterations for testing
    system = VisionEnhancedMultiAgentSystem(max_iterations=5)
    
    # Test with a simple prompt
    test_sprite = {
        "prompt": "green health potion", 
        "description": "small green health potion in a glass bottle",
        "size": (16, 16), 
        "output": "vision_output/test_modular_potion.png"
    }
    
    try:
        report = system.generate_quality_sprite(
            test_sprite["prompt"], 
            test_sprite["description"],
            test_sprite["size"], 
            test_sprite["output"]
        )
        
        loguru.logger.info(f"✅ Modular system test completed!")
        loguru.logger.info(f"Target achieved: {report['target_achieved']}")
        loguru.logger.info(f"Total iterations: {report['iterations']}")
        
        if report['target_achieved']:
            loguru.logger.info("🎯 Perfect match achieved with modular system!")
        else:
            loguru.logger.info("⚠️  Reached max iterations without perfect match")
            
    except Exception as e:
        loguru.logger.error(f"Modular system test failed: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    test_modular_system()
