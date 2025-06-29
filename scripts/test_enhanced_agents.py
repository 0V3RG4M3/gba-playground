#!/usr/bin/env python3
"""
Test script for the enhanced multi-agent system with:
1. Prompt condensing to handle CLIP token limits
2. Transparency processing for background removal
"""

import sys
from pathlib import Path
sys.path.append(str(Path(__file__).parent))

from vision_enhanced_multi_agent import VisionEnhancedMultiAgentSystem
import loguru

def test_enhanced_agents():
    """Test the enhanced multi-agent system with new capabilities."""
    
    loguru.logger.add(sys.stderr, level="INFO", format="{time} {level} {message}", colorize=True)
    loguru.logger.info("Starting enhanced agents test...")
    
    # Initialize system with lower max iterations for testing
    system = VisionEnhancedMultiAgentSystem(max_iterations=8)
    
    # Test with a simple prompt that should be easy to match
    test_sprite = {
        "prompt": "red health potion bottle", 
        "description": "small red health potion in a glass bottle",
        "size": (16, 16), 
        "output": "vision_output/test_enhanced_potion.png"
    }
    
    try:
        report = system.generate_quality_sprite(
            test_sprite["prompt"], 
            test_sprite["description"],
            test_sprite["size"], 
            test_sprite["output"]
        )
        
        loguru.logger.info(f"Test completed!")
        loguru.logger.info(f"Target achieved: {report['target_achieved']}")
        loguru.logger.info(f"Total iterations: {report['iterations']}")
        
        # Check if we successfully handled the token limits and transparency
        if report['iterations'] > 0:
            last_iteration = report['iteration_history'][-1]
            if 'condensed_prompt' in last_iteration:
                loguru.logger.info("✅ Prompt condensing was applied")
            else:
                loguru.logger.warning("❌ Prompt condensing was not applied")
                
        if report['target_achieved']:
            loguru.logger.info("🎯 Perfect match achieved!")
        else:
            loguru.logger.info("⚠️  Reached max iterations without perfect match")
            
    except Exception as e:
        loguru.logger.error(f"Test failed with error: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    test_enhanced_agents()
