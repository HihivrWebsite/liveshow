from flask import Flask, jsonify, render_template_string
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
        if 'data' in data and all(key in data['data'] for key in ('live_status', 'title', 'live_time', 'online')):
            return {
                'status': '向阳Hihi开始直播了，快来快来' if data['data']['live_status'] == 1 else '向阳Hihi还没有直播哦，请看主播的动态查看何时直播，没说的话一般是晚上十点直播哦',
                'color': '#d4edda' if data['data']['live_status'] == 1 else '#f8f9fa',  # 绿色或浅灰色
                'error': False,
                'title': data['data']['title'],
                'live_time': data['data']['live_time'],
                'online': data['data']['online']
            }
        else:
            return {'status': '未知状态', 'color': '#ffffff', 'error': True}  # 默认白色
    except requests.RequestException as e:
        print(f"Error fetching live status: {e}")
        return {'status': '无法获取直播状态', 'color': '#ffcccc', 'error': True}  # 错误时红色

@app.route('/api/status')
def api_status():
    room_id = 31368680  # 替换为实际的直播间ID
    status_info = get_live_status(room_id)
    return jsonify(status_info)

@app.route('/')
def index():
    html_content = """
    <!DOCTYPE html>
    <html lang="zh-cn">
    <head>
        <meta charset="UTF-8">
        <title>向阳Hihi直播状态</title>
        <style>
            body {
                transition: background-color 0.5s;
                font-family: Arial, sans-serif;
            }
            .container {
                text-align: center;
                margin-top: 20%;
            }
            h1, p {
                text-align: center;
            }
            a {
                display: inline-block;
                margin-top: 20px;
                padding: 10px 20px;
                color: white;
                text-decoration: none;
                border-radius: 5px;
                font-weight: bold;
                transition: background-color 0.5s;
            }
            a.live {
                background-color: #FFC633; /* 按钮背景颜色 */
            }
            a.not-live {
                background-color: #ccc; /* 未直播时按钮背景颜色 */
                cursor: default;
            }
            a:hover {
                background-color: #E6B22D; /* 鼠标悬停时的颜色 */
            }
            .error {
                color: red;
                font-weight: bold;
            }
            .details {
                margin-top: 10px;
            }
        </style>
    </head>
    <body id="live-status-body">
        <div class="container">
            <h1 id="live-status-title">向阳Hihi直播状态</h1>
            <p id="live-status-text"></p>
            <div id="live-details" class="details" style="display:none;">
                <p id="live-title"></p>
                <p id="live-time"></p>
                <p id="live-online"></p>
            </div>
            <a href="https://live.bilibili.com/31368680" id="live-status-link" class="live" target="_blank">点击进入向阳Hihi直播间</a>
            <p id="admin-contact" class="error" style="display:none;">联系管理员 <a href="mailto:pma2138@outlook.com">pma2138@outlook.com</a></p>
        </div>

        <script>
            function updateLiveStatus() {
                fetch('/api/status')
                    .then(response => response.json())
                    .then(data => {
                        document.getElementById('live-status-body').style.backgroundColor = data.color;
                        document.getElementById('live-status-text').textContent = data.status;

                        const linkElement = document.getElementById('live-status-link');
                        if (!data.error && data.status === '向阳Hihi开始直播了，快来快来') {
                            linkElement.className = 'live';
                            document.getElementById('live-title').textContent = `直播间名称: ${data.title}`;

                            document.getElementById('live-details').style.display = 'block';
                            document.getElementById('admin-contact').style.display = 'none';
                        } else {
                            linkElement.className = 'not-live';
                            document.getElementById('live-details').style.display = 'none';
                            document.getElementById('admin-contact').style.display = data.error ? 'block' : 'none';
                        }
                    })
                    .catch(error => console.error('Error updating live status:', error));
            }

            // 初次加载页面时更新状态
            window.onload = updateLiveStatus;

            // 每隔一分钟更新一次直播状态
            setInterval(updateLiveStatus, 60000);
        </script>
    </body>
    </html>
    """
    return render_template_string(html_content)

if __name__ == "__main__":
    app.run(port=31368)  # 使用Flask内置的服务器运行应用
                              # document.getElementById('live-online').textContent = `当前同接: ${data.online}`;