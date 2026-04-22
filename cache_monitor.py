#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
缓存监控脚本 - 实时追踪 attention API 调用和缓存命中情况
"""

import sys
import re
from collections import defaultdict
from datetime import datetime

def parse_log_line(line):
    """解析日志行，提取关键信息"""
    
    # 匹配 /gift/attention 请求
    match = re.search(r'GET /gift/attention\?.*?room_id=(\d+).*?month=(\d+)', line)
    if match:
        return {
            'type': 'api_request',
            'room_id': match.group(1),
            'month': match.group(2),
            'timestamp': datetime.now().isoformat()
        }
    
    # 匹配缓存命中日志
    if '🎯 [缓存命中]' in line:
        match = re.search(r'key=attention_\w+_(\d+)_(\d+)', line)
        if match:
            return {
                'type': 'cache_hit',
                'room_id': match.group(1),
                'month': match.group(2),
                'timestamp': datetime.now().isoformat()
            }
    
    # 匹配缓存未命中日志
    if '📡 [缓存未命中]' in line:
        match = re.search(r'key=attention_\w+_(\d+)_(\d+)', line)
        if match:
            return {
                'type': 'cache_miss',
                'room_id': match.group(1),
                'month': match.group(2),
                'timestamp': datetime.now().isoformat()
            }
    
    # 匹配 API 调用日志
    if '🌐 [API调用]' in line:
        match = re.search(r'room_id=(\d+).*?month=(\d+)', line)
        if match:
            return {
                'type': 'external_api_call',
                'room_id': match.group(1),
                'month': match.group(2),
                'timestamp': datetime.now().isoformat()
            }
    
    return None

def main():
    """主函数 - 读取日志并统计"""
    
    stats = defaultdict(lambda: {
        'api_requests': 0,
        'cache_hits': 0,
        'cache_misses': 0,
        'external_calls': 0
    })
    
    print("=" * 80)
    print("🔍 缓存监控系统 - 实时统计")
    print("=" * 80)
    print()
    
    try:
        for line in sys.stdin:
            line = line.rstrip()
            result = parse_log_line(line)
            
            if result:
                room_id = result['room_id']
                event_type = result['type']
                
                if event_type == 'api_request':
                    stats[room_id]['api_requests'] += 1
                    print(f"📊 API 请求: room_id={room_id}, month={result['month']}")
                
                elif event_type == 'cache_hit':
                    stats[room_id]['cache_hits'] += 1
                    print(f"✅ 缓存命中: room_id={room_id}")
                
                elif event_type == 'cache_miss':
                    stats[room_id]['cache_misses'] += 1
                    print(f"⚠️ 缓存未命中: room_id={room_id}")
                
                elif event_type == 'external_api_call':
                    stats[room_id]['external_calls'] += 1
                    print(f"🌐 外部API调用: room_id={room_id}")
    
    except KeyboardInterrupt:
        print("\n\n统计总结:")
        print("=" * 80)
        
        total_requests = 0
        total_hits = 0
        total_misses = 0
        total_calls = 0
        
        for room_id in sorted(stats.keys(), key=lambda x: stats[x]['api_requests'], reverse=True):
            s = stats[room_id]
            hit_rate = (s['cache_hits'] / (s['cache_hits'] + s['cache_misses']) * 100) if (s['cache_hits'] + s['cache_misses']) > 0 else 0
            
            total_requests += s['api_requests']
            total_hits += s['cache_hits']
            total_misses += s['cache_misses']
            total_calls += s['external_calls']
            
            print(f"\n📌 room_id: {room_id}")
            print(f"   API 请求次数: {s['api_requests']}")
            print(f"   缓存命中: {s['cache_hits']}")
            print(f"   缓存未命中: {s['cache_misses']}")
            print(f"   外部API调用: {s['external_calls']}")
            print(f"   缓存命中率: {hit_rate:.1f}%")
        
        print("\n" + "=" * 80)
        print("📊 整体统计:")
        print(f"   总 API 请求: {total_requests}")
        print(f"   总缓存命中: {total_hits}")
        print(f"   总缓存未命中: {total_misses}")
        print(f"   总外部API调用: {total_calls}")
        overall_hit_rate = (total_hits / (total_hits + total_misses) * 100) if (total_hits + total_misses) > 0 else 0
        print(f"   整体缓存命中率: {overall_hit_rate:.1f}%")
        print("=" * 80)

if __name__ == '__main__':
    main()
