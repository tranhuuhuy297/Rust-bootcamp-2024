#!/bin/python3

import math
import os
import random
import re
import sys

import requests


def ipTracker(ip):
    url = f"https://jsonmock.hackerrank.com/api/ip?ip={ip}"

    response = requests.get(url)

    if response.status_code == 200:
        data = response.json()

        if data['total'] == 0:
            return 'No Result Found'

        country = data['data'][0]['country']
        return country
    else:
        return 'API request failed'


if __name__ == '__main__':
    fptr = open(os.environ['OUTPUT_PATH'], 'w')

    ip = input()

    result = ipTracker(ip)

    fptr.write(str(result) + '\n')

    fptr.close()
