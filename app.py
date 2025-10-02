from flask import Flask, render_template, request
import requests
from decimal import Decimal, getcontext
from datetime import datetime
from requests.adapters import Retry
import asyncio
import aiohttp
from cachetools import TTLCache
import logging

app = Flask(__name__)

# 设置Decimal精度环境
getcontext().prec = 10  # 设置10位运算精度

# 增加重试机制以减少超时错误
retries = Retry(
    total=3,  # 总重试次数
    backoff_factor=1,  # 重试间隔时间指数增长
    status_forcelist=[500, 502, 503, 504],  # 针对这些状态码重试
    allowed_methods=["GET"]  # 仅对GET请求启用重试
)
adapter = requests.adapters.HTTPAdapter(max_retries=retries)

# 优化网络性能：启用全局会话和连接池
session = requests.Session()
adapter = requests.adapters.HTTPAdapter(pool_connections=20, pool_maxsize=20, max_retries=retries)
session.mount('https://', adapter)
session.mount('http://', adapter)

# 将 session 作为全局变量，避免每次请求都重新创建
app.config['SESSION'] = session

# 优化：创建全局 aiohttp.ClientSession 实例并复用
class AiohttpClient:
    session = None

    @classmethod
    async def get_session(cls):
        if cls.session is None or cls.session.closed:
            cls.session = aiohttp.ClientSession()
        return cls.session

# 修改 fetch_data 函数以复用全局会话
async def fetch_data(api_url):
    session = await AiohttpClient.get_session()
    try:
        async with session.get(api_url, timeout=10) as response:
            response.raise_for_status()
            return await response.json()
    except aiohttp.ClientError as e:
        app.logger.error(f"API请求失败: {str(e)}")
        raise
    except asyncio.TimeoutError:
        app.logger.error("API请求超时")
        raise
    except Exception as e:
        app.logger.error(f"未知错误: {str(e)}")
        raise

# 添加缓存策略
cache = TTLCache(maxsize=100, ttl=300)  # 缓存最多100个条目，有效期300秒

# 修改 fetch_data_with_cache 以支持分页缓存
async def fetch_data_with_cache(api_url, page=None, per_page=None):
    cache_key = f"{api_url}?page={page}&per_page={per_page}" if page and per_page else api_url
    if cache_key in cache:
        return cache[cache_key]
    data = await fetch_data(api_url)
    if page and per_page:
        start = (page - 1) * per_page
        end = start + per_page
        paginated_data = data[start:end]
        cache[cache_key] = paginated_data
        return paginated_data
    cache[cache_key] = data
    return data

@app.route('/')
async def index():
    # 获取过滤参数
    filter_type = request.args.get('filter', 'all')  # all, vr, psp
    
    # 爬取API数据
    vr_api_url = "https://vr.qianqiuzy.cn/gift"
    psp_api_url = "https://psp.qianqiuzy.cn/gift"
    
    all_data = []
    
    try:
        # 获取VR数据
        if filter_type in ['all', 'vr']:
            vr_data = await fetch_data_with_cache(vr_api_url)
            for item in vr_data:
                item['union'] = 'VirtuaReal'  # 添加工会字段
            all_data.extend(vr_data)
    except (aiohttp.ClientError, asyncio.TimeoutError) as e:
        app.logger.error(f"VR API请求失败: {str(e)}")
        if filter_type == 'vr':
            return render_template('error.html', error_message="无法获取VR主播数据"), 500
    
    try:
        # 获取PSP数据
        if filter_type in ['all', 'psp']:
            psp_data = await fetch_data_with_cache(psp_api_url)
            for item in psp_data:
                item['union'] = 'PSPlive'  # 添加工会字段
            all_data.extend(psp_data)
    except (aiohttp.ClientError, asyncio.TimeoutError) as e:
        app.logger.error(f"PSP API请求失败: {str(e)}")
        if filter_type == 'psp':
            return render_template('error.html', error_message="无法获取PSP主播数据"), 500
    
    if not all_data:
        return render_template('error.html', error_message="无法获取主播数据"), 500

    # 计算总营收并排序
    for item in all_data:
        try:
            gift = Decimal(str(item.get('gift', 0)))
            guard = Decimal(str(item.get('guard', 0)))
            super_chat = Decimal(str(item.get('super_chat', 0)))
            item['total_revenue'] = gift + guard + super_chat
        except (ValueError, TypeError) as e:
            app.logger.error(f"数据处理错误: {item}, 错误: {str(e)}")
            item['total_revenue'] = Decimal(0)

    # 排序数据
    sorted_data = sorted(
        all_data,
        key=lambda x: x['total_revenue'],
        reverse=True
    )

    # 设置标题
    if filter_type == 'vr':
        title = "维阿斗虫榜"
    elif filter_type == 'psp':
        title = "PSPlive斗虫榜"
    else:
        title = "维阿PSP斗虫榜"

    # 将排序后的数据传递给前端
    refresh_time = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
    return render_template('index.html', anchors=sorted_data, title=title, refresh_time=refresh_time, current_filter=filter_type)

@app.teardown_appcontext
async def close_aiohttp_session(exception=None):
    if AiohttpClient.session and not AiohttpClient.session.closed:
        await AiohttpClient.session.close()

# 确保按照总营收排序绝对正确
# 设置日志级别为DEBUG
logging.basicConfig(level=logging.DEBUG)

# 添加详细调试日志，检查 total_revenue 的值和类型
@app.route('/by_month')
async def by_month():
    # 获取年月参数和过滤参数
    month = request.args.get('month', datetime.now().strftime('%Y%m'))
    filter_type = request.args.get('filter', 'all')  # all, vr, psp
    
    all_data = []
    
    try:
        # 获取VR数据
        if filter_type in ['all', 'vr']:
            vr_api_url = f"https://vr.qianqiuzy.cn/gift/by_month?month={month}"
            vr_data = await fetch_data(vr_api_url)
            for item in vr_data:
                item['union'] = 'VirtuaReal'  # 添加工会字段
            all_data.extend(vr_data)
    except (aiohttp.ClientError, asyncio.TimeoutError) as e:
        app.logger.error(f"VR API请求失败: {str(e)}")
        if filter_type == 'vr':
            return render_template('error.html', error_message="无法获取VR主播数据"), 500
    
    try:
        # 获取PSP数据
        if filter_type in ['all', 'psp']:
            psp_api_url = f"https://psp.qianqiuzy.cn/gift/by_month?month={month}"
            psp_data = await fetch_data(psp_api_url)
            for item in psp_data:
                item['union'] = 'PSPlive'  # 添加工会字段
            all_data.extend(psp_data)
    except (aiohttp.ClientError, asyncio.TimeoutError) as e:
        app.logger.error(f"PSP API请求失败: {str(e)}")
        if filter_type == 'psp':
            return render_template('error.html', error_message="无法获取PSP主播数据"), 500
    
    if not all_data:
        return render_template('error.html', error_message="无法获取主播数据"), 500

    for item in all_data:
        try:
            # 确保所有字段都能正确转换为 Decimal 类型
            gift = Decimal(str(item.get('gift', 0)))
            guard = Decimal(str(item.get('guard', 0)))
            super_chat = Decimal(str(item.get('super_chat', 0)))
            item['total_revenue'] = gift + guard + super_chat
            # 打印每个条目的 total_revenue 值和类型
            logging.debug(f"Item: {item}, Total Revenue: {item['total_revenue']}, Type: {type(item['total_revenue'])}")
        except (ValueError, TypeError) as e:
            app.logger.error(f"数据处理错误: {item}, 错误: {str(e)}")
            item['total_revenue'] = Decimal(0)

    # 调试：打印排序前的数据到终端
    logging.debug(f"排序前的数据: {all_data}")

    # 确保排序时使用 Decimal 类型
    sorted_data = sorted(
        all_data,
        key=lambda x: x['total_revenue'],
        reverse=True
    )

    # 调试：打印排序后的数据到终端
    logging.debug(f"排序后的数据: {sorted_data}")

    # 设置标题
    if filter_type == 'vr':
        title = f"维阿斗虫榜_{month[:4]}年{int(month[4:]):02d}月记录数据（点击“正在直播”跳转到对应直播间）"
    elif filter_type == 'psp':
        title = f"PSPlive斗虫榜_{month[:4]}年{int(month[4:]):02d}月记录数据（点击“正在直播”跳转到对应直播间）"
    else:
        title = f"维阿PSP斗虫榜_{month[:4]}年{int(month[4:]):02d}月记录数据（点击“正在直播”跳转到对应直播间）"

    return render_template('index.html', anchors=sorted_data, title=title, refresh_time=datetime.now().strftime('%Y-%m-%d %H:%M:%S'), current_filter=filter_type)

# 替换 before_first_request，确保事件循环在应用启动时正确设置
@app.before_request
def ensure_event_loop():
    try:
        asyncio.get_running_loop()
    except RuntimeError:
        asyncio.set_event_loop(asyncio.new_event_loop())

@app.route('/live_sessions')
async def live_sessions():
    room_id = request.args.get('room_id')
    union = request.args.get('union', 'VirtuaReal')  # 默认为VirtuaReal
    month = request.args.get('month', datetime.now().strftime('%Y%m'))
    if not room_id:
        return render_template('error.html', error_message="缺少room_id参数"), 400

    # 根据工会选择API
    if union == 'PSPlive':
        base_api_url = "https://psp.qianqiuzy.cn/gift"
    else:
        base_api_url = "https://vr.qianqiuzy.cn/gift"

    # 获取主播的详细信息
    try:
        all_data = await fetch_data(base_api_url)
        # 为数据添加工会信息
        for item in all_data:
            item['union'] = union
    except (aiohttp.ClientError, asyncio.TimeoutError) as e:
        app.logger.error(f"API请求失败: {str(e)}")
        return render_template('error.html', error_message="无法获取主播数据"), 500

    # 查找对应 room_id 的主播信息
    anchor_data = next((item for item in all_data if str(item.get('room_id')) == room_id), None)
    if not anchor_data:
        return render_template('error.html', error_message="未找到对应的主播信息"), 404

    queried_user = anchor_data.get('anchor_name', '未知主播')

    # 获取直播场次数据
    api_url = f"{base_api_url}/live_sessions?room_id={room_id}&month={month}"
    try:
        session_data = await fetch_data(api_url)
    except (aiohttp.ClientError, asyncio.TimeoutError) as e:
        app.logger.error(f"API请求失败: {str(e)}")
        return render_template('error.html', error_message="无法获取直播数据"), 500

    for session in session_data.get('sessions', []):
        # 确保所有数值字段都是 float 类型，以便在模板中进行计算
        gift_val = float(session.get('gift', 0) or 0)
        guard_val = float(session.get('guard', 0) or 0)
        super_chat_val = float(session.get('super_chat', 0) or 0)
        session['gift'] = gift_val
        session['guard'] = guard_val
        session['super_chat'] = super_chat_val
        session['total_revenue'] = gift_val + guard_val + super_chat_val
        
        # 计算直播时长（分钟）
        start_time_str = session.get('start_time', '')
        end_time_str = session.get('end_time', '')
        
        duration_minutes = 0
        if start_time_str:
            try:
                start_time = datetime.strptime(start_time_str, '%Y-%m-%d %H:%M:%S')
                if end_time_str and end_time_str != '0000-00-00 00:00:00':
                    end_time = datetime.strptime(end_time_str, '%Y-%m-%d %H:%M:%S')
                    duration = end_time - start_time
                else:
                    # 如果没有结束时间，使用当前时间
                    duration = datetime.now() - start_time
                
                # 转换为分钟，四舍五入到整数
                duration_minutes = int(duration.total_seconds() / 60)
            except (ValueError, TypeError) as e:
                app.logger.error(f"时间解析错误: {start_time_str}, {end_time_str}, 错误: {str(e)}")
                duration_minutes = 0
        
        session['duration_minutes'] = duration_minutes

    return render_template(
        'live_sessions.html',
        sessions=session_data.get('sessions', []),
        room_id=room_id,
        queried_user=queried_user,
        union=union,
        title=f"{month[:4]}年{int(month[4:]):02d}月直播数据",
        refresh_time=datetime.now().strftime('%Y-%m-%d %H:%M:%S')
    )


if __name__ == '__main__':
    app.run(port=2992, debug=True)
