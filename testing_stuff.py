import requests

headers = {
    "User-Agent": "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0",
    "Accept": "application/json, text/plain, */*",
    "Accept-Language": "en-US,en;q=0.5",
    "Accept-Encoding": "gzip, deflate, br",
    "api-version": "2",
    "iii-customer-domain": "mdpls.na.iiivega.com",
    "iii-host-domain": "mdpls.na.iiivega.com",
    "Anonymous-User-Id": "eda0ad8a-f8d4-4b0a-a98f-643fed73d916",
    "Content-Type": "application/json",
    "Origin": "https://mdpls.na.iiivega.com",
    "Connection": "keep-alive",
    "Referer": "https://mdpls.na.iiivega.com/",
    "Sec-Fetch-Dest": "empty",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Site": "same-site",
    "Pragma": "no-cache",
    "Cache-Control": "no-cache",
    "TE": "trailers",
}

data = {
    "searchText": '"Museum pass program"',
    "sorting": "relevance",
    "sortOrder": "asc",
    "searchType": "series",
    "pageNum": 0,
    "pageSize": 40,
    "resourceType": "FormatGroup",
}

response = requests.post(
    "https://na.iiivega.com/api/search-result/search/format-groups",
    headers=headers,
    json=data,
)
print(response.text)
