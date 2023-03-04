import concurrent.futures
import json
from urllib.parse import quote

import requests

from functions_uitlls import red

proxies = []

item = "★ StatTrak™ Gut Knife | Doppler (Factory New)"

headers = {
    'Accept': 'text/javascript, text/html, application/xml, text/xml, */*',
    'Accept-Language': 'pl-PL,pl;q=0.5',
    'Connection': 'keep-alive',
    # 'Cookie': 'sessionid=2743efb45b5eabbf81ea92d9; timezoneOffset=3600,0; steamCountry=DE%7C710a14a608e46764f27c0d683c83e935',
    'Referer': f'https://steamcommunity.com/market/listings/730/{quote(item)}',
    'Sec-Fetch-Dest': 'empty',
    'Sec-Fetch-Mode': 'cors',
    'Sec-Fetch-Site': 'same-origin',
    'Sec-GPC': '1',
    'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',
    'X-Prototype-Version': '1.7',
    'X-Requested-With': 'XMLHttpRequest',
    'sec-ch-ua': '"Chromium";v="110", "Not A(Brand";v="24", "Brave";v="110"',
    'sec-ch-ua-mobile': '?0',
    'sec-ch-ua-platform': '"Linux"',
}

params = {
    'query': '',
    'start': '0',
    # 'count': str(random.randint(5, 10)),
    'count': str(10),
    'country': 'PL',
    'language': 'english',
    'currency': '6',
}

for i in range(20000, 20501):
    # 69.30.227.194:20001:rp.261000.gmail.com:rzzrhk
    # proxies.append(f"http://69.30.227.194:{i}:rp.261000.gmail.com:rzzrhk")
    proxies.append(f"http://rp.261000.gmail.com:rzzrhk@69.30.227.194:{i}")

correct_proxy = json.load(open("correct_proxy.json", "r"))


def check_proxy(proxy):
    global correct_proxy

    if proxy in correct_proxy:
        return

    while True:
        try:
            response = requests.get(
                f'https://steamcommunity.com/market/listings/730/{quote(item)}/render/',
                params=params,
                proxies={
                    "http": f"{proxy}",
                    "https": f"{proxy}",

                },
                # proxies=PROXYFUEL,
                # cookies=cookies,
                headers=headers,
                timeout=30,
            )

            if response.status_code == 200:
                correct_proxy.append(proxy)

            print(red(response.status_code) if response.status_code != 200 else response.status_code)


        except Exception as e:
            # print(red(f"Zapytanie {proxy} - Błąd: {e}"))
            pass
        return


with concurrent.futures.ThreadPoolExecutor() as executor:
    executor.map(check_proxy, proxies)

json.dump(correct_proxy, open("correct_proxy.json", "w"))
