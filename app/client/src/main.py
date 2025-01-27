import requests
import time
from tabulate import tabulate

# Defina aqui as URLs base das três aplicações
APPS = [
    "http://localhost:5000",   # Exemplo de App 1
    "http://localhost:5001",   # Exemplo de App 2
    "http://localhost:5002",   # Exemplo de App 3
]

# Defina os endpoints que deseja testar e as respectivas requests que serão enviadas
# A chave "data" é o payload que será enviado via método POST.
ENDPOINTS = [
    {
        "name": "/math (sum)",
        "endpoint": "/math",
        "method": "POST",
        "data": {
            "numbers": [1, 2, 3, 4, 5],
            "operation": "sum"
        }
    },
    {
        "name": "/math (product)",
        "endpoint": "/math",
        "method": "POST",
        "data": {
            "numbers": [1, 2, 3, 4, 5],
            "operation": "product"
        }
    },
    {
        "name": "/json",
        "endpoint": "/json",
        "method": "POST",
        "data": {
            "key": "username",
            "value": "john_doe"
        }
    },
    {
        "name": "/string",
        "endpoint": "/string",
        "method": "POST",
        "data": {
            "text": "Hello, this is a test string",
            "pattern": "t"
        }
    },
    {
        "name": "/compress",
        "endpoint": "/compress",
        "method": "POST",
        "data": {
            "text": "Compress this data please!"
        }
    },
    {
        "name": "/image",
        "endpoint": "/image",
        "method": "POST",
        "data": {
            "text": "Imagem de teste"
        }
    },
]

def call_endpoint(base_url, endpoint_info):
    """
    Chama um endpoint específico de uma das aplicações e retorna as métricas de tempo.

    Retorna um dicionário com:
    - endpoint: nome do endpoint
    - client_start_time: marca de tempo antes da requisição
    - client_end_time: marca de tempo depois da requisição
    - client_duration: tempo de duração entre client_start_time e client_end_time
    - lambda_start_time: recuperado do cabeçalho X-Lambda-Start-Time (se disponível)
    - lambda_end_time: recuperado do cabeçalho X-Lambda-End-Time (se disponível)
    - lambda_duration: recuperado do cabeçalho X-Lambda-Duration (se disponível)
    - endpoint_start_time: recuperado do cabeçalho X-Endpoint-Start-Time (se disponível)
    - endpoint_end_time: recuperado do cabeçalho X-Endpoint-End-Time (se disponível)
    - endpoint_duration: recuperado do cabeçalho X-Endpoint-Duration (se disponível)
    - status_code: código HTTP retornado
    """

    url = base_url + endpoint_info["endpoint"]
    method = endpoint_info["method"]
    payload = endpoint_info.get("data", {})

    # Marca o início (lado do cliente)
    client_start_time = time.time()

    # Realiza a requisição (aqui somente POST; mas se quiser GET ou PUT, você pode adaptar)
    if method.upper() == "POST":
        response = requests.post(url, json=payload)
    else:
        # Você pode estender para outros métodos se necessário
        raise ValueError(f"Método {method} não suportado neste exemplo.")

    # Marca o fim (lado do cliente)
    client_end_time = time.time()

    # Calcula a duração total do lado do cliente
    client_duration = client_end_time - client_start_time

    # Coleta as informações dos cabeçalhos
    headers = response.headers
    lambda_start_time = headers.get("X-Lambda-Start-Time", None)
    lambda_end_time = headers.get("X-Lambda-End-Time", None)
    lambda_duration = headers.get("X-Lambda-Duration", None)
    endpoint_start_time = headers.get("X-Endpoint-Start-Time", None)
    endpoint_end_time = headers.get("X-Endpoint-End-Time", None)
    endpoint_duration = headers.get("X-Endpoint-Duration", None)

    return {
        "endpoint": endpoint_info["name"],
        "client_start_time": client_start_time,
        "client_end_time": client_end_time,
        "client_duration": client_duration,
        "lambda_start_time": lambda_start_time,
        "lambda_end_time": lambda_end_time,
        "lambda_duration": lambda_duration,
        "endpoint_start_time": endpoint_start_time,
        "endpoint_end_time": endpoint_end_time,
        "endpoint_duration": endpoint_duration,
        "status_code": response.status_code,
    }

def main():
    """
    Roda os testes em cada aplicação, chamando cada endpoint em sequência,
    e consolida os resultados em uma tabela impressa no console.
    """
    results = []

    for app_url in APPS:
        for endpoint_info in ENDPOINTS:
            res = call_endpoint(app_url, endpoint_info)
            results.append({
                "Application": app_url,
                "Endpoint": res["endpoint"],
                "Status": res["status_code"],
                "Client Duration (s)": f"{res['client_duration']:.6f}",
                "Lambda Duration (s)": res["lambda_duration"] or "N/A",
                "Endpoint Duration (s)": res["endpoint_duration"] or "N/A"
            })
            # Aqui, se quiser debug, pode imprimir o response.json() também
            # print("Response JSON:", response.json())

    # Exibir de forma tabulada
    # A biblioteca `tabulate` pode ser instalada com: pip install tabulate
    print(tabulate(results, headers="keys", tablefmt="fancy_grid"))

if __name__ == "__main__":
    main()
