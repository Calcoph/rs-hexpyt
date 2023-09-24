from rs_hexpyt import translate_file

import os

class Enum:
    pass

class Environment(Enum):
    INNER = 0
    OUTER = 1

try:
    pattern_inputs = os.listdir("pattern_inputs")
    env = Environment.INNER
except:
    pattern_inputs = os.listdir("py_test/pattern_inputs")
    env = Environment.OUTER

if env == Environment.INNER:
    in_path = lambda x: f"pattern_inputs/{x}"
    out_path = lambda x: f"pattern_outputs/{x}"
    out_dir = "pattern_outputs"
if env == Environment.OUTER:
    in_path = lambda x: f"py_test/pattern_inputs/{x}"
    out_path = lambda x: f"py_test/pattern_outputs/{x}"
    out_dir = "py_test/pattern_outputs"

if not os.path.exists(out_dir):
    if env == Environment.INNER:
        os.mkdir("pattern_outputs")
    if env == Environment.OUTER:
        os.mkdir("py_test/pattern_outputs")

for pattern_input in pattern_inputs:
    pattern_output = pattern_input.split(".")[0] + ".py"
    translate_file(in_path(pattern_input), out_path(pattern_output))
