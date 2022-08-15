#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
 * 
 *  Coded by Rei-Chi Lin 
 * 
"""

import os
import re
if __name__ != '__main__':
    from fssearch.format_number import format_number_kilo_by_kilo

__version_code = '1.0.0' # version code

def version():
    return __version_code

def search_dir(reg_ex, parent_path) -> tuple[int, int]:
    file_count = 0
    folder_count = 0
    file_system_objects = sorted(os.listdir(parent_path))
    
    for obj in file_system_objects:
        if re.search(reg_ex, obj):
            _line = ''
            obj_path = os.path.join(parent_path, obj)
            if os.path.isdir(obj_path): # is a directory/folder
                _line += 'd  ' + obj_path
            else: # is a file
                _line += '-  ' + obj_path + ' (size = {} byte)'.format(format_number_kilo_by_kilo(int(os.path.getsize(obj_path))))
            print(_line)
            if os.path.isdir(obj_path):
                folder_count += 1
                folder_c, file_c = search_dir(obj_path)
                folder_count += folder_c
                file_count += file_c
            else:
                file_count += 1
    return folder_count, file_count

__author = 'Rei-Chi Lin'
def author():
    return __author

def search_under_path(regular_expression, dir_path_to_search) -> int:
    _status_code = 0
    abs_path = ''
    try:
        if os.path.exists(dir_path_to_search) and os.path.isdir(dir_path_to_search):
            abs_path = os.path.abspath(dir_path_to_search)
        else:
            print('\nERROR: \n\n The process can not access the given path !\n Please check whether the path exists and whether the path is to a directory/folder.\n And check if the access to the path is permitted or not.\n')
            _status_code = -2
            return _status_code
    except:
        print('\nERROR: \n\n The process can not access the path given !\n Please check whether the path exists.\n And check if the access to the path is permitted or not.\n')
        _status_code = -3
        return _status_code
    
    print('###\n\nStart path: \n ' + abs_path + '\n')
    folder_count, file_count = search_dir(regular_expression, abs_path)
    print('\n' + format_number_kilo_by_kilo(file_count) + ' file(s),')
    print(format_number_kilo_by_kilo(folder_count) + ' folder(s) found')
    
    return _status_code

if __name__ == "__main__":
    print("ver. " + version() + " by " + author())
    print("\n\nThe file 'search.py' is a program unit in module 'fssearch', and it should not be used in this way.")
    print("\n---\nAPI usage:\n")
    print("from fssearch.search import search_under_path\n")
    print("status_code = search_under_path(regular_expression, dir_path_to_search)\n")
    print("regular_expression :: str")
    print("dir_path_to_search :: str")
    print("status_code :: int\n---")
