import json
from copy import deepcopy

phase_list = json.loads(open('doppler_phases.json').read())

calculated_phase_keys = {}


def calculate():
    for knife_name, phases in phase_list.items():
        if "StatTrak" in knife_name or "Minimal Wear" in knife_name or not phases:
            continue

        mw_name = knife_name.replace("Factory New", "Minimal Wear")
        st_fn = knife_name.replace("★", "★ StatTrak™")
        st_mw = mw_name.replace("★", "★ StatTrak™")

        calculated_phase_keys[knife_name] = deepcopy(phases)
        calculated_phase_keys[mw_name] = {}
        calculated_phase_keys[st_fn] = {}
        calculated_phase_keys[st_mw] = {}

        for phase_key, phase in phases.items():
            if "https" in phase_key:
                del calculated_phase_keys[knife_name][phase_key]
                phase_key = phase_key.split('/')[-2]
                calculated_phase_keys[knife_name][phase_key] = phase

            calculated_phase_keys[st_fn][phase_key] = phase
            calculated_phase_keys[st_mw][phase_key] = phase
            calculated_phase_keys[mw_name][phase_key] = phase

    json.dump(calculated_phase_keys, open('calculated_phase_keys.json', 'w'), indent=4)


calculate()
