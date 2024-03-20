#!/bin/python3

#
# Copy notes from SMDX files to JSON models
# See: https://github.com/sunspec/models/pull/247
#

import json
from os import listdir, chdir
from xml.etree import ElementTree as ET

chdir('models')

for fn in listdir('smdx'):
    if not (fn.startswith('smdx_') and fn.endswith('.xml')):
        continue
    model_id = int(fn[5:-4], base=10)
    dom = ET.parse(f'smdx/{fn}').getroot()
    strings = dom.find('strings')
    # model notes
    model_string = strings.find('model')
    model_notes = model_string.findtext('notes')
    if model_notes:
        model_notes = model_notes.strip()
    # point notes
    points_notes = {}
    symbols_notes = {}
    for point in strings.findall('point'):
        point_id = point.get('id')
        point_notes = point.findtext('notes')
        if point_notes:
            point_notes = point_notes.strip()
        if point_notes:
            points_notes[point_id] = point_notes
        for symbol in point.findall('symbol'):
            symbol_id = symbol.get('id')
            symbol_notes = symbol.findtext('notes')
            if symbol_notes:
                symbol_notes = symbol_notes.strip()
            if symbol_notes:
                symbols_notes[(point_id, symbol_id)] = symbol_notes
    if not (model_notes or points_notes):
        continue
    with open(f'json/model_{model_id}.json') as fh:
        jsondoc = json.load(fh)
    if not jsondoc['group'].get('notes', '') and model_notes:
        jsondoc['group']['notes'] = model_notes
    for point in jsondoc['group']['points']:
        point_name = point['name']
        point_notes = points_notes.get(point_name, '')
        if not point.get('notes', '') and point_notes:
            point['notes'] = point_notes
        for symbol in point.get('symbols', []):
            symbol_name = symbol['name']
            symbol_notes = symbols_notes.get((point_name, symbol_name), '')
            if not symbol.get('notes', '') and symbol_notes:
                symbol['notes'] = symbol_notes
    with open(f'json/model_{model_id}.json', 'w') as fh:
        json.dump(jsondoc, fh, indent=4)
