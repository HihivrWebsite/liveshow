# HihiVR 斗虫榜数据展示系统

一个基于Flask和WebAssembly的高性能斗虫榜数据展示系统，支持维阿(VirtuaReal)和PSP(PSP live)工会数据的实时展示、分析和对比。

## 主要特性

- **WASM高精度计算**: 使用WebAssembly进行高精度浮点数计算，避免JavaScript精度问题
- **客户端数据处理**: 所有计算在浏览器端完成，保护用户隐私
- **数据缓存**: 自动缓存数据5分钟，减少服务器请求，提升响应速度
- **多维度分析**: 支持营收占比分析、VR/PSP工会数据对比、直播场次趋势分析
- **响应式设计**: 适配各种设备屏幕尺寸
- **国内CDN支持**: WASM模块优先从国内CDN加载，提升加载速度

## 安装与运行

### Windows 系统

1. **安装Python** (推荐3.8+版本)
   - 访问 [Python官网](https://www.python.org/downloads/) 下载并安装
   - 安装时勾选"Add Python to PATH"选项

2. **克隆项目**
   ```cmd
   git clone https://github.com/HihivrWebsite/liveshow.git
   cd liveshow
   ```

3. **创建虚拟环境** (可选但推荐)
   ```cmd
   python -m venv venv
   venv\Scripts\activate
   ```

4. **安装依赖**
   ```cmd
   pip install -r requirements.txt
   ```

5. **运行应用**
   ```cmd
   python app.py
   ```

6. **访问应用**
   - 打开浏览器访问: http://127.0.0.1:2992

### Linux 系统

1. **安装Python** (推荐3.8+版本)
   ```bash
   # Ubuntu/Debian
   sudo apt update
   sudo apt install python3 python3-pip python3-venv
   
   # CentOS/RHEL
   sudo yum install python3 python3-pip
   ```

2. **克隆项目**
   ```bash
   git clone https://github.com/HihivrWebsite/liveshow.git
   cd liveshow
   ```

3. **创建虚拟环境** (可选但推荐)
   ```bash
   python3 -m venv venv
   source venv/bin/activate
   ```

4. **安装依赖**
   ```bash
   pip install -r requirements.txt
   ```

5. **运行应用**
   ```bash
   python3 app.py
   ```

6. **访问应用**
   - 打开浏览器访问: http://127.0.0.1:2992

## 项目结构

```
liveshow/
├── app.py                 # 主应用文件
├── readme.md             # 项目说明文档
├── static/               # 静态文件目录
│   └── wasm_calculator.js # WASM计算器模块
└── templates/            # 模板文件目录
    ├── index.html        # 主页面模板
    ├── live_sessions.html # 详细数据页面模板
    └── error.html        # 错误页面模板
```

## WASM模块说明

本项目使用WebAssembly进行高精度计算，具有以下优势：

- **高精度**: 避免JavaScript浮点数计算的精度问题
- **高性能**: WASM执行速度比纯JavaScript更快
- **自动降级**: 如果浏览器不支持WASM，自动降级到JavaScript实现
- **国内优化**: 优先从国内CDN加载WASM模块，提升加载速度

### 浏览器兼容性

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## API 端点

- `/` - 主页面，显示斗虫榜数据
- `/by_month` - 按月份查询数据
- `/live_sessions` - 查看主播详细直播数据

## 贡献

感谢千秋紫莹提供的斗虫数据API支持！

如果您发现任何问题或有改进建议，请提交Issue或Pull Request。

## 许可证

本项目基于MIT许可证发布。
