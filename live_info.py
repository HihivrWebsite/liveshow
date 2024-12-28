import requests
import re
import socket
import time

# 直播间基本信息提取
def livedata():
    room_id = 31368680
    url = 'https://api.live.bilibili.com/room/v1/Room/get_info'
    params = {
        'room_id': room_id
    }
    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36',
        'Accept': 'application/xml',
        'Referer': 'https://www.bilibili.com/',
        'Origin': 'https://www.bilibili.com',
        'Cookie': 'your_cookie_here',
    }

    while True:
        response = requests.get(url=url, headers=headers, params=params)
        response.encoding = response.apparent_encoding
        data = response.json()
        room_data = data['data']
        
        live_status = '直播中' if room_data['live_status'] == 1 else '未直播'

        # 通过端口 31368 发送 HTML 界面
        html_content = f"""
        <!DOCTYPE html>
        <html lang="zh-cn">
        <head>
            <meta charset="UTF-8">
            <title>直播状态</title>
        </head>
        <body>
            <h1>直播状态</h1>
            <p>{live_status}</p>
        </body>
        </html>
        """
        
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.bind(('localhost', 31368))
            s.listen()
            conn, addr = s.accept()
            with conn:
                print('Connected by', addr)
                response = f"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {len(html_content)}\r\n\r\n{html_content}"
                conn.sendall(response.encode())
        time.sleep(60)  # 每分钟检测一次

if __name__ == "__main__":
    livedata()
