from diffusers import DiffusionPipeline, LCMScheduler
import torch
import gc

model_id = "stabilityai/stable-diffusion-xl-base-1.0"
lcm_lora_id = "latent-consistency/lcm-lora-sdxl"

# Use lower precision and memory optimizations
pipe = DiffusionPipeline.from_pretrained(
    model_id, 
    variant="fp16", 
    torch_dtype=torch.float16,
    use_safetensors=True
)
pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config)

pipe.load_lora_weights(lcm_lora_id, adapter_name="lora")
# Note: Pixel art LoRA file not found, continuing with just LCM LoRA
# If you have a pixel art LoRA file, place it in this directory and uncomment the next line:
# pipe.load_lora_weights("./pixel-art-xl.safetensors", adapter_name="pixel", weight_name="pixel-art-xl.safetensors")

pipe.set_adapters(["lora"], adapter_weights=[1.0])

# Enable all memory efficient features
pipe.enable_attention_slicing()
pipe.enable_model_cpu_offload()
pipe.enable_vae_slicing()
pipe.enable_vae_tiling()

# Enable sequential CPU offload for maximum memory savings (slower but uses much less GPU memory)
pipe.enable_sequential_cpu_offload()

prompt = "pixel art style, a cute corgi, 16-bit, retro game art, pixelated"
negative_prompt = "3d render, realistic, blurry, smooth, high resolution"

num_images = 3  # Reduced from 9 to save memory

for i in range(num_images):
    print(f"Generating image {i+1}/{num_images}...")
    
    # Clear GPU cache before each generation
    if torch.cuda.is_available():
        torch.cuda.empty_cache()
    gc.collect()
    
    img = pipe(
        prompt=prompt,
        negative_prompt=negative_prompt,
        num_inference_steps=4,  # Reduced from 8 for faster generation
        guidance_scale=1.0,     # Reduced for LCM
        height=512,             # Smaller resolution to save memory
        width=512,
    ).images[0]
    
    img.save(f"lcm_lora_{i}.png")
    print(f"Saved lcm_lora_{i}.png")

print(f"Generated {num_images} images with LCM LoRA and pixel art style.")

print(f"Generated {num_images} images with LCM LoRA and pixel art style.")