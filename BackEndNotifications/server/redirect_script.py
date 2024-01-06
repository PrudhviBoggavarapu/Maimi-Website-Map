def request(flow):
    if flow.request.host == "localhost" and flow.request.port == 8080:
        flow.request.port = 8090
