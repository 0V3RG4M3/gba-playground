"""
Prompt Condenser Agent - Condenses prompts to fit within CLIP's 77 token limit.
"""

import loguru

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
