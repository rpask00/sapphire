import requests
import time

HEADER = '\033[95m'
OKBLUE = '\033[94m'
OKCYAN = '\033[96m'
OKGREEN = '\033[92m'
WARNING = '\033[93m'
FAIL = '\033[91m'
ENDC = '\033[0m'
BOLD = '\033[1m'
UNDERLINE = '\033[4m'


def purple(string):
    return f'{HEADER}{str(string)}{ENDC}'


def blue(string):
    return f'{OKBLUE}{str(string)}{ENDC}'


def cyan(string):
    return f'{OKCYAN}{str(string)}{ENDC}'


def link(uri, label=None):
    if label is None:
        label = uri
    parameters = ''

    # OSC 8 ; params ; URI ST <name> OSC 8 ;; ST
    escape_mask = '\033]8;{};{}\033\\{}\033]8;;\033\\'

    return escape_mask.format(parameters, uri, label)


def green(string):
    return f'{OKGREEN}{str(string)}{ENDC}'


def yellow(string):
    return f'{WARNING}{str(string)}{ENDC}'


def red(string):
    return f'{FAIL}{str(string)}{ENDC}'


def bold(string):
    return f'{BOLD}{str(string)}{ENDC}'


def underline(string):
    return f'{UNDERLINE}{str(string)}{ENDC}'


def r(number):
    return round(number, 2)


def add_padding(text, max_length, add_after=False):
    text = str(text)
    if len(text) > max_length:
        return text

    padding = ' ' * (max_length - len(text))
    return text + padding if add_after else padding + text


def sleep(seconds, silent=False):
    if silent:
        time.sleep(seconds)
    else:
        print('sleep', end='', flush=True)
        for i in range(seconds):
            print('.', end='', flush=True)
            time.sleep(1)

        print()


def realTimeCurrencyExchangeRate(from_currency, to_currency, api_key):
    base_url = r"https://www.alphavantage.co/query?function=CURRENCY_EXCHANGE_RATE"
    main_url = base_url + "&from_currency=" + from_currency + "&to_currency=" + to_currency + "&apikey=" + api_key
    req_ob = requests.get(main_url)
    result = req_ob.json()
    return float(result['Realtime Currency Exchange Rate']['5. Exchange Rate'])
