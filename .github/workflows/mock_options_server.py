import socket
import threading
import hashlib
import base64
import os
import tempfile
import json
import time

def handle_client(conn, addr):
    try:
        # Perform WebSocket handshake
        data = conn.recv(4096)
        if not data:
            return
        
        request_text = data.decode('utf-8', errors='ignore')
        if "Upgrade: websocket" not in request_text:
            conn.close()
            return
        
        # Extract Sec-WebSocket-Key
        key = None
        for line in request_text.split("\r\n"):
            if line.startswith("Sec-WebSocket-Key:"):
                key = line.split(":")[1].strip()
                break
        
        if not key:
            conn.close()
            return
        
        # Calculate Accept Key
        accept_guid = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"
        accept_key = base64.b64encode(hashlib.sha1((key + accept_guid).encode()).digest()).decode()
        
        # Send Handshake Response
        response = (
            "HTTP/1.1 101 Switching Protocols\r\n"
            "Upgrade: websocket\r\n"
            "Connection: Upgrade\r\n"
            f"Sec-WebSocket-Accept: {accept_key}\r\n\r\n"
        )
        conn.sendall(response.encode())
        
        # Read WebSocket request frame
        frame_data = conn.recv(4096)
        if not frame_data:
            return
        
        # Decode the request frame
        request_msg = decode_frame(frame_data)
        if not request_msg:
            return
        
        print(f"Received JSON-RPC request: {request_msg}")
        request_json = json.loads(request_msg)
        req_id = request_json.get("id", 0)
        
        # Prepare response
        options_response = {
            "jsonrpc": "2.0",
            "result": {
                "dev": False,
                "features": [],
                "args": [],
                "noise_level": "Polite",
                "vars": {},
                "config": [],
                "target_device": None
            },
            "id": req_id
        }
        
        response_payload = json.dumps(options_response)
        response_frame = make_frame(response_payload)
        conn.sendall(response_frame)
        print("Sent JSON-RPC response.")
    except Exception as e:
        print(f"Error handling client: {e}")
    finally:
        conn.close()

def decode_frame(data):
    if len(data) < 6:
        return None
    second_byte = data[1]
    length = second_byte & 127
    mask_start = 2
    if length == 126:
        mask_start = 4
        length = int.from_bytes(data[2:4], byteorder='big')
    elif length == 127:
        mask_start = 10
        length = int.from_bytes(data[2:10], byteorder='big')
    
    masks = data[mask_start:mask_start+4]
    payload_start = mask_start + 4
    payload_data = data[payload_start:payload_start+length]
    
    decoded = bytearray()
    for i in range(len(payload_data)):
        decoded.append(payload_data[i] ^ masks[i % 4])
    return decoded.decode('utf-8', errors='ignore')

def make_frame(payload):
    payload_bytes = payload.encode('utf-8')
    length = len(payload_bytes)
    if length <= 125:
        header = bytes([0x81, length])
    elif length <= 65535:
        header = bytes([0x81, 126]) + length.to_bytes(2, byteorder='big')
    else:
        header = bytes([0x81, 127]) + length.to_bytes(8, byteorder='big')
    return header + payload_bytes

def main():
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    server.bind(("127.0.0.1", 0))
    server.listen(5)
    
    ip, port = server.getsockname()
    address_str = f"{ip}:{port}"
    print(f"Mock Options Server listening on: ws://{address_str}")
    
    # Write to temp file
    temp_dir = tempfile.gettempdir()
    server_addr_path = os.path.join(temp_dir, "com.wifi.login-server-addr")
    
    with open(server_addr_path, "w") as f:
        f.write(address_str)
    print(f"Wrote address to: {server_addr_path}")
    
    # Also write to /tmp just to be safe
    try:
        alternative_path = "/tmp/com.wifi.login-server-addr"
        with open(alternative_path, "w") as f:
            f.write(address_str)
        print(f"Wrote address to: {alternative_path}")
    except Exception:
        pass
        
    try:
        while True:
            conn, addr = server.accept()
            print(f"Accepted connection from: {addr}")
            t = threading.Thread(target=handle_client, args=(conn, addr))
            t.daemon = True
            t.start()
    except KeyboardInterrupt:
        print("Server shutting down.")
    finally:
        server.close()
        # Clean up files
        try:
            os.remove(server_addr_path)
        except Exception:
            pass

if __name__ == "__main__":
    main()
