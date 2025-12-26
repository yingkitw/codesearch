# Dead Code Demonstration File
# This file intentionally contains dead code for testing the deadcode detection feature

import json
import os
import sys
import re
from typing import Dict, List, Optional
from datetime import datetime

# ============================================
# DEAD CODE EXAMPLES
# ============================================

# Unused constant
MAX_RETRIES = 5
DEFAULT_TIMEOUT = 30
CACHE_TTL = 3600


# Unused class
class UnusedLogger:
    """A logger class that is never used"""
    
    def __init__(self, name: str):
        self.name = name
        self.messages = []
    
    def log(self, msg: str):
        self.messages.append(f"[{self.name}] {msg}")


# Unused function
def deprecated_format_date(dt: datetime) -> str:
    """Old date formatting function - replaced by new version"""
    return dt.strftime("%Y-%m-%d")


# Another unused function
def legacy_parse_config(path: str) -> dict:
    """Legacy config parser - no longer used"""
    with open(path) as f:
        return json.load(f)


# Unused helper
def sanitize_input(text: str) -> str:
    """Input sanitization - superseded by validator"""
    return re.sub(r'[<>]', '', text)


# ============================================
# USED CODE (for comparison)
# ============================================

class Calculator:
    """A simple calculator that is actually used"""
    
    def __init__(self):
        self.history = []
    
    def add(self, a: int, b: int) -> int:
        result = a + b
        self.history.append(f"{a} + {b} = {result}")
        return result
    
    def get_history(self) -> List[str]:
        return self.history


def main():
    calc = Calculator()
    result = calc.add(5, 3)
    print(f"Result: {result}")
    
    for entry in calc.get_history():
        print(entry)


if __name__ == "__main__":
    main()

