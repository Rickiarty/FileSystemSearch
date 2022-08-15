#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
 * 
 *  Coded by Rei-Chi Lin 
 * 
"""

import os
import sys
from fssearch.search import search_under_path, version

def main():
    if len(sys.argv) > 2:
        print("\nERROR: Invalid argument(s) !\n\nType either '--help' or '-h' for more info.\n")
        return -6
    elif len(sys.argv) < 2:
        print("\nERROR: Invalid argument(s) !\n\nType either '--help' or '-h' for more info.\n")
        return -6
    else:
        if sys.argv[1] == '--version':
            print('\n\tversion: ' + version() + '\n\t\tby Rei-Chi Lin\n')
            return 2
        elif sys.argv[1] == '--help' or sys.argv[1] == '-h':
            help_info = "\nusage:\n\n"
            help_info += "\t Give this program a regular expression to search under current path. \n"
            print(help_info)
            return 1
        else:
            regular_expression = sys.argv[1]
            _status_code = search_under_path(regular_expression, os.getcwd())
            return _status_code

if __name__ == '__main__':
    exit_code = main()
    print('\n(exit code: ' + str(exit_code) + ' )')
