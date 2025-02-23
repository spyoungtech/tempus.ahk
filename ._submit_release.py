import os
import requests
import hashlib


virus_total_api_key = os.getenv("VIRUS_TOTAL_KEY")


url = "https://www.virustotal.com/api/v3/files"

files = {
    "file": (
        "tempus_ahk.dll",
        open("dist\\tempus_ahk.dll", "rb"),
        "application/octet-stream",
    )
}
headers = {"accept": "application/json", "x-apikey": virus_total_api_key}

response = requests.post(url, files=files, headers=headers)

response.raise_for_status()

data = response.json()
info_url = data["data"]["links"]["self"]


info_response = requests.get(info_url, headers=headers)
info_response.raise_for_status()
info_data = info_response.json()
vt_sha_256 = info_data["meta"]["file_info"]["sha256"]

print('\n')
print('### File Hashes\n\n')
for fname in reversed(os.listdir("dist")):
    fp = os.path.join("dist", fname)
    sha_256 = hashlib.sha256()
    md5 = hashlib.md5()
    with open(fp, "rb") as f:
        contents = f.read()
    sha_256.update(contents)
    md5.update(contents)
    sha256_hash = sha_256.hexdigest()
    md5_hash = md5.hexdigest()
    md5_text = fp + ".md5.txt"
    sha256_text = fp + ".sha256.txt"

    with open(md5_text, "w", encoding="utf-8") as f:
        f.write(md5_hash)

    with open(sha256_text, "w", encoding="utf-8") as f:
        f.write(sha256_hash)
    print(f'- `{fname}')
    print(f"  SHA256 digest: `{sha256_hash}`")
    print(f"  MD5 digest: `{md5_hash}`")

print('\n\n### VirusTotal')

print(f"\n\n[VirusTotal link](https://www.virustotal.com/gui/file/{vt_sha_256})")
