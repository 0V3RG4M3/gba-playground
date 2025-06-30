#!/usr/bin/env python3
"""
Simple test for egjtools package import.
"""

print("Starting import test...")

try:
    print("Testing individual imports...")
    
    print("1. Importing SpriteGeneratorAgent...")
    from egjtools.sprite_generator import SpriteGeneratorAgent
    print("   ✅ SpriteGeneratorAgent imported successfully")
    
    print("2. Importing VisionEvaluatorAgent...")
    from egjtools.vision_evaluator import VisionEvaluatorAgent
    print("   ✅ VisionEvaluatorAgent imported successfully")
    
    print("3. Importing PromptRefinerAgent...")
    from egjtools.prompt_refiner import PromptRefinerAgent
    print("   ✅ PromptRefinerAgent imported successfully")
    
    print("4. Importing PromptCondenserAgent...")
    from egjtools.prompt_condenser import PromptCondenserAgent
    print("   ✅ PromptCondenserAgent imported successfully")
    
    print("5. Importing TransparencyProcessorAgent...")
    from egjtools.transparency_processor import TransparencyProcessorAgent
    print("   ✅ TransparencyProcessorAgent imported successfully")
    
    print("6. Importing VisionEnhancedMultiAgentSystem...")
    from egjtools.multi_agent_system import VisionEnhancedMultiAgentSystem
    print("   ✅ VisionEnhancedMultiAgentSystem imported successfully")
    
    print("7. Testing package-level import...")
    from egjtools import VisionEnhancedMultiAgentSystem as VEMAS
    print("   ✅ Package-level import successful")
    
    print("\n🎉 All imports successful! EPJTools package is working correctly.")
    
except ImportError as e:
    print(f"❌ Import Error: {e}")
except Exception as e:
    print(f"❌ Unexpected Error: {e}")
    import traceback
    traceback.print_exc()
