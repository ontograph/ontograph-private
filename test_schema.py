import json

def is_valid_json_schema(schema_val):
    if not isinstance(schema_val, dict):
        return False
    if "type" not in schema_val:
        return False
    # simplistic validation for now
    return True
