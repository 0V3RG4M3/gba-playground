# Quick test for single sprite with enhanced iteration
import sys
sys.path.append('/home/fmn/workspace/gba-playground/scripts')

from vision_enhanced_multi_agent import VisionEnhancedMultiAgentSystem
import loguru

def test_single_sprite():
    """Test a single sprite with enhanced iteration."""
    system = VisionEnhancedMultiAgentSystem(max_iterations=15)
    
    # Test with just the potion (most promising from previous runs)
    try:
        loguru.logger.info("Starting enhanced iteration test...")
        report = system.generate_quality_sprite(
            "small health potion bottle", 
            "small glass bottle containing red or green liquid health potion",
            (16, 16), 
            "vision_output/test_potion_enhanced.png"
        )
        loguru.logger.info(f"Test completed! Target achieved: {report['target_achieved']}")
        loguru.logger.info(f"Total iterations: {report['iterations']}")
        
        if report['target_achieved']:
            loguru.logger.success("🎉 SUCCESS! We found a matching sprite!")
        else:
            loguru.logger.warning(f"Reached max iterations ({report['iterations']}) without perfect match")
            
    except Exception as e:
        loguru.logger.error(f"Error during test: {e}")

if __name__ == "__main__":
    test_single_sprite()
