import json

def clean_cook_items():
    """Sort and format cook_items.json"""
    with open("src/cook_items.json", "r", encoding="utf-8") as f:
        items = json.load(f)

    with open("src/cook_items.json", "w", encoding="utf-8") as f:
        f.write("{\n")
        chunks = []
        for actor in sorted(items):
            item_str = fmt_item(items[actor])
            chunks.append(f"  \"{actor}\": {{\n{item_str}\n  }}")
        f.write(",\n".join(chunks))
        f.write("\n}\n")

def clean_names():
    """Sort and format names.json"""
    with open("src/names.json", "r", encoding="utf-8") as f:
        names = json.load(f)

    with open("src/names.json", "w", encoding="utf-8") as f:
        f.write("{\n")
        lines = []
        for key in sorted(names):
            value = names[key]
            lines.append(f"  {json.dumps(key):<20}: {json.dumps(value)}")
        f.write(",\n".join(lines))
        f.write("\n}\n")

def fmt_item(item):
    lines = []
    for line in [
        fmt_item_props(item, ["name", "effect"], fmt_value),
        fmt_item_props(item, ["hp", "time", "potency"], fmt_value_short),
        fmt_item_props(item, ["hp_boost", "time_boost", "boost_success_rate"], fmt_value_short),
        fmt_item_props(item, ["sell_price", "buy_price", "cook_low_price"], fmt_value_short),
        fmt_item_props(item, ["roast_item", "key_item"], fmt_value_short),
        fmt_item_props(item, ["tags"], fmt_value),
    ]:
        if line:
            lines.append(line)
    # dump remaining properties
    for key in item:
        print(key)
        value_str = json.dumps(item[key])
        lines.append(f"    \"{key}\": {value_str},")
    # strip trailing comma
    if lines:
        if lines[-1].endswith(","):
            lines[-1] = lines[-1][:-1]
    return "\n".join(lines)

def fmt_item_props(item, props, fmt_fn):
    """Format the given properties of the item to a single-line string, and remove those properties from the item."""
    output = []
    for prop in props:
        if prop in item:
            output.append(fmt_fn(item, prop))
            del item[prop]
    output = ", ".join(output)
    if not output:
        return None
    return f"    {output},"

def fmt_value(item, prop):
    k = json.dumps(prop) + ":"
    v = json.dumps(item[prop])
    return f"{k:<15}{v:<27}"

def fmt_value_short(item, prop):
    k = json.dumps(prop) + ":"
    v = json.dumps(item[prop])
    return f"{k:<15}{v:<5}"


if __name__ == "__main__":
    clean_cook_items()
    clean_names()
