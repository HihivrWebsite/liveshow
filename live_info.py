from flask import Flask, render_template_string
import requests

app = Flask(__name__)

# 直播间基本信息提取函数
def get_live_status(room_id):
    url = 'https://api.live.bilibili.com/room/v1/Room/get_info'
    params = {
        'room_id': room_id
    }
    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36',
        'Accept': 'application/json',  # 修改为json
        'Referer': 'https://www.bilibili.com/',
        'Origin': 'https://www.bilibili.com'
        # 如果API调用不需要认证，可以删除'Cookie'字段
    }

    try:
        response = requests.get(url=url, headers=headers, params=params)
        response.raise_for_status()
        data = response.json()
        if 'data' in data and 'live_status' in data['data']:
            return {
                'status': '直播中' if data['data']['live_status'] == 1 else '未直播',
                'color': '#d4edda' if data['data']['live_status'] == 1 else '#f8f9fa',  # 绿色或浅灰色
                'error': False
            }
        else:
            return {'status': '未知状态', 'color': '#ffffff', 'error': True}  # 默认白色
    except requests.RequestException as e:
        print(f"Error fetching live status: {e}")
        return {'status': '无法获取直播状态', 'color': '#ffcccc', 'error': True}  # 错误时红色

@app.route('/')
def index():
    room_id = 31368680  # 替换为实际的直播间ID
    status_info = get_live_status(room_id)
    
    html_content = f"""
    <!DOCTYPE html>
    <html lang="zh-cn">
    <head>
        <meta charset="UTF-8">
        <title>直播状态</title>
        <style>
            body {{
                background-color: {status_info['color']};
                transition: background-color 0.5s;
                font-family: Arial, sans-serif;
            }}
            .container {{
                text-align: center;
                margin-top: 20%;
            }}
            h1, p {{
                text-align: center;
            }}
            a {{
                display: inline-block;
                margin-top: 20px;
                padding: 10px 20px;
                background-color: #{'FFC633' if not status_info['error'] else 'transparent'}; /* 按钮背景颜色 */
                color: white;
                text-decoration: none;
                border-radius: 5px;
                font-weight: bold;
            }}
            a:hover {{
                background-color: #{'E6B22D' if not status_info['error'] else 'transparent'}; /* 鼠标悬停时的颜色 */
            }}
            .error {{
                color: red;
                font-weight: bold;
            }}
        </style>
    </head>
    <body>
        <div class="container">
            <h1>直播状态</h1>
            <p>{status_info['status']}</p>
            {'<a href="https://live.bilibili.com/31368680">点击进入向阳Hihi直播间</a>' if not status_info['error'] else ''}
            {'<p class="error">联系管理员 <a href="mailto:pma2138@outlook.com">pma2138@outlook.com</a></p>' if status_info['error'] else ''}
        </div>
    </body>
    </html>
    """
    return render_template_string(html_content)

if __name__ == "__main__":
    app.run(port=31368)  # 使用Flask内置的服务器运行应用