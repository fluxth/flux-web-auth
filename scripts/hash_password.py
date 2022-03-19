#!/usr/bin/env python3
import sys
from argon2 import PasswordHasher

hasher = PasswordHasher(
    time_cost = 3,
    memory_cost = 4096,
    parallelism = 4,
)

input = sys.stdin.read()
print(hasher.hash(input))
