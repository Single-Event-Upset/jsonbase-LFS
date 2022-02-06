from datetime import datetime
import secrets
import requests
import math
import json 
import base64
import aiohttp
from hashlib import pbkdf2_hmac

SAFE_CHUNK_FACTOR = (5000/6300) # To make room for other meta data in payload and accout for inconsistencies 

with open('settings.json', 'r') as f:
    settings = json.load(f)
    # CHUNK_SIZE = settings['CHUNK_SIZE']
    CHUNK_SIZE = math.floor(settings['CHUNK_SIZE']*SAFE_CHUNK_FACTOR)
    BUCKET_NAME = settings['BUCKET_NAME']
    SALT = settings['SALT']

def return_hash(text, salt):
  return pbkdf2_hmac('sha256', text.encode('utf-8'), salt.encode('utf-8'), 100000).hex()

def return_addrstr(index, parent):
    parent = return_hash(parent, SALT)
    index = return_hash(str(index), SALT)
    return f'{parent}-{index}'

def return_chunk_array(bytes: bytes) -> list:
    """
    Return a list of chunks of bytes
    :param bytes: bytes
    :return: list
    """
    return [bytes[index: index+CHUNK_SIZE] for index in  range(0, len(bytes), CHUNK_SIZE)]

def rebuild_chunk_array_b85(chunks: list) -> bytes:
    """
    Rebuild a list of chunks into a bytes in base85
    :param chunks: list
    :return: bytes
    """
    return b''.join([base64.b85decode(chunk) for chunk in chunks])

async def store_chunk(session, index, data, parent):
    if isinstance(data, bytes): data = str(base64.b85encode(data))
    payload = {'d': data, 'i': int(index), 'p': str(parent)}
    url = f'https://jsonbase.com/{BUCKET_NAME}/{return_addrstr(index, parent)}'
    print('putting to ' + url)
    async with session.put(url, json=payload) as resp:
        code = resp.status
    return (index, code)

async def store_meta_data(session, parent, chunks):
    payload = {'s': len(chunks), 'upOn': datetime.now().isoformat()}
    url = f'https://jsonbase.com/{BUCKET_NAME}/{return_hash(parent, SALT)}-meta'
    print('META DATA ' + url)
    await session.put(url, json=payload)

async def store_chunk_array(chunks: list, parent):
    try:
        async with aiohttp.ClientSession() as session:
            tasks = [store_chunk(session, index, chunk, parent) for index, chunk in enumerate(chunks)]
            tasks.append(store_meta_data(session, parent, chunks))
            result = await asyncio.gather(*tasks)
            out = list(filter(lambda x: x != None and x[1] == 200, result))
            while True:
                failed = list(filter(lambda x: x != None and x[1] != 200, result))
                if not failed: break
                print(f'failed to store {len(failed)} chunks')
                tasks = [store_chunk(session, item[0], chunks[item[0]], parent) for item in failed]
                result = await asyncio.gather(*tasks)
                out.extend(list(filter(lambda x: x[1] == 200, result)))
            return True
    except: return False

if __name__ == '__main__':
    import json
    import sys
    import asyncio
    async def main():
        chunks = return_chunk_array(open('files/file.pdf', 'rb').read())
        out = await store_chunk_array(chunks, '99259asdasd')
        # print(res)
        print(out)
    asyncio.run(main())
    # print(chunks[0] == base64.b85decode(base64.b85encode(chunks[0])))
    # b = [{'d': str(base64.b85encode(c)), 'i': 1, 'p': '5651bf702de5d674'} for c in chunks]
    # print(max([sys.getsizeof(a) for a in b]),  min([sys.getsizeof(a) for a in b]))
    # print(len(chunks), len(chunks[0]), chunks[11] == chunks[0], sys.getsizeof())
    # import requests
    # import pickle
    # payload = {'d': str(base64.b85encode(chunks[0])), 'i': 1, 'p': '5651bf702de5d674'}
    # with open('dump.bin', 'wb') as f:
    #     pickle.dump(payload, f)
    # r = requests.put('https://jsonbase.com/demo_bucket/hello', json=payload)
    # print(r.status_code)
    # print(sys.getsizeof())
    # a = {'c': b'1'*99967, 'i': 1, 'p': '5651bf702de5d674'}
    # print(a)
    # print(sys.getsizeof(a))