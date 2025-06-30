"""
Transparency Processor Agent - Detects and adds transparency to sprite images.
"""

import io
import numpy as np
import cv2
from PIL import Image
import loguru
from typing import Dict
from collections import Counter

class TransparencyProcessorAgent:
    """Agent responsible for detecting and adding transparency to sprite images."""
    
    def __init__(self):
        self.setup_transparency_tools()
        
    def setup_transparency_tools(self):
        """Setup tools for transparency processing."""
        try:
            import rembg
            self.rembg_available = True
            loguru.logger.info("rembg library available for background removal")
        except ImportError:
            self.rembg_available = False
            loguru.logger.warning("rembg not available, using fallback transparency methods")
    
    def make_transparent(self, image: Image.Image) -> Image.Image:
        """Make image background transparent using multiple techniques."""
        
        # Convert to RGBA if not already
        if image.mode != 'RGBA':
            image = image.convert('RGBA')
        
        # Method 1: Try rembg if available
        if self.rembg_available:
            try:
                transparent_image = self._remove_background_rembg(image)
                if self._has_transparency(transparent_image):
                    loguru.logger.info("Successfully removed background using rembg")
                    return transparent_image
            except Exception as e:
                loguru.logger.warning(f"rembg failed: {e}")
        
        # Method 2: Color-based background removal
        transparent_image = self._remove_background_color_based(image)
        if self._has_transparency(transparent_image):
            loguru.logger.info("Successfully removed background using color-based method")
            return transparent_image
        
        # Method 3: Edge-based background detection
        transparent_image = self._remove_background_edge_based(image)
        if self._has_transparency(transparent_image):
            loguru.logger.info("Successfully removed background using edge-based method")
            return transparent_image
        
        loguru.logger.warning("Could not create transparent background, returning original")
        return image
    
    def _remove_background_rembg(self, image: Image.Image) -> Image.Image:
        """Remove background using rembg library."""
        from rembg import remove
        
        # Convert PIL image to bytes
        img_byte_arr = io.BytesIO()
        image.convert('RGB').save(img_byte_arr, format='PNG')
        img_byte_arr = img_byte_arr.getvalue()
        
        # Remove background
        result = remove(img_byte_arr)
        
        # Convert back to PIL image
        return Image.open(io.BytesIO(result)).convert('RGBA')
    
    def _remove_background_color_based(self, image: Image.Image) -> Image.Image:
        """Remove background based on dominant colors (corners)."""
        data = np.array(image)
        h, w = data.shape[:2]
        
        # Sample corner colors to determine background
        corners = [
            data[0, 0],      # top-left
            data[0, w-1],    # top-right  
            data[h-1, 0],    # bottom-left
            data[h-1, w-1]   # bottom-right
        ]
        
        # Find most common corner color
        corner_colors = [tuple(corner[:3]) for corner in corners]
        bg_color = Counter(corner_colors).most_common(1)[0][0]
        
        # Create mask for background color (with tolerance)
        tolerance = 30
        mask = np.all(np.abs(data[:, :, :3] - bg_color) <= tolerance, axis=2)
        
        # Set background pixels to transparent
        data[:, :, 3] = np.where(mask, 0, 255)
        
        return Image.fromarray(data, 'RGBA')
    
    def _remove_background_edge_based(self, image: Image.Image) -> Image.Image:
        """Remove background using edge detection."""
        # Convert to numpy array
        data = np.array(image)
        
        # Convert to grayscale for edge detection
        gray = cv2.cvtColor(data[:, :, :3], cv2.COLOR_RGB2GRAY)
        
        # Find edges
        edges = cv2.Canny(gray, 50, 150)
        
        # Create contours
        contours, _ = cv2.findContours(edges, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
        
        if contours:
            # Find largest contour (assumed to be main sprite)
            largest_contour = max(contours, key=cv2.contourArea)
            
            # Create mask
            mask = np.zeros(gray.shape, dtype=np.uint8)
            cv2.fillPoly(mask, [largest_contour], 255)
            
            # Apply mask to alpha channel
            data[:, :, 3] = mask
            
            return Image.fromarray(data, 'RGBA')
        
        return image
    
    def _has_transparency(self, image: Image.Image) -> bool:
        """Check if image has meaningful transparency."""
        if image.mode != 'RGBA':
            return False
            
        alpha = np.array(image)[:, :, 3]
        transparent_pixels = np.sum(alpha < 255)
        total_pixels = alpha.size
        
        # Consider transparent if more than 10% of pixels are transparent
        transparency_ratio = transparent_pixels / total_pixels
        return transparency_ratio > 0.1
    
    def evaluate_transparency(self, image: Image.Image) -> Dict:
        """Evaluate transparency quality of an image."""
        if image.mode != 'RGBA':
            return {
                "has_alpha_channel": False,
                "transparency_ratio": 0.0,
                "background_removed": False
            }
        
        alpha = np.array(image)[:, :, 3]
        transparent_pixels = np.sum(alpha < 255)
        total_pixels = alpha.size
        transparency_ratio = transparent_pixels / total_pixels
        
        return {
            "has_alpha_channel": True,
            "transparency_ratio": transparency_ratio,
            "background_removed": transparency_ratio > 0.1
        }
