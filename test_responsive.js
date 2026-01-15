const puppeteer = require('puppeteer');

(async () => {
  // 启动浏览器
  const browser = await puppeteer.launch({ headless: false }); // 设为true可在无头模式下运行
  const page = await browser.newPage();

  try {
    // 测试不同的视口尺寸
    const viewports = [
      { width: 1200, height: 800, name: 'Desktop' },
      { width: 1024, height: 768, name: 'Tablet-Large' },
      { width: 768, height: 1024, name: 'Tablet-Portrait' },
      { width: 480, height: 800, name: 'Mobile' },
      { width: 360, height: 640, name: 'Small-Mobile' }
    ];

    for (const viewport of viewports) {
      console.log(`Testing ${viewport.name} (${viewport.width}x${viewport.height})`);
      
      // 设置视口
      await page.setViewport({ width: viewport.width, height: viewport.height });
      
      // 导航到前端应用
      await page.goto('http://localhost:3000', { waitUntil: 'networkidle2' });
      
      // 等待页面加载
      await page.waitForTimeout(2000);
      
      // 截图以供检查
      await page.screenshot({ 
        path: `./screenshots/${viewport.name}_${viewport.width}x${viewport.height}.png`,
        fullPage: true 
      });
      
      // 检查关键元素是否存在且可见
      const headerVisible = await page.$eval('header', el => el.offsetParent !== null);
      const tableVisible = await page.$eval('.anchor-table, .sessions-table', el => el.offsetParent !== null);
      
      console.log(`  Header visible: ${headerVisible}`);
      console.log(`  Table visible: ${tableVisible}`);
      
      // 检查是否有水平滚动条（这表明内容可能超出视口）
      const scrollWidth = await page.evaluate(() => document.body.scrollWidth);
      const clientWidth = await page.evaluate(() => document.documentElement.clientWidth);
      const hasHorizontalScroll = scrollWidth > clientWidth;
      
      console.log(`  Horizontal scroll needed: ${hasHorizontalScroll}`);
      console.log(`  Scroll width: ${scrollWidth}, Client width: ${clientWidth}`);
      
      // 检查表格容器是否有溢出
      const tableOverflow = await page.evaluate(() => {
        const tableContainer = document.querySelector('.table-container');
        if (tableContainer) {
          return {
            scrollWidth: tableContainer.scrollWidth,
            clientWidth: tableContainer.clientWidth,
            hasOverflow: tableContainer.scrollWidth > tableContainer.clientWidth
          };
        }
        return null;
      });
      
      if (tableOverflow) {
        console.log(`  Table container overflow: ${tableOverflow.hasOverflow}`);
        console.log(`  Table scroll width: ${tableOverflow.scrollWidth}, client width: ${tableOverflow.clientWidth}`);
      }
      
      console.log('---');
    }
    
    console.log('Responsive design testing completed!');
  } catch (error) {
    console.error('Error during testing:', error);
  } finally {
    await browser.close();
  }
})();