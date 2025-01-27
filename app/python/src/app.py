import time
from flask import Flask, request, jsonify, g
from werkzeug.middleware.dispatcher import DispatcherMiddleware
from werkzeug.wrappers import Response

app = Flask(__name__)

@app.before_request
def before_request():
    g.lambda_start_time = time.time()
    g.endpoint_start_time = time.time()

@app.after_request
def after_request(response):
    lambda_end_time = time.time()
    endpoint_end_time = time.time()

    # Calcula os tempos de execução
    lambda_duration = lambda_end_time - g.lambda_start_time
    endpoint_duration = endpoint_end_time - g.endpoint_start_time

    # Adiciona os tempos de execução no cabeçalho da resposta
    response.headers['X-Lambda-Start-Time'] = str(g.lambda_start_time)
    response.headers['X-Lambda-End-Time'] = str(lambda_end_time)
    response.headers['X-Lambda-Duration'] = str(lambda_duration)
    response.headers['X-Endpoint-Start-Time'] = str(g.endpoint_start_time)
    response.headers['X-Endpoint-End-Time'] = str(endpoint_end_time)
    response.headers['X-Endpoint-Duration'] = str(endpoint_duration)

    return response

@app.route('/math', methods=['POST'])
def math_operations():
    import numpy as np

    data = request.get_json()
    numbers = data.get('numbers', [])
    operation = data.get('operation', 'sum')

    if not numbers:
        return jsonify({'error': 'No numbers provided'}), 400

    np_array = np.array(numbers)

    if operation == 'sum':
        result = int(np.sum(np_array))  # Converte para tipo Python nativo
    elif operation == 'product':
        result = int(np.prod(np_array))  # Converte para tipo Python nativo
    else:
        return jsonify({'error': 'Unsupported operation'}), 400

    return jsonify({'result': result})

@app.route('/json', methods=['POST'])
def json_manipulation():
    import json

    data = request.get_json()
    key = data.get('key')
    value = data.get('value')

    if not key or not value:
        return jsonify({'error': 'Key and value are required'}), 400

    json_data = json.dumps({key: value})
    return jsonify({'json_data': json_data})

@app.route('/string', methods=['POST'])
def string_processing():
    import re

    data = request.get_json()
    text = data.get('text', '')
    pattern = data.get('pattern', '')

    if not text or not pattern:
        return jsonify({'error': 'Text and pattern are required'}), 400

    matches = re.findall(pattern, text)
    return jsonify({'matches': matches})

@app.route('/compress', methods=['POST'])
def compress_data():
    import gzip
    import io

    data = request.get_json()
    text = data.get('text', '')

    if not text:
        return jsonify({'error': 'Text is required'}), 400

    buf = io.BytesIO()
    with gzip.GzipFile(fileobj=buf, mode='wb') as f:
        f.write(text.encode('utf-8'))

    compressed_data = buf.getvalue()
    return Response(compressed_data, content_type='application/gzip')

@app.route('/image', methods=['POST'])
def image_processing():
    from PIL import Image, ImageDraw
    import io
    import base64

    data = request.get_json()
    text = data.get('text', 'Hello, World!')

    image = Image.new('RGB', (200, 100), color=(73, 109, 137))
    draw = ImageDraw.Draw(image)
    draw.text((10, 40), text, fill=(255, 255, 0))

    buf = io.BytesIO()
    image.save(buf, format='PNG')
    byte_im = buf.getvalue()
    encoded_image = base64.b64encode(byte_im).decode('utf-8')

    return jsonify({'image': encoded_image})

def lambda_handler(event, context):
    dispatcher = DispatcherMiddleware(app)
    return dispatcher(event, context)

if __name__ == '__main__':
    app.run(debug=True)