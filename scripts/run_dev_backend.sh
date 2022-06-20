#!/bin/bash

export ALLOWED_URLS=http://localhost
export JWT_PRIVATE_KEY='-----BEGIN PRIVATE KEY-----\nMIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQglBnO+qn+RecAQ31T\njBklNu+AwiFN5eVHBFbnjecmMryhRANCAARGpVef6j7rMQ6lYSwbDkKwH7B3zM6P\nG7S4BIamIY/7Bh9xzW6fIzFxK1sPNSNG45tjwNqVoIn38npSuRCRkG1n\n-----END PRIVATE KEY-----'

cargo watch -x run
