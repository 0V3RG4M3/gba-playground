import os 
import toml
import openai
import dotenv
import loguru


def load_asset_description(toml_file_path):
    """Load asset description from a TOML file."""
    data = None
    with open(toml_file_path, 'r') as f:
        data =  toml.load(f)
    
    loguru.logger.info(f"Loaded asset description from {toml_file_path}")
    loguru.logger.debug(f"Asset description: {data}")
    return data
